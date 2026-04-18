//! Replacement effects — CR 614.
//!
//! Phase 1 Task #18. Depends on tasks 5 (events), 6 (state),
//! 9 (targets), 13 (effects), 16 (combat — the `deal_damage` hook).
//!
//! # Model (CR 614)
//!
//! A **replacement effect** intercepts a would-be event before it
//! happens and modifies or cancels it. "Instead" text (614.1a) and
//! "would" text (614.1b) are the two grammatical forms. Unlike
//! triggered abilities, replacements don't stack — they apply as
//! part of the event itself.
//!
//! Key rules:
//! - **CR 614.5**: each replacement applies at most once to a given
//!   event.
//! - **CR 614.15**: self-replacement effects (coming from the
//!   affected object itself) apply before other replacements.
//! - When multiple replacements would apply to the same event, the
//!   affected player or controller chooses which to apply first
//!   (this Phase 1 impl picks by ordering within gathered
//!   candidates; the engine will upgrade to a real agent decision).
//!
//! # API
//!
//! - `ReplacementEffect` is registered via
//!   [`GameState::add_replacement_effect`] and tracked in
//!   `state.replacement_effects`.
//! - The **damage replacement pipeline** (the one wired in for
//!   Phase 1) runs transparently inside
//!   [`GameState::deal_damage`] — callers don't need to know. It
//!   accepts a proposed (source, target, amount) and returns the
//!   post-replacement tuple or `None` for full prevention.
//! - The **ETB replacement collector** gathers modifications
//!   (additional counters, enter-tapped) for an object about to
//!   enter the battlefield. The `move_object_to_zone` hook that
//!   folds these in arrives with the engine task (Task #20).
//!
//! # Fn-pointer policy
//!
//! [`ReplacementKind`] is a tagged enum for the common cases
//! (prevention, reduction, redirection, doubling, ETB-with-counters,
//! ETB-tapped, exile-instead-of-die). [`ReplacementKind::Custom`] and
//! [`ReplacementCondition::Custom`] retain `fn` pointers as escape
//! hatches. Serde roundtrip covers everything except the two
//! `Custom` variants — same `ConditionFnId` migration plan
//! (addendum Section 12).

use crate::combat::DamageTarget;
use crate::events::GameEvent;
use crate::objects::ObjectId;
use crate::state::GameState;
use crate::targets::{ObjectFilter, TargetChoice, TargetFilter};
use crate::types::*;

// =============================================================================
// CounterTarget — unified object-or-player target for counter placement
// =============================================================================

/// Target of a counter-placement event. Unified so proliferate and
/// other multi-entity counter effects can route through a single
/// pipeline regardless of whether the counters land on a permanent or
/// on a player (poison, energy).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CounterTarget {
    Object(ObjectId),
    Player(PlayerId),
}

/// Filter over the counter kind the replacement cares about.
/// Hardened Scales uses `Only(PlusOnePlusOne)`; Doubling Season /
/// Winding Constrictor use `Any`.
#[derive(Clone, Debug)]
pub enum CounterKindFilter {
    Any,
    Only(CounterKind),
}

impl CounterKindFilter {
    pub fn matches(&self, kind: CounterKind) -> bool {
        match self {
            CounterKindFilter::Any => true,
            CounterKindFilter::Only(k) => *k == kind,
        }
    }
}

// =============================================================================
// ReplacementEffect
// =============================================================================

// TODO(serialize): `ReplacementCondition::Custom` and
// `ReplacementKind::Custom` carry bare `fn` pointers. Migrate per
// addendum Section 12 in Phase 3.
#[derive(Clone, Debug)]
pub struct ReplacementEffect {
    /// Object that created this effect (Circle of Protection's
    /// permanent, Kalonian Hydra for its self-replacement, etc.).
    pub source: ObjectId,
    /// Monotonic id — doubles as the timestamp for ordering within
    /// a single resolution pass. Assigned by
    /// [`GameState::add_replacement_effect`].
    pub id: u64,
    pub condition: ReplacementCondition,
    pub kind: ReplacementKind,
    /// Per CR 614.15, a self-replacement from the affected object
    /// applies before other replacements. Kalonian Hydra's "enters
    /// with four +1/+1 counters" is self-replacement;
    /// Hardened Scales's "another +1/+1 counter" is not.
    pub is_self_replacement: bool,
    pub duration: ReplacementDuration,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReplacementDuration {
    /// No automatic expiry — lives until explicitly removed.
    Permanent,
    /// Removed at the cleanup step (CR 514.2).
    EndOfTurn,
    /// Removed when `source` leaves the battlefield.
    WhileSourceOnBattlefield,
}

// =============================================================================
// ReplacementCondition
// =============================================================================

/// What kind of would-be event can this replacement intercept?
#[derive(Clone, Debug)]
pub enum ReplacementCondition {
    /// "If a [source] would deal damage to [target], …"
    WouldDealDamage {
        source_filter: ObjectFilter,
        target_filter: TargetFilter,
    },
    /// "If a source would deal damage to `target` specifically, …"
    /// Used by spell-installed prevention/redirection shields (Healing
    /// Salve on a particular creature, Palisade Giant-style redirects).
    /// Matches regardless of damage source.
    WouldDealDamageToSpecific {
        target: DamageTarget,
    },
    /// "If a [matching] creature would enter the battlefield, …"
    WouldEnterBattlefield {
        object_filter: ObjectFilter,
    },
    /// "If a [matching] creature would die, …"
    WouldDie {
        object_filter: ObjectFilter,
    },
    /// "If this specific creature would die, …" — used by
    /// [`ReplacementKind::RegenerateShield`] and other single-entity
    /// die-time replacements.
    WouldDieSpecific {
        object_id: ObjectId,
    },
    /// "If a [matching] spell would be countered, …"
    WouldBeCountered {
        spell_filter: ObjectFilter,
    },
    /// "Whenever [player] would draw a card, …"
    WouldDrawCard {
        player: crate::targets::ControllerConstraint,
    },
    /// "At the beginning of [player]'s next turn, …"
    WouldBeginTurn {
        player: PlayerId,
    },
    /// "Whenever [controller] would create a token, …"
    WouldCreateToken {
        token_filter: ObjectFilter,
    },
    /// "If one or more [kind] counters would be placed on [object], …"
    /// (Hardened Scales, Doubling Season, Winding Constrictor).
    /// Matches only object targets — player-counter-placement
    /// replacements (rare / unused in current Standard) are not
    /// covered by this variant; add a sibling variant when needed.
    WouldPlaceCounters {
        object_filter: ObjectFilter,
        kinds: CounterKindFilter,
    },
    /// Custom predicate.
    Custom(fn(&ReplacementEvent, &GameState) -> bool),
}

impl ReplacementCondition {
    /// Does this condition match the in-flight event?
    pub fn matches(
        &self,
        event: &ReplacementEvent,
        source_controller: PlayerId,
        state: &GameState,
    ) -> bool {
        use ReplacementCondition::*;
        match (self, event) {
            (
                WouldDealDamage { source_filter, target_filter },
                ReplacementEvent::Damage { source, target, .. },
            ) => {
                let src_obj = state.objects.get(*source);
                let src_ok = match src_obj {
                    Some(o) => source_filter.matches(o, state, source_controller),
                    // If the source is gone, pass no filter by default.
                    None => source_filter_matches_by_default(source_filter),
                };
                if !src_ok { return false; }
                let target_choice = match target {
                    DamageTarget::Object(id) => TargetChoice::Object(*id),
                    DamageTarget::Player(p) => TargetChoice::Player(*p),
                };
                target_filter.matches(&target_choice, state, source_controller)
            }

            (
                WouldDealDamageToSpecific { target: shielded },
                ReplacementEvent::Damage { target, .. },
            ) => shielded == target,

            (
                WouldEnterBattlefield { object_filter },
                ReplacementEvent::EnterBattlefield { object_id },
            ) => {
                state.objects.get(*object_id)
                    .is_some_and(|o| object_filter.matches(o, state, source_controller))
            }

            (
                WouldDie { object_filter },
                ReplacementEvent::Die { object_id },
            ) => {
                state.objects.get(*object_id)
                    .is_some_and(|o| object_filter.matches(o, state, source_controller))
            }

            (
                WouldDieSpecific { object_id: shielded },
                ReplacementEvent::Die { object_id },
            ) => shielded == object_id,

            (
                WouldBeCountered { spell_filter },
                ReplacementEvent::CounterSpell { stack_entry_id },
            ) => {
                state.objects.get(*stack_entry_id)
                    .is_some_and(|o| spell_filter.matches(o, state, source_controller))
            }

            (WouldDrawCard { player }, ReplacementEvent::DrawCard { player: p }) => {
                player.matches(*p, source_controller)
            }

            (WouldBeginTurn { player: p1 }, ReplacementEvent::BeginTurn { player: p2 }) => {
                *p1 == *p2
            }

            (
                WouldCreateToken { token_filter },
                ReplacementEvent::CreateToken { object_id },
            ) => {
                state.objects.get(*object_id)
                    .is_some_and(|o| token_filter.matches(o, state, source_controller))
            }

            (
                WouldPlaceCounters { object_filter, kinds },
                ReplacementEvent::PlaceCounters { target, kind, .. },
            ) => {
                if !kinds.matches(*kind) { return false; }
                match target {
                    CounterTarget::Object(id) => state.objects.get(*id)
                        .is_some_and(|o| object_filter.matches(o, state, source_controller)),
                    CounterTarget::Player(_) => false,
                }
            }

            (Custom(f), _) => f(event, state),

            _ => false,
        }
    }
}

/// A filter with no constraints passes everything — but if the source
/// object is gone entirely we conservatively treat a default
/// ObjectFilter as matching (so effects like "prevent all damage to
/// you" still fire when the damage source doesn't exist in the arena).
fn source_filter_matches_by_default(f: &ObjectFilter) -> bool {
    f.types.is_none() && f.not_types.is_none() && f.colors.is_none()
        && f.subtypes.is_none() && f.controller.is_none()
        && f.cmc_condition.is_none() && f.power_condition.is_none()
        && f.toughness_condition.is_none() && f.name.is_none()
        && f.is_token.is_none() && f.has_counter.is_none()
        && f.custom.is_none()
}

// =============================================================================
// ReplacementKind
// =============================================================================

/// What the replacement does. The damage pipeline knows how to apply
/// each variant to a [`ReplacementEvent::Damage`]; other variants
/// pass through unchanged for damage events.
// TODO(serialize): `Custom` carries a fn pointer.
#[derive(Clone, Debug)]
pub enum ReplacementKind {
    // --- Damage-event kinds ---
    /// "Prevent all damage that would be dealt by / to …"
    PreventAllDamage,
    /// "Prevent the next N damage …" (Healing Salve style).
    PreventDamageUpTo(u32),
    /// "Damage dealt by this source is doubled" (Furnace of Rath).
    DoubleDamage,
    /// "Instead, that damage is dealt to [target]" (Palisade Giant).
    RedirectDamageTo(DamageTarget),

