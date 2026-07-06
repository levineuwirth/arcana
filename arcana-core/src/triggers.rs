//! Trigger system — [`TriggerCondition`] matching, [`TriggeredAbilityDef`]
//! firing, APNAP ordering, and delayed triggers.
//!
//! Addendum Section 9, Phase 1 Task #15. Depends on tasks 5 (events),
//! 6 (state), 9 (targets), 10 (priority).
//!
//! # Model
//!
//! A **triggered ability** is a (`condition`, `effect`) pair printed on
//! a card (CR 603). When a game event matches the condition and the
//! optional **intervening-if** clause is satisfied, the ability is
//! put on the stack the next time a player would get priority
//! (CR 603.3).
//!
//! This module provides the pure-data parts:
//!
//! - [`TriggerCondition::matches`] is the match predicate — does
//!   `event` satisfy this condition, for an ability sourced from
//!   `source` controlled by `source_controller`?
//! - [`TriggeredAbilityDef::should_fire`] combines the match +
//!   intervening-if + zone check into a single
//!   `Option<PendingTrigger>`.
//! - [`sort_by_apnap`] orders a batch of pending triggers per
//!   CR 603.3b: each player's own triggers appear together, with
//!   players iterated in APNAP order (active first).
//!
//! The higher-level [`check_triggers`] — walk every permanent's
//! ability list, pull the [`TriggeredAbilityDef`] from the card
//! registry, and match against the event log — needs the card
//! registry to exist first. Task #14 (legal actions) and Task #20
//! (engine) will wire it in. What's here is registry-independent so
//! tests can build their own ability defs directly.
//!
//! # Delayed triggers (CR 603.7)
//!
//! Effects like "at the beginning of the next end step, return the
//! exiled card to the battlefield" register a [`DelayedTrigger`]
//! with [`GameState::register_delayed_trigger`]. The engine checks
//! each emitted event against the active delayed triggers; matches
//! fire and are removed (one-shot by default).
//!
//! # Frequency tracking (OncePerTurn / OncePerGame)
//!
//! [`TriggerFrequency::OncePerTurn`] and
//! [`TriggerFrequency::OncePerGame`] need a "has this fired already"
//! ledger. [`GameState`] grows two `HashMap<(ObjectId, TriggerId),
//! u32>`s to track counts; [`TriggeredAbilityDef::should_fire`]
//! respects them by default.

use crate::collections::HashMap;

use crate::events::GameEvent;
use crate::objects::ObjectId;
use crate::priority::apnap_order;
use crate::state::GameState;
use crate::targets::{ObjectFilter, TargetFilter};
use crate::types::*;
use crate::zones::Zone;

// The controller constraint used by filter-style conditions is the same
// shape as the one used by target filters — reuse it rather than
// maintain a parallel enum.
pub use crate::targets::ControllerConstraint;

// =============================================================================
// TriggeredAbilityDef
// =============================================================================

/// Function pointer that builds the effect for a triggered ability.
///
/// The ability-definition callback receives the current state, the
/// [`PendingTrigger`] (carrying source / controller / the triggering
/// event), and the card registry — granting access to the interner
/// for effects that produce fresh named game objects (token
/// subtypes, for instance) at resolve time.
pub type EffectFn = fn(&GameState, &PendingTrigger, &crate::registry::CardRegistry)
    -> Vec<crate::effects::Effect>;

/// Function pointer for intervening-if clauses (CR 603.4).
/// CR 603.4 "intervening if" predicate. Receives the game state, the
/// ability's `source` object id and its `controller` — so a condition
/// can resolve "you"/"this permanent" ("if you control three or more
/// artifacts", "if you have 10 or less life", "if this creature has a
/// +1/+1 counter on it") — plus the [`crate::registry::CardRegistry`]
/// (trailing, matching [`EffectFn`]) so a condition can resolve
/// subtype/card NAMES via its interner ("if you control two or more
/// Gates", "if you control a Chandra planeswalker"). Pair with the
/// [`crate::conditions`] query helpers. Returns `true` to allow the
/// trigger to fire / stay on the stack, `false` to fizzle it.
pub type InterveningIfFn =
    fn(&GameState, ObjectId, PlayerId, &crate::registry::CardRegistry) -> bool;
/// Computes a dynamic X-value at trigger-fire time from a fired-but-not-
/// yet-on-stack [`PendingTrigger`]. The returned u32 is stamped into
/// the triggered-ability stack entry's `x_value` and consumed by any
/// `TargetCount::X` in `target_requirements`. Use for "up to that many"
/// / "X is the damage dealt" / "X = the discarded card's mana value".
pub type DynamicXFn = fn(&PendingTrigger) -> u32;

// TODO(serialize): `TriggeredAbilityDef` carries bare `fn` pointers
// (`intervening_if`, `effect`). Migrate to `ConditionFnId` /
// `EffectFnId` (addendum Section 12) in Phase 3.
/// A triggered ability GRANTED to an object at runtime (rather than
/// printed in its registry definition) — "until end of turn, whenever
/// ~ deals combat damage, …" (Warrior's Lesson) or a permanent static
/// grant. Lives on [`crate::objects::GameObject::granted_triggered_abilities`];
/// scanned by the engine's trigger collection alongside the registry
/// abilities. `duration` drives end-of-turn cleanup.
#[derive(Clone, Debug)]
pub struct GrantedTrigger {
    pub def: TriggeredAbilityDef,
    pub duration: crate::layers::Duration,
}

/// Trigger ids for granted abilities start here, keeping them clear of
/// the small per-card ids registry definitions use (so dispatch can
/// route a pending trigger to the object's granted list unambiguously).
pub const GRANTED_TRIGGER_ID_BASE: TriggerId = 0xF000_0000;

#[derive(Clone, Debug)]
pub struct TriggeredAbilityDef {
    pub id: TriggerId,
    /// What event pattern fires this trigger.
    pub trigger_condition: TriggerCondition,
    /// Optional "intervening if" clause (CR 603.4) — checked both
    /// when the trigger is about to be put on the stack and when it
    /// resolves.
    pub intervening_if: Option<InterveningIfFn>,
    /// The effect that goes on the stack when the trigger fires.
    pub effect: EffectFn,
    /// Zones from which this ability can trigger. Most abilities
    /// only function on the battlefield; leaves-the-battlefield
    /// triggers need the battlefield zone, while graveyard/hand-based
    /// ones ("when this is drawn", "when this is discarded") list
    /// those zones.
    pub trigger_zones: Vec<Zone>,
    /// How often this can fire per turn / game.
    pub frequency: TriggerFrequency,
    /// CR 603.3d — target requirements declared as the trigger goes
    /// on the stack. Empty for non-targeted triggers (most seed
    /// cards). When non-empty, the engine's settle loop pushes a
    /// [`crate::actions::ChoiceKind::ChooseTargets`] prompt at the
    /// same moment it pushes the trigger; if no legal target exists
    /// per the combined requirements, the ability doesn't trigger
    /// and never lands on the stack.
    pub target_requirements: Vec<crate::targets::TargetRequirement>,
}

impl TriggeredAbilityDef {
    /// Attach an optional dynamic-X resolver to this trigger.
    ///
    /// The resolver runs as the trigger fires (between event-match and
    /// stack-add). Its return value is stamped into the resulting
    /// stack entry's `x_value`, where `TargetCount::X` reads it. Use
    /// for "up to that many target X" / "X = the discarded card's
    /// mana value" / "X = damage just dealt" — anything where the
    /// number of targets or scaling depends on the triggering event.
    ///
    /// The mechanism is a parallel map on `CardDefinition` rather than
    /// a struct field so the 1,800+ existing trigger literals don't
    /// need to thread a new field. To wire one, call
    /// `CardDefinition::with_trigger_dynamic_x(id, fn)` on the def.
    pub fn dynamic_x_resolver<'a>(
        def: &'a crate::registry::CardDefinition,
        trigger_id: TriggerId,
    ) -> Option<&'a DynamicXFn> {
        def.dynamic_x.iter().find(|(id, _)| *id == trigger_id).map(|(_, f)| f)
    }
}

impl TriggeredAbilityDef {
    /// Decide whether this ability should fire for `event` given the
    /// current state.
    ///
    /// Returns `Some(PendingTrigger)` iff:
    ///   1. The source object's zone is in [`Self::trigger_zones`]
    ///      (using [`Zone::same_kind`] so `Graveyard(_)` matches any
    ///      player's graveyard).
    ///   2. The trigger condition matches the event.
    ///   3. The intervening-if clause (if any) evaluates true.
    ///   4. The frequency budget for this (source, trigger) pair
    ///      hasn't been exhausted this turn/game.
    pub fn should_fire(
        &self,
        event: &GameEvent,
        source: ObjectId,
        source_controller: PlayerId,
        state: &GameState,
        reg: &crate::registry::CardRegistry,
    ) -> Option<PendingTrigger> {
        // Zone gate.
        if let Some(obj) = state.objects.get(source) {
            let in_valid_zone = self.trigger_zones.iter()
                .any(|z| z.same_kind(obj.zone));
            if !in_valid_zone {
                return None;
            }
        }

        // Condition predicate.
        if !self.trigger_condition.matches(event, source, source_controller, state) {
            return None;
        }

        // Intervening-if.
        if let Some(cond) = self.intervening_if {
            if !cond(state, source, source_controller, reg) { return None; }
        }

        // Frequency budget.
        if !state.trigger_budget_allows(source, self.id, self.frequency) {
            return None;
        }

        Some(PendingTrigger {
            source,
            trigger_id: self.id,
            controller: source_controller,
            trigger_event: event.clone(),
            targets: crate::targets::TargetSelection::new(),
            effect_override: None,
        })
    }
}

// =============================================================================
// TriggerCondition
// =============================================================================

