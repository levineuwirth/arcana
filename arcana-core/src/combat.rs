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
    PostDeclareBlockers,   // ordering, triggers, priority
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
            if !obj.is_creature()
                || !obj.zone.is_battlefield()
                || obj.is_tapped()
                || obj.status.summoning_sick
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

        // CR 702.110a — Menace check happens after duplicate filtering:
        // for any attacker with Menace, it must have 0 or ≥2 blockers.
        let menace_violators: Vec<ObjectId> = {
            let mut violators = Vec::new();
            let by_attacker = |target: ObjectId| valid.iter()
                .filter(|d| d.blocking == target).count();
            for d in &valid {
                if self.has_keyword(d.blocking, &KeywordAbility::Menace)
                    && by_attacker(d.blocking) < 2
                    && !violators.contains(&d.blocking)
                {
                    violators.push(d.blocking);
                }
            }
            violators
        };
        if !menace_violators.is_empty() {
            valid.retain(|d| !menace_violators.contains(&d.blocking));
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
        combat.phase = CombatPhase::PostDeclareBlockers;
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
    /// Subsequent calls for the same attacker replace the prior
    /// assignment.
    pub fn set_damage_assignment(&mut self, assignment: DamageAssignment) {
        let Some(combat) = self.combat.as_mut() else { return; };
        combat.damage_assignments.retain(|a| a.attacker != assignment.attacker);
        combat.damage_assignments.push(assignment);
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
        let dead_at_start: std::collections::HashSet<ObjectId> = combat.attackers.iter()
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

        // Attacker's controller chooses: 3 to blk1, 1 to blk2.
        // (Not actually legal per CR 510.1c since 2 lethal to blk1
        // not required — but we test the override is honored.)
        s.set_damage_assignment(DamageAssignment {
            attacker: atk,
            distribution: vec![(blk1, 3), (blk2, 1)],
        });
        s.deal_combat_damage();
        assert_eq!(s.objects.get(blk1).unwrap().damage_marked, 3);
        assert_eq!(s.objects.get(blk2).unwrap().damage_marked, 1);
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