    // --- ETB-event kinds ---
    /// "X enters the battlefield with N [counters]" (Kalonian Hydra).
    EtbWithCounters { kind: CounterKind, count: u32 },
    /// "X enters the battlefield tapped" (tap-lands).
    EtbTapped,

    // --- Die-event kinds ---
    /// "If a [matching] creature would die, exile it instead"
    /// (Rest in Peace).
    ExileInsteadOfDying,
    /// CR 701.25 — Regenerate shield. Fires once, then the shield is
    /// consumed. Effects of regenerating are applied by the caller:
    /// remove all damage, tap, remove from combat. The replacement
    /// effect itself is removed from `state.replacement_effects`
    /// after it fires. Typically used on a one-shot basis as
    /// [`ReplacementDuration::EndOfTurn`].
    RegenerateShield,

    // --- Counter-placement-event kinds ---
    /// "N additional counters of that kind are placed instead"
    /// (Hardened Scales = AddAdditionalCounters(1) on +1/+1;
    /// Winding Constrictor = AddAdditionalCounters(1) on any counter).
    /// N counters → N + m counters.
    AddAdditionalCounters(u32),
    /// "That many times N counters are placed instead"
    /// (Doubling Season = MultiplyCounters(2)). N counters → N * m.
    MultiplyCounters(u32),

    // --- Draw-event kinds ---
    /// "If you would draw a card, draw two cards instead" (Howling Mine
    /// style; simplified).
    DrawAdditional(u32),
    /// "You don't draw for the turn".
    SkipDraw,

    /// Custom escape hatch. Takes the current event and state, returns
    /// the replaced event (or `None` for full cancellation).
    Custom(fn(&ReplacementEvent, &GameState) -> Option<ReplacementEvent>),
}

// =============================================================================
// ReplacementEvent — what the pipeline passes around
// =============================================================================

/// A snapshot of the would-be event flowing through the replacement
/// pipeline. Mutations by successive replacements build up here
/// before committing.
#[derive(Clone, Debug)]
pub enum ReplacementEvent {
    Damage {
        source: ObjectId,
        target: DamageTarget,
        amount: u32,
    },
    EnterBattlefield {
        object_id: ObjectId,
    },
    Die {
        object_id: ObjectId,
    },
    CounterSpell {
        stack_entry_id: ObjectId,
    },
    DrawCard {
        player: PlayerId,
    },
    BeginTurn {
        player: PlayerId,
    },
    CreateToken {
        object_id: ObjectId,
    },
    /// A would-place-counters event. Every counter placement in the
    /// rules engine (ETB counters, Effect::AddCounters,
    /// Effect::MoveCounter destination, proliferate) routes through
    /// this so Hardened Scales / Doubling Season / Winding Constrictor
    /// can intercept.
    PlaceCounters {
        target: CounterTarget,
        kind: CounterKind,
        count: u32,
    },
}

// =============================================================================
// ETB replacement collector output
// =============================================================================

/// Accumulated ETB replacements for an object about to enter the
/// battlefield. The caller folds these into the ETB commit.
#[derive(Clone, Debug, Default)]
pub struct EtbReplacements {
    /// Counters to add as the object enters.
    pub additional_counters: Vec<(CounterKind, u32)>,
    /// Object should enter tapped.
    pub enter_tapped: bool,
    /// Object should be exiled instead of entering. (Rare; e.g.
    /// Mirror Gallery counter-scenarios.) Deferred — no variant yet.
    pub exile_instead: bool,
}

// =============================================================================
// GameState integration
// =============================================================================

impl GameState {
    /// Register a replacement effect. Assigns a fresh id so the
    /// pipeline can track "already used this event" per CR 614.5.
    pub fn add_replacement_effect(&mut self, mut effect: ReplacementEffect) {
        effect.id = self.next_timestamp();
        self.replacement_effects.push(effect);
    }