// TODO(serialize): `TriggerCondition::Custom` carries a bare `fn`
// pointer. Migrate per Section 12 in Phase 3.
#[derive(Clone, Debug)]
pub enum TriggerCondition {
    /// "When ~ enters the battlefield".
    SelfEntersBattlefield,
    /// "When ~ enters the battlefield UNTAPPED" — the ELD check-land
    /// cycle (Dwarven Mine: deals 1 damage; Idyllic Grange: gain 1 life;
    /// Gingerbread Cabin: create a Food). Distinct from an intervening-if
    /// "if untapped": the untapped state is captured at the EntersBattlefield
    /// event (after the enters-tapped clause applies), so the trigger fires
    /// or not based on how it entered and then resolves regardless of any
    /// later tap (e.g. the land being tapped for mana in response).
    SelfEntersBattlefieldUntapped,
    /// "When ~ dies".
    SelfDies,
    /// "When ~ leaves the battlefield" (broader than SelfDies — fires
    /// on a move to ANY zone: graveyard, exile, hand, library). Fires
    /// from LKI for the pre-move id. The canonical consumer is a
    /// control-change Aura reverting its host's controller when the
    /// Aura leaves (Control Magic), plus "when ~ leaves, …" payoffs.
    SelfLeavesBattlefield,
    /// "When ~ attacks".
    SelfAttacks,
    /// Aura/Equipment host trigger — "When/Whenever ENCHANTED (or
    /// equipped) creature <does X>". Wraps an inner condition and
    /// evaluates it as though the source were `source.attached_to`
    /// (the host), so `AttachedCreatureDoes(Box::new(SelfDies))` fires
    /// when the enchanted creature dies, `…(SelfAttacks)` when it
    /// attacks, etc. Inert while the Aura is unattached. The triggered
    /// ability's effect still runs with `trig.source` = the Aura
    /// (use the `PendingTrigger` accessors — `dying_object`,
    /// `attacking_creature` — to reach the host). `source_controller`
    /// stays the Aura's controller (a faithful approximation for the
    /// rare host trigger that reads it).
    AttachedCreatureDoes { condition: Box<TriggerCondition> },
    /// CR 702.21a — "Whenever ~ becomes the target of a spell or
    /// ability an opponent controls." Matches [`GameEvent::BecomesTarget`]
    /// where `target == source` and the targeting player passes
    /// `caster`. Phase 2-B scope: spells + activated abilities only
    /// (triggered-ability targets are still chosen mid-resolution and
    /// don't emit the event).
    SelfBecomesTarget { caster: ControllerConstraint },
    /// "When ~ becomes blocked" (CR 509). Matches
    /// [`GameEvent::CreatureBlocked`] whose attacker is this source.
    SelfBecomesBlocked,
    /// "Whenever ~ blocks" / "Whenever ~ blocks a creature".
    /// Matches [`GameEvent::CreatureBlocks`] whose blocker is this
    /// source.
    SelfBlocks,
    /// "Whenever ~ blocks or becomes blocked by a creature."
    /// Matches either side of a block — [`GameEvent::CreatureBlocks`]
    /// where source is the blocker, OR [`GameEvent::CreatureBlocked`]
    /// where source is the attacker. The OTHER creature in the
    /// combat event is reachable from [`PendingTrigger::other_combatant`].
    SelfBlocksOrBecomesBlocked,
    /// "Whenever ~ becomes blocked by [a creature matching filter]" —
    /// the FILTERED form (Tel-Jilad Wolf "by an artifact creature",
    /// Phyrexian Reaper "by a green creature"). Fires when at least
    /// one declared blocker matches; the full blocker set is
    /// recoverable in the effect via [`crate::script::blockers_of`].
    SelfBecomesBlockedBy { filter: ObjectFilter },
    /// "Whenever ~ blocks or becomes blocked by [filter]" — the
    /// filtered form of [`Self::SelfBlocksOrBecomesBlocked`] (Dwarven
    /// Soldier "one or more Orcs", Serra Inquisitors "black
    /// creatures", Arrogant Bloodlord "power 1 or less"). The paired
    /// creature(s) on the other side must include a filter match.
    SelfBlocksOrBecomesBlockedBy { filter: ObjectFilter },
    /// "Whenever ~ becomes tapped". Matches [`GameEvent::Tapped`]
    /// on this source.
    SelfBecomesTapped,
    /// "Whenever [filter] becomes tapped" — the filtered sibling of
    /// [`Self::SelfBecomesTapped`] (Fatigue "a creature an opponent
    /// controls", Quicksilver Fountain "an Island", Manabarbs-kin
    /// watching lands). Matches [`GameEvent::Tapped`] whose object
    /// satisfies the filter; read the tapped object in the effect via
    /// the trigger event.
    BecomesTapped { filter: ObjectFilter },
    /// CR 711 / 716 — "When this creature specializes". Matches
    /// [`GameEvent::Specialized`] on this source (emitted by
    /// [`crate::effects::Effect::Specialize`]).
    SelfSpecializes,
    /// CR 712 — "When this creature transforms [into <back-face
    /// name>]" (Avacynian Missionaries / Lunarch Inquisitors,
    /// werewolf flip riders). Matches [`GameEvent::Transformed`] on
    /// this source; `to_face: Some(1)` = only when it flips INTO the
    /// back face, `Some(0)` = back→front, `None` = either direction.
    /// The face check reads `visible_face` from live state, which is
    /// already flipped when the event fires.
    SelfTransforms { to_face: Option<u8> },
    /// "Whenever ~ is dealt damage". `combat_only: true` restricts
    /// to CR 510.1c combat damage; `false` accepts any damage source.
    SelfIsDealtDamage { combat_only: bool },
    /// "Whenever ~ attacks and isn't blocked". Matches
    /// [`GameEvent::CreatureNotBlocked`] whose attacker is this
    /// source. The unblocked classification is settled by combat in
    /// the DeclareBlockers step.
    SelfAttacksUnblocked,
    /// "Whenever ~ attacks alone" (CR 506.5 sole attacker — Yuan
    /// Shao's Infantry, Tempered in Solitude, the non-keyword exalted
    /// kin). Matches the batch [`GameEvent::AttacksDeclared`] whose
    /// declaration list is exactly this source — the per-creature
    /// `CreatureAttacks` event can't carry the alone-ness (it fires
    /// before the full attacker set is recorded), but the batch event
    /// is self-contained.
    SelfAttacksAlone,
    /// "Whenever a [filter] creature you control attacks alone" — the
    /// filtered sibling of [`Self::SelfAttacksAlone`] for watchers
    /// that aren't the attacker (Tempered in Solitude, the NEO
    /// Samurai cluster, Thoughtweft Imbuer). Matches the batch
    /// [`GameEvent::AttacksDeclared`] whose declaration list is
    /// exactly ONE creature satisfying the filter. Read the sole
    /// attacker in the effect via
    /// [`PendingTrigger::lone_attacker`].
    AttacksAlone { filter: ObjectFilter },
    /// "Whenever a creature enters the battlefield under your control".
    ZoneChange { filter: ObjectFilter, from: Option<Zone>, to: Zone },
    /// "Whenever you cast a spell" (optionally filtered).
    SpellCast { filter: Option<ObjectFilter>, caster: ControllerConstraint },
    /// "Whenever you cast a spell from [zone]" (CR 601.2a) — flashback /
    /// foretell / impulse / escape / Disturb / commander payoffs. Reads the
    /// cast spell's `StackEntry::cast_from_zone`. New variant (not a field on
    /// `SpellCast`) so the 100+ existing SpellCast triggers stay stable.
    SpellCastFromZone {
        filter: Option<ObjectFilter>,
        caster: ControllerConstraint,
        from_zone: Zone,
    },
    /// "Whenever a creature deals combat damage to a player".
    DamageDealt {
        source_filter: ObjectFilter,
        target_filter: TargetFilter,
        combat_only: bool,
    },
    /// "At the beginning of your upkeep".
    StepBegins { step: crate::turn::Step, whose: ControllerConstraint },
    /// "At the beginning of each end step".
    PhaseBegins { phase: crate::turn::Phase, whose: ControllerConstraint },
    /// "Whenever you gain life".
    LifeGained { player: ControllerConstraint },
    /// "Whenever a counter is put on ~." When `chapter: Some(n)`,
    /// only fires when the post-add count on the target object equals
    /// `n` — used for Saga chapter dispatch (CR 716.5: chapter N
    /// triggers when the N-th lore counter is placed). For the
    /// non-saga case (Hardened Scales-style "whenever a +1/+1 is
    /// placed on ~"), set `chapter: None`.
    CounterAdded {
        on: TriggerSelf,
        kind: Option<CounterKind>,
        chapter: Option<u32>,
    },
    /// "Whenever you draw a card".
    CardDrawn { player: ControllerConstraint },
    /// "Whenever an opponent discards a card".
    CardDiscarded { player: ControllerConstraint },
    /// CR 706 — "Whenever you roll one or more dice" ("dice matters").
    /// Matches [`GameEvent::DieRolled`] whose roller passes `player`.
    /// Phase 1: one event per die, so a single multi-die roll fires this
    /// per die (the "one or more" wording reads naturally for the common
    /// single-die case; a true once-per-roll-batch is a follow-up). The
    /// rolled result is reachable in the effect via the trigger event.
    DiceRolled { player: ControllerConstraint },
    /// "Whenever a creature you control attacks".
    CreatureAttacks { filter: ObjectFilter },
    /// "Whenever you sacrifice a permanent".
    Sacrificed { filter: ObjectFilter },
    /// Catch-all for complex triggers.
    Custom(fn(&GameEvent, &GameState, ObjectId) -> bool),
}

