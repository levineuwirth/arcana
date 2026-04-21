//! Combat: declare-attackers, declare-blockers, damage assignment,
//! and the damage-dealing pipeline.
//!
//! Addendum Section 10 / Phase 1 Task #16. Depends on tasks 4
//! (objects), 5 (events), 6 (state), 8 (actions), 13 (effects).
//!
//! # Model (CR 508–512)
//!
//! Combat unfolds as a fixed sequence of sub-steps inside the combat
//! phase. This module owns [`CombatState`] and the state transitions:
//!
//! ```text
//!   begin_combat                ->  PreDeclareAttackers
//!   enter_declare_attackers     ->  DeclareAttackers     (active player decides)
//!   apply_declared_attackers    ->  PostDeclareAttackers
//!   enter_declare_blockers      ->  DeclareBlockers      (defenders decide)
//!   apply_declared_blockers     ->  PostDeclareBlockers
//!   deal_combat_damage          ->  RegularDamage / EndOfCombat
//!   end_combat                  ->  (combat cleared)
//! ```
//!
//! The engine (Task #20) walks this sequence, yielding
//! [`EngineYield::PendingDecision`](crate::engine::EngineYield) at
//! DeclareAttackers and DeclareBlockers.
//!
//! # What's implemented
//!
//! - Declaration application: taps attackers (vigilance-unaware), wires
//!   `blocked_by` relationships, emits `CreatureAttacks`,
//!   `CreatureBlocks`, `CreatureBlocked` / `CreatureNotBlocked`,
//!   `AttacksDeclared`, `BlocksDeclared` events.
//! - Damage assignment: a **default distribution** that matches CR
//!   510.1c's "at least lethal damage in order" rule. An explicit
//!   [`DamageAssignment`] posted via [`GameState::set_damage_assignment`]
//!   overrides the default for that attacker.
//! - Damage dealing: unblocked → defending player (or planeswalker);
//!   blocked → distributed among blockers per assignment; blockers →
//!   attacker they block.
//! - Shared `GameState::deal_damage` primitive used by both combat
//!   and the [`crate::effects::Effect::DealDamage`] variant.
//!
//! # Deferred (keyword-dependent)
//!
//! - **First strike / double strike** (CR 702.7 / 702.4): scaffolding
//!   is in place via `deal_combat_damage(first_strike: bool)` and
//!   `CombatState.has_first_strike`, but the keyword lookup is a
//!   no-op today, so the first-strike sub-step never fires. Both
//!   sub-step methods exist for the engine to call once keywords
//!   are tracked.
//! - **Lifelink** (CR 702.15), **Deathtouch** (CR 702.2), **Trample**
//!   (CR 702.19), **Vigilance** (CR 702.20): each waits on keyword
//!   tracking. Trample in particular changes the default damage
//!   distribution (excess spills to the defender).

use serde::{Serialize, Deserialize};

use crate::effects::KeywordAbility;
use crate::events::GameEvent;
use crate::objects::ObjectId;
use crate::state::GameState;
use crate::types::*;

// DamageTarget is defined canonically in events.rs. Re-export for
// convenience so callers can `use crate::combat::DamageTarget` if they
// prefer.
pub use crate::events::DamageTarget;

// =============================================================================
// CombatState
// =============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatState {
    pub phase: CombatPhase,
    pub attackers: Vec<AttackerInfo>,
    pub blockers: Vec<BlockerInfo>,
    /// Explicit damage distributions, one per multi-blocked attacker
    /// whose controller has posted a choice. Missing entries fall
    /// back to [`default_damage_distribution`].
    pub damage_assignments: Vec<DamageAssignment>,
    pub has_first_strike: bool,
    pub first_strike_done: bool,
    /// CR 510.1c — set when `advance_phase` enters a damage sub-step
    /// that has at least one attacker requiring a player-chosen
    /// distribution (≥2 blockers dealing damage this pass). The engine
    /// yields a [`DecisionContext::DistributeDamage`] in that state and
    /// defers the actual damage deal until the active player responds
    /// with [`Action::AssignCombatDamage`].
    pub pending_damage_assignment: Option<PendingDamagePass>,
}

/// Which damage sub-step is awaiting CR 510.1c assignment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PendingDamagePass {
    FirstStrike,
    Regular,
}

impl CombatState {
    pub fn new() -> Self {
        Self {
            phase: CombatPhase::PreDeclareAttackers,
            attackers: Vec::new(),
            blockers: Vec::new(),
            damage_assignments: Vec::new(),
            has_first_strike: false,
            first_strike_done: false,
            pending_damage_assignment: None,
        }
    }

    /// Find the attacker record for `id`, if any.
    pub fn attacker(&self, id: ObjectId) -> Option<&AttackerInfo> {
        self.attackers.iter().find(|a| a.object_id == id)
    }

    pub fn attacker_mut(&mut self, id: ObjectId) -> Option<&mut AttackerInfo> {
        self.attackers.iter_mut().find(|a| a.object_id == id)
    }

    /// Is `id` an attacker in this combat?
    pub fn is_attacker(&self, id: ObjectId) -> bool {
        self.attacker(id).is_some()
    }

    /// Is `id` blocking something in this combat?
    pub fn is_blocker(&self, id: ObjectId) -> bool {
        self.blockers.iter().any(|b| b.object_id == id)
    }
}

impl Default for CombatState {
    fn default() -> Self { Self::new() }
}

// =============================================================================
// Info records, declarations, phases
// =============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AttackerInfo {
    pub object_id: ObjectId,
    pub defending_player: PlayerId,
    /// Set iff this attacker chose a planeswalker or battle instead
    /// of the player as its defender.
    pub defending_planeswalker: Option<ObjectId>,
    /// IDs of blockers assigned to this attacker, in declared order.
    pub blocked_by: Vec<ObjectId>,
    pub is_blocked: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockerInfo {
    pub object_id: ObjectId,
    /// The attacker being blocked.
    pub blocking: ObjectId,
}

