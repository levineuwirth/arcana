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

use std::collections::HashMap;

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
/// event), and a unit placeholder that keeps the signature stable for
/// the future `&CardRegistry` parameter.
pub type EffectFn = fn(&GameState, &PendingTrigger, &())
    -> Vec<crate::effects::Effect>;

/// Function pointer for intervening-if clauses (CR 603.4).
pub type InterveningIfFn = fn(&GameState) -> bool;

// TODO(serialize): `TriggeredAbilityDef` carries bare `fn` pointers
// (`intervening_if`, `effect`). Migrate to `ConditionFnId` /
// `EffectFnId` (addendum Section 12) in Phase 3.
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
            if !cond(state) { return None; }
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
    /// "When ~ dies".
    SelfDies,
    /// "When ~ attacks".
    SelfAttacks,
    /// "Whenever a creature enters the battlefield under your control".
    ZoneChange { filter: ObjectFilter, from: Option<Zone>, to: Zone },
    /// "Whenever you cast a spell" (optionally filtered).
    SpellCast { filter: Option<ObjectFilter>, caster: ControllerConstraint },
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
    /// "Whenever a counter is put on ~".
    CounterAdded { on: TriggerSelf, kind: Option<CounterKind> },
    /// "Whenever you draw a card".
    CardDrawn { player: ControllerConstraint },
    /// "Whenever an opponent discards a card".
    CardDiscarded { player: ControllerConstraint },
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

            SelfDies => matches!(event,
                GameEvent::Dies { object_id } if *object_id == source),

            SelfAttacks => matches!(event,
                GameEvent::CreatureAttacks { attacker, .. } if *attacker == source),

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
                            state, source_controller);
                    }
                });
                target_filter.matches(&choice, state, source_controller)
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

            CounterAdded { on, kind } => {
                let GameEvent::CounterAdded { object_id, kind: k, .. } = event
                    else { return false; };
                if let Some(want) = kind {
                    if k != want { return false; }
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
// a non-match.
fn match_filter_on(
    state: &GameState,
    id: ObjectId,
    filter: &ObjectFilter,
    source_controller: PlayerId,
) -> bool {
    state.objects.get(id)
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
) -> Vec<PendingTrigger> {
    let mut out: Vec<PendingTrigger> = abilities.into_iter()
        .filter_map(|(source, ctrl, def)|
            def.should_fire(event, source, ctrl, state))
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
    pub fn match_delayed_triggers(&self, event: &GameEvent) -> Vec<usize> {
        self.delayed_triggers.iter().enumerate()
            .filter_map(|(i, t)| {
                if t.condition.matches(event, t.source, t.controller, self) {
                    // Intervening-if runs at both stack-add and resolve;
                    // check it here as the stack-add check.
                    if let Some(f) = t.intervening_if {
                        if !f(self) { return None; }
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
    ) -> Vec<PendingTrigger> {
        let indices = self.match_delayed_triggers(event);
        // Build pending triggers first (needs indexed access).
        let mut out: Vec<PendingTrigger> = indices.iter().map(|&i| {
            let t = &self.delayed_triggers[i];
            PendingTrigger {
                source: t.source,
                trigger_id: 0, // delayed triggers have no persistent id
                controller: t.controller,
                trigger_event: event.clone(),
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

    fn no_effect(_: &GameState, _: &PendingTrigger, _: &()) -> Vec<Effect> {
        Vec::new()
    }

    fn draw_card_effect(_: &GameState, t: &PendingTrigger, _: &()) -> Vec<Effect> {
        vec![Effect::DrawCards { player: t.controller, count: 1 }]
    }

    // --- TriggerCondition::matches ------------------------------------------

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
    fn self_dies_matches() {
        let s = GameState::new(2, 0);
        let event = GameEvent::Dies { object_id: 1 };
        assert!(TriggerCondition::SelfDies.matches(&event, 1, 0, &s));
        assert!(!TriggerCondition::SelfDies.matches(&event, 2, 0, &s));
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
    fn spell_cast_caster_constraint() {
        let s = GameState::new(2, 0);
        let event = GameEvent::SpellCast {
            object_id: 5,
            card_id: 10,
            controller: 1,
            targets: crate::targets::TargetSelection::new(),
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
    fn counter_added_on_source() {
        let s = GameState::new(2, 0);
        let event = GameEvent::CounterAdded {
            object_id: 42, kind: CounterKind::PlusOnePlusOne, count: 1,
        };
        let cond = TriggerCondition::CounterAdded {
            on: TriggerSelf::Source,
            kind: Some(CounterKind::PlusOnePlusOne),
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
        };
        assert!(!cond.matches(&event, 42, 0, &s));
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
        };
        let event = GameEvent::Dies { object_id: src };
        assert!(def.should_fire(&event, src, 0, &s).is_none());

        // Move to battlefield; now the zone gate passes.
        s.objects.get_mut(src).unwrap().zone = Zone::Battlefield;
        assert!(def.should_fire(&event, src, 0, &s).is_some());
    }

    #[test]
    fn should_fire_respects_intervening_if() {
        let mut s = GameState::new(2, 0);
        let src = put_creature(&mut s, 0, Zone::Battlefield);
        fn always_false(_: &GameState) -> bool { false }
        let def = TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: Some(always_false),
            effect: no_effect,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
        };
        let event = GameEvent::EntersBattlefield {
            object_id: src, from_zone: Zone::Hand(0), was_cast: true,
        };
        assert!(def.should_fire(&event, src, 0, &s).is_none());
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
        };
        let event = GameEvent::EntersBattlefield {
            object_id: src, from_zone: Zone::Hand(0), was_cast: true,
        };
        assert!(def.should_fire(&event, src, 0, &s).is_some());
        // Simulate firing.
        s.record_trigger_fired(src, 1);
        assert!(def.should_fire(&event, src, 0, &s).is_none());
        // New turn resets.
        s.clear_per_turn_trigger_ledger();
        assert!(def.should_fire(&event, src, 0, &s).is_some());
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
        };
        let event = GameEvent::EntersBattlefield {
            object_id: src, from_zone: Zone::Hand(0), was_cast: true,
        };
        assert!(def.should_fire(&event, src, 0, &s).is_some());
        s.record_trigger_fired(src, 1);
        assert!(def.should_fire(&event, src, 0, &s).is_none());
        s.clear_per_turn_trigger_ledger();
        // Still exhausted for the game.
        assert!(def.should_fire(&event, src, 0, &s).is_none());
    }

    // --- APNAP sort --------------------------------------------------------

    #[test]
    fn sort_by_apnap_active_first() {
        let mut triggers = vec![
            PendingTrigger { source: 1, trigger_id: 1, controller: 1,
                trigger_event: GameEvent::TurnEnds { player: 0 } },
            PendingTrigger { source: 2, trigger_id: 2, controller: 0,
                trigger_event: GameEvent::TurnEnds { player: 0 } },
            PendingTrigger { source: 3, trigger_id: 3, controller: 1,
                trigger_event: GameEvent::TurnEnds { player: 0 } },
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
        let mut triggers = vec![
            PendingTrigger { source: 1, trigger_id: 1, controller: 0,
                trigger_event: GameEvent::TurnEnds { player: 0 } },
            PendingTrigger { source: 2, trigger_id: 2, controller: 2,
                trigger_event: GameEvent::TurnEnds { player: 0 } },
            PendingTrigger { source: 3, trigger_id: 3, controller: 1,
                trigger_event: GameEvent::TurnEnds { player: 0 } },
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
        };
        let event = GameEvent::StepBegins { step: Step::Upkeep };
        // Two copies of the ability, one controlled by each player.
        let triggers = collect_triggers_for_event(
            [(a, 0, &def), (b, 1, &def)],
            &event, &s,
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
        let matches = s.match_delayed_triggers(&event);
        assert_eq!(matches, vec![0]);

        // Non-matching event: draw step.
        let event2 = GameEvent::StepBegins { step: Step::Draw };
        assert!(s.match_delayed_triggers(&event2).is_empty());
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
        let fired = s.take_matching_delayed_triggers(&event);
        assert_eq!(fired.len(), 1);
        assert!(s.delayed_triggers.is_empty());

        // Firing again matches nothing.
        let fired_again = s.take_matching_delayed_triggers(&event);
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
        s.take_matching_delayed_triggers(&event);
        s.take_matching_delayed_triggers(&event);
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
}