impl TriggerCondition {
    /// Does `event` match this condition?
    ///
    /// `source` is the triggered-ability's source object (used by the
    /// `Self*` variants and by `TriggerSelf::Source`).
    /// `source_controller` is used to resolve `You`/`Opponent` in the
    /// controller constraint.
    pub fn matches(
        &self,
        event: &GameEvent,
        source: ObjectId,
        source_controller: PlayerId,
        state: &GameState,
    ) -> bool {
        use TriggerCondition::*;
        match self {
            SelfEntersBattlefield => matches!(event,
                GameEvent::EntersBattlefield { object_id, .. } if *object_id == source),

            SelfEntersBattlefieldUntapped => matches!(event,
                GameEvent::EntersBattlefield { object_id, .. }
                    if *object_id == source
                        && state.objects.get(source).is_some_and(|o| !o.is_tapped())),

            SelfDies => matches!(event,
                GameEvent::Dies { object_id } if *object_id == source),

            SelfLeavesBattlefield => matches!(event,
                GameEvent::LeavesBattlefield { object_id, .. } if *object_id == source),

            SelfAttacks => matches!(event,
                GameEvent::CreatureAttacks { attacker, .. } if *attacker == source),

            SelfBecomesTarget { caster } => {
                let GameEvent::BecomesTarget { target, controller, .. } = event
                    else { return false; };
                *target == source
                    && caster.matches(*controller, source_controller)
            }

            SelfBecomesBlocked => matches!(event,
                GameEvent::CreatureBlocked { attacker, .. } if *attacker == source),

            SelfBlocks => matches!(event,
                GameEvent::CreatureBlocks { blocker, .. } if *blocker == source),

            SelfBlocksOrBecomesBlocked => match event {
                GameEvent::CreatureBlocks { blocker, .. } => *blocker == source,
                GameEvent::CreatureBlocked { attacker, .. } => *attacker == source,
                _ => false,
            },

            SelfBecomesBlockedBy { filter } => matches!(event,
                GameEvent::CreatureBlocked { attacker, blockers }
                    if *attacker == source && blockers.iter().any(|b|
                        match_filter_on(state, *b, filter, source_controller))),

            SelfBlocksOrBecomesBlockedBy { filter } => match event {
                GameEvent::CreatureBlocks { blocker, attacker } =>
                    *blocker == source
                        && match_filter_on(state, *attacker, filter, source_controller),
                GameEvent::CreatureBlocked { attacker, blockers } =>
                    *attacker == source && blockers.iter().any(|b|
                        match_filter_on(state, *b, filter, source_controller)),
                _ => false,
            },

            SelfBecomesTapped => matches!(event,
                GameEvent::Tapped { object_id } if *object_id == source),

            BecomesTapped { filter } => {
                let GameEvent::Tapped { object_id } = event
                    else { return false; };
                match_filter_on(state, *object_id, filter, source_controller)
            }

            SelfSpecializes => matches!(event,
                GameEvent::Specialized { object_id } if *object_id == source),

            SelfTransforms { to_face } => {
                let GameEvent::Transformed { object_id } = event
                    else { return false; };
                *object_id == source
                    && to_face.is_none_or(|f|
                        state.objects.get(source)
                            .is_some_and(|o| o.visible_face == f))
            }

            SelfAttacksAlone => matches!(event,
                GameEvent::AttacksDeclared { attackers }
                    if attackers.len() == 1
                        && attackers[0].attacker == source),

            AttacksAlone { filter } => {
                let GameEvent::AttacksDeclared { attackers } = event
                    else { return false; };
                attackers.len() == 1
                    && match_filter_on(
                        state, attackers[0].attacker, filter,
                        source_controller)
            }

            SelfIsDealtDamage { combat_only } => {
                let GameEvent::DamageDealt { target, is_combat, .. } = event
                    else { return false; };
                if *combat_only && !is_combat { return false; }
                matches!(target,
                    crate::events::DamageTarget::Object(id) if *id == source)
            }

            SelfAttacksUnblocked => matches!(event,
                GameEvent::CreatureNotBlocked { attacker } if *attacker == source),

            ZoneChange { filter, from, to } => {
                let GameEvent::ZoneChange { object_id, from: evf, to: evt, .. } = event
                    else { return false; };
                if !evt.same_kind(*to) { return false; }
                if let Some(f) = from {
                    if !evf.same_kind(*f) { return false; }
                }
                match_filter_on(state, *object_id, filter, source_controller)
            }

            SpellCast { filter, caster } => {
                let GameEvent::SpellCast { object_id, controller, .. } = event
                    else { return false; };
                if !caster.matches(*controller, source_controller) { return false; }
                match filter {
                    None => true,
                    Some(f) => match_filter_on(state, *object_id, f, source_controller),
                }
            }

            SpellCastFromZone { filter, caster, from_zone } => {
                let GameEvent::SpellCast { object_id, controller, .. } = event
                    else { return false; };
                if !caster.matches(*controller, source_controller) { return false; }
                // The spell is still on the stack when SpellCast fires; read the
                // zone it was cast from off its entry.
                match state.find_stack_entry(*object_id).map(|e| e.cast_from_zone) {
                    Some(z) if z.same_kind(*from_zone) => {}
                    _ => return false,
                }
                match filter {
                    None => true,
                    Some(f) => match_filter_on(state, *object_id, f, source_controller),
                }
            }

            DamageDealt { source_filter, target_filter, combat_only } => {
                let GameEvent::DamageDealt { source: dmg_src, target, is_combat, .. } = event
                    else { return false; };
                if *combat_only && !is_combat { return false; }
                if !match_filter_on(state, *dmg_src, source_filter, source_controller) {
                    return false;
                }
                let choice = crate::targets::TargetChoice::Object(match target {
                    crate::events::DamageTarget::Object(id) => *id,
                    crate::events::DamageTarget::Player(p) => {
                        // Target is a player — translate to the target-
                        // filter Player-shape choice.
                        return target_filter.matches(
                            &crate::targets::TargetChoice::Player(*p),
                            state, source, source_controller);
                    }
                });
                target_filter.matches(&choice, state, source, source_controller)
            }

            StepBegins { step, whose } => {
                let GameEvent::StepBegins { step: ev_step } = event
                    else { return false; };
                if ev_step != step { return false; }
                whose.matches(state.active_player(), source_controller)
            }

            PhaseBegins { phase, whose } => {
                let GameEvent::PhaseBegins { phase: ev_phase } = event
                    else { return false; };
                if ev_phase != phase { return false; }
                whose.matches(state.active_player(), source_controller)
            }

            LifeGained { player } => {
                let GameEvent::LifeGained { player: p, .. } = event
                    else { return false; };
                player.matches(*p, source_controller)
            }

            CounterAdded { on, kind, chapter } => {
                let GameEvent::CounterAdded { object_id, kind: k, count: ev_count } = event
                    else { return false; };
                if let Some(want) = kind {
                    if k != want { return false; }
                }
                if let Some(want_count) = chapter {
                    if *ev_count != *want_count { return false; }
                }
                on.matches(*object_id, source, source_controller, state)
            }

            CardDrawn { player } => {
                let GameEvent::DrawCard { player: p, .. } = event
                    else { return false; };
                player.matches(*p, source_controller)
            }

            CardDiscarded { player } => {
                let GameEvent::Discarded { player: p, .. } = event
                    else { return false; };
                player.matches(*p, source_controller)
            }

            DiceRolled { player } => {
                let GameEvent::DieRolled { player: p, .. } = event
                    else { return false; };
                player.matches(*p, source_controller)
            }

            CreatureAttacks { filter } => {
                let GameEvent::CreatureAttacks { attacker, .. } = event
                    else { return false; };
                match_filter_on(state, *attacker, filter, source_controller)
            }

            Sacrificed { filter } => {
                let GameEvent::Sacrifice { object_id, .. } = event
                    else { return false; };
                match_filter_on(state, *object_id, filter, source_controller)
            }

            AttachedCreatureDoes { condition } => {
                // Re-evaluate the inner condition with the host
                // substituted for the source AND the host's controller
                // substituted for source_controller. CR 303.4: the
                // Aura's ability watches events on the enchanted
                // permanent — so `Self*` object conditions key on the
                // host, and controller-relative conditions ("your
                // upkeep", StepBegins{whose: You}) key on the enchanted
                // creature's controller (host-controller-upkeep triggers
                // like Underworld Dreams-on-a-creature, Sky Swallower).
                match state.objects.get(source).and_then(|o| o.attached_to) {
                    Some(host) => {
                        let host_ctrl = state.objects.get(host)
                            .map(|o| o.controller).unwrap_or(source_controller);
                        condition.matches(event, host, host_ctrl, state)
                    }
                    None => false,
                }
            }

            Custom(f) => f(event, state, source),
        }
    }
}

/// Resolve a `TriggerSelf` against a specific event object.
#[derive(Clone, Debug)]
pub enum TriggerSelf {
    /// The event must be about the triggered ability's own source.
    Source,
    /// Any object matching the filter counts.
    AnyMatching(ObjectFilter),
    /// Any object matching the filter EXCEPT the triggered ability's own
    /// source — MTG's "another" (e.g. "whenever another creature you
    /// control ..."). Excluding the source is not just fidelity: for
    /// triggers whose effect re-creates the watched event on the source
    /// itself (Wildwood Scourge: counter added → add a counter to self),
    /// `AnyMatching` would re-fire forever. `AnotherMatching` breaks that
    /// loop (random-game harness seed 295).
    AnotherMatching(ObjectFilter),
}

impl TriggerSelf {
    pub fn matches(
        &self,
        event_object: ObjectId,
        source: ObjectId,
        source_controller: PlayerId,
        state: &GameState,
    ) -> bool {
        match self {
            TriggerSelf::Source => event_object == source,
            TriggerSelf::AnyMatching(f) =>
                match_filter_on(state, event_object, f, source_controller),
            TriggerSelf::AnotherMatching(f) =>
                event_object != source
                    && match_filter_on(state, event_object, f, source_controller),
        }
    }
}

/// How often the same `(source, trigger)` can fire per turn / game.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriggerFrequency {
    EachTime,
    OncePerTurn,
    OncePerGame,
}

// Helper: apply an ObjectFilter to an id, handling missing objects as
// a non-match. Consults LKI as a fallback so leaves-the-battlefield
// triggers can filter on the pre-move characteristics of a permanent
// that has already been re-id'd into its new zone (CR 603.10 / 400.7).
fn match_filter_on(
    state: &GameState,
    id: ObjectId,
    filter: &ObjectFilter,
    source_controller: PlayerId,
) -> bool {
    state.object_or_lki(id)
        .is_some_and(|o| filter.matches(o, state, source_controller))
}

// =============================================================================
// PendingTrigger
// =============================================================================

/// A trigger that has matched an event and is waiting to be put on
/// the stack by the engine (CR 603.3).
#[derive(Clone, Debug)]
pub struct PendingTrigger {
    pub source: ObjectId,
    pub trigger_id: TriggerId,
    pub controller: PlayerId,
    pub trigger_event: GameEvent,
    /// Targets declared as the trigger went on the stack (CR 603.3b).
    /// Empty for non-targeted triggers and for enqueued targeted
    /// triggers that haven't been prompted yet; populated from
    /// [`crate::stack::StackEntry::targets`] at resolution time so
    /// the effect callback can read the chosen targets.
    pub targets: crate::targets::TargetSelection,
    /// For DELAYED triggers: the scheduled effect fn, carried from
    /// [`DelayedTrigger::effect`] through the stack entry to
    /// resolution. Delayed triggers have no registry-backed
    /// `TriggeredAbilityDef` (their `trigger_id` is 0), so without
    /// this snapshot the resolution dispatch found nothing and the
    /// effect SILENTLY VANISHED — every "sacrifice it at end of
    /// turn" / control-revert / next-cast rider no-op'd at
    /// resolution. `None` for registry-backed triggers.
    pub effect_override: Option<EffectFn>,
}

impl PendingTrigger {
    // --- typed event accessors ------------------------------------------
    //
    // These pull structured fields out of `trigger_event` so an effect
    // fn never has to pattern-match `GameEvent` directly. They are the
    // engine-side surface card-gen prompts advertise to model authors
    // (see `arcana-gen::prompt::TRIGGER_PENDING_ACCESSORS`). Each
    // returns `Option<…>` keyed on whether the originating event
    // carries the requested datum.

    /// The `ObjectId` of the creature that just died, if this trigger
    /// fired on [`GameEvent::Dies`] or a [`GameEvent::ZoneChange`]
    /// into any graveyard. For a `SelfDies` trigger this equals
    /// [`Self::source`]; for graveyard-bound `ZoneChange` triggers
    /// (e.g. "whenever a creature dies") it's the moved object.
    pub fn dying_object(&self) -> Option<ObjectId> {
        match &self.trigger_event {
            GameEvent::Dies { object_id } => Some(*object_id),
            GameEvent::ZoneChange { object_id, to: Zone::Graveyard(_), .. } =>
                Some(*object_id),
            _ => None,
        }
    }

    /// Damage amount, if this trigger fired on
    /// [`GameEvent::DamageDealt`].
    pub fn damage_amount(&self) -> Option<u32> {
        match &self.trigger_event {
            GameEvent::DamageDealt { amount, .. } => Some(*amount),
            _ => None,
        }
    }

    /// The total mana spent to cast the spell, if this trigger fired on a
    /// [`GameEvent::SpellCast`] (CR 107.3 — Smoldering Egg's ember counters).
    /// 0 for free casts.
    pub fn mana_spent(&self) -> Option<u32> {
        match &self.trigger_event {
            GameEvent::SpellCast { mana_spent, .. } => Some(*mana_spent),
            _ => None,
        }
    }

    /// The player who was dealt damage, if this trigger fired on a
    /// [`GameEvent::DamageDealt`] whose target is a player.
    pub fn damaged_player(&self) -> Option<PlayerId> {
        match &self.trigger_event {
            GameEvent::DamageDealt {
                target: crate::events::DamageTarget::Player(p), ..
            } => Some(*p),
            _ => None,
        }
    }

    /// The defending player of an attack, if this trigger fired on a
    /// [`GameEvent::CreatureAttacks`] whose defender is a player (not
    /// a planeswalker or battle).
    pub fn defending_player(&self) -> Option<PlayerId> {
        match &self.trigger_event {
            GameEvent::CreatureAttacks {
                defending: crate::combat::DefendingEntity::Player(p), ..
            } => Some(*p),
            _ => None,
        }
    }

    /// The controller of the spell that triggered this ability, if
    /// this trigger fired on [`GameEvent::SpellCast`].
    pub fn triggering_caster(&self) -> Option<PlayerId> {
        match &self.trigger_event {
            GameEvent::SpellCast { controller, .. } => Some(*controller),
            _ => None,
        }
    }