    /// Remove every replacement effect matching `pred`. Returns count.
    pub fn remove_replacement_effects<F>(&mut self, mut pred: F) -> usize
    where F: FnMut(&ReplacementEffect) -> bool,
    {
        let before = self.replacement_effects.len();
        let mut keep = Vec::with_capacity(before);
        for e in self.replacement_effects.drain(..) {
            if pred(&e) {
                // discard
            } else {
                keep.push(e);
            }
        }
        self.replacement_effects = keep;
        before - self.replacement_effects.len()
    }

    /// Expire end-of-turn replacement effects. Called at cleanup.
    pub fn expire_end_of_turn_replacements(&mut self) {
        self.remove_replacement_effects(|e|
            matches!(e.duration, ReplacementDuration::EndOfTurn));
    }

    /// Expire replacements sourced from a leaving object.
    pub fn expire_replacements_from_source(&mut self, source: ObjectId) {
        self.remove_replacement_effects(|e|
            e.source == source
            && matches!(e.duration, ReplacementDuration::WhileSourceOnBattlefield));
    }

    /// Route (source, target, amount) through all applicable
    /// replacement effects. Returns the post-replacement tuple, or
    /// `None` if damage is fully prevented.
    ///
    /// Algorithm (CR 614.15 + 614.5):
    /// - Loop: find all applicable replacements that haven't been
    ///   used for this event yet; pick one, apply it; repeat until
    ///   no more apply.
    /// - Within each round, self-replacement effects are chosen
    ///   before others. Otherwise picks by registration order (a
    ///   Phase 1 stand-in for the affected-player-chooses rule).
    pub fn replace_damage(
        &self,
        source: ObjectId,
        target: DamageTarget,
        amount: u32,
    ) -> Option<(ObjectId, DamageTarget, u32)> {
        let mut current = ReplacementEvent::Damage { source, target, amount };
        let mut used: std::collections::HashSet<u64> = std::collections::HashSet::new();

        loop {
            let candidates: Vec<&ReplacementEffect> = self.replacement_effects.iter()
                .filter(|e| {
                    if used.contains(&e.id) { return false; }
                    let source_ctrl = source_controller_of(e, self);
                    e.condition.matches(&current, source_ctrl, self)
                })
                .collect();
            if candidates.is_empty() { break; }

            // CR 614.15: self-replacement first.
            let pick = candidates.iter()
                .find(|e| e.is_self_replacement)
                .copied()
                .or_else(|| candidates.first().copied());
            let Some(effect) = pick else { break; };

            used.insert(effect.id);
            current = apply_kind_to_event(&effect.kind, &current, self)?;
        }

        match current {
            ReplacementEvent::Damage { source, target, amount } =>
                Some((source, target, amount)),
            _ => None, // morphed into a different event kind (unusual)
        }
    }

    /// Route a would-place-counters event through replacements and
    /// commit the placement.
    ///
    /// Hardened Scales ([`ReplacementKind::AddAdditionalCounters`])
    /// and Doubling Season ([`ReplacementKind::MultiplyCounters`])
    /// intercept here. Each replacement applies at most once
    /// (CR 614.5); self-replacements apply before others (CR 614.15);
    /// agent-choice ordering among genuinely-multiple non-self
    /// replacements is a Phase 2-B stand-in via id order.
    ///
    /// On `Some((kind, count))`: the target has been mutated (for
    /// object targets) and a [`GameEvent::CounterAdded`] emitted. On
    /// `None`: the event was fully cancelled — no mutation, no event.
    ///
    /// Player targets currently only support the counter kinds the
    /// engine already tracks (`Poison`, `Energy`). Other kinds on a
    /// player are a no-op — the pipeline still runs replacements,
    /// but nothing is committed.
    pub fn place_counters(
        &mut self,
        target: CounterTarget,
        kind: CounterKind,
        count: u32,
    ) -> Option<(CounterKind, u32)> {
        if count == 0 { return None; }
        let mut current = ReplacementEvent::PlaceCounters { target, kind, count };
        let mut used: std::collections::HashSet<u64> = std::collections::HashSet::new();

        loop {
            let candidates: Vec<u64> = self.replacement_effects.iter()
                .filter(|e| {
                    if used.contains(&e.id) { return false; }
                    // "You" in the filter refers to the replacement's
                    // own controller (Hardened Scales' controller, not
                    // the player whose counter is being placed).
                    let source_ctrl = source_controller_of(e, self);
                    e.condition.matches(&current, source_ctrl, self)
                })
                .map(|e| e.id)
                .collect();
            if candidates.is_empty() { break; }

            // CR 614.15: self-replacement first; id order among the rest.
            let pick = {
                let self_first = self.replacement_effects.iter()
                    .find(|e| candidates.contains(&e.id) && e.is_self_replacement)
                    .map(|e| e.id);
                self_first.or_else(|| candidates.first().copied())
            };
            let Some(pick_id) = pick else { break; };

            used.insert(pick_id);
            let Some(rk) = self.replacement_effects.iter()
                .find(|e| e.id == pick_id).map(|e| e.kind.clone())
            else { break; };
            current = apply_kind_to_event(&rk, &current, self)?;
        }

        let (final_target, final_kind, final_count) = match current {
            ReplacementEvent::PlaceCounters { target, kind, count } =>
                (target, kind, count),
            _ => return None, // morphed into a different event (unusual)
        };
        if final_count == 0 { return None; }

        match final_target {
            CounterTarget::Object(id) => {
                let obj = self.objects.get_mut(id)?;
                obj.add_counters(final_kind, final_count);
                self.emit(GameEvent::CounterAdded {
                    object_id: id, kind: final_kind, count: final_count,
                });
            }
            CounterTarget::Player(p) => {
                let pl = self.player_mut(p);
                match final_kind {
                    CounterKind::Poison => pl.poison_counters += final_count,
                    CounterKind::Energy => pl.energy += final_count,
                    // Player experience and set-specific player counters
                    // are not first-class `CounterKind` variants yet.
                    _ => {}
                }
            }
        }
        Some((final_kind, final_count))
    }

    /// Collect ETB replacement modifications for an object about to
    /// enter the battlefield. Does NOT mutate state — the caller
    /// commits the modifications.
    pub fn collect_etb_replacements(&self, object_id: ObjectId) -> EtbReplacements {
        let mut out = EtbReplacements::default();
        let event = ReplacementEvent::EnterBattlefield { object_id };

        let mut applicable: Vec<&ReplacementEffect> = self.replacement_effects.iter()
            .filter(|e| {
                let source_ctrl = source_controller_of(e, self);
                e.condition.matches(&event, source_ctrl, self)
            })
            .collect();
        // Self-replacements first, then others; within each, by id.
        applicable.sort_by_key(|e| (!e.is_self_replacement, e.id));

        for effect in applicable {
            match &effect.kind {
                ReplacementKind::EtbWithCounters { kind, count } => {
                    out.additional_counters.push((*kind, *count));
                }
                ReplacementKind::EtbTapped => {
                    out.enter_tapped = true;
                }
                _ => {} // not an ETB replacement
            }
        }
        out
    }