/// Which damage pass is running: the first-strike sub-step or the
/// regular combat-damage step. Drives [`GameState::deal_damage_pass`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DamagePass {
    FirstStrike,
    Regular { first_strike_already_ran: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CombatPhase {
    PreDeclareAttackers,
    DeclareAttackers,
    PostDeclareAttackers,  // triggers, priority
    DeclareBlockers,
    /// CR 509.2 — the active player chooses the damage-assignment
    /// order for each attacker blocked by two or more creatures. The
    /// engine yields a [`DecisionContext::OrderBlockers`] to the
    /// active player; the answering [`Action::OrderBlockers`]
    /// rewrites each attacker's `blocked_by` vector in the chosen
    /// order. Entered only when at least one attacker is multi-blocked;
    /// otherwise [`apply_declared_blockers`] routes straight to
    /// [`CombatPhase::PostDeclareBlockers`].
    OrderBlockers,
    PostDeclareBlockers,   // triggers, priority
    FirstStrikeDamage,
    RegularDamage,
    EndOfCombat,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttackerDeclaration {
    pub attacker: ObjectId,
    pub defending: DefendingEntity,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefendingEntity {
    Player(PlayerId),
    Planeswalker(ObjectId),
    Battle(ObjectId),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockerDeclaration {
    pub blocker: ObjectId,
    pub blocking: ObjectId,
}

/// CR-derived aggregate count constraints on how many blockers may be
/// declared against a single attacker. Computed per attacker by
/// [`GameState::block_constraints`] from the attacker's current
/// characteristics (via layer-aware keyword lookup).
///
/// Two axes factor cleanly from per-blocker eligibility (Flying/Reach,
/// Protection, Skulk-style power filters, etc.):
///
/// * `min_blockers` — the minimum number of blockers required **when
///   the attacker is blocked at all**. Zero blockers (no block) is
///   always legal at the aggregate level; the minimum only constrains
///   non-empty block sets. CR 702.110 Menace raises this to 2;
///   "can't be blocked except by three or more" variants raise it
///   higher. Default 1 (the ordinary "any single blocker is legal"
///   case).
/// * `max_blockers` — the maximum number of blockers allowed, or
///   `None` for unbounded. `Some(0)` expresses "can't be blocked"
///   (a.k.a. unblockable) at the constraint layer — the enumerator
///   emits only the empty block, and the apply-side drops any
///   declared blockers. `Some(1)` expresses "can't be blocked by
///   more than one" effects. Default `None`.
///
/// The layer-aware derivation runs every time combat legality is
/// checked, so Menace granted by a layer-6 effect (Goblin War Drums,
/// Kazuul's Fury mid-game) composes naturally.
///
/// DEBT: at high eligible-blocker counts with multi-Menace (or any
/// min >= 2 constraint on multiple attackers in the same combat),
/// subset enumeration can blow up — C(10,2) = 45 per attacker, three
/// attackers × 45² ≈ 90k declarations. Apply
/// characteristic-equivalence dedup (grouping blockers by computed
/// characteristics the same way delve-subset dedup groups graveyard
/// cards) when the first AI-training profile surfaces the pressure.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct AttackerBlockConstraints {
    pub min_blockers: u32,
    pub max_blockers: Option<u32>,
}

impl AttackerBlockConstraints {
    /// Default constraints — any single eligible blocker is legal,
    /// no upper bound. Matches the ordinary "creature without a
    /// combat-count-restriction keyword" case.
    pub fn default_unrestricted() -> Self {
        Self { min_blockers: 1, max_blockers: None }
    }

    /// Is `count` an allowed non-empty block size under these
    /// constraints? Zero is always legal at the aggregate level (no
    /// block at all); this check gates non-empty counts.
    pub fn allows_block_count(&self, count: u32) -> bool {
        if count == 0 { return true; }
        if count < self.min_blockers { return false; }
        match self.max_blockers {
            Some(max) if count > max => false,
            _ => true,
        }
    }
}

/// How an attacker distributes its damage among its blockers.
/// `distribution` is a `(blocker_id, amount)` list whose sum must
/// equal the attacker's combat-damage-dealt amount and whose order
/// follows CR 510.1c — each entry before the last must be at least
/// the blocker's remaining lethal.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct DamageAssignment {
    pub attacker: ObjectId,
    pub distribution: Vec<(ObjectId, u32)>,
}

// =============================================================================
// GameState integration
// =============================================================================

impl GameState {
    // --- Shared damage primitive ------------------------------------------

    /// Apply `amount` damage from `source` to `target`. Used by both
    /// combat and the effects pipeline.
    ///
    /// For object targets: marks damage on the object and emits
    /// [`GameEvent::DamageDealt`].
    /// For player targets: emits `DamageDealt`, then applies life
    /// loss (CR 120.3c), which itself emits [`GameEvent::LifeLost`].
    pub fn deal_damage(
        &mut self,
        source: ObjectId,
        target: DamageTarget,
        amount: u32,
        is_combat: bool,
    ) {
        if amount == 0 { return; }
        // CR 702.16d — Protection: all damage from a qualifying source
        // is prevented. Check before the replacement pipeline so the
        // DamageDealt event never fires for fully-protected targets.
        if let DamageTarget::Object(target_id) = target {
            if let Some(src_chars) = self.compute_characteristics(source) {
                if self.is_protected_from(target_id, &src_chars) {
                    return;
                }
            }
        }
        // CR 614: route through the replacement-effect pipeline first.
        // May fully prevent the damage (returns None) or modify source,
        // target, amount.
        let (source, target, amount) =
            match self.replace_damage(source, target, amount) {
                Some(tuple) => tuple,
                None => return,
            };
        if amount == 0 { return; }
        match target {
            DamageTarget::Object(id) => {
                // CR 702.2b — Deathtouch damage flags the target for
                // the SBA regardless of how much damage is marked.
                let source_has_dt = self.has_keyword(source, &KeywordAbility::Deathtouch);
                let Some(obj) = self.objects.get_mut(id) else { return; };
                obj.mark_damage(amount);
                if source_has_dt {
                    obj.has_deathtouch_damage = true;
                }
                self.emit(GameEvent::DamageDealt {
                    source, target, amount, is_combat,
                });
            }
            DamageTarget::Player(p) => {
                if (p as usize) >= self.players.len() { return; }
                self.emit(GameEvent::DamageDealt {
                    source, target, amount, is_combat,
                });
                self.player_mut(p).life -= amount as i32;
                self.emit(GameEvent::LifeLost { player: p, amount });
            }
        }

        // CR 702.15b — Lifelink: damage dealt by a source with
        // lifelink also causes its controller to gain that much life.
        // Reads the current controller of the source; if the source
        // has left the battlefield, its controller is still recorded
        // on the departed object (arena survives past the zone).
        if self.has_keyword(source, &KeywordAbility::Lifelink) {
            if let Some(controller) = self.objects.get(source).map(|o| o.controller) {
                self.player_mut(controller).life += amount as i32;
                self.emit(GameEvent::LifeGained { player: controller, amount });
            }
        }
    }

    // --- Combat phase transitions ----------------------------------------

    /// CR 508: begin combat. Initializes `self.combat`.
    pub fn begin_combat(&mut self) {
        self.combat = Some(CombatState::new());
    }

    /// Move into the DeclareAttackers sub-step.
    pub fn enter_declare_attackers(&mut self) {
        if self.combat.is_none() { self.combat = Some(CombatState::new()); }
        self.combat.as_mut().unwrap().phase = CombatPhase::DeclareAttackers;
    }

    /// CR 509: apply the active player's declared attackers.
    ///
    /// - Taps each attacker (skipped for Vigilance, CR 702.20a).
    /// - Records [`AttackerInfo`] for each.
    /// - Emits `CreatureAttacks` per attacker and `AttacksDeclared`
    ///   for the batch.
    /// - Advances the combat phase to `PostDeclareAttackers`.
    ///
    /// Invalid entries (missing object, not a creature, already
    /// tapped, or summoning-sick) are silently skipped — the
    /// legal-action enumerator is the canonical gate.
    pub fn apply_declared_attackers(&mut self, decls: Vec<AttackerDeclaration>) {
        // Ensure combat state exists.
        if self.combat.is_none() { self.combat = Some(CombatState::new()); }

        // Validate and build AttackerInfos.
        let mut infos: Vec<AttackerInfo> = Vec::with_capacity(decls.len());
        let mut events: Vec<GameEvent> = Vec::new();
        let mut tap_ids: Vec<ObjectId> = Vec::new();

        for d in &decls {
            let Some(obj) = self.objects.get(d.attacker) else { continue; };
            // Every gate here must match `legal_actions::can_attack` so a
            // malicious or buggy agent can't smuggle an illegal attacker
            // past the apply path:
            //
            // * summoning sickness (CR 302.1), overridden by Haste (CR 702.10b)
            // * Defender (CR 702.3b)
            // * Pacifism-style "can't attack" continuous effects
            //
            // All three go through the layer-aware `has_keyword` /
            // `cant_attack` dispatch so granted variants (e.g. an
            // opponent's Lure-adjacent effect granting Defender, or a
            // Pacifism enchantment installing `CantAttack`) compose
            // the same as printed ones. Keyword-query layer audit:
            // clean as of 2026-04-20.
            let sick_and_no_haste = obj.status.summoning_sick
                && !self.has_keyword(d.attacker, &KeywordAbility::Haste);
            let has_defender = self.has_keyword(
                d.attacker, &KeywordAbility::Defender);
            let restricted = self.cant_attack(d.attacker);
            if !obj.is_creature()
                || !obj.zone.is_battlefield()
                || obj.is_tapped()
                || sick_and_no_haste
                || has_defender
                || restricted
            {
                continue;
            }
            // Determine the defending player from the declaration.
            let (defending_player, defending_pw) = match d.defending {
                DefendingEntity::Player(p) => (p, None),
                DefendingEntity::Planeswalker(pw_id) => {
                    let pw_ctrl = self.objects.get(pw_id).map(|o| o.controller);
                    (pw_ctrl.unwrap_or(0), Some(pw_id))
                }
                DefendingEntity::Battle(b_id) => {
                    let b_ctrl = self.objects.get(b_id).map(|o| o.controller);
                    (b_ctrl.unwrap_or(0), Some(b_id))
                }
            };
            infos.push(AttackerInfo {
                object_id: d.attacker,
                defending_player,
                defending_planeswalker: defending_pw,
                blocked_by: Vec::new(),
                is_blocked: false,
            });
            // CR 702.20a — Vigilance: attacking doesn't cause the
            // creature to tap.
            if !self.has_keyword(d.attacker, &KeywordAbility::Vigilance) {
                tap_ids.push(d.attacker);
            }
            events.push(GameEvent::CreatureAttacks {
                attacker: d.attacker,
                defending: d.defending.clone(),
            });
        }

        // Apply taps.
        for id in tap_ids {
            if let Some(obj) = self.objects.get_mut(id) {
                if obj.tap() {
                    self.emit(GameEvent::Tapped { object_id: id });
                }
            }
        }

        // Emit CreatureAttacks events.
        for ev in events { self.emit(ev); }

        // Emit the batch AttacksDeclared event.
        self.emit(GameEvent::AttacksDeclared { attackers: decls.clone() });

        // Record.
        let combat = self.combat.as_mut().unwrap();
        combat.attackers = infos;
        combat.phase = CombatPhase::PostDeclareAttackers;
    }

    /// CR 509/702 — combat block-count constraints for `attacker`.
    /// Factors count restrictions (Menace, can't-be-blocked, etc.)
    /// away from per-blocker eligibility (Flying/Reach/Protection),
    /// so combat-restriction keywords slot into this single
    /// dispatch without per-keyword enumeration branches. See
    /// [`AttackerBlockConstraints`] for the rationale.
    ///
    /// Layer-aware via [`Self::has_keyword`] — granted Menace
    /// composes the same as printed Menace.
    pub fn block_constraints(&self, attacker: ObjectId)
        -> AttackerBlockConstraints
    {
        let mut c = AttackerBlockConstraints::default_unrestricted();
        // CR 702.110a — Menace: can't be blocked except by two or
        // more creatures. Expressed as min=2.
        if self.has_keyword(attacker, &KeywordAbility::Menace) {
            c.min_blockers = c.min_blockers.max(2);
        }
        // Future constraint-keyword hooks slot here:
        //   - "Can't be blocked except by three or more" → raise min.
        //   - "Can't be blocked by more than one creature" → cap max.
        //   - Unblockable (printed text, not a keyword today) → max=0.
        c
    }

    /// Move into the DeclareBlockers sub-step.
    pub fn enter_declare_blockers(&mut self) {
        if let Some(c) = self.combat.as_mut() {
            c.phase = CombatPhase::DeclareBlockers;
        }
    }

    /// CR 510: apply defenders' declared blockers.
    ///
    /// - Records [`BlockerInfo`] for each valid pairing.
    /// - Wires each blocker into its attacker's `blocked_by` list
    ///   (in declared order).
    /// - Marks multi-blocker attackers as `is_blocked`.
    /// - Emits `CreatureBlocks` per pairing, `CreatureBlocked` /
    ///   `CreatureNotBlocked` per attacker, and `BlocksDeclared` for
    ///   the batch.
    /// - Advances the phase to `PostDeclareBlockers`.
    ///
    /// Invalid entries are silently skipped (see
    /// [`Self::apply_declared_attackers`]).
    pub fn apply_declared_blockers(&mut self, decls: Vec<BlockerDeclaration>) {
        // Ensure combat exists — otherwise declarations are meaningless.
        if self.combat.is_none() { return; }

        // First pass: validate against current state + combat.attackers.
        let mut valid: Vec<BlockerDeclaration> = Vec::with_capacity(decls.len());
        for d in &decls {
            let Some(blk) = self.objects.get(d.blocker) else { continue; };
            if !blk.is_creature()
                || !blk.zone.is_battlefield()
                || blk.is_tapped()
            { continue; }
            let combat = self.combat.as_ref().unwrap();
            if !combat.is_attacker(d.blocking) { continue; }
            // A creature can only block once; skip duplicates.
            if valid.iter().any(|v| v.blocker == d.blocker) { continue; }
            // CR 702.9a — Flying.
            if self.has_keyword(d.blocking, &KeywordAbility::Flying)
                && !self.has_keyword(d.blocker, &KeywordAbility::Flying)
                && !self.has_keyword(d.blocker, &KeywordAbility::Reach)
            { continue; }
            valid.push(d.clone());
        }

        // Aggregate block-count enforcement (CR 702.110 Menace and
        // future variants). Per-attacker: count declared blockers,
        // compare against the attacker's
        // [`AttackerBlockConstraints`]. Violators (counts below
        // `min_blockers` or above `max_blockers`) drop all their
        // blocker declarations — same shape as the previous
        // Menace-specific check, but now factored to handle every
        // constraint keyword through a single path.
        let constraint_violators: Vec<ObjectId> = {
            let mut violators = Vec::new();
            let mut seen: std::collections::HashSet<ObjectId>
                = std::collections::HashSet::new();
            for d in &valid {
                if !seen.insert(d.blocking) { continue; }
                let constraints = self.block_constraints(d.blocking);
                let count = valid.iter()
                    .filter(|v| v.blocking == d.blocking).count() as u32;
                if !constraints.allows_block_count(count) {
                    violators.push(d.blocking);
                }
            }
            violators
        };
        if !constraint_violators.is_empty() {
            valid.retain(|d| !constraint_violators.contains(&d.blocking));
        }

        // Second pass: update combat state + emit per-pairing events.
        let mut blocker_infos: Vec<BlockerInfo> = Vec::with_capacity(valid.len());
        for d in &valid {
            blocker_infos.push(BlockerInfo {
                object_id: d.blocker,
                blocking: d.blocking,
            });
            self.emit(GameEvent::CreatureBlocks {
                blocker: d.blocker,
                attacker: d.blocking,
            });
        }

        // Fold blockers into their attackers' blocked_by lists.
        let combat = self.combat.as_mut().unwrap();
        combat.blockers = blocker_infos;
        for info in &combat.blockers {
            if let Some(atk) = combat.attackers.iter_mut()
                .find(|a| a.object_id == info.blocking)
            {
                atk.blocked_by.push(info.object_id);
                atk.is_blocked = true;
            }
        }

        // Per-attacker block-summary events.
        let attackers_snapshot: Vec<(ObjectId, Vec<ObjectId>, bool)> = combat.attackers.iter()
            .map(|a| (a.object_id, a.blocked_by.clone(), a.is_blocked))
            .collect();
        for (atk, blockers, blocked) in attackers_snapshot {
            if blocked {
                self.emit(GameEvent::CreatureBlocked { attacker: atk, blockers });
            } else {
                self.emit(GameEvent::CreatureNotBlocked { attacker: atk });
            }
        }

        // Batch event.
        self.emit(GameEvent::BlocksDeclared { blockers: valid });

        // Flag first-strike for the engine to gate the FS sub-step.
        // Recompute every declare-blockers — blockers may grant reach
        // to attackers, FS to blockers, etc.
        let has_fs = self.any_combatant_has_first_strike();
        let combat = self.combat.as_mut().unwrap();
        combat.has_first_strike = has_fs;
        // CR 509.2 — if any attacker is blocked by multiple creatures,
        // pause in OrderBlockers for the active player to choose the
        // damage-assignment order. Otherwise skip the substep.
        let needs_ordering = combat.attackers.iter()
            .any(|a| a.blocked_by.len() >= 2);
        combat.phase = if needs_ordering {
            CombatPhase::OrderBlockers
        } else {
            CombatPhase::PostDeclareBlockers
        };
    }

    /// CR 509.2 — apply the active player's damage-assignment ordering.
    ///
    /// Each ordering is `(attacker, ordered_blockers)`. For every
    /// multi-blocked attacker, the ordering must be a permutation of
    /// that attacker's current `blocked_by` set; otherwise the entry
    /// is rejected. Attackers with 0 or 1 blocker don't need an
    /// ordering and may be omitted; including one is a no-op.
    ///
    /// Returns `true` if every multi-blocked attacker received a valid
    /// ordering and the phase advanced to [`CombatPhase::PostDeclareBlockers`].
    /// Returns `false` (phase unchanged) if any multi-blocked attacker
    /// is missing a valid ordering — lets the caller re-prompt.
    pub fn apply_blocker_ordering(
        &mut self,
        orderings: Vec<(ObjectId, Vec<ObjectId>)>,
    ) -> bool {
        let Some(combat) = self.combat.as_mut() else { return false; };
        if combat.phase != CombatPhase::OrderBlockers { return false; }

        use crate::collections::HashSet;
        // Validate + apply each ordering.
        for (atk_id, new_order) in &orderings {
            let Some(atk) = combat.attackers.iter_mut()
                .find(|a| a.object_id == *atk_id) else { continue; };
            if new_order.len() != atk.blocked_by.len() { continue; }
            let current: HashSet<ObjectId> = atk.blocked_by.iter().copied().collect();
            let proposed: HashSet<ObjectId> = new_order.iter().copied().collect();
            if current != proposed { continue; }
            atk.blocked_by = new_order.clone();
        }

        // Verify every multi-blocked attacker now has an accepted
        // ordering. Simplest check: every multi-blocked attacker must
        // appear in `orderings` with a valid permutation — which, since
        // invalid entries are skipped above, means the attacker's
        // `blocked_by` now matches the corresponding `new_order`.
        let all_ordered = combat.attackers.iter()
            .filter(|a| a.blocked_by.len() >= 2)
            .all(|a| {
                orderings.iter().any(|(id, order)|
                    *id == a.object_id && *order == a.blocked_by)
            });
        if !all_ordered { return false; }

        combat.phase = CombatPhase::PostDeclareBlockers;
        true
    }

    /// Does any attacker or blocker have First Strike or Double Strike?
    fn any_combatant_has_first_strike(&self) -> bool {
        let Some(combat) = self.combat.as_ref() else { return false; };
        let ids = combat.attackers.iter().map(|a| a.object_id)
            .chain(combat.blockers.iter().map(|b| b.object_id));
        for id in ids {
            if self.has_keyword(id, &KeywordAbility::FirstStrike)
                || self.has_keyword(id, &KeywordAbility::DoubleStrike)
            {
                return true;
            }
        }
        false
    }

    /// Post an explicit damage distribution for a multi-blocker
    /// attacker. Overrides the default for that attacker when
    /// `deal_combat_damage` runs.
    ///
    /// Rejects distributions that violate CR 510.1c (order mismatch,
    /// sub-lethal to an earlier blocker, wrong total). Returns `true`
    /// on acceptance. Subsequent accepted calls for the same attacker
    /// replace the prior assignment.
    pub fn set_damage_assignment(&mut self, assignment: DamageAssignment) -> bool {
        if !self.is_legal_damage_assignment(
            assignment.attacker, &assignment.distribution)
        {
            return false;
        }
        let Some(combat) = self.combat.as_mut() else { return false; };
        combat.damage_assignments.retain(|a| a.attacker != assignment.attacker);
        combat.damage_assignments.push(assignment);
        true
    }

    /// CR 510.1c legality predicate for a proposed assignment.
    ///
    /// Rules enforced:
    /// 1. Each `(blocker, amount)` entry appears in `blocked_by` order
    ///    (distribution is a prefix of the ordered blocker list).
    /// 2. Every entry **except possibly the last** assigns ≥ the
    ///    blocker's lethal damage (toughness − marked damage, or 1 if
    ///    the attacker has deathtouch per CR 702.2c).
    /// 3. Sum rule:
    ///    - non-trample: sum == attacker's computed power.
    ///    - trample (CR 702.19b): sum ≤ computed power; any shortfall
    ///      (overflow to defender) requires every blocker to have
    ///      received ≥ lethal.
    pub fn is_legal_damage_assignment(
        &self,
        attacker: ObjectId,
        distribution: &[(ObjectId, u32)],
    ) -> bool {
        let Some(combat) = self.combat.as_ref() else { return false; };
        let Some(atk) = combat.attackers.iter().find(|a| a.object_id == attacker)
            else { return false; };
        let blocked_by = &atk.blocked_by;

        // Rule 1a — length ≤ blocked_by length.
        if distribution.len() > blocked_by.len() { return false; }
        // Rule 1b — order matches prefix of blocked_by.
        for (i, (id, _)) in distribution.iter().enumerate() {
            if blocked_by[i] != *id { return false; }
        }

        let has_dt = self.has_keyword(attacker, &KeywordAbility::Deathtouch);
        let has_trample = self.has_keyword(attacker, &KeywordAbility::Trample);
        let lethal_of = |blk: ObjectId| -> u32 {
            if has_dt { 1 } else { remaining_lethal(self, blk) }
        };

        // Rule 2 — earlier entries must have ≥ lethal.
        for i in 0..distribution.len().saturating_sub(1) {
            let (blk, amt) = distribution[i];
            if amt < lethal_of(blk) { return false; }
        }

        // Rule 3 — sum constraint.
        let atk_power = self.computed_power(attacker).unwrap_or(0);
        if atk_power <= 0 {
            return distribution.iter().all(|(_, a)| *a == 0);
        }
        let atk_power = atk_power as u32;
        let total: u32 = distribution.iter().map(|(_, a)| *a).sum();

        if has_trample {
            if total > atk_power { return false; }
            if total < atk_power {
                // Overflow to defender — every live blocker must have
                // received ≥ lethal.
                if distribution.len() < blocked_by.len() { return false; }
                for (blk, amt) in distribution {
                    if *amt < lethal_of(*blk) { return false; }
                }
            }
            true
        } else {
            total == atk_power
        }
    }

    /// CR 511: assign and deal combat damage (regular strike).
    ///
    /// Each attacker:
    /// - If unblocked: full damage → defending player (or planeswalker).
    /// - If blocked: distribute per [`DamageAssignment`] if one was
    ///   posted for this attacker; otherwise use
    ///   [`default_damage_distribution`].
    /// Each blocker deals its power to the attacker it blocks.
    ///
    /// Creatures with First Strike (and without Double Strike) that
    /// already dealt damage in the first-strike sub-step are skipped —
    /// they've already hit. Creatures that died during first-strike
    /// (marked damage ≥ toughness) are also skipped, matching the
    /// CR 510.2 intent without relying on the SBA to remove them
    /// mid-combat. Double Strike deals damage in both passes.
    ///
    /// Advances combat phase to `EndOfCombat`. Damage events carry
    /// `is_combat = true`.
    pub fn deal_combat_damage(&mut self) {
        let Some(combat) = self.combat.clone() else { return; };
        let first_strike_already_ran = combat.first_strike_done;
        self.deal_damage_pass(&combat, DamagePass::Regular { first_strike_already_ran });
        if let Some(c) = self.combat.as_mut() {
            c.phase = CombatPhase::EndOfCombat;
        }
    }

    /// CR 510.2-510.5 — first-strike damage sub-step.
    ///
    /// Runs the damage pass filtered to creatures that have First Strike
    /// or Double Strike. Creatures without either keyword deal no damage
    /// in this pass. Called by the engine when
    /// [`CombatState::has_first_strike`] is set.
    pub fn deal_first_strike_damage(&mut self) {
        let Some(combat) = self.combat.clone() else { return; };
        self.deal_damage_pass(&combat, DamagePass::FirstStrike);
        if let Some(c) = self.combat.as_mut() {
            c.first_strike_done = true;
            c.phase = CombatPhase::RegularDamage;
        }
    }

    /// Shared damage-pass implementation used by both the first-strike
    /// sub-step and regular combat damage.
    fn deal_damage_pass(&mut self, combat: &CombatState, pass: DamagePass) {
        // CR 510.1 — damage within a single pass is assigned
        // simultaneously, so snapshot which combatants are dead going
        // INTO this pass. Mid-pass deaths don't prevent their damage.
        let dead_at_start: crate::collections::HashSet<ObjectId> = combat.attackers.iter()
            .map(|a| a.object_id)
            .chain(combat.blockers.iter().map(|b| b.object_id))
            .filter(|id| self.is_dead_in_combat(*id))
            .collect();

        for atk in &combat.attackers {
            if dead_at_start.contains(&atk.object_id) { continue; }
            if !self.should_deal_damage_this_pass(atk.object_id, pass) { continue; }
            let atk_power = self.computed_power(atk.object_id).unwrap_or(0);
            if atk_power <= 0 { continue; }

            let has_trample = self.has_keyword(atk.object_id, &KeywordAbility::Trample);
            let has_dt = self.has_keyword(atk.object_id, &KeywordAbility::Deathtouch);

            // --- Attacker → defender / blockers ---
            if !atk.is_blocked {
                let target = match atk.defending_planeswalker {
                    Some(pw) => DamageTarget::Object(pw),
                    None => DamageTarget::Player(atk.defending_player),
                };
                self.deal_damage(atk.object_id, target, atk_power as u32, true);
            } else {
                let live_blockers: Vec<ObjectId> = atk.blocked_by.iter()
                    .copied()
                    .filter(|id| !dead_at_start.contains(id))
                    .collect();
                if live_blockers.is_empty() {
                    // CR 702.19d — a blocked creature with Trample
                    // dumps all its damage onto the defender when its
                    // blockers are already dead. Non-trample attackers
                    // simply drop the damage (CR 510.1c-d).
                    if has_trample {
                        let target = match atk.defending_planeswalker {
                            Some(pw) => DamageTarget::Object(pw),
                            None => DamageTarget::Player(atk.defending_player),
                        };
                        self.deal_damage(
                            atk.object_id, target, atk_power as u32, true);
                    }
                    continue;
                }
                // Decide the distribution. Trample is only consulted
                // for the default path — an explicit
                // [`DamageAssignment`] is trusted verbatim (agent's
                // responsibility to be legal).
                let explicit = combat.damage_assignments.iter()
                    .find(|a| a.attacker == atk.object_id);
                let (dist, overflow_to_defender) = match explicit {
                    Some(a) => (a.distribution.clone(), 0u32),
                    None if has_trample => trample_damage_distribution(
                        self, &live_blockers, atk_power as u32, has_dt),
                    None => (
                        default_damage_distribution(
                            self, atk.object_id, &live_blockers,
                            atk_power as u32),
                        0u32,
                    ),
                };
                for (blk, amt) in dist {
                    self.deal_damage(
                        atk.object_id,
                        DamageTarget::Object(blk),
                        amt, true);
                }
                if overflow_to_defender > 0 {
                    let target = match atk.defending_planeswalker {
                        Some(pw) => DamageTarget::Object(pw),
                        None => DamageTarget::Player(atk.defending_player),
                    };
                    self.deal_damage(
                        atk.object_id, target,
                        overflow_to_defender, true);
                }
            }
        }

        // --- Blockers → their attacker ---
        for blk in &combat.blockers {
            if dead_at_start.contains(&blk.object_id) { continue; }
            if !self.should_deal_damage_this_pass(blk.object_id, pass) { continue; }
            let blk_power = self.computed_power(blk.object_id).unwrap_or(0);
            if blk_power <= 0 { continue; }
            self.deal_damage(
                blk.object_id,
                DamageTarget::Object(blk.blocking),
                blk_power as u32, true);
        }
    }

    /// CR 510.1c — does `pass` have any attacker requiring a player-
    /// chosen damage distribution (blocked by ≥2 live creatures and
    /// dealing damage this pass)? When false, the engine can call the
    /// damage-deal function directly; when true, it yields a
    /// [`DecisionContext::DistributeDamage`] and waits for
    /// [`Action::AssignCombatDamage`].
    pub fn needs_damage_assignment(&self, pass: PendingDamagePass) -> bool {
        !self.attackers_needing_damage_assignment(pass).is_empty()
    }

    /// Ordered list (by `combat.attackers` order) of attackers that
    /// require a CR 510.1c distribution in `pass`.
    pub fn attackers_needing_damage_assignment(
        &self,
        pass: PendingDamagePass,
    ) -> Vec<ObjectId> {
        let Some(combat) = self.combat.as_ref() else { return Vec::new(); };
        use crate::collections::HashSet;
        let dead: HashSet<ObjectId> = combat.attackers.iter()
            .map(|a| a.object_id)
            .chain(combat.blockers.iter().map(|b| b.object_id))
            .filter(|id| self.is_dead_in_combat(*id))
            .collect();
        let internal_pass = match pass {
            PendingDamagePass::FirstStrike => DamagePass::FirstStrike,
            PendingDamagePass::Regular => DamagePass::Regular {
                first_strike_already_ran: combat.first_strike_done,
            },
        };
        combat.attackers.iter()
            .filter(|a| {
                if dead.contains(&a.object_id) { return false; }
                if !self.should_deal_damage_this_pass(a.object_id, internal_pass) {
                    return false;
                }
                let live_blockers = a.blocked_by.iter()
                    .filter(|id| !dead.contains(id)).count();
                live_blockers >= 2
            })
            .map(|a| a.object_id)
            .collect()
    }

    /// Is this creature eligible to deal damage in the given pass?
    fn should_deal_damage_this_pass(&self, id: ObjectId, pass: DamagePass) -> bool {
        let has_fs = self.has_keyword(id, &KeywordAbility::FirstStrike);
        let has_ds = self.has_keyword(id, &KeywordAbility::DoubleStrike);
        match pass {
            DamagePass::FirstStrike => has_fs || has_ds,
            DamagePass::Regular { first_strike_already_ran } => {
                if !first_strike_already_ran { return true; }
                // FS pass already ran: FS-only creatures skip this pass.
                // DoubleStrike hits both passes.
                has_ds || !has_fs
            }
        }
    }

    /// Has this creature accumulated lethal damage? Used to skip
    /// combatants that died during the first-strike pass so they don't
    /// strike in the regular pass. Mirrors CR 704.5g (lethal damage)
    /// and CR 702.2b (deathtouch lethality) without involving the SBA
    /// loop.
    fn is_dead_in_combat(&self, id: ObjectId) -> bool {
        let Some(obj) = self.objects.get(id) else { return true; };
        let Some(t) = self.computed_toughness(id) else { return false; };
        if t <= 0 { return true; }
        if (obj.damage_marked as i32) >= t { return true; }
        if obj.damage_marked > 0 && obj.has_deathtouch_damage { return true; }
        false
    }

    /// End the combat phase. Clears combat state; the engine follows
    /// up by transitioning the turn to the post-combat main phase.
    pub fn end_combat(&mut self) {
        self.combat = None;
    }
}

// =============================================================================
// Default damage distribution (CR 510.1c)
// =============================================================================

/// Distribute `amount` across `blockers` in their declared order,
/// assigning at least lethal to each before moving on.
///
/// "Lethal" means enough to bring the blocker from its current marked
/// damage to its toughness — so a 2/2 blocker that already has 1
/// damage requires only 1 more to die.
///
/// If the attacker can't meet the lethal threshold for the next
/// blocker in order, remaining damage piles on the last blocker it
/// could damage — which matches CR 510.1c's "assigns the rest of its
/// damage to the next blocker" wording in edge cases.
///
/// Returns a list of `(blocker_id, amount)` entries. Blockers that
/// receive 0 damage are omitted.
pub fn default_damage_distribution(
    state: &GameState,
    _attacker: ObjectId,
    blockers: &[ObjectId],
    amount: u32,
) -> Vec<(ObjectId, u32)> {
    if amount == 0 || blockers.is_empty() { return Vec::new(); }
    if blockers.len() == 1 {
        return vec![(blockers[0], amount)];
    }

    let mut out: Vec<(ObjectId, u32)> = Vec::new();
    let mut remaining = amount;
    for (i, &blk) in blockers.iter().enumerate() {
        let last = i == blockers.len() - 1;
        let lethal = remaining_lethal(state, blk);
        let assign = if last {
            remaining
        } else {
            // Assign min(remaining, lethal) to this blocker, then
            // move on. If we can't meet lethal, the blocker takes
            // whatever's left — but we *must* meet lethal before
            // moving to the next; the "can't meet" case collapses
            // to assigning the full remainder here.
            if remaining < lethal { remaining } else { lethal }
        };
        if assign > 0 { out.push((blk, assign)); }
        remaining -= assign;
        if remaining == 0 { break; }
    }
    out
}

fn remaining_lethal(state: &GameState, id: ObjectId) -> u32 {
    let Some(obj) = state.objects.get(id) else { return 0; };
    let Some(toughness) = obj.raw_toughness_with_counters(None) else { return 0; };
    if toughness <= 0 { return 0; }
    (toughness as u32).saturating_sub(obj.damage_marked)
}

/// CR 702.19b — Trample damage distribution. Assigns the per-blocker
/// lethal threshold to each blocker in declared order and spills the
/// remainder to the defender (returned as the second component).
///
/// With Deathtouch (CR 702.2c) the lethal threshold drops to 1 per
/// blocker — a tramping deathtouch creature pushes nearly all its
/// damage through.
///
/// If the attacker doesn't have enough damage to meet the sum of
/// lethal thresholds, CR 702.19b forbids spilling to the defender;
/// the available damage is distributed as [`default_damage_distribution`]
/// would.
pub fn trample_damage_distribution(
    state: &GameState,
    blockers: &[ObjectId],
    amount: u32,
    has_deathtouch: bool,
) -> (Vec<(ObjectId, u32)>, u32) {
    if blockers.is_empty() || amount == 0 {
        return (Vec::new(), amount);
    }

    let per_blocker_lethal = |id: ObjectId| -> u32 {
        if has_deathtouch { 1 } else { remaining_lethal(state, id) }
    };
    let total_lethal: u32 = blockers.iter()
        .map(|&id| per_blocker_lethal(id))
        .sum();

    if amount < total_lethal {
        // Not enough to trample — fall back to the default per-order
        // distribution. No overflow.
        return (
            default_damage_distribution(state, 0, blockers, amount),
            0,
        );
    }

    // Enough damage for at-least-lethal to every blocker; remainder
    // overflows to the defender.
    let mut out: Vec<(ObjectId, u32)> = Vec::new();
    let mut remaining = amount;
    for &blk in blockers {
        let lethal = per_blocker_lethal(blk);
        if lethal > 0 { out.push((blk, lethal)); }
        remaining -= lethal;
    }
    (out, remaining)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mana::ManaCost;
    use crate::objects::{Characteristics, GameObject};
    use crate::zones::Zone;

    fn creature_chars(p: i32, t: i32) -> Characteristics {
        Characteristics {
            mana_cost: Some(ManaCost::parse("{G}").unwrap()),
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(p)),
            toughness: Some(PtValue::Fixed(t)),
            ..Default::default()
        }
    }

    fn planeswalker_chars(loyalty: i32) -> Characteristics {
        Characteristics {
            types: TypeLine::PLANESWALKER.into(),
            loyalty: Some(loyalty),
            ..Default::default()
        }
    }

    fn put_creature(s: &mut GameState, owner: PlayerId, p: i32, t: i32) -> ObjectId {
        let id = s.allocate_object_id();
        let mut obj = GameObject::new(id, owner, Zone::Battlefield, 1, creature_chars(p, t));
        obj.controller = owner;
        s.objects.insert(obj);
        id
    }

    fn ready(id: ObjectId, s: &mut GameState) {
        let obj = s.objects.get_mut(id).unwrap();
        obj.status.summoning_sick = false;
    }

    // --- CombatState basics -------------------------------------------------

    #[test]
    fn new_combat_state_is_predeclare() {
        let c = CombatState::new();
        assert_eq!(c.phase, CombatPhase::PreDeclareAttackers);
        assert!(c.attackers.is_empty());
    }

    // --- begin_combat / phase transitions -----------------------------------

    #[test]
    fn begin_combat_sets_state() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        assert!(s.combat.is_some());
    }

    #[test]
    fn enter_declare_attackers_transitions_phase() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        s.enter_declare_attackers();
        assert_eq!(s.combat.as_ref().unwrap().phase, CombatPhase::DeclareAttackers);
    }

    // --- apply_declared_attackers ------------------------------------------

    #[test]
    fn declare_attackers_taps_and_records() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        ready(atk, &mut s);

        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk,
            defending: DefendingEntity::Player(1),
        }]);

        assert!(s.objects.get(atk).unwrap().is_tapped());
        let combat = s.combat.as_ref().unwrap();
        assert_eq!(combat.phase, CombatPhase::PostDeclareAttackers);
        assert_eq!(combat.attackers.len(), 1);
        assert_eq!(combat.attackers[0].object_id, atk);
        assert_eq!(combat.attackers[0].defending_player, 1);
    }

    #[test]
    fn declare_attackers_emits_events() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 2, 2);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk,
            defending: DefendingEntity::Player(1),
        }]);
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::CreatureAttacks { attacker, .. } if *attacker == atk)));
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::AttacksDeclared { .. })));
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::Tapped { object_id } if *object_id == atk)));
    }

    #[test]
    fn declare_attackers_filters_invalid() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        // Summoning-sick → invalid attacker.
        let sick = put_creature(&mut s, 0, 2, 2);
        s.objects.get_mut(sick).unwrap().status.summoning_sick = true;
        // Already tapped → invalid.
        let tapped = put_creature(&mut s, 0, 2, 2);
        s.objects.get_mut(tapped).unwrap().tap();

        s.apply_declared_attackers(vec![
            AttackerDeclaration { attacker: sick, defending: DefendingEntity::Player(1) },
            AttackerDeclaration { attacker: tapped, defending: DefendingEntity::Player(1) },
        ]);
        assert!(s.combat.as_ref().unwrap().attackers.is_empty());
    }

    /// CR 702.3b — a creature with Defender can't attack. The apply
    /// path must silently drop a Defender declaration even though the
    /// creature is otherwise eligible; otherwise an agent feeding raw
    /// declarations could bypass `legal_actions::can_attack`.
    #[test]
    fn declare_attackers_drops_defender_keyword() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let wall = put_creature(&mut s, 0, 0, 4);
        ready(wall, &mut s);
        s.objects.get_mut(wall).unwrap().characteristics.keywords
            .push(KeywordAbility::Defender);

        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: wall,
            defending: DefendingEntity::Player(1),
        }]);

        assert!(s.combat.as_ref().unwrap().attackers.is_empty(),
            "Defender creature must not be recorded as attacker");
        assert!(!s.objects.get(wall).unwrap().is_tapped(),
            "dropped attacker must not be tapped");
        assert!(!s.event_log.iter().any(|e|
            matches!(e, GameEvent::CreatureAttacks { attacker, .. }
                if *attacker == wall)),
            "no CreatureAttacks event for a dropped Defender attacker");
    }

    /// CR 509.1a — a Pacifism-style "can't attack" continuous effect
    /// blocks attack declaration. Apply-path re-check mirrors
    /// `legal_actions::can_attack`.
    #[test]
    fn declare_attackers_drops_cant_attack_restriction() {
        use crate::effects::Effect;
        use crate::layers::Duration;
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 2, 2);
        ready(atk, &mut s);
        Effect::ForbidAttacking {
            target: atk,
            duration: Duration::EndOfTurn,
        }.execute(&mut s);
        assert!(s.cant_attack(atk), "test precondition");

        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk,
            defending: DefendingEntity::Player(1),
        }]);

        assert!(s.combat.as_ref().unwrap().attackers.is_empty(),
            "cant_attack creature must not be recorded as attacker");
        assert!(!s.objects.get(atk).unwrap().is_tapped(),
            "dropped attacker must not be tapped");
    }

    #[test]
    fn vigilance_attacker_does_not_tap() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        ready(atk, &mut s);
        // Give the attacker Vigilance as a base characteristic.
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::Vigilance);

        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk,
            defending: DefendingEntity::Player(1),
        }]);

        // Recorded as an attacker but not tapped.
        assert!(!s.objects.get(atk).unwrap().is_tapped());
        assert_eq!(s.combat.as_ref().unwrap().attackers.len(), 1);
        // No Tapped event emitted for this attacker.
        assert!(!s.event_log.iter().any(|e|
            matches!(e, GameEvent::Tapped { object_id } if *object_id == atk)));
    }

    #[test]
    fn declare_attackers_against_planeswalker_sets_defender_from_controller() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        ready(atk, &mut s);
        let pw_id = s.allocate_object_id();
        let mut pw = GameObject::new(pw_id, 1, Zone::Battlefield, 2, planeswalker_chars(4));
        pw.controller = 1;
        s.objects.insert(pw);

        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk,
            defending: DefendingEntity::Planeswalker(pw_id),
        }]);
        let combat = s.combat.as_ref().unwrap();
        assert_eq!(combat.attackers[0].defending_player, 1);
        assert_eq!(combat.attackers[0].defending_planeswalker, Some(pw_id));
    }

    // --- apply_declared_blockers -------------------------------------------

    #[test]
    fn declare_blockers_records_pairings() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();

        let blk = put_creature(&mut s, 1, 2, 2);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: blk, blocking: atk,
        }]);

        let combat = s.combat.as_ref().unwrap();
        assert_eq!(combat.phase, CombatPhase::PostDeclareBlockers);
        assert_eq!(combat.blockers.len(), 1);
        let a = combat.attacker(atk).unwrap();
        assert!(a.is_blocked);
        assert_eq!(a.blocked_by, vec![blk]);
    }

    #[test]
    fn declare_blockers_filters_tapped_or_missing() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();

        let tapped_blk = put_creature(&mut s, 1, 2, 2);
        s.objects.get_mut(tapped_blk).unwrap().tap();

        s.apply_declared_blockers(vec![
            BlockerDeclaration { blocker: tapped_blk, blocking: atk },
            BlockerDeclaration { blocker: 999, blocking: atk }, // missing
        ]);
        assert!(s.combat.as_ref().unwrap().blockers.is_empty());
    }

    #[test]
    fn declare_blockers_ignores_duplicate_blocker_entries() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();

        let blk = put_creature(&mut s, 1, 2, 2);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![
            BlockerDeclaration { blocker: blk, blocking: atk },
            BlockerDeclaration { blocker: blk, blocking: atk }, // dup
        ]);
        assert_eq!(s.combat.as_ref().unwrap().blockers.len(), 1);
    }

    #[test]
    fn flying_attacker_is_not_blocked_by_non_flying_non_reach() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::Flying);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();

        let ground = put_creature(&mut s, 1, 2, 2);
        ready(ground, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: ground, blocking: atk,
        }]);

        // Ground blocker was rejected — attacker remains unblocked.
        let combat = s.combat.as_ref().unwrap();
        assert!(combat.blockers.is_empty());
        assert!(!combat.attacker(atk).unwrap().is_blocked);
    }

    #[test]
    fn reach_creature_can_block_flying() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::Flying);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();

        let reach = put_creature(&mut s, 1, 2, 3);
        s.objects.get_mut(reach).unwrap().characteristics.keywords
            .push(KeywordAbility::Reach);
        ready(reach, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: reach, blocking: atk,
        }]);

        assert_eq!(s.combat.as_ref().unwrap().blockers.len(), 1);
    }

    #[test]
    fn menace_attacker_cannot_be_single_blocked() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::Menace);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();

        let blk = put_creature(&mut s, 1, 2, 2);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: blk, blocking: atk,
        }]);
        // Menace: single blocker dropped.
        assert!(s.combat.as_ref().unwrap().blockers.is_empty());
    }

    #[test]
    fn menace_attacker_can_be_double_blocked() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::Menace);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();

        let b1 = put_creature(&mut s, 1, 1, 1);
        let b2 = put_creature(&mut s, 1, 1, 1);
        ready(b1, &mut s);
        ready(b2, &mut s);
        s.apply_declared_blockers(vec![
            BlockerDeclaration { blocker: b1, blocking: atk },
            BlockerDeclaration { blocker: b2, blocking: atk },
        ]);
        assert_eq!(s.combat.as_ref().unwrap().blockers.len(), 2);
    }

    #[test]
    fn declare_blockers_emits_blocked_and_not_blocked() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let blocked_atk = put_creature(&mut s, 0, 3, 3);
        let unblocked_atk = put_creature(&mut s, 0, 1, 1);
        ready(blocked_atk, &mut s);
        ready(unblocked_atk, &mut s);
        s.apply_declared_attackers(vec![
            AttackerDeclaration { attacker: blocked_atk, defending: DefendingEntity::Player(1) },
            AttackerDeclaration { attacker: unblocked_atk, defending: DefendingEntity::Player(1) },
        ]);
        s.enter_declare_blockers();

        let blk = put_creature(&mut s, 1, 2, 2);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: blk, blocking: blocked_atk,
        }]);

        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::CreatureBlocked { attacker, .. } if *attacker == blocked_atk)));
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::CreatureNotBlocked { attacker } if *attacker == unblocked_atk)));
    }

    // --- First strike / Double strike -------------------------------------

    #[test]
    fn first_strike_attacker_kills_non_fs_blocker_without_taking_damage() {
        // 2/2 First Strike vs 2/2 vanilla blocker.
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 2, 2);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::FirstStrike);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        let blk = put_creature(&mut s, 1, 2, 2);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: blk, blocking: atk,
        }]);
        assert!(s.combat.as_ref().unwrap().has_first_strike);

        s.deal_first_strike_damage();
        // Blocker took 2 (lethal). Attacker has no damage yet.
        assert_eq!(s.objects.get(blk).unwrap().damage_marked, 2);
        assert_eq!(s.objects.get(atk).unwrap().damage_marked, 0);

        s.deal_combat_damage();
        // Regular pass: blocker is dead, doesn't strike back.
        assert_eq!(s.objects.get(atk).unwrap().damage_marked, 0);
    }

    #[test]
    fn double_strike_hits_both_passes() {
        // 2/2 Double Strike vs 4/4 vanilla blocker.
        // FS pass: attacker strikes for 2, blocker doesn't (no FS/DS).
        // Regular pass: attacker strikes again for 2 AND blocker
        // strikes back for 4 (CR 510.1 simultaneous — blocker wasn't
        // dead at pass start, so it swings). Blocker ends at 4/4 lethal;
        // attacker ends at 4 damage, lethal.
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 2, 2);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::DoubleStrike);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        let blk = put_creature(&mut s, 1, 4, 4);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: blk, blocking: atk,
        }]);

        s.deal_first_strike_damage();
        s.deal_combat_damage();
        assert_eq!(s.objects.get(blk).unwrap().damage_marked, 4);
        assert_eq!(s.objects.get(atk).unwrap().damage_marked, 4);
    }

    #[test]
    fn double_strike_kills_smaller_blocker_in_fs_pass_without_taking_damage() {
        // 2/2 Double Strike vs 2/2 vanilla blocker.
        // FS pass: DS attacker deals 2 → blocker dies. Blocker never strikes.
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 2, 2);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::DoubleStrike);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        let blk = put_creature(&mut s, 1, 2, 2);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: blk, blocking: atk,
        }]);

        s.deal_first_strike_damage();
        s.deal_combat_damage();
        assert_eq!(s.objects.get(blk).unwrap().damage_marked, 2);
        assert_eq!(s.objects.get(atk).unwrap().damage_marked, 0);
    }

    #[test]
    fn regular_strike_only_deals_no_first_strike_damage() {
        // Plain 2/2 attacker vs plain 2/2 blocker, no FS involved.
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 2, 2);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        let blk = put_creature(&mut s, 1, 2, 2);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: blk, blocking: atk,
        }]);
        assert!(!s.combat.as_ref().unwrap().has_first_strike);

        s.deal_first_strike_damage();
        // No FS or DS present → no damage.
        assert_eq!(s.objects.get(blk).unwrap().damage_marked, 0);
        assert_eq!(s.objects.get(atk).unwrap().damage_marked, 0);

        s.deal_combat_damage();
        // Regular pass: mutual 2 damage.
        assert_eq!(s.objects.get(blk).unwrap().damage_marked, 2);
        assert_eq!(s.objects.get(atk).unwrap().damage_marked, 2);
    }

    #[test]
    fn first_strike_and_double_strike_trade_correctly() {
        // 3/3 First Strike attacker vs 2/2 Double Strike blocker.
        // FS pass: both deal damage (FS attacker 3, DS blocker 2).
        // Attacker now has 2 marked (not lethal, toughness 3).
        // Blocker dies (2 marked ≥ 2 toughness).
        // Regular pass: only DS deals again — but blocker is dead, so 0.
        // Net: attacker survives with 2 damage, blocker dies.
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::FirstStrike);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        let blk = put_creature(&mut s, 1, 2, 2);
        s.objects.get_mut(blk).unwrap().characteristics.keywords
            .push(KeywordAbility::DoubleStrike);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: blk, blocking: atk,
        }]);

        s.deal_first_strike_damage();
        s.deal_combat_damage();
        assert_eq!(s.objects.get(blk).unwrap().damage_marked, 3);
        assert_eq!(s.objects.get(atk).unwrap().damage_marked, 2);
    }

    // --- deal_combat_damage ------------------------------------------------

    #[test]
    fn unblocked_attacker_damages_defending_player() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        s.apply_declared_blockers(vec![]);
        s.deal_combat_damage();

        assert_eq!(s.player(1).life, 20 - 3);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::DamageDealt {
                target: DamageTarget::Player(1), amount: 3, is_combat: true, ..
            })));
    }

    #[test]
    fn unblocked_attacker_damages_planeswalker() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        ready(atk, &mut s);
        let pw_id = s.allocate_object_id();
        let mut pw = GameObject::new(pw_id, 1, Zone::Battlefield, 2, planeswalker_chars(4));
        pw.controller = 1;
        s.objects.insert(pw);

        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk,
            defending: DefendingEntity::Planeswalker(pw_id),
        }]);
        s.enter_declare_blockers();
        s.apply_declared_blockers(vec![]);
        s.deal_combat_damage();

        // Damage goes on the planeswalker as an object.
        assert_eq!(s.objects.get(pw_id).unwrap().damage_marked, 3);
        // Defending player took no life loss.
        assert_eq!(s.player(1).life, 20);
    }

    #[test]
    fn blocked_single_blocker_trades_damage_both_ways() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        let blk = put_creature(&mut s, 1, 2, 2);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: blk, blocking: atk,
        }]);
        s.deal_combat_damage();

        // Blocker takes 3 damage; attacker takes 2.
        assert_eq!(s.objects.get(blk).unwrap().damage_marked, 3);
        assert_eq!(s.objects.get(atk).unwrap().damage_marked, 2);
        // Defending player takes 0 (all damage to blocker).
        assert_eq!(s.player(1).life, 20);
    }

    #[test]
    fn blocked_attacker_with_multiple_blockers_default_distribution() {
        // 4-power attacker, blockers 2/2 and 3/3. Default assigns
        // lethal to first (2) then remaining (2) to second.
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 4, 4);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        let blk1 = put_creature(&mut s, 1, 1, 2);
        let blk2 = put_creature(&mut s, 1, 2, 3);
        ready(blk1, &mut s);
        ready(blk2, &mut s);
        s.apply_declared_blockers(vec![
            BlockerDeclaration { blocker: blk1, blocking: atk },
            BlockerDeclaration { blocker: blk2, blocking: atk },
        ]);
        s.deal_combat_damage();

        assert_eq!(s.objects.get(blk1).unwrap().damage_marked, 2);
        assert_eq!(s.objects.get(blk2).unwrap().damage_marked, 2);
        // Each blocker returns its power (1 + 2 = 3) to the attacker.
        assert_eq!(s.objects.get(atk).unwrap().damage_marked, 3);
    }

    #[test]
    fn explicit_damage_assignment_overrides_default() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 4, 4);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        let blk1 = put_creature(&mut s, 1, 1, 2);
        let blk2 = put_creature(&mut s, 1, 2, 3);
        ready(blk1, &mut s);
        ready(blk2, &mut s);
        s.apply_declared_blockers(vec![
            BlockerDeclaration { blocker: blk1, blocking: atk },
            BlockerDeclaration { blocker: blk2, blocking: atk },
        ]);

        // Attacker's controller chooses: 3 to blk1, 1 to blk2. Legal
        // per CR 510.1c — blk1's lethal is 2, and 3 ≥ 2, so assigning
        // more than lethal to an earlier blocker and spilling the
        // remainder to the next is a valid choice.
        let accepted = s.set_damage_assignment(DamageAssignment {
            attacker: atk,
            distribution: vec![(blk1, 3), (blk2, 1)],
        });
        assert!(accepted, "distribution satisfies CR 510.1c");
        s.deal_combat_damage();
        assert_eq!(s.objects.get(blk1).unwrap().damage_marked, 3);
        assert_eq!(s.objects.get(blk2).unwrap().damage_marked, 1);
    }

    // --- CR 509.2 blocker ordering -----------------------------------------

    fn put_multi_block_scenario(s: &mut GameState)
        -> (ObjectId, ObjectId, ObjectId)
    {
        s.begin_combat();
        let atk = put_creature(s, 0, 5, 5);
        ready(atk, s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        let b1 = put_creature(s, 1, 2, 2);
        let b2 = put_creature(s, 1, 1, 4);
        ready(b1, s);
        ready(b2, s);
        s.apply_declared_blockers(vec![
            BlockerDeclaration { blocker: b1, blocking: atk },
            BlockerDeclaration { blocker: b2, blocking: atk },
        ]);
        (atk, b1, b2)
    }

    #[test]
    fn apply_declared_blockers_multi_block_enters_order_phase() {
        let mut s = GameState::new(2, 0);
        let _ = put_multi_block_scenario(&mut s);
        assert_eq!(s.combat.as_ref().unwrap().phase,
            CombatPhase::OrderBlockers,
            "multi-block must pause in OrderBlockers for CR 509.2");
    }

    #[test]
    fn apply_declared_blockers_single_block_skips_order_phase() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        let blk = put_creature(&mut s, 1, 2, 2);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![
            BlockerDeclaration { blocker: blk, blocking: atk },
        ]);
        assert_eq!(s.combat.as_ref().unwrap().phase,
            CombatPhase::PostDeclareBlockers,
            "single-blocker combat skips the ordering substep");
    }

    #[test]
    fn apply_blocker_ordering_reorders_blocked_by() {
        let mut s = GameState::new(2, 0);
        let (atk, b1, b2) = put_multi_block_scenario(&mut s);
        // Declared order was (b1, b2); active player reorders to (b2, b1).
        let applied = s.apply_blocker_ordering(vec![(atk, vec![b2, b1])]);
        assert!(applied, "valid permutation must be accepted");
        assert_eq!(s.combat.as_ref().unwrap()
            .attacker(atk).unwrap().blocked_by, vec![b2, b1]);
        assert_eq!(s.combat.as_ref().unwrap().phase,
            CombatPhase::PostDeclareBlockers);
    }

    #[test]
    fn apply_blocker_ordering_rejects_non_permutation() {
        let mut s = GameState::new(2, 0);
        let (atk, b1, _b2) = put_multi_block_scenario(&mut s);
        // Missing b2 — not a permutation.
        let applied = s.apply_blocker_ordering(vec![(atk, vec![b1])]);
        assert!(!applied, "non-permutation must be rejected");
        assert_eq!(s.combat.as_ref().unwrap().phase,
            CombatPhase::OrderBlockers, "phase stays for re-prompt");
    }

    #[test]
    fn apply_blocker_ordering_affects_default_damage_distribution() {
        // Declared order (b1=2/2, b2=1/4) would default to 2 damage
        // to b1 (lethal) and 3 to b2 (not lethal). After reordering to
        // (b2 first, b1 second), b2's lethal is 4, so 4 go to b2 and
        // 1 to b1.
        let mut s = GameState::new(2, 0);
        let (atk, b1, b2) = put_multi_block_scenario(&mut s);
        assert!(s.apply_blocker_ordering(vec![(atk, vec![b2, b1])]));
        s.deal_combat_damage();
        assert_eq!(s.objects.get(b2).unwrap().damage_marked, 4,
            "reordered first blocker takes its lethal first");
        assert_eq!(s.objects.get(b1).unwrap().damage_marked, 1,
            "remaining damage piles on the second blocker");
    }

    // --- CR 510.1c legality -----------------------------------------------

    /// Set up a 5/5 attacker with two blockers in declared order
    /// (b1=2/2, b2=1/4). Returns `(atk, b1, b2)`.
    fn legality_scenario(s: &mut GameState) -> (ObjectId, ObjectId, ObjectId) {
        let (atk, b1, b2) = put_multi_block_scenario(s);
        // Put the ordering into PostDeclareBlockers so the validator's
        // `combat.attackers[].blocked_by` reflects the declared order.
        s.apply_blocker_ordering(vec![(atk, vec![b1, b2])]);
        (atk, b1, b2)
    }

    #[test]
    fn legality_accepts_exactly_lethal_in_order() {
        let mut s = GameState::new(2, 0);
        let (atk, b1, b2) = legality_scenario(&mut s);
        // atk power = 5. Lethal to b1 (t=2) is 2; rest = 3 to b2.
        assert!(s.is_legal_damage_assignment(
            atk, &[(b1, 2), (b2, 3)]));
    }

    #[test]
    fn legality_accepts_overkill_to_earlier_blocker() {
        let mut s = GameState::new(2, 0);
        let (atk, b1, b2) = legality_scenario(&mut s);
        assert!(s.is_legal_damage_assignment(
            atk, &[(b1, 3), (b2, 2)]));
    }

    #[test]
    fn legality_accepts_all_damage_to_first_blocker() {
        let mut s = GameState::new(2, 0);
        let (atk, b1, _b2) = legality_scenario(&mut s);
        // Prefix distribution — b2 implicitly gets 0.
        assert!(s.is_legal_damage_assignment(atk, &[(b1, 5)]));
    }

    #[test]
    fn legality_rejects_sub_lethal_to_earlier_blocker() {
        let mut s = GameState::new(2, 0);
        let (atk, b1, b2) = legality_scenario(&mut s);
        // b1 gets 1 (< lethal 2), b2 gets 4. Illegal.
        assert!(!s.is_legal_damage_assignment(
            atk, &[(b1, 1), (b2, 4)]));
    }

    #[test]
    fn legality_rejects_order_mismatch() {
        let mut s = GameState::new(2, 0);
        let (atk, b1, b2) = legality_scenario(&mut s);
        // Distribution swaps b1/b2; blocked_by order is [b1, b2].
        assert!(!s.is_legal_damage_assignment(
            atk, &[(b2, 4), (b1, 1)]));
    }

    #[test]
    fn legality_rejects_sum_mismatch() {
        let mut s = GameState::new(2, 0);
        let (atk, b1, b2) = legality_scenario(&mut s);
        // Sum 4 ≠ atk_power 5.
        assert!(!s.is_legal_damage_assignment(
            atk, &[(b1, 2), (b2, 2)]));
    }

    #[test]
    fn legality_trample_allows_overflow_when_all_lethal() {
        let mut s = GameState::new(2, 0);
        let (atk, b1, b2) = legality_scenario(&mut s);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::Trample);
        // b1 lethal=2, b2 lethal=4. Assign [2, 4] = 6, but atk_power=5,
        // so sum > power → reject. Use [2, 3]: sum 5, b2 needs 4 lethal
        // but got 3. It's the last entry, so legal for non-trample.
        // For trample with sum == power, no overflow, same legality
        // check as non-trample (last entry free).
        assert!(s.is_legal_damage_assignment(
            atk, &[(b1, 2), (b2, 3)]));
    }

    #[test]
    fn legality_trample_rejects_overflow_without_lethal_to_all() {
        let mut s = GameState::new(2, 0);
        let (atk, b1, b2) = legality_scenario(&mut s);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::Trample);
        // Set atk_power higher to make overflow possible.
        s.objects.get_mut(atk).unwrap().characteristics.power
            = Some(PtValue::Fixed(10));
        // b1 lethal=2, b2 lethal=4. Assign [2, 3] (b2 sub-lethal) sum=5,
        // overflow=5. Illegal — overflow requires all blockers lethal.
        assert!(!s.is_legal_damage_assignment(
            atk, &[(b1, 2), (b2, 3)]));
        // Valid: [2, 4] sum=6, overflow=4, all blockers lethal. OK.
        assert!(s.is_legal_damage_assignment(
            atk, &[(b1, 2), (b2, 4)]));
    }

    #[test]
    fn set_damage_assignment_rejects_illegal() {
        let mut s = GameState::new(2, 0);
        let (atk, b1, b2) = legality_scenario(&mut s);
        let accepted = s.set_damage_assignment(DamageAssignment {
            attacker: atk,
            distribution: vec![(b1, 0), (b2, 5)],
        });
        assert!(!accepted, "sub-lethal to earlier blocker must be rejected");
        assert!(s.combat.as_ref().unwrap().damage_assignments.is_empty(),
            "rejected assignment must not be stored");
    }

    // --- default_damage_distribution ---------------------------------------

    #[test]
    fn default_distribution_single_blocker_gets_all() {
        let mut s = GameState::new(2, 0);
        let b = put_creature(&mut s, 1, 2, 2);
        let dist = default_damage_distribution(&s, 1, &[b], 5);
        assert_eq!(dist, vec![(b, 5)]);
    }

    #[test]
    fn default_distribution_assigns_lethal_in_order() {
        let mut s = GameState::new(2, 0);
        let b1 = put_creature(&mut s, 1, 1, 2);
        let b2 = put_creature(&mut s, 1, 1, 3);
        let dist = default_damage_distribution(&s, 1, &[b1, b2], 4);
        assert_eq!(dist, vec![(b1, 2), (b2, 2)]);
    }

    #[test]
    fn default_distribution_insufficient_damage_piles_on_first() {
        let mut s = GameState::new(2, 0);
        let b1 = put_creature(&mut s, 1, 1, 5); // needs 5 for lethal
        let b2 = put_creature(&mut s, 1, 1, 3);
        // Only 3 damage, can't meet lethal on b1 — pile on b1 anyway.
        let dist = default_damage_distribution(&s, 1, &[b1, b2], 3);
        assert_eq!(dist, vec![(b1, 3)]);
    }

    #[test]
    fn default_distribution_respects_existing_damage() {
        let mut s = GameState::new(2, 0);
        let b1 = put_creature(&mut s, 1, 1, 2);
        s.objects.get_mut(b1).unwrap().mark_damage(1); // lethal now 1
        let b2 = put_creature(&mut s, 1, 1, 3);
        let dist = default_damage_distribution(&s, 1, &[b1, b2], 4);
        assert_eq!(dist, vec![(b1, 1), (b2, 3)]);
    }

    // --- end_combat --------------------------------------------------------

    #[test]
    fn end_combat_clears_state() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        s.end_combat();
        assert!(s.combat.is_none());
    }

    // --- deal_damage primitive ---------------------------------------------

    #[test]
    fn deal_damage_to_player_loses_life() {
        let mut s = GameState::new(2, 0);
        s.deal_damage(42, DamageTarget::Player(1), 3, false);
        assert_eq!(s.player(1).life, 17);
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::LifeLost { player: 1, amount: 3 })));
    }

    // --- Trample ----------------------------------------------------------

    #[test]
    fn trample_overflow_damages_defender() {
        // 5/5 Trample vs 2/2 blocker. 2 lethal to blocker, 3 to player.
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 5, 5);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::Trample);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        let blk = put_creature(&mut s, 1, 2, 2);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: blk, blocking: atk,
        }]);
        s.deal_combat_damage();
        assert_eq!(s.objects.get(blk).unwrap().damage_marked, 2);
        assert_eq!(s.player(1).life, 20 - 3);
    }

    #[test]
    fn trample_without_overflow_all_on_blocker() {
        // 3/3 Trample vs 4/4 blocker. Can't meet lethal (4 > 3), so
        // all 3 go to blocker; no damage to player.
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::Trample);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        let blk = put_creature(&mut s, 1, 4, 4);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: blk, blocking: atk,
        }]);
        s.deal_combat_damage();
        assert_eq!(s.objects.get(blk).unwrap().damage_marked, 3);
        assert_eq!(s.player(1).life, 20);
    }

    #[test]
    fn trample_with_deathtouch_pushes_through() {
        // 3/3 Trample + Deathtouch vs 5/5 blocker. DT makes 1 lethal.
        // 1 to blocker, 2 to player.
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 3, 3);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::Trample);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::Deathtouch);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        let blk = put_creature(&mut s, 1, 5, 5);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: blk, blocking: atk,
        }]);
        s.deal_combat_damage();
        assert_eq!(s.objects.get(blk).unwrap().damage_marked, 1);
        assert_eq!(s.player(1).life, 20 - 2);
    }

    #[test]
    fn trample_with_dead_blocker_damages_defender_fully() {
        // FS attacker with Trample kills blocker first, then regular
        // pass trample-dumps all 4 onto the player.
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 4, 4);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::FirstStrike);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::Trample);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        let blk = put_creature(&mut s, 1, 2, 2);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: blk, blocking: atk,
        }]);
        s.deal_first_strike_damage();
        // FS pass: 2 to blocker (lethal) + 2 trample to defender.
        assert_eq!(s.objects.get(blk).unwrap().damage_marked, 2);
        assert_eq!(s.player(1).life, 20 - 2);
        s.deal_combat_damage();
        // Regular pass: attacker has FS only (no DS), so doesn't strike
        // again. Player life unchanged.
        assert_eq!(s.player(1).life, 20 - 2);
    }

    // --- Deathtouch -------------------------------------------------------

    #[test]
    fn deathtouch_marks_flag_on_damaged_creature() {
        let mut s = GameState::new(2, 0);
        let src = put_creature(&mut s, 0, 1, 1);
        s.objects.get_mut(src).unwrap().characteristics.keywords
            .push(KeywordAbility::Deathtouch);
        let tgt = put_creature(&mut s, 1, 5, 5);

        s.deal_damage(src, DamageTarget::Object(tgt), 1, true);
        assert!(s.objects.get(tgt).unwrap().has_deathtouch_damage);
    }

    #[test]
    fn deathtouch_kills_larger_creature_via_sba() {
        let mut s = GameState::new(2, 0);
        let src = put_creature(&mut s, 0, 1, 1);
        s.objects.get_mut(src).unwrap().characteristics.keywords
            .push(KeywordAbility::Deathtouch);
        let tgt = put_creature(&mut s, 1, 5, 5);

        // 1 damage from deathtouch source; 1 < 5 toughness but lethal.
        s.deal_damage(src, DamageTarget::Object(tgt), 1, true);
        crate::sba::apply_state_based_actions(&mut s);
        assert_eq!(s.zone_count(crate::zones::Zone::Graveyard(1)), 1);
        assert!(s.event_log.iter().any(|e| matches!(e,
            crate::events::GameEvent::Dies { object_id } if *object_id == tgt)));
    }

    #[test]
    fn clear_damage_also_clears_deathtouch_flag() {
        let mut s = GameState::new(2, 0);
        let src = put_creature(&mut s, 0, 1, 1);
        s.objects.get_mut(src).unwrap().characteristics.keywords
            .push(KeywordAbility::Deathtouch);
        let tgt = put_creature(&mut s, 1, 5, 5);
        s.deal_damage(src, DamageTarget::Object(tgt), 1, true);
        s.objects.get_mut(tgt).unwrap().clear_damage();
        assert!(!s.objects.get(tgt).unwrap().has_deathtouch_damage);
    }

    #[test]
    fn lifelink_source_gains_life_equal_to_damage_dealt_to_player() {
        let mut s = GameState::new(2, 0);
        // Non-combat lifelink bolt (effects-pipeline style): put a
        // creature as the damage source with Lifelink. Controller is 0.
        let src = put_creature(&mut s, 0, 3, 3);
        s.objects.get_mut(src).unwrap().characteristics.keywords
            .push(KeywordAbility::Lifelink);

        let start = s.player(0).life;
        s.deal_damage(src, DamageTarget::Player(1), 3, /*combat=*/ false);
        assert_eq!(s.player(0).life, start + 3);
        assert_eq!(s.player(1).life, 20 - 3);
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::LifeGained { player: 0, amount: 3 })));
    }

    #[test]
    fn lifelink_combat_damage_gains_life() {
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 4, 4);
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::Lifelink);
        ready(atk, &mut s);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        let blk = put_creature(&mut s, 1, 2, 2);
        ready(blk, &mut s);
        s.apply_declared_blockers(vec![BlockerDeclaration {
            blocker: blk, blocking: atk,
        }]);
        let before = s.player(0).life;
        s.deal_combat_damage();
        // Attacker dealt 4 to blocker → +4 life.
        assert_eq!(s.player(0).life, before + 4);
    }

    #[test]
    fn lifelink_zero_damage_does_not_trigger_gain() {
        let mut s = GameState::new(2, 0);
        let src = put_creature(&mut s, 0, 0, 1);
        s.objects.get_mut(src).unwrap().characteristics.keywords
            .push(KeywordAbility::Lifelink);
        let before = s.player(0).life;
        s.deal_damage(src, DamageTarget::Player(1), 0, true);
        assert_eq!(s.player(0).life, before);
    }

    #[test]
    fn deal_damage_zero_is_noop() {
        let mut s = GameState::new(2, 0);
        s.deal_damage(0, DamageTarget::Player(0), 0, true);
        assert!(s.event_log.is_empty());
    }

    // --- Full combat integration -------------------------------------------

    #[test]
    fn combat_integration_bear_vs_bolt() {
        // Full cycle: begin → declare atk → declare blk → damage → end.
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = put_creature(&mut s, 0, 2, 2);
        ready(atk, &mut s);
        s.enter_declare_attackers();
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        s.apply_declared_blockers(vec![]); // no blocks
        s.deal_combat_damage();
        s.end_combat();

        assert_eq!(s.player(1).life, 18);
        assert!(s.combat.is_none());
    }
}