    /// The `ObjectId` of the object that just entered the battlefield,
    /// if this trigger fired on [`GameEvent::EntersBattlefield`] or a
    /// battlefield-bound [`GameEvent::ZoneChange`]. For a
    /// `SelfEntersBattlefield` trigger this equals [`Self::source`];
    /// for filtered ETB `ZoneChange` triggers it's the entering
    /// creature.
    pub fn entering_object(&self) -> Option<ObjectId> {
        match &self.trigger_event {
            GameEvent::EntersBattlefield { object_id, .. } => Some(*object_id),
            // CR 400.7 — the object is re-ided when it changes zones; the entered
            // permanent lives at `new_id`, so effects that act on it (pump,
            // counters, tap…) must use that, NOT the stale pre-move `object_id`.
            GameEvent::ZoneChange { new_id, to: Zone::Battlefield, .. } =>
                Some(*new_id),
            _ => None,
        }
    }

    /// The zone the object entered the battlefield FROM, if this trigger fired
    /// on [`GameEvent::EntersBattlefield`] (or a battlefield-bound
    /// [`GameEvent::ZoneChange`]). Lets "enters from your graveyard" cards
    /// (Archfiend's Vessel) gate their effect: a normally-cast permanent enters
    /// from the STACK, a reanimated one from a graveyard. (Note: a spell CAST
    /// from a graveyard still enters from the stack — distinguishing that needs
    /// the cast-from-zone, not this.)
    pub fn entered_from_zone(&self) -> Option<Zone> {
        match &self.trigger_event {
            GameEvent::EntersBattlefield { from_zone, .. } => Some(*from_zone),
            GameEvent::ZoneChange { from, to: Zone::Battlefield, .. } => Some(*from),
            _ => None,
        }
    }

    /// The attacking creature, if this trigger fired on
    /// [`GameEvent::CreatureAttacks`] — "whenever a creature attacks,
    /// [do something to/with that creature]" (Hissing Iguanar's kin,
    /// attack-tax punishers, pump-the-attacker enchantments). For a
    /// `SelfAttacks` trigger this equals [`Self::source`]; for
    /// filtered `CreatureAttacks { filter }` triggers it's the
    /// creature that satisfied the filter.
    ///
    /// Note "that player" for step/phase triggers ("at the beginning
    /// of each player's upkeep, that player…") is NOT an accessor:
    /// [`GameEvent::StepBegins`] carries no player because steps
    /// always belong to the active player, and an upkeep trigger
    /// resolves during that same upkeep — read
    /// `state.active_player()` in the effect fn.
    pub fn attacking_creature(&self) -> Option<ObjectId> {
        match &self.trigger_event {
            GameEvent::CreatureAttacks { attacker, .. } => Some(*attacker),
            _ => None,
        }
    }

    /// The sole declared attacker, if this trigger fired on a
    /// single-attacker [`GameEvent::AttacksDeclared`] — pairs with
    /// [`TriggerCondition::AttacksAlone`] ("whenever a creature you
    /// control attacks alone, [it gets +2/+0 / put a counter on
    /// it]"). For `SelfAttacksAlone` this equals [`Self::source`].
    pub fn lone_attacker(&self) -> Option<ObjectId> {
        match &self.trigger_event {
            GameEvent::AttacksDeclared { attackers }
                if attackers.len() == 1 => Some(attackers[0].attacker),
            _ => None,
        }
    }

    /// The OTHER creature in a block event. For
    /// [`GameEvent::CreatureBlocks`] (this source is the blocker), the
    /// attacker. For [`GameEvent::CreatureBlocked`] (this source is the
    /// attacker), the first declared blocker — multi-blocker phrasings
    /// like "destroy all creatures blocking this" should iterate the
    /// full `blockers` vec instead, but cards saying "that creature"
    /// (Aisling Leprechaun, Tangle Asp, Rock Basilisk) refer to a
    /// single antecedent, so first-blocker is the standard rendering.
    /// Pairs with [`TriggerCondition::SelfBlocksOrBecomesBlocked`].
    pub fn other_combatant(&self) -> Option<ObjectId> {
        match &self.trigger_event {
            GameEvent::CreatureBlocks { attacker, .. } => Some(*attacker),
            GameEvent::CreatureBlocked { blockers, .. } =>
                blockers.first().copied(),
            _ => None,
        }
    }
}

/// Sort `triggers` in-place by APNAP order of their `controller` —
/// active player's triggers first, then each subsequent opponent in
/// turn order (CR 603.3b). Stable within a player's own triggers, so
/// input order is preserved per-player; the engine lets the player
/// choose that ordering explicitly as a follow-up decision.
pub fn sort_by_apnap(
    triggers: &mut [PendingTrigger],
    active_player: PlayerId,
    num_players: u8,
) {
    let order: Vec<PlayerId> = apnap_order(active_player, num_players).collect();
    triggers.sort_by_key(|t|
        order.iter().position(|&p| p == t.controller).unwrap_or(usize::MAX));
}

/// Convenience wrapper: collect fires from a batch of
/// `(source, controller, ability_def)` tuples for a single event.
/// Returns pending triggers already APNAP-sorted.
///
/// This is the registry-independent slice of `check_triggers` — a
/// full engine loop consults the registry for each permanent's
/// abilities and calls this helper.
pub fn collect_triggers_for_event<'a>(
    abilities: impl IntoIterator<Item = (ObjectId, PlayerId, &'a TriggeredAbilityDef)>,
    event: &GameEvent,
    state: &GameState,
    reg: &crate::registry::CardRegistry,
) -> Vec<PendingTrigger> {
    let mut out: Vec<PendingTrigger> = abilities.into_iter()
        .filter_map(|(source, ctrl, def)|
            def.should_fire(event, source, ctrl, state, reg))
        .collect();
    sort_by_apnap(&mut out, state.active_player(), state.num_players());
    out
}

// =============================================================================
// Delayed triggers (CR 603.7)
// =============================================================================

/// A trigger scheduled by a resolving spell or ability — "at the
/// beginning of the next end step, return the exiled card to its
/// owner's hand". When an event matching [`Self::condition`] fires,
/// the trigger is put on the stack like a normal triggered ability
/// and (by default) is removed from `GameState.delayed_triggers`.
///
/// Fields mirror [`TriggeredAbilityDef`] minus the zone/frequency
/// bookkeeping, which doesn't apply to delayed triggers — they
/// disappear after firing.
///
/// **TODO(serialize)**: `effect` and `intervening_if` are bare `fn`
/// pointers, so this struct can't derive serde yet. Same migration
/// plan as `TriggeredAbilityDef`.
#[derive(Clone, Debug)]
pub struct DelayedTrigger {
    pub source: ObjectId,
    pub controller: PlayerId,
    pub condition: TriggerCondition,
    pub intervening_if: Option<InterveningIfFn>,
    pub effect: EffectFn,
    /// Fire once and remove. Rare triggers ("at the beginning of each
    /// end step for the rest of the game") set this `false`.
    pub fire_once: bool,
    /// "…this turn" duration: the trigger lapses (is removed without
    /// firing) at the turn boundary if it hasn't fired. The engine's
    /// turn-start hook retains only non-expiring delayed triggers.
    pub expires_end_of_turn: bool,
    /// "Until your next turn, whenever …" floating windows: the
    /// trigger lapses when this player's turn BEGINS (engine
    /// turn-start hook). Pair with `fire_once: false` for the
    /// repeating form (Don't Move, Tamiyo Meets the Story Circle).
    pub expires_at_turn_of: Option<PlayerId>,
}

impl DelayedTrigger {
    /// A standard one-shot delayed trigger.
    pub fn one_shot(
        source: ObjectId,
        controller: PlayerId,
        condition: TriggerCondition,
        effect: EffectFn,
    ) -> Self {
        Self {
            source, controller, condition, effect,
            intervening_if: None,
            fire_once: true,
            expires_end_of_turn: false,
            expires_at_turn_of: None,
        }
    }

    /// A REPEATING floating trigger window — "until [player]'s next
    /// turn, whenever [condition], [effect]". Fires every match until
    /// that player's turn begins.
    pub fn repeating_until_turn_of(
        source: ObjectId,
        controller: PlayerId,
        condition: TriggerCondition,
        effect: EffectFn,
        player: PlayerId,
    ) -> Self {
        Self {
            fire_once: false,
            expires_at_turn_of: Some(player),
            ..Self::one_shot(source, controller, condition, effect)
        }
    }

    /// A REPEATING floating window that lapses at end of turn —
    /// "until end of turn, whenever [condition], [effect]".
    pub fn repeating_this_turn(
        source: ObjectId,
        controller: PlayerId,
        condition: TriggerCondition,
        effect: EffectFn,
    ) -> Self {
        Self {
            fire_once: false,
            expires_end_of_turn: true,
            ..Self::one_shot(source, controller, condition, effect)
        }
    }

    /// A one-shot delayed trigger that lapses at end of turn — "when
    /// you next cast a creature spell THIS TURN, …" (CR 603.7e).
    pub fn one_shot_this_turn(
        source: ObjectId,
        controller: PlayerId,
        condition: TriggerCondition,
        effect: EffectFn,
    ) -> Self {
        Self {
            expires_end_of_turn: true,
            ..Self::one_shot(source, controller, condition, effect)
        }
    }
}

// =============================================================================
// GameState integration
// =============================================================================

impl GameState {
    /// Register a delayed trigger. It remains on the state until it
    /// either fires (see [`Self::take_matching_delayed_triggers`]) or
    /// is removed explicitly.
    pub fn register_delayed_trigger(&mut self, trigger: DelayedTrigger) {
        self.delayed_triggers.push(trigger);
    }

    /// Indices of delayed triggers whose condition matches `event`,
    /// in registration (FIFO) order.
    pub fn match_delayed_triggers(
        &self,
        event: &GameEvent,
        reg: &crate::registry::CardRegistry,
    ) -> Vec<usize> {
        self.delayed_triggers.iter().enumerate()
            .filter_map(|(i, t)| {
                if t.condition.matches(event, t.source, t.controller, self) {
                    // Intervening-if runs at both stack-add and resolve;
                    // check it here as the stack-add check.
                    if let Some(f) = t.intervening_if {
                        if !f(self, t.source, t.controller, reg) { return None; }
                    }
                    Some(i)
                } else { None }
            })
            .collect()
    }

    /// Collect delayed triggers matching `event` into `PendingTrigger`s
    /// and remove the ones marked `fire_once` from
    /// `self.delayed_triggers`. Returned triggers are APNAP-sorted.
    pub fn take_matching_delayed_triggers(
        &mut self,
        event: &GameEvent,
        reg: &crate::registry::CardRegistry,
    ) -> Vec<PendingTrigger> {
        let indices = self.match_delayed_triggers(event, reg);
        // Build pending triggers first (needs indexed access).
        let mut out: Vec<PendingTrigger> = indices.iter().map(|&i| {
            let t = &self.delayed_triggers[i];
            PendingTrigger {
                source: t.source,
                trigger_id: 0, // delayed triggers have no persistent id
                controller: t.controller,
                trigger_event: event.clone(),
                targets: crate::targets::TargetSelection::new(),
                // Delayed triggers have no registry def to dispatch
                // from at resolution — the fn rides the trigger.
                effect_override: Some(t.effect),
            }
        }).collect();
        sort_by_apnap(&mut out, self.active_player(), self.num_players());

        // Remove one-shot ones in reverse so earlier indices stay valid.
        let mut to_remove: Vec<usize> = indices.into_iter()
            .filter(|&i| self.delayed_triggers[i].fire_once)
            .collect();
        to_remove.sort_unstable();
        for i in to_remove.into_iter().rev() {
            self.delayed_triggers.remove(i);
        }
        out
    }