    /// Run a would-die event through replacements. Returns `Some(id)`
    /// if the creature still dies (possibly a different id if a
    /// replacement redirected); `None` if the death was replaced
    /// (e.g. exiled instead).
    ///
    /// The caller is responsible for performing whatever alternate
    /// action the replacement dictated — see
    /// [`EtbReplacements`] / the doc on `ReplacementKind` for
    /// what variants may do. For Phase 1, only
    /// [`ReplacementKind::ExileInsteadOfDying`] is wired, and it
    /// returns `None` (caller routes the object to exile).
    pub fn replace_die(&mut self, object_id: ObjectId) -> DieOutcome {
        let event = ReplacementEvent::Die { object_id };

        let mut applicable_ids: Vec<u64> = self.replacement_effects.iter()
            .filter(|e| {
                let source_ctrl = source_controller_of(e, self);
                e.condition.matches(&event, source_ctrl, self)
            })
            .map(|e| e.id)
            .collect();
        // Sort by (not-self-replacement, id) to prioritize self-replacement.
        applicable_ids.sort_by_key(|id| {
            let e = self.replacement_effects.iter().find(|e| e.id == *id).unwrap();
            (!e.is_self_replacement, *id)
        });

        for id in applicable_ids {
            let kind = self.replacement_effects.iter()
                .find(|e| e.id == id).map(|e| e.kind.clone());
            match kind {
                Some(ReplacementKind::ExileInsteadOfDying) => {
                    return DieOutcome::ExileInstead;
                }
                Some(ReplacementKind::RegenerateShield) => {
                    // Shield fires and is consumed.
                    self.replacement_effects.retain(|e| e.id != id);
                    return DieOutcome::Regenerated;
                }
                _ => {}
            }
        }
        DieOutcome::StillDies
    }
}

/// Outcome of a death-replacement pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DieOutcome {
    /// Nothing overrode the death — proceed with move to graveyard.
    StillDies,
    /// Route the object to exile instead.
    ExileInstead,
    /// CR 701.25 — regenerate: clear damage, tap, remove from combat,
    /// stay on the battlefield.
    Regenerated,
}

// =============================================================================
// Apply-kind helpers
// =============================================================================

/// Apply a [`ReplacementKind`] to a [`ReplacementEvent`]. Returns the
/// new event, or `None` if it was cancelled.
fn apply_kind_to_event(
    kind: &ReplacementKind,
    event: &ReplacementEvent,
    state: &GameState,
) -> Option<ReplacementEvent> {
    use ReplacementEvent::*;
    use ReplacementKind::*;
    match (kind, event) {
        (PreventAllDamage, Damage { .. }) => None,

        (PreventDamageUpTo(n), Damage { source, target, amount }) => {
            let new_amt = amount.saturating_sub(*n);
            if new_amt == 0 { None }
            else { Some(Damage { source: *source, target: *target, amount: new_amt }) }
        }

        (DoubleDamage, Damage { source, target, amount }) => {
            let new_amt = amount.saturating_mul(2);
            Some(Damage { source: *source, target: *target, amount: new_amt })
        }

        (RedirectDamageTo(new_target), Damage { source, amount, .. }) => {
            Some(Damage { source: *source, target: *new_target, amount: *amount })
        }

        (DrawAdditional(n), DrawCard { player }) => {
            // Modelled as the same event with an "n additional draws"
            // semantic — but since DrawCard is identity-by-player, the
            // caller (effects.rs) is expected to interpret this via
            // explicit follow-up draws. For now, pass the event
            // through unchanged.
            let _ = (n, player);
            Some(event.clone())
        }

        (SkipDraw, DrawCard { .. }) => None,

        (AddAdditionalCounters(m), PlaceCounters { target, kind, count }) => {
            let new_count = count.saturating_add(*m);
            Some(PlaceCounters { target: *target, kind: *kind, count: new_count })
        }

        (MultiplyCounters(m), PlaceCounters { target, kind, count }) => {
            let new_count = count.saturating_mul(*m);
            if new_count == 0 { None }
            else { Some(PlaceCounters { target: *target, kind: *kind, count: new_count }) }
        }

        (Custom(f), _) => f(event, state),

        // ETB / Die kinds are handled by their dedicated collectors
        // (`collect_etb_replacements`, `replace_die`) — pass through
        // here since they don't modify a single ReplacementEvent.
        _ => Some(event.clone()),
    }
}

/// The controller of the replacement effect's source object. Used to
/// resolve `ControllerConstraint::You` inside the replacement's own
/// filters (e.g. "creature **you** control" on Hardened Scales). If
/// the source object is gone from the arena, falls back to 0.
fn source_controller_of(effect: &ReplacementEffect, state: &GameState) -> PlayerId {
    state.objects.get(effect.source).map(|o| o.controller).unwrap_or(0)
}

// NOTE: the "affected player" of a replacement event — the one who
// chooses ordering among multiple applicable replacements per CR 616.1
// — is distinct from `source_controller_of`. We don't model that yet;
// it comes in as part of agent-choice replacement ordering (Phase 2-B).


// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mana::ManaCost;
    use crate::objects::{Characteristics, GameObject};
    use crate::targets::ControllerConstraint;
    use crate::zones::Zone;

    // --- helpers -----------------------------------------------------------

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

    fn red_creature_chars(p: i32, t: i32) -> Characteristics {
        Characteristics {
            mana_cost: Some(ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(p)),
            toughness: Some(PtValue::Fixed(t)),
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

    fn put_red_creature(s: &mut GameState, owner: PlayerId, p: i32, t: i32) -> ObjectId {
        let id = s.allocate_object_id();
        let mut obj = GameObject::new(id, owner, Zone::Battlefield, 1,
            red_creature_chars(p, t));
        obj.controller = owner;
        s.objects.insert(obj);
        id
    }

    fn base_effect(
        source: ObjectId,
        condition: ReplacementCondition,
        kind: ReplacementKind,
    ) -> ReplacementEffect {
        ReplacementEffect {
            source,
            id: 0,
            condition,
            kind,
            is_self_replacement: false,
            duration: ReplacementDuration::Permanent,
        }
    }

    // --- Damage: prevent all ----------------------------------------------

    #[test]
    fn prevent_all_damage_cancels() {
        let mut s = GameState::new(2, 0);
        s.add_replacement_effect(base_effect(
            /*source=*/ 0,
            ReplacementCondition::WouldDealDamage {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
            },
            ReplacementKind::PreventAllDamage,
        ));
        let out = s.replace_damage(99, DamageTarget::Player(0), 3);
        assert!(out.is_none());
    }

    #[test]
    fn prevent_damage_up_to_reduces_amount() {
        let mut s = GameState::new(2, 0);
        s.add_replacement_effect(base_effect(
            0,
            ReplacementCondition::WouldDealDamage {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
            },
            ReplacementKind::PreventDamageUpTo(2),
        ));
        let out = s.replace_damage(99, DamageTarget::Player(1), 5);
        assert_eq!(out, Some((99, DamageTarget::Player(1), 3)));
    }

    #[test]
    fn prevent_damage_up_to_fully_absorbs_smaller_damage() {
        let mut s = GameState::new(2, 0);
        s.add_replacement_effect(base_effect(
            0,
            ReplacementCondition::WouldDealDamage {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
            },
            ReplacementKind::PreventDamageUpTo(5),
        ));
        let out = s.replace_damage(99, DamageTarget::Player(1), 3);
        assert!(out.is_none());
    }

    // --- Damage: double ---------------------------------------------------

    #[test]
    fn double_damage_applies() {
        let mut s = GameState::new(2, 0);
        let _src = put_red_creature(&mut s, 0, 3, 3);
        s.add_replacement_effect(base_effect(
            0,
            ReplacementCondition::WouldDealDamage {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
            },
            ReplacementKind::DoubleDamage,
        ));
        let out = s.replace_damage(99, DamageTarget::Player(1), 3);
        assert_eq!(out, Some((99, DamageTarget::Player(1), 6)));
    }

    // --- Damage: redirect --------------------------------------------------

    #[test]
    fn redirect_damage_target() {
        let mut s = GameState::new(2, 0);
        s.add_replacement_effect(base_effect(
            0,
            ReplacementCondition::WouldDealDamage {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
            },
            ReplacementKind::RedirectDamageTo(DamageTarget::Player(1)),
        ));
        let out = s.replace_damage(99, DamageTarget::Player(0), 3);
        assert_eq!(out, Some((99, DamageTarget::Player(1), 3)));
    }

    // --- Damage: source / target filters -----------------------------------

    #[test]
    fn damage_replacement_respects_source_filter() {
        // Prevent only damage from red sources.
        let mut s = GameState::new(2, 0);
        let red_src = put_red_creature(&mut s, 0, 2, 2);
        let green_src = put_creature(&mut s, 0, 2, 2);
        s.add_replacement_effect(base_effect(
            0,
            ReplacementCondition::WouldDealDamage {
                source_filter: ObjectFilter::new().with_colors(ColorSet::red()),
                target_filter: TargetFilter::Player,
            },
            ReplacementKind::PreventAllDamage,
        ));

        assert!(s.replace_damage(red_src, DamageTarget::Player(1), 3).is_none());
        assert_eq!(s.replace_damage(green_src, DamageTarget::Player(1), 3),
            Some((green_src, DamageTarget::Player(1), 3)));
    }

    #[test]
    fn damage_replacement_with_no_filters_passes_through() {
        let s = GameState::new(2, 0);
        let out = s.replace_damage(99, DamageTarget::Player(0), 4);
        assert_eq!(out, Some((99, DamageTarget::Player(0), 4)));
    }

    // --- Chained replacements ---------------------------------------------

    #[test]
    fn two_prevent_up_to_effects_stack() {
        let mut s = GameState::new(2, 0);
        for n in [1, 2] {
            s.add_replacement_effect(base_effect(
                0,
                ReplacementCondition::WouldDealDamage {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                },
                ReplacementKind::PreventDamageUpTo(n),
            ));
        }
        // Total reduction: 3 points from 5 → 2 remaining.
        let out = s.replace_damage(99, DamageTarget::Player(0), 5);
        assert_eq!(out.map(|t| t.2), Some(2));
    }

    #[test]
    fn each_replacement_applies_at_most_once_per_event() {
        // With a single PreventDamageUpTo(1) registered, 3 damage
        // must become 2 — not 0 (which would happen if the replacement
        // fired repeatedly).
        let mut s = GameState::new(2, 0);
        s.add_replacement_effect(base_effect(
            0,
            ReplacementCondition::WouldDealDamage {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
            },
            ReplacementKind::PreventDamageUpTo(1),
        ));
        let out = s.replace_damage(99, DamageTarget::Player(0), 3);
        assert_eq!(out.map(|t| t.2), Some(2));
    }

    // --- Self-replacement ordering (CR 614.15) ----------------------------

    #[test]
    fn self_replacement_applies_before_others_for_same_event() {
        // Two applicable replacements: one self-replacement that
        // doubles damage, one non-self that prevents 5. If double
        // applies first: 3 → 6, then prevent-5 → 1. If prevent
        // applies first: 3 → (prevented), None. We register the
        // non-self first so insertion order favors it — but CR 614.15
        // says the self-replacement still goes first.
        let mut s = GameState::new(2, 0);
        let mut other = base_effect(
            0,
            ReplacementCondition::WouldDealDamage {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
            },
            ReplacementKind::PreventDamageUpTo(5),
        );
        other.is_self_replacement = false;
        s.add_replacement_effect(other);

        let mut self_repl = base_effect(
            0,
            ReplacementCondition::WouldDealDamage {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
            },
            ReplacementKind::DoubleDamage,
        );
        self_repl.is_self_replacement = true;
        s.add_replacement_effect(self_repl);

        // Self-first ordering: double to 6, then prevent 5 → 1.
        let out = s.replace_damage(99, DamageTarget::Player(0), 3);
        assert_eq!(out.map(|t| t.2), Some(1));
    }

    // --- Custom ------------------------------------------------------------

    #[test]
    fn custom_replacement_fires() {
        fn shrink(
            event: &ReplacementEvent,
            _: &GameState,
        ) -> Option<ReplacementEvent> {
            match event {
                ReplacementEvent::Damage { source, target, amount } if *amount > 1 =>
                    Some(ReplacementEvent::Damage {
                        source: *source, target: *target, amount: amount - 1,
                    }),
                _ => Some(event.clone()),
            }
        }
        let mut s = GameState::new(2, 0);
        s.add_replacement_effect(base_effect(
            0,
            ReplacementCondition::WouldDealDamage {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
            },
            ReplacementKind::Custom(shrink),
        ));
        let out = s.replace_damage(99, DamageTarget::Player(0), 5);
        assert_eq!(out.map(|t| t.2), Some(4));
    }

    // --- Duration / expiry -------------------------------------------------

    #[test]
    fn expire_end_of_turn_drops_only_eot_replacements() {
        let mut s = GameState::new(2, 0);
        s.add_replacement_effect(ReplacementEffect {
            duration: ReplacementDuration::EndOfTurn,
            ..base_effect(0,
                ReplacementCondition::WouldDealDamage {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                },
                ReplacementKind::PreventAllDamage)
        });
        s.add_replacement_effect(base_effect(
            0,
            ReplacementCondition::WouldDealDamage {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
            },
            ReplacementKind::PreventDamageUpTo(1),
        ));
        s.expire_end_of_turn_replacements();
        assert_eq!(s.replacement_effects.len(), 1);
    }

    #[test]
    fn expire_from_source_drops_while_on_bf_effects() {
        let mut s = GameState::new(2, 0);
        let src_a = 10;
        let src_b = 20;
        s.add_replacement_effect(ReplacementEffect {
            source: src_a,
            duration: ReplacementDuration::WhileSourceOnBattlefield,
            ..base_effect(src_a,
                ReplacementCondition::WouldDealDamage {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                },
                ReplacementKind::PreventAllDamage)
        });
        s.add_replacement_effect(ReplacementEffect {
            source: src_b,
            duration: ReplacementDuration::WhileSourceOnBattlefield,
            ..base_effect(src_b,
                ReplacementCondition::WouldDealDamage {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                },
                ReplacementKind::PreventAllDamage)
        });
        s.expire_replacements_from_source(src_a);
        assert_eq!(s.replacement_effects.len(), 1);
        assert_eq!(s.replacement_effects[0].source, src_b);
    }

    // --- ETB collector -----------------------------------------------------

    #[test]
    fn collect_etb_enters_with_counters() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.add_replacement_effect(base_effect(
            c, // source is the creature entering
            ReplacementCondition::WouldEnterBattlefield {
                object_filter: ObjectFilter::creature(),
            },
            ReplacementKind::EtbWithCounters {
                kind: CounterKind::PlusOnePlusOne,
                count: 2,
            },
        ));
        let r = s.collect_etb_replacements(c);
        assert_eq!(r.additional_counters.len(), 1);
        assert_eq!(r.additional_counters[0].0, CounterKind::PlusOnePlusOne);
        assert_eq!(r.additional_counters[0].1, 2);
    }

    #[test]
    fn collect_etb_enters_tapped() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.add_replacement_effect(base_effect(
            c,
            ReplacementCondition::WouldEnterBattlefield {
                object_filter: ObjectFilter::default(),
            },
            ReplacementKind::EtbTapped,
        ));
        let r = s.collect_etb_replacements(c);
        assert!(r.enter_tapped);
        assert!(r.additional_counters.is_empty());
    }

    #[test]
    fn collect_etb_no_effects_returns_default() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        let r = s.collect_etb_replacements(c);
        assert!(r.additional_counters.is_empty());
        assert!(!r.enter_tapped);
    }

    // --- Die replacement ---------------------------------------------------

    #[test]
    fn exile_instead_of_dying_returns_exile_outcome() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.add_replacement_effect(base_effect(
            0,
            ReplacementCondition::WouldDie {
                object_filter: ObjectFilter::creature(),
            },
            ReplacementKind::ExileInsteadOfDying,
        ));
        assert_eq!(s.replace_die(c), DieOutcome::ExileInstead);
    }

    #[test]
    fn die_without_matching_replacement_still_dies() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        assert_eq!(s.replace_die(c), DieOutcome::StillDies);
    }

    // --- DrawCard skip ----------------------------------------------------

    #[test]
    fn skip_draw_event_is_cancelled() {
        // Not wired into draw pipeline yet — test the pipeline predicate.
        let mut s = GameState::new(2, 0);
        s.add_replacement_effect(base_effect(
            0,
            ReplacementCondition::WouldDrawCard { player: ControllerConstraint::Any },
            ReplacementKind::SkipDraw,
        ));
        let cond_matches = s.replacement_effects[0].condition.matches(
            &ReplacementEvent::DrawCard { player: 0 },
            /*source_controller=*/ 0,
            &s,
        );
        assert!(cond_matches);
    }

    // =====================================================================
    // ETB pipeline integration — `after_enter_battlefield` fires at every
    // real ETB path (move_object_to_zone, finalize_resolved_spell,
    // create_token, copy_permanent) and folds in global ETB-event
    // replacements.
    // =====================================================================

    /// Register a global "every creature enters tapped" replacement
    /// against a stand-in source id.
    fn install_global_enter_tapped(s: &mut GameState) {
        s.add_replacement_effect(base_effect(
            /*source=*/ 999,
            ReplacementCondition::WouldEnterBattlefield {
                object_filter: ObjectFilter::creature(),
            },
            ReplacementKind::EtbTapped,
        ));
    }

    /// Register a global "every creature enters with an extra +1/+1
    /// counter" ETB-event replacement. NOT Hardened Scales — this is
    /// an ETB-event-level rewrite, not a counter-placement-level one.
    /// See `modular_plus_hardened_scales_yields_four_counters` for the
    /// canonical distinction.
    fn install_global_etb_counter(s: &mut GameState) {
        s.add_replacement_effect(base_effect(
            /*source=*/ 999,
            ReplacementCondition::WouldEnterBattlefield {
                object_filter: ObjectFilter::creature(),
            },
            ReplacementKind::EtbWithCounters {
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            },
        ));
    }

    #[test]
    fn etb_via_move_object_to_zone_applies_enter_tapped() {
        let mut s = GameState::new(2, 0);
        install_global_enter_tapped(&mut s);
        let c = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            c, 0, Zone::Graveyard(0), 1, creature_chars(2, 2)));
        let new_id = s.move_object_to_zone(
            c, Zone::Battlefield,
            crate::events::MoveCause::SpellResolution,
        ).unwrap();
        assert!(s.objects.get(new_id).unwrap().is_tapped(),
            "graveyard → battlefield path should honor global EtbTapped");
        assert!(s.objects.get(new_id).unwrap().status.summoning_sick,
            "unified after_enter_battlefield sets summoning sickness");
    }

    #[test]
    fn etb_via_move_object_to_zone_applies_counters() {
        let mut s = GameState::new(2, 0);
        install_global_etb_counter(&mut s);
        let c = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            c, 0, Zone::Hand(0), 1, creature_chars(2, 2)));
        let new_id = s.move_object_to_zone(
            c, Zone::Battlefield,
            crate::events::MoveCause::SpellResolution,
        ).unwrap();
        assert_eq!(
            s.objects.get(new_id).unwrap()
                .count_counters(CounterKind::PlusOnePlusOne),
            1,
            "global ETB-event counter replacement should add a counter"
        );
    }

    #[test]
    fn etb_via_finalize_resolved_spell_applies_enter_tapped() {
        use crate::targets::TargetSelection;
        let mut s = GameState::new(2, 0);
        install_global_enter_tapped(&mut s);
        let c = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            c, 0, Zone::Hand(0), 1, creature_chars(2, 2)));
        let stack_id = s.announce_spell_on_stack(
            c, 0, TargetSelection::new(), vec![], None);
        let entry = s.pop_stack_entry().unwrap();
        assert_eq!(entry.id, stack_id);
        s.finalize_resolved_spell(entry);
        let bf = s.objects.objects_in_zone(Zone::Battlefield).next().unwrap();
        assert!(bf.is_tapped(), "cast path should honor global EtbTapped");
        assert!(bf.status.summoning_sick);
    }

    #[test]
    fn etb_via_create_token_applies_enter_tapped() {
        let mut s = GameState::new(2, 0);
        install_global_enter_tapped(&mut s);
        let def = crate::effects::TokenDefinition {
            name: 0,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: crate::types::SubtypeSet::default(),
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: Vec::new(),
            abilities: Vec::new(),
        };
        crate::effects::Effect::CreateToken {
            controller: 0, token: def,
        }.execute(&mut s);
        let tok = s.objects.objects_in_zone(Zone::Battlefield).next().unwrap();
        assert!(tok.is_tapped(),
            "tokens should also go through the ETB replacement pipeline");
        assert!(tok.status.summoning_sick);
    }

    #[test]
    fn etb_via_copy_permanent_applies_enter_tapped() {
        let mut s = GameState::new(2, 0);
        // Source creature already on the battlefield (pre-existing).
        let src = put_creature(&mut s, 0, 2, 2);
        // Install the global replacement AFTER putting the source out
        // so that `src` itself isn't double-counted.
        install_global_enter_tapped(&mut s);
        crate::effects::Effect::CopyPermanent { target: src }.execute(&mut s);
        let bf_tapped: Vec<_> = s.objects.objects_in_zone(Zone::Battlefield)
            .filter(|o| o.is_tapped()).collect();
        assert_eq!(bf_tapped.len(), 1,
            "the copy enters tapped; the pre-existing source is untapped");
    }

    // =====================================================================
    // Counter-placement pipeline (Hardened Scales, Doubling Season,
    // Winding Constrictor). Every production counter-placement routes
    // through `GameState::place_counters`.
    // =====================================================================

    fn install_hardened_scales(s: &mut GameState) {
        // "If one or more +1/+1 counters would be placed on a creature
        // you control, that many plus one +1/+1 counters are placed
        // instead."
        s.add_replacement_effect(base_effect(
            /*source=*/ 999,
            ReplacementCondition::WouldPlaceCounters {
                object_filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You),
                kinds: CounterKindFilter::Only(CounterKind::PlusOnePlusOne),
            },
            ReplacementKind::AddAdditionalCounters(1),
        ));
    }

    fn install_winding_constrictor(s: &mut GameState) {
        // "If one or more counters would be placed on an artifact or
        // creature you control, one more of each of those kinds of
        // counter is placed on that permanent instead." Phase 1 approx:
        // any counter on a creature you control (artifact filter
        // combinator not needed for the tests here).
        s.add_replacement_effect(base_effect(
            /*source=*/ 998,
            ReplacementCondition::WouldPlaceCounters {
                object_filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You),
                kinds: CounterKindFilter::Any,
            },
            ReplacementKind::AddAdditionalCounters(1),
        ));
    }

    fn install_doubling_season(s: &mut GameState) {
        // "If one or more counters would be placed on a permanent you
        // control, twice that many of those counters are placed instead."
        s.add_replacement_effect(base_effect(
            /*source=*/ 997,
            ReplacementCondition::WouldPlaceCounters {
                object_filter: ObjectFilter::default()
                    .controlled_by(ControllerConstraint::You),
                kinds: CounterKindFilter::Any,
            },
            ReplacementKind::MultiplyCounters(2),
        ));
    }

    #[test]
    fn hardened_scales_adds_one_counter() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        install_hardened_scales(&mut s);
        let out = s.place_counters(
            CounterTarget::Object(c), CounterKind::PlusOnePlusOne, 1);
        assert_eq!(out, Some((CounterKind::PlusOnePlusOne, 2)));
        assert_eq!(
            s.objects.get(c).unwrap().count_counters(CounterKind::PlusOnePlusOne),
            2);
    }

    #[test]
    fn doubling_season_doubles_counters() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        install_doubling_season(&mut s);
        let out = s.place_counters(
            CounterTarget::Object(c), CounterKind::PlusOnePlusOne, 2);
        assert_eq!(out, Some((CounterKind::PlusOnePlusOne, 4)));
    }

    #[test]
    fn hardened_scales_plus_winding_constrictor_both_fire() {
        // Both AddAdditionalCounters(1). They commute (both +1), so
        // order-independent. N=1 → 1+1+1 = 3.
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        install_hardened_scales(&mut s);
        install_winding_constrictor(&mut s);
        let out = s.place_counters(
            CounterTarget::Object(c), CounterKind::PlusOnePlusOne, 1);
        assert_eq!(out, Some((CounterKind::PlusOnePlusOne, 3)),
            "each replacement fires once per CR 614.5");
    }

    #[test]
    fn hardened_scales_does_not_apply_to_loyalty_counters() {
        // Kind filter: Hardened Scales only affects +1/+1, not loyalty.
        let mut s = GameState::new(2, 0);
        let pw = put_creature(&mut s, 0, 2, 2); // stand-in permanent
        install_hardened_scales(&mut s);
        let out = s.place_counters(
            CounterTarget::Object(pw), CounterKind::Loyalty, 3);
        assert_eq!(out, Some((CounterKind::Loyalty, 3)),
            "Hardened Scales only rewrites +1/+1 placements");
    }

    #[test]
    fn hardened_scales_respects_controller_filter() {
        // Opponent's creature, player-0's Hardened Scales. Should NOT
        // fire (filter is "creature you control" = controller-You
        // relative to the replacement's source/controller).
        let mut s = GameState::new(2, 0);
        let enemy = put_creature(&mut s, 1, 2, 2);
        // Hardened Scales installed by source 999, controller defaults.
        // The base_effect helper uses source=0 which is player 0's
        // controller context. So "You" = player 0 here.
        install_hardened_scales(&mut s);
        let out = s.place_counters(
            CounterTarget::Object(enemy), CounterKind::PlusOnePlusOne, 1);
        assert_eq!(out, Some((CounterKind::PlusOnePlusOne, 1)),
            "Hardened Scales should not apply to a creature you don't control");
    }

    /// Non-commuting pair — pins the current 2-A id-order behavior.
    /// Hardened Scales (AddAdditional(1)) + Doubling Season (Multiply(2))
    /// on a 1-counter placement:
    ///
    /// - HS first, then DS: 1 → 2 → 4
    /// - DS first, then HS: 1 → 2 → 3
    ///
    /// CR 616.1 says the affected controller chooses. Until agent-choice
    /// ordering lands (Phase 2-B), we pick by registration id, so HS
    /// (registered first) applies first → 4 counters.
    ///
    /// If agent-choice ordering lands and this test starts failing,
    /// update it to explicitly pick an order rather than deleting it —
    /// it's the regression anchor for the ordering contract.
    #[test]
    fn hardened_scales_then_doubling_season_pins_id_order() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        install_hardened_scales(&mut s);   // id lower → applied first
        install_doubling_season(&mut s);
        let out = s.place_counters(
            CounterTarget::Object(c), CounterKind::PlusOnePlusOne, 1);
        assert_eq!(out, Some((CounterKind::PlusOnePlusOne, 4)),
            "Phase 2-A: id-order picks HS first (1+1=2), then DS (×2=4). \
             Agent-choice ordering in 2-B will make this a player decision.");
    }

    #[test]
    fn self_replacement_applies_before_others_on_counter_placement() {
        // Self-replacement that MultiplyCounters(3) + non-self
        // AddAdditional(1). Non-self registered first so insertion order
        // would prefer it, but self must go first per CR 614.15.
        //   Self-first: 1 → 3 (×3) → 4 (+1)
        //   Other-first: 1 → 2 (+1) → 6 (×3)   [wrong]
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        let mut other = base_effect(
            0,
            ReplacementCondition::WouldPlaceCounters {
                object_filter: ObjectFilter::default()
                    .controlled_by(ControllerConstraint::You),
                kinds: CounterKindFilter::Any,
            },
            ReplacementKind::AddAdditionalCounters(1),
        );
        other.is_self_replacement = false;
        s.add_replacement_effect(other);
        let mut selfrep = base_effect(
            0,
            ReplacementCondition::WouldPlaceCounters {
                object_filter: ObjectFilter::default()
                    .controlled_by(ControllerConstraint::You),
                kinds: CounterKindFilter::Any,
            },
            ReplacementKind::MultiplyCounters(3),
        );
        selfrep.is_self_replacement = true;
        s.add_replacement_effect(selfrep);
        let out = s.place_counters(
            CounterTarget::Object(c), CounterKind::PlusOnePlusOne, 1);
        assert_eq!(out, Some((CounterKind::PlusOnePlusOne, 4)));
    }

    #[test]
    fn effect_add_counters_routes_through_pipeline() {
        use crate::effects::Effect;
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        install_hardened_scales(&mut s);
        Effect::AddCounters {
            target: c, kind: CounterKind::PlusOnePlusOne, count: 2,
        }.execute(&mut s);
        assert_eq!(
            s.objects.get(c).unwrap().count_counters(CounterKind::PlusOnePlusOne),
            3,
            "Effect::AddCounters must route through place_counters");
    }

    #[test]
    fn proliferate_routes_through_pipeline() {
        use crate::effects::Effect;
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        // Seed a +1/+1 counter directly so proliferate has a kind to add.
        s.objects.get_mut(c).unwrap()
            .add_counters(CounterKind::PlusOnePlusOne, 1);
        install_hardened_scales(&mut s);
        Effect::Proliferate.execute(&mut s);
        assert_eq!(
            s.objects.get(c).unwrap().count_counters(CounterKind::PlusOnePlusOne),
            1 /* seeded */ + 2 /* proliferate's +1, then HS +1 */,
            "proliferate's placement must route through place_counters");
    }

    /// Modular N + Hardened Scales → N + 1 +1/+1 counters.
    ///
    /// This test uses a TEST-ONLY FIXTURE `register_test_modular_etb` to
    /// stand in for card-inherent Modular (which requires Phase 3
    /// per-card replacement hooks on `CardDefinition`). When the real
    /// Modular keyword lands, delete that fixture and point the test at
    /// the actual card.
    ///
    /// The contract being tested here is that ETB-event self-replacements
    /// (Modular-shaped "enters with N counters") cascade into the
    /// counter-placement pipeline so downstream replacements
    /// (Hardened Scales) apply to the resulting placement. That is, two
    /// distinct pipelines chain in the correct order.
    #[test]
    fn modular_plus_hardened_scales_yields_four_counters() {
        // TEST-ONLY FIXTURE: Modular-on-entering, as a self-replacement
        // scoped to a specific object-id. Real Modular will be a card-
        // inherent replacement (Phase 3, per-card `CardDefinition`
        // replacement hooks). grep for "TEST-ONLY FIXTURE" to find it.
        fn register_test_modular_etb(
            s: &mut GameState,
            object_id: ObjectId,
            n: u32,
        ) {
            let mut e = base_effect(
                /*source=*/ object_id,
                ReplacementCondition::WouldEnterBattlefield {
                    object_filter: ObjectFilter::default(),
                },
                ReplacementKind::EtbWithCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    count: n,
                },
            );
            e.is_self_replacement = true; // CR 614.15: self-first
            s.add_replacement_effect(e);
        }

        let mut s = GameState::new(2, 0);
        install_hardened_scales(&mut s);

        // Object enters from the graveyard as a "Modular 3" creature.
        let c = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            c, 0, Zone::Graveyard(0), 1, creature_chars(2, 2)));
        register_test_modular_etb(&mut s, c, 3);

        let new_id = s.move_object_to_zone(
            c, Zone::Battlefield,
            crate::events::MoveCause::SpellResolution,
        ).unwrap();

        assert_eq!(
            s.objects.get(new_id).unwrap()
                .count_counters(CounterKind::PlusOnePlusOne),
            4,
            "Modular 3 + Hardened Scales: ETB self-replacement places 3 +1/+1 \
             counters, that placement is itself replaced to 3+1 = 4");
    }

    // =====================================================================
    // source_controller sweep regression tests — "You" in a replacement's
    // filter resolves against the REPLACEMENT's source controller, not
    // the affected party. These tests would fail under the old
    // affected_controller_of semantic for damage / ETB / die pipelines.
    // =====================================================================

    /// Helper: install a permanent controlled by `owner` and return its id.
    /// Used as a stand-in "source" object for a replacement effect so the
    /// pipeline has something to compute source_controller_of against.
    fn install_replacement_source(s: &mut GameState, owner: PlayerId) -> ObjectId {
        put_creature(s, owner, 1, 1)
    }

    #[test]
    fn damage_replacement_source_filter_resolves_you_against_replacement_controller() {
        // "Prevent all damage dealt by a creature you control." Installed
        // by player 0's permanent. Player 0's attacker → prevented;
        // player 1's attacker → still deals damage.
        let mut s = GameState::new(2, 0);
        let my_perm = install_replacement_source(&mut s, 0);
        let my_attacker = put_creature(&mut s, 0, 2, 2);
        let enemy_attacker = put_creature(&mut s, 1, 2, 2);
        s.add_replacement_effect(base_effect(
            /*source=*/ my_perm,
            ReplacementCondition::WouldDealDamage {
                source_filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::Player,
            },
            ReplacementKind::PreventAllDamage,
        ));
        assert!(
            s.replace_damage(my_attacker, DamageTarget::Player(1), 3).is_none(),
            "my creature's damage to opponent should be prevented");
        assert_eq!(
            s.replace_damage(enemy_attacker, DamageTarget::Player(0), 3),
            Some((enemy_attacker, DamageTarget::Player(0), 3)),
            "opponent's creature is not 'you control' — damage passes through");
    }

    #[test]
    fn etb_replacement_object_filter_resolves_you_against_replacement_controller() {
        // "Creatures you control enter tapped." Installed by player 0's
        // permanent. Player 1's creature entering should NOT be tapped.
        let mut s = GameState::new(2, 0);
        let my_perm = install_replacement_source(&mut s, 0);
        s.add_replacement_effect(base_effect(
            /*source=*/ my_perm,
            ReplacementCondition::WouldEnterBattlefield {
                object_filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You),
            },
            ReplacementKind::EtbTapped,
        ));
        // Opponent's creature entering via move_object_to_zone.
        let enemy = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            enemy, 1, Zone::Hand(1), 1, creature_chars(2, 2)));
        let new_id = s.move_object_to_zone(
            enemy, Zone::Battlefield,
            crate::events::MoveCause::SpellResolution,
        ).unwrap();
        assert!(
            !s.objects.get(new_id).unwrap().is_tapped(),
            "opponent's creature is not 'you control' — should not enter tapped");
    }

    #[test]
    fn die_replacement_object_filter_resolves_you_against_replacement_controller() {
        // "If a creature you control would die, exile it instead."
        // Installed by player 0. Opponent's creature dying → still dies.
        let mut s = GameState::new(2, 0);
        let my_perm = install_replacement_source(&mut s, 0);
        s.add_replacement_effect(base_effect(
            /*source=*/ my_perm,
            ReplacementCondition::WouldDie {
                object_filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You),
            },
            ReplacementKind::ExileInsteadOfDying,
        ));
        let enemy = put_creature(&mut s, 1, 2, 2);
        assert_eq!(
            s.replace_die(enemy),
            DieOutcome::StillDies,
            "opponent's creature dying is not 'you control' — no exile swap");
    }
}