    // --- Frequency bookkeeping -----------------------------------------

    /// Has (`source`, `trigger_id`) still got fires left under
    /// `frequency`? Does not record a fire — use
    /// [`Self::record_trigger_fired`] once the engine actually puts
    /// the trigger on the stack.
    pub fn trigger_budget_allows(
        &self,
        source: ObjectId,
        trigger_id: TriggerId,
        frequency: TriggerFrequency,
    ) -> bool {
        match frequency {
            TriggerFrequency::EachTime => true,
            TriggerFrequency::OncePerTurn => {
                self.triggers_fired_this_turn.get(&(source, trigger_id))
                    .copied().unwrap_or(0) == 0
            }
            TriggerFrequency::OncePerGame => {
                self.triggers_fired_this_game.get(&(source, trigger_id))
                    .copied().unwrap_or(0) == 0
            }
        }
    }

    /// Record that (`source`, `trigger_id`) fired, for frequency
    /// bookkeeping. Called by the engine after pushing the trigger
    /// onto the stack.
    pub fn record_trigger_fired(&mut self, source: ObjectId, trigger_id: TriggerId) {
        *self.triggers_fired_this_turn.entry((source, trigger_id)).or_insert(0) += 1;
        *self.triggers_fired_this_game.entry((source, trigger_id)).or_insert(0) += 1;
    }

    /// Clear the per-turn trigger ledger. Called by the engine at the
    /// start of each new turn.
    pub fn clear_per_turn_trigger_ledger(&mut self) {
        self.triggers_fired_this_turn.clear();
    }
}

// New state fields needed by frequency tracking.
//
// The `GameState` struct itself lives in `state.rs`; the two
// HashMaps are added there. We expose their aliases here so the
// struct field types read naturally in state.rs.
pub type TriggerLedger = HashMap<(ObjectId, TriggerId), u32>;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effects::Effect;
    use crate::events::{DamageTarget, MoveCause};
    use crate::objects::{Characteristics, GameObject};
    use crate::turn::{Phase, Step};

    fn creature_chars() -> Characteristics {
        Characteristics {
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        }
    }

    fn put_creature(state: &mut GameState, owner: PlayerId, zone: Zone) -> ObjectId {
        let id = state.allocate_object_id();
        let mut obj = GameObject::new(id, owner, zone, 1, creature_chars());
        obj.controller = owner;
        state.objects.insert(obj);
        id
    }

    fn no_effect(_: &GameState, _: &PendingTrigger, _: &crate::registry::CardRegistry) -> Vec<Effect> {
        Vec::new()
    }

    fn draw_card_effect(_: &GameState, t: &PendingTrigger, _: &crate::registry::CardRegistry) -> Vec<Effect> {
        vec![Effect::DrawCards { player: t.controller, count: 1 }]
    }

    // --- TriggerCondition::matches ------------------------------------------

    #[test]
    fn filtered_becomes_blocked_conditions_check_the_paired_creature() {
        let mut s = GameState::new(2, 0);
        let me = put_creature(&mut s, 0, Zone::Battlefield);
        // An artifact-creature blocker and a plain-creature blocker.
        let artifact_blocker = {
            let id = s.allocate_object_id();
            let mut chars = creature_chars();
            chars.types = crate::types::TypeLine(
                crate::types::TypeLine::CREATURE | crate::types::TypeLine::ARTIFACT).into();
            let mut obj = GameObject::new(id, 1, Zone::Battlefield, 1, chars);
            obj.controller = 1;
            s.objects.insert(obj);
            id
        };
        let plain_blocker = put_creature(&mut s, 1, Zone::Battlefield);

        let by_artifact = TriggerCondition::SelfBecomesBlockedBy {
            filter: ObjectFilter::new().with_types(
                crate::types::TypeLine::ARTIFACT.into()),
        };
        // Blocked by the plain creature only: no match.
        let ev = GameEvent::CreatureBlocked {
            attacker: me, blockers: vec![plain_blocker] };
        assert!(!by_artifact.matches(&ev, me, 0, &s));
        // Blocked by both — at least one matches: fires.
        let ev = GameEvent::CreatureBlocked {
            attacker: me, blockers: vec![plain_blocker, artifact_blocker] };
        assert!(by_artifact.matches(&ev, me, 0, &s));
        // Someone ELSE blocked by an artifact: not this source.
        let ev = GameEvent::CreatureBlocked {
            attacker: plain_blocker, blockers: vec![artifact_blocker] };
        assert!(!by_artifact.matches(&ev, me, 0, &s));

        // Either-direction form: as a blocker, the ATTACKER is checked.
        let either = TriggerCondition::SelfBlocksOrBecomesBlockedBy {
            filter: ObjectFilter::new().with_types(
                crate::types::TypeLine::ARTIFACT.into()),
        };
        let ev = GameEvent::CreatureBlocks {
            blocker: me, attacker: artifact_blocker };
        assert!(either.matches(&ev, me, 0, &s));
        let ev = GameEvent::CreatureBlocks {
            blocker: me, attacker: plain_blocker };
        assert!(!either.matches(&ev, me, 0, &s));
    }

    #[test]
    fn self_enters_battlefield_matches_on_own_etb() {
        let s = GameState::new(2, 0);
        let src = 42;
        let event = GameEvent::EntersBattlefield {
            object_id: src,
            from_zone: Zone::Hand(0),
            was_cast: true,
        };
        assert!(TriggerCondition::SelfEntersBattlefield.matches(&event, src, 0, &s));
    }

    #[test]
    fn self_enters_battlefield_ignores_other_etb() {
        let s = GameState::new(2, 0);
        let event = GameEvent::EntersBattlefield {
            object_id: 99,
            from_zone: Zone::Hand(0),
            was_cast: false,
        };
        assert!(!TriggerCondition::SelfEntersBattlefield.matches(&event, 42, 0, &s));
    }

    #[test]
    fn self_enters_untapped_matches_only_when_untapped() {
        use crate::objects::{Characteristics, GameObject};
        let mut s = GameState::new(2, 0);
        let chars = Characteristics {
            types: crate::types::TypeLine::LAND.into(),
            ..Default::default()
        };
        let id = s.allocate_object_id();
        s.objects.insert(GameObject::new(id, 0, Zone::Battlefield, 1, chars));
        let event = GameEvent::EntersBattlefield {
            object_id: id, from_zone: Zone::Stack, was_cast: true,
        };
        // Entered untapped → the trigger fires.
        assert!(TriggerCondition::SelfEntersBattlefieldUntapped
            .matches(&event, id, 0, &s),
            "untapped entry fires the trigger");
        // Tap it (as the enters-tapped clause would) → it does not fire.
        s.objects.get_mut(id).unwrap().tap();
        assert!(!TriggerCondition::SelfEntersBattlefieldUntapped
            .matches(&event, id, 0, &s),
            "a tapped entry does not fire the trigger");
    }

    #[test]
    fn self_dies_matches() {
        let s = GameState::new(2, 0);
        let event = GameEvent::Dies { object_id: 1 };
        assert!(TriggerCondition::SelfDies.matches(&event, 1, 0, &s));
        assert!(!TriggerCondition::SelfDies.matches(&event, 2, 0, &s));
    }

    #[test]
    fn attached_creature_does_substitutes_host() {
        // An Aura (id=aura) attached to a creature (id=host). Its
        // AttachedCreatureDoes(SelfDies) fires when the HOST dies, not
        // when the Aura itself dies, and not while unattached.
        let mut s = GameState::new(2, 0);
        let host = put_creature(&mut s, 0, Zone::Battlefield);
        let aura = put_creature(&mut s, 0, Zone::Battlefield); // stand-in object
        let cond = TriggerCondition::AttachedCreatureDoes {
            condition: Box::new(TriggerCondition::SelfDies),
        };

        // Unattached: inert.
        let host_dies = GameEvent::Dies { object_id: host };
        assert!(!cond.matches(&host_dies, aura, 0, &s));

        // Attach the Aura to the host.
        s.objects.get_mut(aura).unwrap().attached_to = Some(host);

        // Host dies → the Aura's trigger fires.
        assert!(cond.matches(&host_dies, aura, 0, &s));
        // The Aura itself dying does NOT fire this trigger.
        let aura_dies = GameEvent::Dies { object_id: aura };
        assert!(!cond.matches(&aura_dies, aura, 0, &s));
    }

    #[test]
    fn self_leaves_battlefield_matches_any_destination() {
        let s = GameState::new(2, 0);
        for dest in [Zone::Graveyard(0), Zone::Exile, Zone::Hand(0)] {
            let ev = GameEvent::LeavesBattlefield { object_id: 7, destination: dest };
            assert!(TriggerCondition::SelfLeavesBattlefield.matches(&ev, 7, 0, &s));
            assert!(!TriggerCondition::SelfLeavesBattlefield.matches(&ev, 8, 0, &s));
        }
        // Does NOT fire on a plain Dies-only consumer's event shape.
        let dies = GameEvent::Dies { object_id: 7 };
        assert!(!TriggerCondition::SelfLeavesBattlefield.matches(&dies, 7, 0, &s));
    }

    #[test]
    fn attached_creature_does_keys_step_on_host_controller() {
        // "At the beginning of enchanted creature's controller's upkeep"
        // — AttachedCreatureDoes(StepBegins{Upkeep, You}) must fire on
        // the HOST's controller's upkeep, not the aura controller's.
        use crate::turn::Step;
        use crate::targets::ControllerConstraint;
        let mut s = GameState::new(2, 0);
        let host = put_creature(&mut s, 1, Zone::Battlefield); // controlled by player 1
        let aura = put_creature(&mut s, 0, Zone::Battlefield);  // aura controlled by player 0
        s.objects.get_mut(aura).unwrap().attached_to = Some(host);
        let cond = TriggerCondition::AttachedCreatureDoes {
            condition: Box::new(TriggerCondition::StepBegins {
                step: Step::Upkeep, whose: ControllerConstraint::You,
            }),
        };
        let ev = GameEvent::StepBegins { step: Step::Upkeep };

        // Host's controller (1) is the active player → fires.
        s.turn.active_player = 1;
        assert!(cond.matches(&ev, aura, 0, &s), "fires on enchanted creature's controller's upkeep");
        // Aura controller (0) is active → does NOT fire.
        s.turn.active_player = 0;
        assert!(!cond.matches(&ev, aura, 0, &s), "not on the aura controller's upkeep");
    }

    #[test]
    fn self_becomes_blocked_matches() {
        let s = GameState::new(2, 0);
        let ev = GameEvent::CreatureBlocked { attacker: 7, blockers: vec![8, 9] };
        assert!(TriggerCondition::SelfBecomesBlocked.matches(&ev, 7, 0, &s));
        assert!(!TriggerCondition::SelfBecomesBlocked.matches(&ev, 8, 0, &s));
    }

    #[test]
    fn self_blocks_matches() {
        let s = GameState::new(2, 0);
        let ev = GameEvent::CreatureBlocks { blocker: 8, attacker: 7 };
        assert!(TriggerCondition::SelfBlocks.matches(&ev, 8, 0, &s));
        // Source is the attacker, not the blocker — doesn't fire.
        assert!(!TriggerCondition::SelfBlocks.matches(&ev, 7, 0, &s));
    }

    #[test]
    fn self_blocks_or_becomes_blocked_matches_either_side() {
        let s = GameState::new(2, 0);
        // We are blocking — source is the blocker.
        let ev = GameEvent::CreatureBlocks { blocker: 8, attacker: 7 };
        assert!(TriggerCondition::SelfBlocksOrBecomesBlocked
            .matches(&ev, 8, 0, &s));
        // We are blocked — source is the attacker.
        let ev = GameEvent::CreatureBlocked { attacker: 7, blockers: vec![8] };
        assert!(TriggerCondition::SelfBlocksOrBecomesBlocked
            .matches(&ev, 7, 0, &s));
        // We are not in the event at all.
        assert!(!TriggerCondition::SelfBlocksOrBecomesBlocked
            .matches(&ev, 99, 0, &s));
    }

    #[test]
    fn other_combatant_accessor_returns_partner() {
        // Blocking side: partner is the attacker.
        let trig = PendingTrigger {
            source: 8, trigger_id: 0, controller: 0,
            trigger_event: GameEvent::CreatureBlocks {
                blocker: 8, attacker: 7,
            },
            targets: crate::targets::TargetSelection::new(),
            effect_override: None,
        };
        assert_eq!(trig.other_combatant(), Some(7));

        // Blocked side: partner is the first blocker.
        let trig = PendingTrigger {
            source: 7, trigger_id: 0, controller: 0,
            trigger_event: GameEvent::CreatureBlocked {
                attacker: 7, blockers: vec![8, 9, 10],
            },
            targets: crate::targets::TargetSelection::new(),
            effect_override: None,
        };
        assert_eq!(trig.other_combatant(), Some(8));

        // Non-block event: None.
        let trig = PendingTrigger {
            source: 1, trigger_id: 0, controller: 0,
            trigger_event: GameEvent::Dies { object_id: 1 },
            targets: crate::targets::TargetSelection::new(),
            effect_override: None,
        };
        assert_eq!(trig.other_combatant(), None);
    }

    #[test]
    fn attacking_creature_accessor_returns_attacker() {
        let trig = PendingTrigger {
            source: 3, trigger_id: 0, controller: 0,
            trigger_event: GameEvent::CreatureAttacks {
                attacker: 7,
                defending: crate::combat::DefendingEntity::Player(1),
            },
            targets: crate::targets::TargetSelection::new(),
            effect_override: None,
        };
        assert_eq!(trig.attacking_creature(), Some(7));

        // Non-attack event: None.
        let trig = PendingTrigger {
            source: 3, trigger_id: 0, controller: 0,
            trigger_event: GameEvent::Dies { object_id: 7 },
            targets: crate::targets::TargetSelection::new(),
            effect_override: None,
        };
        assert_eq!(trig.attacking_creature(), None);
    }

    #[test]
    fn self_becomes_tapped_matches() {
        let s = GameState::new(2, 0);
        let ev = GameEvent::Tapped { object_id: 42 };
        assert!(TriggerCondition::SelfBecomesTapped.matches(&ev, 42, 0, &s));
        assert!(!TriggerCondition::SelfBecomesTapped.matches(&ev, 7, 0, &s));
    }

    #[test]
    fn self_transforms_face_gate_reads_live_state() {
        let mut s = GameState::new(2, 0);
        let wolf = put_creature(&mut s, 0, Zone::Battlefield);
        // Object just transformed and is now showing the back face.
        s.objects.get_mut(wolf).unwrap().visible_face = 1;
        let ev = GameEvent::Transformed { object_id: wolf };

        let any_dir = TriggerCondition::SelfTransforms { to_face: None };
        assert!( any_dir.matches(&ev, wolf, 0, &s));
        assert!(!any_dir.matches(&ev, 999, 0, &s));
        let into_back = TriggerCondition::SelfTransforms { to_face: Some(1) };
        assert!(into_back.matches(&ev, wolf, 0, &s));
        let into_front = TriggerCondition::SelfTransforms { to_face: Some(0) };
        assert!(!into_front.matches(&ev, wolf, 0, &s));
    }

    #[test]
    fn filtered_attacks_alone_checks_the_sole_attacker() {
        use crate::combat::{AttackerDeclaration, DefendingEntity};
        let mut s = GameState::new(2, 0);
        let mine = put_creature(&mut s, 0, Zone::Battlefield);
        let theirs = put_creature(&mut s, 1, Zone::Battlefield);
        let cond = TriggerCondition::AttacksAlone {
            filter: ObjectFilter::creature()
                .controlled_by(ControllerConstraint::You),
        };
        // My creature attacks alone: fires (source 99 is a watcher).
        let ev = GameEvent::AttacksDeclared { attackers: vec![
            AttackerDeclaration { attacker: mine, defending: DefendingEntity::Player(1) },
        ]};
        assert!(cond.matches(&ev, 99, 0, &s));
        // Accessor reads the sole attacker.
        let trig = PendingTrigger {
            source: 99, trigger_id: 0, controller: 0,
            trigger_event: ev, targets: crate::targets::TargetSelection::new(),
            effect_override: None,
        };
        assert_eq!(trig.lone_attacker(), Some(mine));
        // Opponent's creature attacking alone: filter rejects.
        let ev = GameEvent::AttacksDeclared { attackers: vec![
            AttackerDeclaration { attacker: theirs, defending: DefendingEntity::Player(0) },
        ]};
        assert!(!cond.matches(&ev, 99, 0, &s));
        // Two attackers: not alone.
        let ev = GameEvent::AttacksDeclared { attackers: vec![
            AttackerDeclaration { attacker: mine, defending: DefendingEntity::Player(1) },
            AttackerDeclaration { attacker: theirs, defending: DefendingEntity::Player(1) },
        ]};
        assert!(!cond.matches(&ev, 99, 0, &s));
    }

    #[test]
    fn self_attacks_alone_requires_a_sole_attacker() {
        use crate::combat::{AttackerDeclaration, DefendingEntity};
        let s = GameState::new(2, 0);
        let solo = GameEvent::AttacksDeclared { attackers: vec![
            AttackerDeclaration { attacker: 7, defending: DefendingEntity::Player(1) },
        ]};
        assert!( TriggerCondition::SelfAttacksAlone.matches(&solo, 7, 0, &s));
        assert!(!TriggerCondition::SelfAttacksAlone.matches(&solo, 8, 0, &s));
        let pair = GameEvent::AttacksDeclared { attackers: vec![
            AttackerDeclaration { attacker: 7, defending: DefendingEntity::Player(1) },
            AttackerDeclaration { attacker: 8, defending: DefendingEntity::Player(1) },
        ]};
        assert!(!TriggerCondition::SelfAttacksAlone.matches(&pair, 7, 0, &s));
    }

    #[test]
    fn becomes_tapped_filter_checks_the_tapped_object() {
        let mut s = GameState::new(2, 0);
        let my_creature = put_creature(&mut s, 0, Zone::Battlefield);
        let opp_creature = put_creature(&mut s, 1, Zone::Battlefield);

        // "Whenever a creature an opponent controls becomes tapped"
        // (source controlled by player 0).
        let cond = TriggerCondition::BecomesTapped {
            filter: ObjectFilter::creature()
                .controlled_by(ControllerConstraint::Opponent),
        };
        let ev = GameEvent::Tapped { object_id: opp_creature };
        assert!(cond.matches(&ev, 99, 0, &s));
        let ev = GameEvent::Tapped { object_id: my_creature };
        assert!(!cond.matches(&ev, 99, 0, &s));
        // Non-tap event: no match.
        let ev = GameEvent::Dies { object_id: opp_creature };
        assert!(!cond.matches(&ev, 99, 0, &s));
    }

    #[test]
    fn self_is_dealt_damage_combat_only_flag() {
        let s = GameState::new(2, 0);
        let combat = GameEvent::DamageDealt {
            source: 1,
            target: crate::events::DamageTarget::Object(42),
            amount: 3,
            is_combat: true,
        };
        let noncombat = GameEvent::DamageDealt {
            source: 1,
            target: crate::events::DamageTarget::Object(42),
            amount: 3,
            is_combat: false,
        };
        // combat_only=false accepts both.
        let any = TriggerCondition::SelfIsDealtDamage { combat_only: false };
        assert!(any.matches(&combat, 42, 0, &s));
        assert!(any.matches(&noncombat, 42, 0, &s));
        // combat_only=true accepts only combat damage.
        let only = TriggerCondition::SelfIsDealtDamage { combat_only: true };
        assert!(only.matches(&combat, 42, 0, &s));
        assert!(!only.matches(&noncombat, 42, 0, &s));
        // Damage to a different object isn't us.
        assert!(!any.matches(&combat, 7, 0, &s));
        // Damage to a player isn't us.
        let to_player = GameEvent::DamageDealt {
            source: 1,
            target: crate::events::DamageTarget::Player(0),
            amount: 3,
            is_combat: true,
        };
        assert!(!any.matches(&to_player, 42, 0, &s));
    }

    #[test]
    fn self_attacks_unblocked_matches() {
        let s = GameState::new(2, 0);
        let ev = GameEvent::CreatureNotBlocked { attacker: 7 };
        assert!(TriggerCondition::SelfAttacksUnblocked.matches(&ev, 7, 0, &s));
        assert!(!TriggerCondition::SelfAttacksUnblocked.matches(&ev, 8, 0, &s));
        // A BLOCKED-creature event must NOT match unblocked.
        let blocked = GameEvent::CreatureBlocked { attacker: 7, blockers: vec![8] };
        assert!(!TriggerCondition::SelfAttacksUnblocked.matches(&blocked, 7, 0, &s));
    }

    #[test]
    fn zone_change_from_to_filter() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Graveyard(0));
        let event = GameEvent::ZoneChange {
            object_id: c,
            from: Zone::Battlefield,
            to: Zone::Graveyard(0),
            new_id: c,
            cause: MoveCause::StateBasedAction,
        };
        let cond = TriggerCondition::ZoneChange {
            filter: ObjectFilter::creature(),
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard(0),
        };
        assert!(cond.matches(&event, 999, 0, &s));

        // Wrong `to`.
        let cond = TriggerCondition::ZoneChange {
            filter: ObjectFilter::creature(),
            from: Some(Zone::Battlefield),
            to: Zone::Exile,
        };
        assert!(!cond.matches(&event, 999, 0, &s));
    }

    #[test]
    fn zone_change_graveyard_to_any_owner() {
        // Filter "into any graveyard" → `to: Graveyard(arbitrary)` uses
        // `same_kind` semantics.
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 1, Zone::Graveyard(1));
        let event = GameEvent::ZoneChange {
            object_id: c,
            from: Zone::Battlefield,
            to: Zone::Graveyard(1),
            new_id: c,
            cause: MoveCause::StateBasedAction,
        };
        let cond = TriggerCondition::ZoneChange {
            filter: ObjectFilter::creature(),
            from: None,
            to: Zone::Graveyard(0),   // player id doesn't matter post same_kind
        };
        assert!(cond.matches(&event, 999, 0, &s));
    }

    #[test]
    fn spell_cast_from_zone_matches_origin() {
        use crate::targets::ControllerConstraint;
        let mut s = GameState::new(2, 0);
        // A spell on the stack, cast from exile (foretell / impulse).
        let mut entry = crate::stack::StackEntry::new_spell(
            5, 0, 0, crate::objects::Characteristics::default(),
            crate::targets::TargetSelection::new(), vec![], None);
        entry.cast_from_zone = Zone::Exile;
        s.push_stack_entry(entry);
        let event = GameEvent::SpellCast {
            object_id: 5, card_id: 0, controller: 0,
            targets: crate::targets::TargetSelection::new(), mana_spent: 0 };

        let from_exile = TriggerCondition::SpellCastFromZone {
            filter: None, caster: ControllerConstraint::You, from_zone: Zone::Exile };
        let from_gy = TriggerCondition::SpellCastFromZone {
            filter: None, caster: ControllerConstraint::You,
            from_zone: Zone::Graveyard(0) };
        // source_controller 0 = "you"; the spell's controller is 0.
        assert!( from_exile.matches(&event, 999, 0, &s), "cast from exile matches");
        assert!(!from_gy.matches(&event, 999, 0, &s), "exile != graveyard");
        // Opponent-cast: caster constraint You fails for source_controller 1.
        assert!(!from_exile.matches(&event, 999, 1, &s), "opponent's cast, You-gated");
    }

    #[test]
    fn spell_cast_caster_constraint() {
        let s = GameState::new(2, 0);
        let event = GameEvent::SpellCast {
            object_id: 5,
            card_id: 10,
            controller: 1,
            targets: crate::targets::TargetSelection::new(),
            mana_spent: 0,
        };
        // "Whenever your opponent casts a spell" — source_controller = 0
        let cond = TriggerCondition::SpellCast {
            filter: None,
            caster: ControllerConstraint::Opponent,
        };
        assert!(cond.matches(&event, 999, /*src_ctrl=*/ 0, &s));
        assert!(!cond.matches(&event, 999, /*src_ctrl=*/ 1, &s));
    }

    #[test]
    fn damage_dealt_combat_only() {
        let mut s = GameState::new(2, 0);
        let src = put_creature(&mut s, 0, Zone::Battlefield);
        let combat_event = GameEvent::DamageDealt {
            source: src, target: DamageTarget::Player(1), amount: 3, is_combat: true,
        };
        let non_combat_event = GameEvent::DamageDealt {
            source: src, target: DamageTarget::Player(1), amount: 3, is_combat: false,
        };
        let cond = TriggerCondition::DamageDealt {
            source_filter: ObjectFilter::default(),
            target_filter: TargetFilter::Player,
            combat_only: true,
        };
        assert!(cond.matches(&combat_event, 0, 0, &s));
        assert!(!cond.matches(&non_combat_event, 0, 0, &s));
    }

    #[test]
    fn step_begins_with_whose_constraint() {
        let s = GameState::new(2, 0);
        let event = GameEvent::StepBegins { step: Step::Upkeep };
        // "At the beginning of your upkeep" when I control the source
        // and the active player is me.
        let cond = TriggerCondition::StepBegins {
            step: Step::Upkeep,
            whose: ControllerConstraint::You,
        };
        assert!(cond.matches(&event, 0, /*my id=*/ 0, &s));
        assert!(!cond.matches(&event, 0, /*I'm not active=*/ 1, &s));
    }

    #[test]
    fn phase_begins_matches() {
        let s = GameState::new(2, 0);
        let event = GameEvent::PhaseBegins { phase: Phase::Ending };
        let cond = TriggerCondition::PhaseBegins {
            phase: Phase::Ending,
            whose: ControllerConstraint::Any,
        };
        assert!(cond.matches(&event, 0, 0, &s));
    }

    #[test]
    fn life_gained_whose_constraint() {
        let s = GameState::new(2, 0);
        let event = GameEvent::LifeGained { player: 0, amount: 2 };
        let cond = TriggerCondition::LifeGained {
            player: ControllerConstraint::You,
        };
        assert!(cond.matches(&event, 0, 0, &s));
        assert!(!cond.matches(&event, 0, 1, &s));
    }

    #[test]
    fn dice_rolled_whose_constraint() {
        let s = GameState::new(2, 0);
        let event = GameEvent::DieRolled { player: 0, sides: 20, result: 17 };
        let yours = TriggerCondition::DiceRolled { player: ControllerConstraint::You };
        // Fires for the roller, not for the other player's perspective.
        assert!(yours.matches(&event, 0, 0, &s));
        assert!(!yours.matches(&event, 0, 1, &s));
        // Any-roller form fires regardless of whose controller.
        let any = TriggerCondition::DiceRolled { player: ControllerConstraint::Any };
        assert!(any.matches(&event, 0, 1, &s));
        // A non-DieRolled event never matches.
        let other = GameEvent::CoinFlipped { player: 0, won: true };
        assert!(!yours.matches(&other, 0, 0, &s));
    }

    #[test]
    fn counter_added_on_source() {
        let s = GameState::new(2, 0);
        let event = GameEvent::CounterAdded {
            object_id: 42, kind: CounterKind::PlusOnePlusOne, count: 1,
        };
        let cond = TriggerCondition::CounterAdded {
            on: TriggerSelf::Source,
            kind: Some(CounterKind::PlusOnePlusOne),
            chapter: None,
        };
        assert!(cond.matches(&event, 42, 0, &s));
        assert!(!cond.matches(&event, 99, 0, &s));
    }

    #[test]
    fn counter_added_kind_filter() {
        let s = GameState::new(2, 0);
        let event = GameEvent::CounterAdded {
            object_id: 42, kind: CounterKind::Loyalty, count: 1,
        };
        let cond = TriggerCondition::CounterAdded {
            on: TriggerSelf::Source,
            kind: Some(CounterKind::PlusOnePlusOne),
            chapter: None,
        };
        assert!(!cond.matches(&event, 42, 0, &s));
    }

    #[test]
    fn counter_added_chapter_dispatch_only_at_matching_count() {
        // Saga chapter dispatch (CR 716.5): chapter N fires only when
        // the lore-counter-add event's `count` equals N.
        let s = GameState::new(2, 0);
        let chapter_ii = TriggerCondition::CounterAdded {
            on: TriggerSelf::Source,
            kind: Some(CounterKind::Lore),
            chapter: Some(2),
        };
        let count_one = GameEvent::CounterAdded {
            object_id: 42, kind: CounterKind::Lore, count: 1,
        };
        let count_two = GameEvent::CounterAdded {
            object_id: 42, kind: CounterKind::Lore, count: 2,
        };
        let count_three = GameEvent::CounterAdded {
            object_id: 42, kind: CounterKind::Lore, count: 3,
        };
        assert!(!chapter_ii.matches(&count_one, 42, 0, &s));
        assert!( chapter_ii.matches(&count_two, 42, 0, &s));
        assert!(!chapter_ii.matches(&count_three, 42, 0, &s));
    }

    #[test]
    fn card_drawn_your_trigger() {
        let s = GameState::new(2, 0);
        let event = GameEvent::DrawCard { player: 0, object_id: 1 };
        let cond = TriggerCondition::CardDrawn { player: ControllerConstraint::You };
        assert!(cond.matches(&event, 0, 0, &s));
        assert!(!cond.matches(&event, 0, 1, &s));
    }

    #[test]
    fn custom_matcher_is_invoked() {
        let s = GameState::new(2, 0);
        fn only_on_source_7(_: &GameEvent, _: &GameState, src: ObjectId) -> bool {
            src == 7
        }
        let event = GameEvent::TurnEnds { player: 0 };
        assert!(TriggerCondition::Custom(only_on_source_7).matches(&event, 7, 0, &s));
        assert!(!TriggerCondition::Custom(only_on_source_7).matches(&event, 8, 0, &s));
    }

    // --- TriggeredAbilityDef::should_fire ----------------------------------

    #[test]
    fn should_fire_respects_zones() {
        let mut s = GameState::new(2, 0);
        let src = put_creature(&mut s, 0, Zone::Hand(0));
        let def = TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: no_effect,
            // Ability only functions on the battlefield.
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        };
        let event = GameEvent::Dies { object_id: src };
        assert!(def.should_fire(&event, src, 0, &s, &crate::registry::CardRegistry::new()).is_none());

        // Move to battlefield; now the zone gate passes.
        s.objects.get_mut(src).unwrap().zone = Zone::Battlefield;
        assert!(def.should_fire(&event, src, 0, &s, &crate::registry::CardRegistry::new()).is_some());
    }

    #[test]
    fn should_fire_respects_intervening_if() {
        let mut s = GameState::new(2, 0);
        let src = put_creature(&mut s, 0, Zone::Battlefield);
        fn always_false(_: &GameState, _: ObjectId, _: PlayerId, _: &crate::registry::CardRegistry) -> bool { false }
        let def = TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: Some(always_false),
            effect: no_effect,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        };
        let event = GameEvent::EntersBattlefield {
            object_id: src, from_zone: Zone::Hand(0), was_cast: true,
        };
        assert!(def.should_fire(&event, src, 0, &s, &crate::registry::CardRegistry::new()).is_none());
    }

    #[test]
    fn should_fire_respects_frequency_once_per_turn() {
        let mut s = GameState::new(2, 0);
        let src = put_creature(&mut s, 0, Zone::Battlefield);
        let def = TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: no_effect,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::OncePerTurn,
            target_requirements: Vec::new(),
        };
        let event = GameEvent::EntersBattlefield {
            object_id: src, from_zone: Zone::Hand(0), was_cast: true,
        };
        assert!(def.should_fire(&event, src, 0, &s, &crate::registry::CardRegistry::new()).is_some());
        // Simulate firing.
        s.record_trigger_fired(src, 1);
        assert!(def.should_fire(&event, src, 0, &s, &crate::registry::CardRegistry::new()).is_none());
        // New turn resets.
        s.clear_per_turn_trigger_ledger();
        assert!(def.should_fire(&event, src, 0, &s, &crate::registry::CardRegistry::new()).is_some());
    }

    #[test]
    fn should_fire_respects_frequency_once_per_game() {
        let mut s = GameState::new(2, 0);
        let src = put_creature(&mut s, 0, Zone::Battlefield);
        let def = TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: no_effect,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::OncePerGame,
            target_requirements: Vec::new(),
        };
        let event = GameEvent::EntersBattlefield {
            object_id: src, from_zone: Zone::Hand(0), was_cast: true,
        };
        assert!(def.should_fire(&event, src, 0, &s, &crate::registry::CardRegistry::new()).is_some());
        s.record_trigger_fired(src, 1);
        assert!(def.should_fire(&event, src, 0, &s, &crate::registry::CardRegistry::new()).is_none());
        s.clear_per_turn_trigger_ledger();
        // Still exhausted for the game.
        assert!(def.should_fire(&event, src, 0, &s, &crate::registry::CardRegistry::new()).is_none());
    }

    // --- APNAP sort --------------------------------------------------------

    #[test]
    fn sort_by_apnap_active_first() {
        let empty = crate::targets::TargetSelection::new();
        let mut triggers = vec![
            PendingTrigger { source: 1, trigger_id: 1, controller: 1,
                trigger_event: GameEvent::TurnEnds { player: 0 }, targets: empty.clone(),
                effect_override: None },
            PendingTrigger { source: 2, trigger_id: 2, controller: 0,
                trigger_event: GameEvent::TurnEnds { player: 0 }, targets: empty.clone(),
                effect_override: None },
            PendingTrigger { source: 3, trigger_id: 3, controller: 1,
                trigger_event: GameEvent::TurnEnds { player: 0 }, targets: empty.clone(),
                effect_override: None },
        ];
        sort_by_apnap(&mut triggers, /*active=*/ 0, /*N=*/ 2);
        let controllers: Vec<_> = triggers.iter().map(|t| t.controller).collect();
        assert_eq!(controllers, vec![0, 1, 1]);
        // Stable: the two P1 triggers stay in their original order.
        let ids: Vec<_> = triggers.iter().map(|t| t.source).collect();
        assert_eq!(ids, vec![2, 1, 3]);
    }

    #[test]
    fn sort_by_apnap_three_players() {
        let empty = crate::targets::TargetSelection::new();
        let mut triggers = vec![
            PendingTrigger { source: 1, trigger_id: 1, controller: 0,
                trigger_event: GameEvent::TurnEnds { player: 0 }, targets: empty.clone(),
                effect_override: None },
            PendingTrigger { source: 2, trigger_id: 2, controller: 2,
                trigger_event: GameEvent::TurnEnds { player: 0 }, targets: empty.clone(),
                effect_override: None },
            PendingTrigger { source: 3, trigger_id: 3, controller: 1,
                trigger_event: GameEvent::TurnEnds { player: 0 }, targets: empty.clone(),
                effect_override: None },
        ];
        // Active = 1, so APNAP order is 1, 2, 0.
        sort_by_apnap(&mut triggers, 1, 3);
        let controllers: Vec<_> = triggers.iter().map(|t| t.controller).collect();
        assert_eq!(controllers, vec![1, 2, 0]);
    }

    // --- collect_triggers_for_event ----------------------------------------

    #[test]
    fn collect_triggers_combines_match_and_sort() {
        let mut s = GameState::new(2, 0);
        let a = put_creature(&mut s, 0, Zone::Battlefield);
        let b = put_creature(&mut s, 1, Zone::Battlefield);
        let def = TriggeredAbilityDef {
            id: 7,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: draw_card_effect,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        };
        let event = GameEvent::StepBegins { step: Step::Upkeep };
        // Two copies of the ability, one controlled by each player.
        let triggers = collect_triggers_for_event(
            [(a, 0, &def), (b, 1, &def)],
            &event, &s, &crate::registry::CardRegistry::new(),
        );
        assert_eq!(triggers.len(), 2);
        // APNAP order with active=0 → controller 0 first.
        assert_eq!(triggers[0].controller, 0);
        assert_eq!(triggers[1].controller, 1);
    }

    // --- Delayed triggers ---------------------------------------------------

    #[test]
    fn register_and_match_delayed_trigger() {
        let mut s = GameState::new(2, 0);
        s.register_delayed_trigger(DelayedTrigger::one_shot(
            /*source=*/ 1,
            /*controller=*/ 0,
            TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::Any,
            },
            no_effect,
        ));
        let event = GameEvent::StepBegins { step: Step::End };
        let reg = crate::registry::CardRegistry::new();
        let matches = s.match_delayed_triggers(&event, &reg);
        assert_eq!(matches, vec![0]);

        // Non-matching event: draw step.
        let event2 = GameEvent::StepBegins { step: Step::Draw };
        assert!(s.match_delayed_triggers(&event2, &reg).is_empty());
    }

    #[test]
    fn take_matching_delayed_triggers_removes_one_shot() {
        let mut s = GameState::new(2, 0);
        s.register_delayed_trigger(DelayedTrigger::one_shot(
            1, 0,
            TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::Any,
            },
            no_effect,
        ));
        let event = GameEvent::StepBegins { step: Step::End };
        let reg = crate::registry::CardRegistry::new();
        let fired = s.take_matching_delayed_triggers(&event, &reg);
        assert_eq!(fired.len(), 1);
        assert!(s.delayed_triggers.is_empty());

        // Firing again matches nothing.
        let fired_again = s.take_matching_delayed_triggers(&event, &reg);
        assert!(fired_again.is_empty());
    }

    #[test]
    fn non_one_shot_delayed_trigger_persists() {
        let mut s = GameState::new(2, 0);
        let mut t = DelayedTrigger::one_shot(
            1, 0,
            TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::Any,
            },
            no_effect,
        );
        t.fire_once = false;
        s.register_delayed_trigger(t);

        let event = GameEvent::StepBegins { step: Step::End };
        let reg = crate::registry::CardRegistry::new();
        s.take_matching_delayed_triggers(&event, &reg);
        s.take_matching_delayed_triggers(&event, &reg);
        assert_eq!(s.delayed_triggers.len(), 1);
    }

    // --- TriggerSelf -------------------------------------------------------

    #[test]
    fn trigger_self_source_and_any_matching() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield);
        assert!(TriggerSelf::Source.matches(c, c, 0, &s));
        assert!(!TriggerSelf::Source.matches(c, 999, 0, &s));

        let ts = TriggerSelf::AnyMatching(ObjectFilter::creature());
        assert!(ts.matches(c, 0, 0, &s));
    }

    #[test]
    fn trigger_self_another_matching_excludes_source() {
        // Regression (random-game harness seed 295): Wildwood Scourge watches
        // "+1/+1 counter put on ANOTHER creature you control" and its effect
        // adds a counter to itself. AnyMatching would re-fire on the source's
        // own counter forever; AnotherMatching must exclude the source.
        let mut s = GameState::new(2, 0);
        let a = put_creature(&mut s, 0, Zone::Battlefield);
        let b = put_creature(&mut s, 0, Zone::Battlefield);
        let ts = TriggerSelf::AnotherMatching(ObjectFilter::creature());
        // Event on a DIFFERENT creature (b) with source=a → fires.
        assert!(ts.matches(b, a, 0, &s));
        // Event on the SOURCE itself (a) with source=a → must NOT fire
        // (this is the loop-breaker).
        assert!(!ts.matches(a, a, 0, &s));
    }

    // --- PendingTrigger accessors ------------------------------------------

    fn pending(event: GameEvent) -> PendingTrigger {
        PendingTrigger {
            source: 42,
            trigger_id: 1,
            controller: 0,
            trigger_event: event,
            targets: crate::targets::TargetSelection::new(),
            effect_override: None,
        }
    }

    #[test]
    fn pending_trigger_reads_mana_spent() {
        let pt = pending(GameEvent::SpellCast {
            object_id: 1, card_id: 0, controller: 0,
            targets: crate::targets::TargetSelection::new(), mana_spent: 5 });
        assert_eq!(pt.mana_spent(), Some(5));
        // Non-SpellCast event → None.
        assert_eq!(pending(GameEvent::Dies { object_id: 1 }).mana_spent(), None);
    }

    #[test]
    fn pending_dying_object_from_dies_or_zone_change() {
        let t = pending(GameEvent::Dies { object_id: 7 });
        assert_eq!(t.dying_object(), Some(7));
        let t = pending(GameEvent::ZoneChange {
            object_id: 9, from: Zone::Battlefield, to: Zone::Graveyard(0),
            new_id: 9, cause: crate::events::MoveCause::StateBasedAction,
        });
        assert_eq!(t.dying_object(), Some(9));
        // ZoneChange to exile is not "dying".
        let t = pending(GameEvent::ZoneChange {
            object_id: 9, from: Zone::Battlefield, to: Zone::Exile,
            new_id: 9, cause: crate::events::MoveCause::StateBasedAction,
        });
        assert_eq!(t.dying_object(), None);
        // Non-zone event with no death — None.
        let t = pending(GameEvent::TurnEnds { player: 0 });
        assert_eq!(t.dying_object(), None);
    }

    #[test]
    fn pending_damage_amount_and_damaged_player() {
        let t = pending(GameEvent::DamageDealt {
            source: 1,
            target: crate::events::DamageTarget::Player(2),
            amount: 5,
            is_combat: true,
        });
        assert_eq!(t.damage_amount(), Some(5));
        assert_eq!(t.damaged_player(), Some(2));
        // Damage to an object: amount present, damaged_player None.
        let t = pending(GameEvent::DamageDealt {
            source: 1,
            target: crate::events::DamageTarget::Object(99),
            amount: 3,
            is_combat: false,
        });
        assert_eq!(t.damage_amount(), Some(3));
        assert_eq!(t.damaged_player(), None);
        // Not a damage event: both None.
        let t = pending(GameEvent::Dies { object_id: 1 });
        assert_eq!(t.damage_amount(), None);
        assert_eq!(t.damaged_player(), None);
    }

    #[test]
    fn pending_defending_player_from_attack() {
        let t = pending(GameEvent::CreatureAttacks {
            attacker: 5,
            defending: crate::combat::DefendingEntity::Player(1),
        });
        assert_eq!(t.defending_player(), Some(1));
        // Planeswalker / battle defender: None.
        let t = pending(GameEvent::CreatureAttacks {
            attacker: 5,
            defending: crate::combat::DefendingEntity::Planeswalker(99),
        });
        assert_eq!(t.defending_player(), None);
    }

    #[test]
    fn pending_triggering_caster_from_spell_cast() {
        let t = pending(GameEvent::SpellCast {
            object_id: 5,
            card_id: 10,
            controller: 1,
            targets: crate::targets::TargetSelection::new(),
            mana_spent: 0,
        });
        assert_eq!(t.triggering_caster(), Some(1));
        let t = pending(GameEvent::Dies { object_id: 1 });
        assert_eq!(t.triggering_caster(), None);
    }

    #[test]
    fn pending_entering_object() {
        let t = pending(GameEvent::EntersBattlefield {
            object_id: 7, from_zone: Zone::Hand(0), was_cast: true,
        });
        assert_eq!(t.entering_object(), Some(7));
        let t = pending(GameEvent::ZoneChange {
            object_id: 9, from: Zone::Hand(0), to: Zone::Battlefield,
            new_id: 9, cause: crate::events::MoveCause::StateBasedAction,
        });
        assert_eq!(t.entering_object(), Some(9));
        // ZoneChange to graveyard is not "entering" the battlefield.
        let t = pending(GameEvent::ZoneChange {
            object_id: 9, from: Zone::Battlefield, to: Zone::Graveyard(0),
            new_id: 9, cause: crate::events::MoveCause::StateBasedAction,
        });
        assert_eq!(t.entering_object(), None);
    }

    #[test]
    fn zone_change_enter_uses_post_move_id() {
        // Ardoz, Cobbler of War: a creature you control enters -> pump IT +2/+0.
        // A real move re-ids the object (CR 400.7); entering_object() must return
        // the ON-BATTLEFIELD id (new_id), or a Pump lands on the stale pre-move id
        // and silently no-ops.
        let mut s = GameState::new(2, 0);
        let old = put_creature(&mut s, 0, Zone::Hand(0));
        let new_id = s
            .move_object_to_zone(old, Zone::Battlefield, MoveCause::StateBasedAction)
            .expect("moved to battlefield");
        assert_ne!(old, new_id, "entering re-ids the object");
        assert!(s.objects.get(new_id).map_or(false, |o| o.zone.is_battlefield()),
            "the entered creature lives at new_id");

        let ev = GameEvent::ZoneChange {
            object_id: old, from: Zone::Hand(0), to: Zone::Battlefield,
            new_id, cause: MoveCause::StateBasedAction,
        };
        let cond = TriggerCondition::ZoneChange {
            filter: crate::targets::ObjectFilter::creature()
                .controlled_by(crate::targets::ControllerConstraint::You),
            from: None,
            to: Zone::Battlefield,
        };
        assert!(cond.matches(&ev, 999, 0, &s),
            "'a creature you control enters' must fire");

        let t = PendingTrigger {
            source: 999,
            trigger_id: 1,
            controller: 0,
            trigger_event: ev,
            targets: Default::default(),
            effect_override: None,
        };
        assert_eq!(t.entering_object(), Some(new_id),
            "entering_object must be the on-battlefield id, not the stale pre-move id");
    }
}
