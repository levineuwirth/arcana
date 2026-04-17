//! Core engine: [`step`], [`new_game`], phase/step progression,
//! mulligan, stack resolution.
//!
//! Addendum Section 6 / Listing 12, Phase 1 Task #20. Depends on
//! tasks 6–19.
//!
//! # Pure-function surface
//!
//! ```text
//!   (GameState, Action) -> (GameState, EngineYield)
//! ```
//!
//! The engine is trivially parallelizable: any branch of a tree
//! search can clone the state, submit an action, and recurse. No
//! shared mutable state, no I/O.
//!
//! # Flow
//!
//! 1. **[`step`]** applies the incoming [`Action`] via
//!    [`apply_action`], which is the match on every `Action` variant.
//! 2. **[`settle`]** drives the state machine through any steps that
//!    don't require agent input — `Untap` and `Cleanup` run inline,
//!    phase transitions cascade, and SBAs + triggers run between
//!    every potential decision point (CR 117.5).
//! 3. **[`compute_next_decision`]** inspects the settled state and
//!    returns the appropriate [`EngineYield`] — game-over, a pending
//!    special action, a combat declaration, or a priority window.
//!
//! # What's implemented (Phase 1)
//!
//! - Full turn/step/combat progression
//! - Cast spells, play lands, priority passing, stack resolution
//! - Combat declarations and damage dealing
//! - State-based actions between every decision
//! - Concede, mulligan keep/again, bottom cards (deterministic)
//! - Deterministic deck shuffling via seeded ChaCha8
//!
//! # Stubbed (Task #21 and beyond)
//!
//! - Spell / ability effect dispatch at resolution — without
//!   [`CardRegistry`] entries, resolving spells just finalizes the
//!   stack entry (card moves to graveyard or battlefield). Task #21
//!   wires in the registry callback.
//! - Trigger collection from registered permanents — the delayed
//!   trigger pipeline is connected, but "enters the battlefield"
//!   triggers and their kin sleep until Task #21.
//! - Target recheck at resolution uses an empty requirement list —
//!   targets are effectively always legal at resolution time. Task
//!   #21 threads real `TargetRequirement`s through the registry.


use crate::actions::{
    Action, ChoiceAction, DecisionContext,
};
use crate::combat::CombatPhase;
use crate::events::{GameEvent, LoseReason, MoveCause};
use crate::legal_actions::legal_actions;
use crate::objects::{Characteristics, GameObject, ObjectId};
use crate::priority::{PriorityOutcome, SpecialAction, receives_priority_at};
use crate::registry::CardRegistry;
use crate::sba::apply_state_based_actions;
use crate::state::{GameResult, GameState};
use crate::stack::ResolutionOutcome;
use crate::turn::{Phase, Step};
use crate::types::PlayerId;
use crate::zones::Zone;

// =============================================================================
// EngineYield
// =============================================================================

/// What the engine hands back after a call to [`step`] or [`new_game`].
#[derive(Debug)]
pub enum EngineYield {
    /// The game needs a decision from a player. `legal_actions`
    /// enumerates everything they can do; `context` says what *kind*
    /// of decision is being asked for.
    PendingDecision {
        player: PlayerId,
        legal_actions: Vec<Action>,
        context: DecisionContext,
    },
    /// The game has ended. The engine will stop accepting actions —
    /// subsequent `step` calls would short-circuit back to the same
    /// `GameOver` yield.
    GameOver(GameResult),
}

impl EngineYield {
    pub fn is_pending(&self) -> bool {
        matches!(self, EngineYield::PendingDecision { .. })
    }
    pub fn is_game_over(&self) -> bool {
        matches!(self, EngineYield::GameOver(_))
    }
}

// =============================================================================
// step
// =============================================================================

/// The core engine function. Apply `action` to `state`, settle the
/// state machine, and return the next [`EngineYield`].
///
/// Consumes `state` by value — the caller clones first if they need
/// to retain the pre-action state (typical for MCTS branching).
pub fn step(
    mut state: GameState,
    action: Action,
    registry: &CardRegistry,
) -> (GameState, EngineYield) {
    // Fast path: already ended.
    if let Some(result) = state.result.clone() {
        return (state, EngineYield::GameOver(result));
    }

    apply_action(&mut state, action, registry);
    settle(&mut state, registry);

    let yld = compute_next_decision(&state, registry);
    (state, yld)
}

// =============================================================================
// apply_action
// =============================================================================

/// Dispatch on the incoming [`Action`] and mutate state accordingly.
/// Runs no follow-up — the caller invokes [`settle`] afterward.
fn apply_action(state: &mut GameState, action: Action, registry: &CardRegistry) {
    match action {
        Action::PassPriority => apply_pass_priority(state, registry),

        Action::CastSpell {
            object_id, targets, modes, mana_payment, additional_costs, x_value,
        } => {
            apply_cast_spell(
                state, object_id, targets, modes,
                mana_payment, additional_costs, x_value,
            );
        }

        Action::ActivateAbility {
            source, ability_index, targets, mana_payment, additional_costs,
        } => apply_activate_ability(
            state, registry, source, ability_index, targets,
            mana_payment, additional_costs,
        ),

        Action::PlayLand { object_id } => apply_play_land(state, object_id),

        Action::DeclareAttackers { attackers } => {
            state.apply_declared_attackers(attackers);
            // Active player retains priority in the DeclareAttackers
            // step for responses.
            let ap = state.active_player();
            state.priority.give_to(ap);
        }

        Action::DeclareBlockers { blockers } => {
            state.apply_declared_blockers(blockers);
            // After blocks are declared, the active player receives
            // priority first for any damage-order reassignments and
            // then the usual response window opens.
            let ap = state.active_player();
            state.priority.give_to(ap);
        }

        Action::OrderBlockers { assignments } => {
            for a in assignments {
                state.set_damage_assignment(a);
            }
            let ap = state.active_player();
            state.priority.give_to(ap);
        }

        Action::MakeChoice(choice) => apply_make_choice(state, choice),

        Action::SubmitResolutionChoice { id, response } => {
            apply_resolution_choice(state, id, response);
        }

        Action::MulliganKeep => apply_mulligan_keep(state),
        Action::MulliganAgain => apply_mulligan_again(state),
        Action::BottomCards(ids) => apply_bottom_cards(state, ids),

        Action::Concede => apply_concede(state),
    }
}

// =============================================================================
// Action handlers
// =============================================================================

fn apply_pass_priority(state: &mut GameState, registry: &CardRegistry) {
    let num_players = state.num_players();
    match state.priority.pass(num_players) {
        PriorityOutcome::PassedTo(_) => { /* already rotated */ }
        PriorityOutcome::EveryonePassed => {
            if !state.stack_is_empty() {
                // Resolve the top-of-stack object.
                resolve_top_of_stack(state, registry);
                // After resolution, active player gets priority again
                // (CR 117.3b). SBA + triggers run in settle().
                let ap = state.active_player();
                state.priority.give_to(ap);
            } else {
                // Stack empty and everyone passed: advance the
                // phase/step. advance_phase handles priority for the
                // new step.
                advance_phase(state, registry);
            }
        }
    }
}

fn apply_cast_spell(
    state: &mut GameState,
    object_id: ObjectId,
    targets: crate::targets::TargetSelection,
    modes: Vec<crate::stack::ModeChoice>,
    mana_payment: crate::actions::ManaPaymentPlan,
    additional_costs: Vec<crate::actions::AdditionalCostPayment>,
    x_value: Option<u32>,
) {
    let controller = state.priority_player();

    // 1. Spend the mana payment (pool units + convoke taps + delve
    //    exiles + Phyrexian life). Note: convoke and delve reduce
    //    *generic* cost; they must run alongside the pool spend so
    //    generic pips get satisfied.
    spend_mana_plan(state, controller, &mana_payment);

    // 2. Pay additional costs (sacrifice, discard, life, etc.).
    apply_additional_costs(state, controller, &additional_costs);

    // 3. Announce on the stack (CR 601.2a). Moves card Hand→Stack,
    //    emits ZoneChange(Cast).
    let entry_id = state.announce_spell_on_stack(
        object_id, controller, targets, modes, x_value);

    // 4. Emit SpellCast (CR 601.2e) — triggers pick this up.
    state.emit_spell_cast(entry_id);

    // 5. Record the action (priority retained, pass counter reset).
    state.priority.record_action();
}

fn apply_activate_ability(
    state: &mut GameState,
    registry: &CardRegistry,
    source: ObjectId,
    ability_index: usize,
    targets: crate::targets::TargetSelection,
    mana_payment: crate::actions::ManaPaymentPlan,
    additional_costs: Vec<crate::actions::AdditionalCostPayment>,
) {
    let controller = state.priority_player();

    // Look up the ability. Bail silently on unregistered / invalid —
    // legal_actions shouldn't have emitted such an action.
    let (is_mana_ability, tap, sacrifice, life) = {
        let Some(obj) = state.objects.get(source) else { return; };
        let Some(def) = registry.get(obj.card_id) else { return; };
        let Some(ability) = def.activated_abilities.get(ability_index) else { return; };
        (
            ability.is_mana_ability,
            ability.cost.tap,
            ability.cost.sacrifice,
            ability.cost.life,
        )
    };

    // --- Pay costs ---
    spend_mana_plan(state, controller, &mana_payment);
    if tap {
        if let Some(obj) = state.objects.get_mut(source) {
            if obj.tap() {
                state.emit(GameEvent::Tapped { object_id: source });
            }
        }
    }
    if life > 0 {
        state.player_mut(controller).life -= life as i32;
        state.emit(GameEvent::LifeLost { player: controller, amount: life });
    }
    if sacrifice {
        state.move_object_to_zone(
            source, Zone::Graveyard(controller), MoveCause::Cost);
    }
    // Any additional costs beyond the fixed cost (tap/sacrifice-self/
    // life) — sacrifice-a-different-permanent, discard, exile from
    // graveyard, reveal, remove-counters — flow through the shared
    // helper that also powers `apply_cast_spell`.
    apply_additional_costs(state, controller, &additional_costs);

    if is_mana_ability {
        // CR 605.3a — mana abilities resolve immediately without
        // using the stack. Dispatch the effect directly.
        let ctx = crate::registry::ActivationContext {
            source,
            controller,
            ability_index,
            targets,
            x_value: None,
        };
        let effects: Vec<crate::effects::Effect> = {
            let def = registry.get(
                state.objects.get(source).map(|o| o.card_id).unwrap_or(0));
            match def {
                Some(d) => d.activated_abilities.get(ability_index)
                    .map(|a| (a.effect)(state, &ctx, registry))
                    .unwrap_or_default(),
                None => Vec::new(),
            }
        };
        for effect in effects {
            effect.execute(state);
        }
    } else {
        // Non-mana activated abilities go on the stack (CR 602.2).
        // Phase 1: we track the ability_index via the `ability_id`
        // slot on the stack entry — resolution uses it to look up
        // the ability definition again.
        let entry_id = state.allocate_object_id();
        let entry = crate::stack::StackEntry::new_activated_ability(
            entry_id,
            source,
            controller,
            ability_index as crate::types::AbilityId,
            /*text=*/ String::new(),
            targets,
            Vec::new(),
            None,
        );
        state.push_stack_entry(entry);
    }

    state.priority.record_action();
}

fn apply_play_land(state: &mut GameState, object_id: ObjectId) {
    let controller = state.priority_player();
    state.player_mut(controller).land_plays_remaining =
        state.player(controller).land_plays_remaining.saturating_sub(1);
    // Land play is NOT a spell — no stack entry. Direct ETB.
    state.move_object_to_zone(object_id, Zone::Battlefield, MoveCause::PlayLand);
    // Set controller explicitly (owner != controller is possible for
    // some edge cases; for Play Land the active player takes control).
    if let Some(obj) = state.objects.get_mut(object_id) {
        obj.controller = controller;
        // Lands don't have summoning sickness for mana purposes
        // (CR 302.1 / 305.4), but we set the flag consistently; it
        // only matters for creature attacks anyway.
        obj.status.summoning_sick = true;
    }
    state.priority.record_action();
}

/// Route a [`Action::SubmitResolutionChoice`] to its dispatcher. The
/// id in the action must match [`GameState::pending_choice`]'s id —
/// a mismatch panics (stale response from the session layer after
/// the choice it was answering has already resolved).
pub(crate) fn apply_resolution_choice(
    state: &mut GameState,
    id: u64,
    response: crate::actions::ChoiceResponse,
) {
    let pending = state.pending_choice.take()
        .expect("apply_resolution_choice: no pending choice");
    assert_eq!(
        pending.id, id,
        "apply_resolution_choice: stale id {id}, expected {}",
        pending.id,
    );

    use crate::actions::{ChoiceKind, ChoiceContext, ChoiceResponse};

    match (&pending.kind, &pending.context, &response) {
        // --- OrderCards (Scry, Surveil, Fateseal) --------------------
        (
            ChoiceKind::OrderCards { cards, allowed },
            ChoiceContext::ResolvingStack(_),
            ChoiceResponse::OrderCards { placements },
        ) => {
            apply_order_cards(state, pending.choosing_player,
                cards, allowed, placements);
        }

        // --- PickCards mid-resolution (Tutor, Search, Reanimate, …) -
        // The pushing effect stored semantics in
        // `pending_choice_follow_up`; we dispatch on that.
        (
            ChoiceKind::PickCards { candidates, min, max },
            ChoiceContext::ResolvingStack(_),
            ChoiceResponse::PickCards { picked },
        ) => {
            assert_pick_cards_well_formed(candidates, picked, *min, *max);
            let follow_up = state.pending_choice_follow_up.take()
                .expect("apply_resolution_choice: PickCards at \
                         ResolvingStack context requires a \
                         pending_choice_follow_up");
            apply_choice_follow_up(state, follow_up, picked);
        }

        // --- PickCards at SBA time: Legend rule (CR 704.5j) ----------
        (
            ChoiceKind::PickCards { candidates, min, max },
            ChoiceContext::Sba,
            ChoiceResponse::PickCards { picked },
        ) => {
            assert_pick_cards_well_formed(candidates, picked, *min, *max);
            // Current SBA users of PickCards: Legend rule — `picked`
            // is the keeper set (typically one id); everything else in
            // `candidates` is sacrificed to its owner's graveyard.
            let sacrificed: Vec<crate::objects::ObjectId> = candidates.iter()
                .copied()
                .filter(|id| !picked.contains(id))
                .collect();
            for id in sacrificed {
                let Some(owner) = state.objects.get(id).map(|o| o.owner)
                    else { continue; };
                state.move_object_to_zone(
                    id,
                    crate::zones::Zone::Graveyard(owner),
                    crate::events::MoveCause::StateBasedAction,
                );
            }
            // Re-enter the SBA loop: more legend groups may still be
            // conflicting, and the just-sacrificed moves may have
            // triggered other SBAs.
            crate::sba::apply_state_based_actions(state);
        }

        // --- PayOrDecline mid-resolution (Ward — Phase 2-A stopgap) --
        (
            ChoiceKind::PayOrDecline { cost, on_decline },
            ChoiceContext::ResolvingStack(_),
            ChoiceResponse::PayOrDecline { pay },
        ) => {
            if *pay {
                auto_pay_ward_cost(state, pending.choosing_player, cost);
            } else {
                apply_decline_consequence(state, on_decline.clone());
            }
        }

        // --- Mismatched response kinds: hard error (spec §41.6 R4) ---
        _ => {
            panic!(
                "apply_resolution_choice: response {:?} does not match \
                 pending kind {:?}",
                response, pending.kind,
            );
        }
    }

    // Resume any parked resolution now that the choice is cleared.
    // A handler may have pushed a *new* pending_choice itself (e.g. a
    // second Scry in one resolution); in that case resume_* parks
    // again and yields back out.
    resume_parked_resolution(state);
}

/// Validate a `PickCards` answer against its prompt. Panics on count
/// out-of-range or id-not-in-candidates (agent / programmer bug).
fn assert_pick_cards_well_formed(
    candidates: &[ObjectId],
    picked: &[ObjectId],
    min: u32,
    max: u32,
) {
    assert!(picked.len() >= min as usize && picked.len() <= max as usize,
        "PickCards: picked.len() = {} not in [{min}, {max}]",
        picked.len());
    for id in picked {
        assert!(candidates.contains(id),
            "PickCards: picked id {id} not in candidates");
    }
}

/// Route cards to their chosen destinations in an `OrderCards` answer.
/// Destinations are validated against `allowed`; invalid placements
/// panic (agent-layer bug).
fn apply_order_cards(
    state: &mut GameState,
    choosing_player: PlayerId,
    cards: &[ObjectId],
    allowed: &[crate::actions::CardDestination],
    placements: &[(ObjectId, crate::actions::CardDestination)],
) {
    use crate::actions::CardDestination;

    assert_eq!(placements.len(), cards.len(),
        "apply_order_cards: placement count {} ≠ card count {}",
        placements.len(), cards.len());

    // Validate every placement's card + destination.
    for (pid, dest) in placements {
        assert!(cards.contains(pid),
            "apply_order_cards: {pid} not in prompt's cards");
        assert!(allowed.contains(dest),
            "apply_order_cards: destination {dest:?} not in allowed set");
    }

    // Placements are applied in submitted order. `Top` preserves the
    // declared order (first submitted = ends up on top); `Bottom` puts
    // cards at the bottom in submitted order.
    //
    // Simplest route: for each card, remove it from its current library
    // position (if present) and move to the target zone. For Top/Bottom
    // we rebuild library ordering.
    //
    // First: move any non-library destinations.
    for (pid, dest) in placements {
        match dest {
            CardDestination::Graveyard => {
                let owner = state.objects.get(*pid)
                    .map(|o| o.owner).unwrap_or(choosing_player);
                state.move_object_to_zone(
                    *pid, crate::zones::Zone::Graveyard(owner),
                    crate::events::MoveCause::SpellResolution);
            }
            CardDestination::Hand => {
                state.move_object_to_zone(
                    *pid, crate::zones::Zone::Hand(choosing_player),
                    crate::events::MoveCause::SpellResolution);
            }
            CardDestination::Exile => {
                state.move_object_to_zone(
                    *pid, crate::zones::Zone::Exile,
                    crate::events::MoveCause::SpellResolution);
            }
            CardDestination::Battlefield => {
                // Direct-to-battlefield via OrderCards (Collected
                // Company-style). Controller is the choosing player.
                if let Some(obj) = state.objects.get_mut(*pid) {
                    obj.controller = choosing_player;
                }
                state.move_object_to_zone(
                    *pid, crate::zones::Zone::Battlefield,
                    crate::events::MoveCause::SpellResolution);
            }
            CardDestination::TopOfLibrary | CardDestination::BottomOfLibrary => {
                // handled below via library ordering
            }
        }
    }

    // Remove all prompt-cards that are still in the library from the
    // current library ordering, then re-insert per submitted order.
    let lib = &mut state.player_mut(choosing_player).library_top_to_bottom;
    lib.retain(|id| !cards.contains(id));

    // Top: first-submitted ends up on top, so we need to prepend in
    // REVERSE insertion order so the first becomes the top.
    // Bottom: first-submitted ends up at the bottom-most slot, i.e.
    // appended in insertion order.
    let mut top_ids: Vec<ObjectId> = Vec::new();
    let mut bottom_ids: Vec<ObjectId> = Vec::new();
    for (pid, dest) in placements {
        match dest {
            CardDestination::TopOfLibrary => top_ids.push(*pid),
            CardDestination::BottomOfLibrary => bottom_ids.push(*pid),
            _ => {}
        }
    }
    // Apply Top (prepend in reverse so first = topmost).
    for id in top_ids.into_iter().rev() {
        state.player_mut(choosing_player)
            .library_top_to_bottom.insert(0, id);
    }
    // Apply Bottom (append in submitted order).
    for id in bottom_ids {
        state.player_mut(choosing_player)
            .library_top_to_bottom.push(id);
    }
}

/// Greedily deduct `cost` from `player`'s mana pool. Called when an
/// agent answers `true` to a PayOrDecline (Ward, for now).
///
/// Phase 2-A stopgap: we pick the first valid payment plan the solver
/// produces rather than letting the agent author one. This means the
/// agent can't choose *which* hybrid pip to pay with, *which* snow
/// source to apply, etc. Those are Phase 2-B work, where PayOrDecline
/// will carry a `ManaPaymentPlan` instead of a bare bool.
fn auto_pay_ward_cost(
    state: &mut GameState,
    player: PlayerId,
    cost: &crate::mana::ManaCost,
) {
    let plans = crate::mana::enumerate_payment_plans(
        cost,
        &state.player(player).mana_pool,
        /*x_value=*/ None,
        &crate::mana::SpendContext::unrestricted(),
    );
    let plan = plans.into_iter().next()
        .expect("auto_pay_ward_cost: agent answered 'pay' but solver \
                 found no valid plan — legal-action filter should \
                 have hidden the pay option");
    spend_mana_plan(state, player, &plan);
}

/// Consume the parked resolution and apply the decline consequence
/// from a PayOrDecline (Phase 2-A: Ward → CounterStackEntry).
fn apply_decline_consequence(
    state: &mut GameState,
    consequence: crate::actions::DeclineConsequence,
) {
    use crate::actions::DeclineConsequence;
    match consequence {
        DeclineConsequence::CounterStackEntry(_entry_id) => {
            // Take the parked entry — we never run its effects. Its
            // id in `consequence` matches `parked.entry.id` by
            // construction, but we don't double-check (the handler
            // only runs under a specific pending_choice, and that
            // choice was freshly pushed by `begin_ward_check`).
            let Some(parked) = state.pending_resolution.take() else {
                return;
            };
            state.currently_resolving = None;
            if parked.is_spell {
                state.counter_resolved_spell(parked.entry);
            } else {
                state.counter_resolved_ability(parked.entry);
            }
        }
        DeclineConsequence::SkipEffect => {
            // "May" effects: just clear currently_resolving and let
            // resume_parked_resolution advance to the next effect.
            // (Not used in Phase 2-A; stubbed for forward-compat.)
        }
    }
}

/// Apply a [`crate::actions::ChoiceFollowUp`] to a list of picked ids.
/// Called by the dispatcher after resolving a `PickCards` answer.
fn apply_choice_follow_up(
    state: &mut GameState,
    follow_up: crate::actions::ChoiceFollowUp,
    chosen: &[ObjectId],
) {
    use crate::actions::ChoiceFollowUp;
    use crate::events::GameEvent;
    use crate::zones::Zone;
    match follow_up {
        ChoiceFollowUp::MoveToZone {
            destination, reveal, shuffle_library_owner,
        } => {
            for id in chosen {
                let new_id = state.move_object_to_zone(
                    *id, destination, MoveCause::SpellResolution);
                if reveal {
                    if let Some(visible_id) = new_id {
                        for p in 0..state.num_players() {
                            state.player_mut(p).known_cards.insert(visible_id);
                        }
                    }
                }
            }
            if let Some(p) = shuffle_library_owner {
                state.shuffle_library(p);
            }
        }
        ChoiceFollowUp::MoveToBattlefield {
            controller, tapped, shuffle_library_owner,
        } => {
            for id in chosen {
                // Set controller pre-move so the move preserves it on
                // entry to the battlefield.
                if let Some(obj) = state.objects.get_mut(*id) {
                    obj.controller = controller;
                }
                let new_id = state.move_object_to_zone(
                    *id, Zone::Battlefield, MoveCause::SpellResolution);
                if let Some(bf_id) = new_id {
                    if let Some(obj) = state.objects.get_mut(bf_id) {
                        obj.controller = controller;
                        if tapped { obj.tap(); }
                    }
                }
            }
            if let Some(p) = shuffle_library_owner {
                state.shuffle_library(p);
            }
        }
        ChoiceFollowUp::Sacrifice { player } => {
            for id in chosen {
                state.emit(GameEvent::Sacrifice { player, object_id: *id });
                let owner = state.objects.get(*id)
                    .map(|o| o.owner).unwrap_or(player);
                state.move_object_to_zone(
                    *id, Zone::Graveyard(owner), MoveCause::SpellResolution);
            }
        }
        ChoiceFollowUp::Discard { player } => {
            for id in chosen {
                let _ = state.move_object_to_zone(
                    *id, Zone::Graveyard(player), MoveCause::Cost);
                // `Discarded` refers to the pre-move id — that matches
                // the old behavior and is what trigger filters compare
                // against via LKI.
                state.emit(GameEvent::Discarded { player, object_id: *id });
            }
        }
    }
}

fn apply_make_choice(state: &mut GameState, choice: ChoiceAction) {
    // MakeChoice is the catch-all action for state-driven decisions
    // outside the normal cast/play/declare flow. The active
    // `SpecialAction` (or a stack-entry resolution context, once
    // Phase 2 wires them) disambiguates what the choice means.
    let player = state.priority_player();

    match state.priority.special_action.clone() {
        Some(SpecialAction::DiscardToHandSize) => {
            if let ChoiceAction::ChooseObject(id) = choice {
                state.move_object_to_zone(
                    id, Zone::Graveyard(player), MoveCause::Cost);
                state.emit(GameEvent::Discarded { player, object_id: id });
            }
            // Clear the special action once the hand is within size;
            // otherwise re-prompt (compute_next_decision re-reads
            // `priority.special_action`).
            if state.objects.count_in_zone(Zone::Hand(player))
                <= state.format.max_hand_size as usize
            {
                state.priority.end_special_action();
            }
        }

        Some(SpecialAction::ChooseFirstPlayer) => {
            if let ChoiceAction::ChoosePlayer(p) = choice {
                // The chosen player takes turn 1. Bump the turn's
                // active_player and reset priority to them.
                state.turn.active_player = p;
                state.priority.end_special_action();
                // Begin the mulligan decision for the newly active
                // player; settle() will take it from there.
                state.priority.begin_special_action(
                    SpecialAction::MulliganDecision, p);
            }
        }

        // Sideboarding isn't a Phase 1 concern; no-op for now.
        Some(SpecialAction::Sideboarding) => {}

        // Mulligan decision and London-bottom-cards use their own
        // dedicated actions (MulliganKeep/Again, BottomCards) —
        // MakeChoice would be spurious.
        Some(SpecialAction::MulliganDecision)
        | Some(SpecialAction::LondonMulliganBottomCards(_)) => {}

        // No pending special action: the choice must be for a
        // mid-resolution prompt. Phase 1 doesn't expose any spell
        // whose resolution yields such a prompt — Phase 2 wires the
        // `PendingResolutionChoice` pipeline in. Silently drop
        // unexpected choices so a future card author sees a "no
        // effect" bug surface quickly rather than a panic on an
        // incorrect choice.
        None => {
            // TODO(phase-2): route through a `PendingResolutionChoice`
            // queue on GameState once modal/scry/distribute-style
            // resolution-time decisions are implemented.
        }
    }
}

fn apply_mulligan_keep(state: &mut GameState) {
    // CR 103.4a — when a player keeps after taking N mulligans,
    // they must put N cards from hand on the bottom of their
    // library before the game begins.
    let player = state.priority_player();
    let owed = state.player(player).mulligans_taken;

    if owed > 0 {
        // Transition into the bottoming window; stay on this player
        // until they submit an `Action::BottomCards(ids)` of the
        // right length.
        state.priority.begin_special_action(
            SpecialAction::LondonMulliganBottomCards(owed), player);
        return;
    }

    // No mulligans → they're done deciding.
    state.player_mut(player).mulligan_decided = true;

    if let Some(next) = next_undecided_mulligan_player(state) {
        state.priority.begin_special_action(
            SpecialAction::MulliganDecision, next);
    } else {
        end_mulligan_phase(state);
    }
}

fn apply_mulligan_again(state: &mut GameState) {
    let player = state.priority_player();
    // London mulligan: shuffle hand into library, redraw 7. Owes one
    // card to the bottom per mulligan taken.
    shuffle_hand_into_library(state, player);
    state.player_mut(player).mulligans_taken =
        state.player(player).mulligans_taken.saturating_add(1);
    let hand_size = state.format.starting_hand_size;
    for _ in 0..hand_size {
        state.draw_one_card(player);
    }
    // Stay in mulligan decision for this player.
    state.priority.begin_special_action(
        SpecialAction::MulliganDecision, player);
}

fn apply_bottom_cards(state: &mut GameState, ids: Vec<ObjectId>) {
    let player = state.priority_player();

    // Validate length against the owed count. Only meaningful during
    // a London-mulligan bottoming window — everywhere else this
    // action is spurious and silently dropped.
    let owed = match state.priority.special_action {
        Some(SpecialAction::LondonMulliganBottomCards(n)) => n,
        _ => return,
    };
    if ids.len() as u32 != owed {
        // Malformed submission — per CR 103.4a the engine should
        // prompt again. For Phase 1 we panic because a correctly
        // behaving agent consulting `legal_actions` would never
        // submit the wrong length.
        panic!(
            "BottomCards: submitted {} ids but {} are owed",
            ids.len(), owed,
        );
    }

    // Ensure every id really is in this player's hand — otherwise
    // we'd move some opponent's card. Malformed actions panic rather
    // than silently skip; agents should validate before submission.
    for &id in &ids {
        let zone = state.objects.get(id).map(|o| o.zone);
        assert_eq!(zone, Some(Zone::Hand(player)),
            "BottomCards: id {id} is not in player {player}'s hand");
    }

    for id in ids {
        state.move_object_to_zone(
            id, Zone::Library(player), MoveCause::Cost);
        // `move_object_to_zone` appends to the library's order vec,
        // which is the bottom — exactly what we want.
    }

    // Bottoming complete; lock in this player's opening hand and
    // move on.
    state.priority.end_special_action();
    state.player_mut(player).mulligan_decided = true;
    if let Some(next) = next_undecided_mulligan_player(state) {
        state.priority.begin_special_action(
            SpecialAction::MulliganDecision, next);
    } else {
        end_mulligan_phase(state);
    }
}

fn apply_concede(state: &mut GameState) {
    let player = state.priority_player();
    state.player_mut(player).has_lost = true;
    state.player_mut(player).has_conceded = true;
    state.emit(GameEvent::PlayerLoses {
        player, reason: LoseReason::Concession,
    });
}

// =============================================================================
// settle — drive the state machine to the next decision point
// =============================================================================

/// Advance the state through every transition that does not require
/// agent input. Runs SBAs + triggers between each advance.
fn settle(state: &mut GameState, registry: &CardRegistry) {
    // Cap iterations — if we ever advance for this many passes
    // without reaching a yield, there's a state machine bug. The
    // panic is a programmer bug per the design principle.
    for _ in 0..MAX_SETTLE_ITERATIONS {
        run_sba_and_triggers(state, registry);
        if state.is_game_over() { return; }

        if decision_pending(state) { return; }

        // No decision pending: advance the state machine.
        advance_phase(state, registry);
    }
    panic!("engine::settle: state machine failed to reach a yield in \
            {MAX_SETTLE_ITERATIONS} passes — likely a bug in advance_phase");
}

/// Is a decision currently required from a player?
fn decision_pending(state: &GameState) -> bool {
    if state.pending_choice.is_some() { return true; }
    if state.priority.in_special_action() { return true; }
    if combat_decision_pending(state) { return true; }
    if receives_priority_at(state.turn.step) { return true; }
    false
}

fn combat_decision_pending(state: &GameState) -> bool {
    let Some(combat) = state.combat.as_ref() else { return false; };
    matches!(
        combat.phase,
        CombatPhase::DeclareAttackers | CombatPhase::DeclareBlockers
    )
}

/// CR 117.5 / 603.3 — between every potential priority window, run
/// state-based actions to stability, then collect any triggered
/// abilities that matched events since the last scan, push them on
/// the stack in APNAP order, and loop. Delayed triggers are drained
/// alongside registered-permanent triggers.
///
/// Termination: the loop is bounded by `MAX_SETTLE_ITERATIONS`. SBAs
/// either settle (producing no new events) or a game-over condition
/// short-circuits; triggers can only fire from events that existed
/// when the loop iteration started, so each pass makes forward
/// progress.
fn run_sba_and_triggers(state: &mut GameState, registry: &CardRegistry) {
    for _ in 0..MAX_SETTLE_ITERATIONS {
        // 1. SBAs — CR 704.3.
        apply_state_based_actions(state);
        if state.is_game_over() { state.clear_lki(); return; }

        // 2. Collect triggers from events since the last scan. We
        //    snapshot the cursor, advance it to the current tail, and
        //    operate on the snapshot. Triggers pushed during this
        //    pass produce new events, but those events will be
        //    scanned on the NEXT iteration — not this one — which
        //    matches CR 603.2's "each event is checked once against
        //    each trigger".
        let start = state.trigger_event_cursor;
        let end = state.event_log.len();
        if start == end {
            // Nothing new happened; scanner has nothing to do.
            state.clear_lki();
            return;
        }
        let new_events: Vec<crate::events::GameEvent> =
            state.event_log[start..end].to_vec();
        state.trigger_event_cursor = end;

        let pending = collect_pending_triggers(state, registry, &new_events);

        // LKI served this pass's trigger scan; drop it before we
        // head into the next iteration's SBAs so subsequent moves
        // don't accumulate on top of stale entries.
        state.clear_lki();

        if pending.is_empty() { return; }

        // 3. Push each pending trigger as a TriggeredAbility stack
        //    entry. Order is already APNAP-sorted by
        //    `collect_pending_triggers`.
        for pt in pending {
            let entry_id = state.allocate_object_id();
            let entry = crate::stack::StackEntry::new_triggered_ability(
                entry_id,
                pt.source,
                pt.controller,
                pt.trigger_id,
                pt.trigger_event.clone(),
                /*text=*/ String::new(),
                crate::targets::TargetSelection::new(),
                Vec::new(),
            );
            state.push_stack_entry(entry);
            state.record_trigger_fired(pt.source, pt.trigger_id);
        }
        // Loop — the push emitted no direct events, but on the next
        // iteration SBAs re-run just in case the trigger list
        // referenced dead objects, and any further delayed triggers
        // watching for the events we just processed get a chance.
    }
    // Hitting the iteration cap indicates a genuine engine bug
    // (infinite trigger storm, cursor not advancing, etc.).
    panic!("run_sba_and_triggers: failed to settle in \
            {MAX_SETTLE_ITERATIONS} iterations");
}

/// Walk every registered triggered ability on permanents currently
/// on the battlefield (and in triggering zones for zone-change
/// triggers), testing each against every event in `events`. Also
/// drain matching delayed triggers. Returns the merged list
/// APNAP-sorted.
fn collect_pending_triggers(
    state: &mut GameState,
    registry: &CardRegistry,
    events: &[crate::events::GameEvent],
) -> Vec<crate::triggers::PendingTrigger> {
    let mut pending: Vec<crate::triggers::PendingTrigger> = Vec::new();

    // 1. Registered triggers from permanents currently in a zone that
    //    can produce triggers, PLUS last-known-information snapshots
    //    of objects that already moved this sweep. LKI makes dies /
    //    leaves-the-battlefield triggers fire for the OLD id the
    //    event carries, per CR 603.10 / 400.7. We snapshot first so
    //    we don't hold a borrow while iterating.
    let mut trigger_sources: Vec<(ObjectId, crate::types::CardId, PlayerId)> =
        state.objects.iter()
            .map(|o| (o.id, o.card_id, o.controller))
            .collect();
    // LKI entries are keyed by the pre-move id — which is the id the
    // leaving/dying events carry — so their triggers correctly match.
    trigger_sources.extend(
        state.lki.iter().map(|(id, o)| (*id, o.card_id, o.controller)));

    for event in events {
        for &(source, card_id, controller) in &trigger_sources {
            let Some(def) = registry.get(card_id) else { continue; };
            for ability in &def.triggered_abilities {
                if let Some(pt) = ability.should_fire(
                    event, source, controller, state)
                {
                    pending.push(pt);
                }
            }
        }

        // 2. Delayed triggers matching this event. `take_matching_delayed_triggers`
        //    is APNAP-sorted internally; we re-sort after merging with
        //    registered-trigger matches anyway.
        let delayed = state.take_matching_delayed_triggers(event);
        pending.extend(delayed);

        // 3. Auto-applied keyword triggers that bypass the stack in
        //    Phase 1. These are known, bounded behaviors we don't yet
        //    route through the full TriggeredAbilityDef system.
        apply_prowess_on_cast(state, event);
    }

    crate::triggers::sort_by_apnap(
        &mut pending, state.active_player(), state.num_players());
    pending
}

/// CR 702.108 — Prowess: "Whenever you cast a noncreature spell, this
/// creature gets +1/+1 until end of turn." Phase 1 applies the pump
/// directly (bypasses the stack) because the trigger has no agent
/// choice. Runs once per SpellCast event.
fn apply_prowess_on_cast(state: &mut GameState, event: &crate::events::GameEvent) {
    use crate::effects::KeywordAbility;
    let crate::events::GameEvent::SpellCast {
        object_id: cast_id, controller: caster, ..
    } = event else { return; };
    // Is the cast spell noncreature?
    let is_creature_cast = state.objects.get(*cast_id)
        .is_some_and(|o| o.is_creature());
    if is_creature_cast { return; }

    let targets: Vec<ObjectId> = state.objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == *caster
            && state.has_keyword(o.id, &KeywordAbility::Prowess))
        .map(|o| o.id)
        .collect();
    for id in targets {
        state.add_continuous_effect(
            crate::layers::ContinuousEffect::pump(
                /*source=*/ id, id,
                1, 1, crate::layers::Duration::EndOfTurn));
    }
}

// =============================================================================
// advance_phase — the (phase, step) state machine
// =============================================================================

/// Transition the turn state machine by one step. After transition,
/// priority is (re)granted according to CR 117 rules for the new step.
pub fn advance_phase(state: &mut GameState, _registry: &CardRegistry) {
    match (state.turn.phase, state.turn.step) {
        (Phase::Beginning, Step::Untap) => {
            untap_step(state);
            state.turn.step = Step::Upkeep;
            emit_step_begins(state);
        }
        (Phase::Beginning, Step::Upkeep) => {
            state.turn.step = Step::Draw;
            emit_step_begins(state);
        }
        (Phase::Beginning, Step::Draw) => {
            // CR 103.7a: active player on turn 1 of a 2-player game
            // does not draw. For 3+ players, everyone draws on turn 1.
            let skip_draw = state.turn.turn_number == 1
                && state.num_players() == 2;
            if !skip_draw {
                let ap = state.active_player();
                state.draw_one_card(ap);
            }
            state.turn.phase = Phase::PreCombatMain;
            state.turn.step = Step::Main;
            emit_phase_and_step_begins(state);
        }
        (Phase::PreCombatMain, Step::Main) => {
            state.turn.phase = Phase::Combat;
            state.turn.step = Step::BeginCombat;
            state.begin_combat();
            emit_phase_and_step_begins(state);
        }
        (Phase::Combat, Step::BeginCombat) => {
            state.turn.step = Step::DeclareAttackers;
            state.enter_declare_attackers();
            emit_step_begins(state);
        }
        (Phase::Combat, Step::DeclareAttackers) => {
            state.turn.step = Step::DeclareBlockers;
            state.enter_declare_blockers();
            emit_step_begins(state);
        }
        (Phase::Combat, Step::DeclareBlockers) => {
            // First-strike gating: enter the FS damage sub-step only
            // if some creature in combat has FS/double strike. Until
            // keywords are real, skip straight to regular damage.
            let has_fs = state.combat.as_ref()
                .map(|c| c.has_first_strike).unwrap_or(false);
            state.turn.step = if has_fs {
                Step::CombatDamage
            } else {
                Step::CombatDamageRegular
            };
            emit_step_begins(state);
        }
        (Phase::Combat, Step::CombatDamage) => {
            state.deal_first_strike_damage();
            state.turn.step = Step::CombatDamageRegular;
            emit_step_begins(state);
        }
        (Phase::Combat, Step::CombatDamageRegular) => {
            state.deal_combat_damage();
            state.turn.step = Step::EndCombat;
            emit_step_begins(state);
        }
        (Phase::Combat, Step::EndCombat) => {
            state.end_combat();
            state.turn.phase = Phase::PostCombatMain;
            state.turn.step = Step::Main;
            // Extra-combat check: if there's another combat queued,
            // go back to the combat phase instead.
            if state.turn.consume_extra_combat() {
                state.turn.phase = Phase::Combat;
                state.turn.step = Step::BeginCombat;
                state.begin_combat();
            }
            emit_phase_and_step_begins(state);
        }
        (Phase::PostCombatMain, Step::Main) => {
            state.turn.phase = Phase::Ending;
            state.turn.step = Step::End;
            emit_phase_and_step_begins(state);
        }
        (Phase::Ending, Step::End) => {
            state.turn.step = Step::Cleanup;
            emit_step_begins(state);
        }
        (Phase::Ending, Step::Cleanup) => {
            cleanup_step(state);
            next_turn(state);
        }
        // Any other (phase, step) pair is invalid — but we've covered
        // every CR-legal combo above. Treat the unreachable branch as
        // a programmer bug.
        (p, s) => panic!(
            "advance_phase: impossible (phase, step) = ({p:?}, {s:?})"),
    }
    // After the transition, grant priority to whoever should receive
    // it in the new step. `receives_priority_at` handles Untap and
    // Cleanup (returns false).
    grant_priority_for_current_step(state);
}

fn grant_priority_for_current_step(state: &mut GameState) {
    if !receives_priority_at(state.turn.step) { return; }
    // Default: active player receives priority (CR 117.1).
    let player = state.active_player();
    state.priority.give_to(player);
}

// =============================================================================
// Turn-boundary helpers: untap / cleanup / next turn
// =============================================================================

fn untap_step(state: &mut GameState) {
    let ap = state.active_player();
    state.player_mut(ap).reset_land_plays();
    // CR 502.1 — untap the active player's permanents; remove
    // summoning sickness from their creatures.
    // TODO(keywords): honor "doesn't untap" effects (e.g., Stasis).
    // TODO(task-21): honor phasing (CR 502.1 step order).
    let ids: Vec<ObjectId> = state.objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == ap)
        .map(|o| o.id)
        .collect();
    for id in ids {
        let untapped = {
            let obj = state.objects.get_mut(id).unwrap();
            obj.status.summoning_sick = false;
            obj.untap()
        };
        if untapped {
            state.emit(GameEvent::Untapped { object_id: id });
        }
    }
}

fn cleanup_step(state: &mut GameState) {
    let ap = state.active_player();

    // CR 514.1 — discard to hand size. Stubbed for Phase 1; if the
    // hand is over size, we forcibly discard the lowest-id cards
    // until it fits. A proper engine would yield a
    // SpecialAction::DiscardToHandSize here; that path exists in
    // legal_actions and apply_make_choice for future use.
    let hand_ids = state.objects.ids_in_zone_sorted(Zone::Hand(ap));
    let over_by = hand_ids.len().saturating_sub(state.format.max_hand_size as usize);
    for id in hand_ids.into_iter().take(over_by) {
        state.move_object_to_zone(
            id, Zone::Graveyard(ap), MoveCause::Cost);
        state.emit(GameEvent::Discarded { player: ap, object_id: id });
    }

    // CR 514.2 — clear damage from all permanents.
    let bf_ids: Vec<ObjectId> = state.objects
        .objects_in_zone(Zone::Battlefield)
        .map(|o| o.id).collect();
    for id in bf_ids {
        if let Some(obj) = state.objects.get_mut(id) {
            obj.clear_damage();
        }
    }

    // CR 514.2 — end "until end of turn" effects and replacements.
    state.expire_end_of_turn_effects();
    state.expire_end_of_turn_replacements();

    // CR 603.3 — clear once-per-turn trigger ledger.
    state.clear_per_turn_trigger_ledger();

    // CR 106.4 — empty each player's mana pool at the end of each
    // phase; cleanup inherits that and it never hurts to empty again.
    for p in 0..state.num_players() {
        state.player_mut(p).mana_pool.clear();
    }
}

fn next_turn(state: &mut GameState) {
    let ap = state.active_player();
    state.emit(GameEvent::TurnEnds { player: ap });

    // Extra turn queue (CR 500.7) — the active player takes another
    // turn if they're queued; otherwise the next player in turn order
    // takes a turn.
    let next_ap = match state.turn.take_extra_turn() {
        Some(p) => p,
        None => crate::priority::next_in_turn_order(ap, state.num_players()),
    };
    state.turn.start_next_turn(next_ap);
    state.emit(GameEvent::TurnBegins {
        player: next_ap, turn_number: state.turn.turn_number,
    });
}

// =============================================================================
// Stack resolution
// =============================================================================

/// CR 608 — resolve the top stack object. Called when every player
/// has passed priority in succession with the stack non-empty.
///
/// Flow:
/// 1. Pop the top entry.
/// 2. For spells, look up the card definition's
///    [`SpellAbilityDef`] to find the per-clause
///    [`TargetRequirement`]s and the effect callback.
/// 3. Recheck targets (CR 608.2b) against those requirements. If
///    every target has become illegal, counter the spell.
/// 4. Otherwise invoke the effect callback to produce a
///    [`Vec<Effect>`], execute each in order, and finalize.
///
/// Triggered and activated abilities mirror the same flow via their
/// ability definitions — Phase 1 covers activated abilities only
/// (Task #21); triggered-ability stack routing waits for when
/// triggers are collected into the stack.
fn resolve_top_of_stack(state: &mut GameState, registry: &CardRegistry) {
    let Some(entry) = state.pop_stack_entry() else { return; };

    // Collect the target requirements for this entry (empty for
    // unregistered cards and for entries we don't have a definition
    // for — treats all chosen targets as legal, which is the Phase 1
    // fallback).
    let requirements = resolution_target_requirements(&entry, registry);
    let outcome = state.recheck_and_classify_resolution(&entry, &requirements);

    match outcome {
        ResolutionOutcome::CounteredIllegalTargets => {
            if entry.is_spell() {
                state.counter_resolved_spell(entry);
            } else {
                state.counter_resolved_ability(entry);
            }
        }
        ResolutionOutcome::Resolve { target_legality: _ } => {
            let effects = resolution_effects(state, &entry, registry);
            let is_spell = entry.is_spell();
            let ward_queue = collect_ward_queue(state, &entry);
            if ward_queue.is_empty() {
                execute_effects_or_park(state, entry, effects, is_spell);
            } else {
                begin_ward_check(state, entry, effects, is_spell, ward_queue);
            }
        }
    }
}

/// Scan `entry`'s targets for Ward abilities owned by players other
/// than the stack-entry's controller. Returns `(target_id, ward_cost)`
/// pairs to be resolved sequentially as PayOrDecline prompts.
///
/// Phase 2-A stopgap: Ward is *actually* a triggered ability per
/// CR 702.21a — each instance should go on the stack independently,
/// where it can be Stifled. We approximate by prompting at resolution
/// time inside the triggering spell/ability. The Stifle-Ward gap is
/// pinned by an `#[ignore]`'d test in the framework test suite.
fn collect_ward_queue(
    state: &GameState,
    entry: &crate::stack::StackEntry,
) -> Vec<(ObjectId, crate::mana::ManaCost)> {
    use crate::effects::KeywordAbility;
    let mut queue = Vec::new();
    for choice in &entry.targets.targets {
        let Some(id) = choice.object_id() else { continue; };
        let Some(obj) = state.objects.get(id) else { continue; };
        if obj.controller == entry.controller { continue; }
        for kw in state.effective_keywords(id) {
            if let KeywordAbility::Ward(cost) = kw {
                queue.push((id, cost.clone()));
                break;
            }
        }
    }
    queue
}

fn begin_ward_check(
    state: &mut GameState,
    entry: crate::stack::StackEntry,
    effects: Vec<crate::effects::Effect>,
    is_spell: bool,
    mut ward_queue: Vec<(ObjectId, crate::mana::ManaCost)>,
) {
    let (target, cost) = ward_queue.remove(0);
    let spell_controller = entry.controller;
    let entry_id = entry.id;
    state.currently_resolving = Some(entry_id);
    state.pending_resolution = Some(crate::actions::PendingResolution {
        entry,
        remaining_effects: effects,
        is_spell,
        ward_queue,
    });
    state.push_pending_choice(
        spell_controller,
        crate::actions::ChoiceContext::ResolvingStack(entry_id),
        crate::actions::ChoiceKind::PayOrDecline {
            cost,
            on_decline: crate::actions::DeclineConsequence::CounterStackEntry(entry_id),
        },
    );
    // target-id is recoverable from the choice_context caller if
    // needed; we don't feed it back to the agent explicitly because
    // Phase 2-A treats Ward as "one prompt per target, caster pays or
    // the spell dies". A UI can look up which target caused the
    // prompt via the ward_queue metadata on pending_resolution.
    let _ = target;
}

/// Drive the effect sequence of a stack entry, pausing if any effect
/// pushes a [`crate::actions::PendingChoice`]. On pause, stash the
/// remaining effects + entry into
/// [`crate::state::GameState::pending_resolution`] so the engine can
/// resume after the agent answers.
///
/// Sets [`GameState::currently_resolving`] for the duration so
/// stack-resolution effects can reach the entry id for their
/// [`crate::actions::ChoiceContext::ResolvingStack`] token.
fn execute_effects_or_park(
    state: &mut GameState,
    entry: crate::stack::StackEntry,
    effects: Vec<crate::effects::Effect>,
    is_spell: bool,
) {
    state.currently_resolving = Some(entry.id);
    let mut iter = effects.into_iter();
    while let Some(effect) = iter.next() {
        effect.execute(state);
        if state.pending_choice.is_some() {
            state.pending_resolution = Some(crate::actions::PendingResolution {
                entry,
                remaining_effects: iter.collect(),
                is_spell,
                ward_queue: Vec::new(),
            });
            // Leave `currently_resolving` as-is — the parked state
            // still logically belongs to this entry. Cleared on resume
            // after the last effect drains (or on finalization).
            return;
        }
    }
    // All effects drained — finalize now.
    state.currently_resolving = None;
    if is_spell {
        state.finalize_resolved_spell(entry);
    } else {
        state.finalize_resolved_ability(entry);
    }
}

/// Called after [`apply_resolution_choice`] clears a pending choice.
/// If a parked resolution is waiting on this slot, continue its
/// remaining effects. If another choice gets pushed mid-way, the
/// resolution parks again.
fn resume_parked_resolution(state: &mut GameState) {
    let Some(parked) = state.pending_resolution.take() else { return; };
    debug_assert!(state.pending_choice.is_none(),
        "resume_parked_resolution: pending_choice still set");
    // More Ward prompts queued? Emit the next one.
    if !parked.ward_queue.is_empty() {
        begin_ward_check(
            state, parked.entry, parked.remaining_effects,
            parked.is_spell, parked.ward_queue);
        return;
    }
    execute_effects_or_park(
        state, parked.entry, parked.remaining_effects, parked.is_spell);
}

/// Look up the target-requirement vector for a stack entry from the
/// registry. Returns an empty vector for unregistered cards or
/// abilities that don't target.
fn resolution_target_requirements(
    entry: &crate::stack::StackEntry,
    registry: &CardRegistry,
) -> Vec<crate::targets::TargetRequirement> {
    match &entry.kind {
        crate::stack::StackEntryKind::Spell { card_id, .. } => {
            registry.get(*card_id)
                .and_then(|def| def.spell_ability.as_ref())
                .map(|sa| sa.target_requirements.clone())
                .unwrap_or_default()
        }
        crate::stack::StackEntryKind::ActivatedAbility { .. } => {
            // TODO(task-21 follow-up): route activated-ability
            // targets through the registry too. Phase 1 non-mana
            // activated abilities with targets aren't yet used.
            Vec::new()
        }
        crate::stack::StackEntryKind::TriggeredAbility { .. } => Vec::new(),
    }
}

/// Run the effect callback for a stack entry, returning the list
/// of [`Effect`]s it produced. Unregistered cards produce an empty
/// list (permanent spells rely on `finalize_resolved_spell` to put
/// the card onto the battlefield — no effect callback needed).
fn resolution_effects(
    state: &GameState,
    entry: &crate::stack::StackEntry,
    registry: &CardRegistry,
) -> Vec<crate::effects::Effect> {
    match &entry.kind {
        crate::stack::StackEntryKind::Spell { card_id, .. } => {
            let Some(def) = registry.get(*card_id) else { return Vec::new(); };
            let Some(sa) = def.spell_ability.as_ref() else { return Vec::new(); };
            (sa.effect)(state, entry, registry)
        }
        crate::stack::StackEntryKind::ActivatedAbility { ability_id, .. } => {
            // Re-dispatch through the registry using the source
            // object's card_id + ability_index packed into ability_id.
            // Phase 1: ability_id encodes `ability_index` directly.
            let source = entry.source;
            let Some(obj) = state.objects.get(source) else { return Vec::new(); };
            let Some(def) = registry.get(obj.card_id) else { return Vec::new(); };
            let idx = *ability_id as usize;
            let Some(ability) = def.activated_abilities.get(idx) else { return Vec::new(); };
            let ctx = crate::registry::ActivationContext {
                source,
                controller: entry.controller,
                ability_index: idx,
                targets: entry.targets.clone(),
                x_value: entry.x_value,
            };
            (ability.effect)(state, &ctx, registry)
        }
        crate::stack::StackEntryKind::TriggeredAbility { .. } => Vec::new(),
    }
}

// =============================================================================
// Mana payment
// =============================================================================

/// Drain the mana units referenced by a [`ManaPaymentPlan`] from the
/// player's pool, tap convoke creatures, exile delve cards, and pay
/// Phyrexian life.
///
/// Convoke (CR 702.51) — each tapped creature pays for `{1}` or a
/// pip of its color. Delve (CR 702.66) — each exiled graveyard card
/// pays for `{1}`. The plan records *which* creatures / cards were
/// chosen; this function performs the physical tap / exile / life
/// change. The engine is not checking that the chosen units actually
/// cover the stated mana cost — that validation lives in the solver
/// (Task #12) and the registry's cost metadata.
fn spend_mana_plan(
    state: &mut GameState,
    player: PlayerId,
    plan: &crate::actions::ManaPaymentPlan,
) {
    // 1. Remove pool units (highest index first so earlier indices
    //    stay valid).
    let mut indices: Vec<usize> = plan.assignments.iter()
        .map(|a| a.pool_index).collect();
    indices.sort_unstable();
    indices.dedup();
    let pool = &mut state.player_mut(player).mana_pool.pool;
    for idx in indices.into_iter().rev() {
        if idx < pool.len() {
            pool.remove(idx);
        }
    }

    // 2. Convoke — tap each designated creature (CR 702.51). The
    //    creature must be a permanent the caster controls; malformed
    //    plans panic so solver bugs surface immediately.
    for &id in &plan.convoke_creatures {
        let ok = state.objects.get(id)
            .map(|o| o.controller == player
                 && o.is_creature()
                 && o.zone.is_battlefield()
                 && !o.is_tapped())
            .unwrap_or(false);
        assert!(ok, "spend_mana_plan: convoke creature {id} not tappable by player {player}");
        if let Some(obj) = state.objects.get_mut(id) {
            obj.tap();
        }
        state.emit(GameEvent::Tapped { object_id: id });
    }

    // 3. Delve — exile each designated graveyard card (CR 702.66).
    for &id in &plan.delve_cards {
        let ok = state.objects.get(id)
            .map(|o| o.owner == player && matches!(o.zone, Zone::Graveyard(p) if p == player))
            .unwrap_or(false);
        assert!(ok, "spend_mana_plan: delve card {id} not in player {player}'s graveyard");
        state.move_object_to_zone(id, Zone::Exile, MoveCause::Cost);
    }

    // 4. Phyrexian life — 2 life per payment, one event per payment
    //    so triggers that count life-lost events fire correctly.
    for _ in &plan.phyrexian_life_payments {
        state.player_mut(player).life -= 2;
        state.emit(GameEvent::LifeLost { player, amount: 2 });
    }
}

/// Pay the non-mana additional costs on a cast / activation. Ordering
/// within the batch matches action-submission order so replays are
/// deterministic.
fn apply_additional_costs(
    state: &mut GameState,
    player: PlayerId,
    costs: &[crate::actions::AdditionalCostPayment],
) {
    use crate::actions::AdditionalCostPayment as A;
    for cost in costs {
        match cost {
            A::Sacrifice(id) => {
                state.move_object_to_zone(
                    *id, Zone::Graveyard(player), MoveCause::Cost);
            }
            A::Discard(id) => {
                state.move_object_to_zone(
                    *id, Zone::Graveyard(player), MoveCause::Cost);
                state.emit(GameEvent::Discarded { player, object_id: *id });
            }
            A::ExileFromGraveyard(ids) => {
                for id in ids {
                    state.move_object_to_zone(
                        *id, Zone::Exile, MoveCause::Cost);
                }
            }
            A::PayLife(amount) => {
                state.player_mut(player).life -= *amount as i32;
                state.emit(GameEvent::LifeLost {
                    player, amount: *amount,
                });
            }
            A::TapCreatures(ids) => {
                // Convoke's TapCreatures is redundant with the plan's
                // `convoke_creatures` field; this branch handles
                // generic "tap a creature as cost" payments.
                for id in ids {
                    let already_tapped = state.objects.get(*id)
                        .map(|o| o.is_tapped()).unwrap_or(true);
                    if already_tapped { continue; }
                    if let Some(obj) = state.objects.get_mut(*id) {
                        obj.tap();
                    }
                    state.emit(GameEvent::Tapped { object_id: *id });
                }
            }
            A::RemoveCounters { source, kind, count } => {
                if let Some(obj) = state.objects.get_mut(*source) {
                    obj.remove_counters(*kind, *count);
                }
            }
            A::RevealCard(_id) => {
                // Reveals are informational; no state change until
                // Phase 2 wires the observer/UI path. The known_cards
                // HashSet on PlayerState is where a reveal would
                // ultimately get recorded.
            }
        }
    }
}

// =============================================================================
// Mulligan machinery
// =============================================================================

/// CR 103.4 — historical default opening hand size.
/// Kept as a fallback constant; the engine now reads
/// `state.format.starting_hand_size` at runtime.
pub const OPENING_HAND_SIZE: u32 = 7;
/// CR 402.2 — historical default maximum hand size.
/// Kept as a fallback constant; the engine now reads
/// `state.format.max_hand_size` at runtime.
pub const MAX_HAND_SIZE: u32 = 7;
/// Cap on [`settle`] iterations. Guards against infinite advance
/// loops from a state-machine bug.
const MAX_SETTLE_ITERATIONS: u32 = 64;

fn next_undecided_mulligan_player(state: &GameState) -> Option<PlayerId> {
    let active = state.active_player();
    crate::priority::apnap_order(active, state.num_players())
        .find(|&p| !state.player(p).mulligan_decided)
}

fn end_mulligan_phase(state: &mut GameState) {
    state.priority.end_special_action();
    // Begin turn 1. State is already at (Beginning, Untap) — settle()
    // will advance through untap/upkeep/draw to the first priority
    // window.
    state.emit(GameEvent::TurnBegins {
        player: state.active_player(),
        turn_number: state.turn.turn_number,
    });
    emit_phase_and_step_begins(state);
}

fn shuffle_hand_into_library(state: &mut GameState, player: PlayerId) {
    let hand = state.objects.ids_in_zone_sorted(Zone::Hand(player));
    for id in hand {
        state.move_object_to_zone(
            id, Zone::Library(player), MoveCause::Cost);
    }
    state.shuffle_library(player);
}

// =============================================================================
// new_game
// =============================================================================

/// Construct a fresh two-or-more-player game, build each library
/// from the supplied [`CardId`]s, shuffle deterministically via
/// `seed`, deal opening hands, and return the initial
/// [`EngineYield`] — a mulligan decision for the first player in
/// APNAP order (the active player).
///
/// Each inner `Vec<CardId>` is one player's deck. The registry must
/// contain a definition for every id in every deck; an unknown id
/// panics.
pub fn new_game(
    decks: Vec<Vec<crate::types::CardId>>,
    registry: &CardRegistry,
    seed: u64,
) -> (GameState, EngineYield) {
    new_game_with_format(
        decks, crate::format::FormatConfig::standard_2026(), registry, seed)
}

/// Like [`new_game`] but with an explicit [`FormatConfig`]. The
/// format drives starting life, opening hand size, max hand size,
/// and mulligan rule. Deck validation is *not* run here — call
/// [`FormatConfig::validate_deck`] separately if you need it.
///
/// The stored [`GameState::format`] remains authoritative for the
/// rest of the game (cleanup-step discard, London-mulligan rule,
/// etc.). Mid-game format swaps are not supported.
pub fn new_game_with_format(
    decks: Vec<Vec<crate::types::CardId>>,
    format: crate::format::FormatConfig,
    registry: &CardRegistry,
    seed: u64,
) -> (GameState, EngineYield) {
    let num_players = decks.len() as u8;
    assert!(num_players >= 1, "new_game needs at least one deck");
    let hand_size = format.starting_hand_size;
    let mut state = GameState::with_format(num_players, seed, format);

    // Populate each library from the registry.
    for (pid, deck) in decks.into_iter().enumerate() {
        let player = pid as PlayerId;
        for card_id in deck {
            let def = registry.get(card_id).unwrap_or_else(||
                panic!("new_game: card_id {card_id} not in registry"));
            let id = state.allocate_object_id();
            let obj = GameObject::new(
                id, player, Zone::Library(player),
                card_id, def.base_characteristics.clone());
            state.objects.insert(obj);
            state.player_mut(player).library_top_to_bottom.push(id);
        }
        state.shuffle_library(player);
    }

    // Deal opening hands.
    for p in 0..num_players {
        for _ in 0..hand_size {
            state.draw_one_card(p);
        }
    }

    // Begin the mulligan decision for the active player.
    let ap = state.active_player();
    state.priority.begin_special_action(SpecialAction::MulliganDecision, ap);

    let yld = compute_next_decision(&state, registry);
    (state, yld)
}

/// Low-level alternative to [`new_game`] for tests that want to
/// skip the registry and build decks from raw [`Characteristics`].
/// Cards created this way carry `card_id = 0` (the unregistered
/// sentinel) so resolution dispatch is a no-op — fine for tests
/// that exercise the state machine without card effects.
pub fn new_game_from_characteristics(
    decks: Vec<Vec<Characteristics>>,
    seed: u64,
) -> (GameState, EngineYield) {
    let num_players = decks.len() as u8;
    assert!(num_players >= 1, "new_game needs at least one deck");
    let mut state = GameState::new(num_players, seed);

    for (pid, deck) in decks.into_iter().enumerate() {
        let player = pid as PlayerId;
        for chars in deck {
            let id = state.allocate_object_id();
            let obj = GameObject::new(
                id, player, Zone::Library(player), /*card_id=*/ 0, chars);
            state.objects.insert(obj);
            state.player_mut(player).library_top_to_bottom.push(id);
        }
        state.shuffle_library(player);
    }

    let hand_size = state.format.starting_hand_size;
    for p in 0..num_players {
        for _ in 0..hand_size {
            state.draw_one_card(p);
        }
    }

    let ap = state.active_player();
    state.priority.begin_special_action(SpecialAction::MulliganDecision, ap);

    // Use a fresh empty registry for no-registry callers. The
    // legal-action enumerator tolerates unregistered cards and
    // the engine state machine doesn't consult the registry here.
    let yld = compute_next_decision(&state, &CardRegistry::new());
    (state, yld)
}

// =============================================================================
// compute_next_decision
// =============================================================================

/// Build the [`EngineYield`] appropriate for the current state.
/// Assumes the state has been settled (no pending auto-advance).
fn compute_next_decision(state: &GameState, registry: &CardRegistry) -> EngineYield {
    if let Some(r) = state.result.clone() {
        return EngineYield::GameOver(r);
    }
    // A mid-resolution PendingChoice preempts everything else: the
    // engine is mid-spell-resolution (or mid-SBA) and cannot make
    // further progress until the agent answers. Concede remains legal.
    if let Some(pending) = state.pending_choice.as_ref() {
        return EngineYield::PendingDecision {
            player: pending.choosing_player,
            legal_actions: legal_actions(state, registry),
            context: DecisionContext::ResolutionChoice {
                stack_entry: match &pending.context {
                    crate::actions::ChoiceContext::ResolvingStack(id) => *id,
                    _ => crate::objects::NULL_OBJECT_ID,
                },
                prompt: format!("{:?}", pending.kind),
            },
        };
    }
    if let Some(ref special) = state.priority.special_action {
        let player = state.priority_player();
        let context = mulligan_like_context(special);
        return EngineYield::PendingDecision {
            player,
            legal_actions: legal_actions(state, registry),
            context,
        };
    }
    if let Some(combat) = state.combat.as_ref() {
        match combat.phase {
            CombatPhase::DeclareAttackers => {
                return EngineYield::PendingDecision {
                    player: state.active_player(),
                    legal_actions: legal_actions(state, registry),
                    context: DecisionContext::DeclareAttackers,
                };
            }
            CombatPhase::DeclareBlockers => {
                // Defender declares first. For 2-player games this is
                // the non-active player.
                let defender = (state.active_player() + 1) % state.num_players();
                return EngineYield::PendingDecision {
                    player: defender,
                    legal_actions: legal_actions(state, registry),
                    context: DecisionContext::DeclareBlockers,
                };
            }
            _ => {}
        }
    }
    // Normal priority window.
    EngineYield::PendingDecision {
        player: state.priority_player(),
        legal_actions: legal_actions(state, registry),
        context: DecisionContext::Priority,
    }
}

fn mulligan_like_context(special: &SpecialAction) -> DecisionContext {
    match special {
        SpecialAction::MulliganDecision => DecisionContext::Mulligan,
        SpecialAction::LondonMulliganBottomCards(count) => {
            DecisionContext::BottomCards { count: *count }
        }
        SpecialAction::DiscardToHandSize => {
            DecisionContext::DiscardToHandSize { count: 1 }
        }
        other => DecisionContext::SpecialAction(other.clone()),
    }
}

// =============================================================================
// Event-emission helpers
// =============================================================================

fn emit_step_begins(state: &mut GameState) {
    state.emit(GameEvent::StepBegins { step: state.turn.step });
}

fn emit_phase_and_step_begins(state: &mut GameState) {
    state.emit(GameEvent::PhaseBegins { phase: state.turn.phase });
    state.emit(GameEvent::StepBegins { step: state.turn.step });
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod resolution_choice_framework_tests {
    use super::*;
    use crate::actions::{
        CardDestination, ChoiceContext, ChoiceKind, ChoiceResponse,
    };
    use crate::types::{ColorSet, PtValue, SupertypeSet, TypeLine};

    /// Pushing a pending choice and reading it back round-trips the id.
    #[test]
    fn push_pending_choice_allocates_monotonic_id() {
        let mut s = GameState::new(2, 0);
        let id1 = s.push_pending_choice(
            0,
            ChoiceContext::Sba,
            ChoiceKind::YesNo { prompt: 0 },
        );
        assert!(s.pending_choice.is_some());
        assert_eq!(s.pending_choice.as_ref().unwrap().id, id1);
        // Clear and push again to verify monotonicity.
        s.pending_choice = None;
        let id2 = s.push_pending_choice(
            1,
            ChoiceContext::Sba,
            ChoiceKind::YesNo { prompt: 1 },
        );
        assert!(id2 > id1, "ids must be monotonic");
    }

    /// The engine yields ResolutionChoice when pending_choice is set,
    /// preempting any other decision family.
    #[test]
    fn pending_choice_preempts_priority_yield() {
        let mut s = GameState::new(2, 0);
        s.push_pending_choice(
            0,
            ChoiceContext::ResolvingStack(crate::objects::NULL_OBJECT_ID),
            ChoiceKind::YesNo { prompt: 0 },
        );
        let registry = CardRegistry::new();
        let yielded = compute_next_decision(&s, &registry);
        match yielded {
            EngineYield::PendingDecision { context, .. } => {
                assert!(matches!(context, DecisionContext::ResolutionChoice { .. }));
            }
            _ => panic!("expected PendingDecision::ResolutionChoice"),
        }
    }

    /// Submitting a response with the wrong id panics (stale reply).
    #[test]
    #[should_panic(expected = "stale id")]
    fn stale_id_panics() {
        let mut s = GameState::new(2, 0);
        let good = s.push_pending_choice(
            0,
            ChoiceContext::Sba,
            ChoiceKind::YesNo { prompt: 0 },
        );
        apply_resolution_choice(&mut s, good + 999, ChoiceResponse::YesNo { answer: true });
    }

    /// Mismatched response shape (OrderCards answer to YesNo prompt) panics.
    #[test]
    #[should_panic(expected = "does not match pending kind")]
    fn mismatched_shape_panics() {
        let mut s = GameState::new(2, 0);
        let id = s.push_pending_choice(
            0,
            ChoiceContext::ResolvingStack(crate::objects::NULL_OBJECT_ID),
            ChoiceKind::YesNo { prompt: 0 },
        );
        apply_resolution_choice(
            &mut s, id,
            ChoiceResponse::OrderCards { placements: vec![] });
    }

    /// Submitting with the right id clears the slot.
    #[test]
    fn valid_response_clears_pending_slot() {
        let mut s = GameState::new(2, 0);
        let id = s.push_pending_choice(
            0,
            ChoiceContext::Sba,
            ChoiceKind::PickCards {
                candidates: vec![1, 2, 3],
                min: 1, max: 1,
            },
        );
        apply_resolution_choice(
            &mut s, id,
            ChoiceResponse::PickCards { picked: vec![1] });
        assert!(s.pending_choice.is_none());
    }

    /// `legal_actions` for a pending YesNo lists both answers plus Concede.
    #[test]
    fn legal_actions_for_yesno_includes_both_choices_and_concede() {
        let mut s = GameState::new(2, 0);
        s.push_pending_choice(
            0,
            ChoiceContext::Sba,
            ChoiceKind::YesNo { prompt: 0 },
        );
        let registry = CardRegistry::new();
        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let yes_count = actions.iter().filter(|a| matches!(a,
            Action::SubmitResolutionChoice { response: ChoiceResponse::YesNo { answer: true }, .. }
        )).count();
        let no_count = actions.iter().filter(|a| matches!(a,
            Action::SubmitResolutionChoice { response: ChoiceResponse::YesNo { answer: false }, .. }
        )).count();
        assert_eq!(yes_count, 1);
        assert_eq!(no_count, 1);
        assert!(actions.iter().any(|a| matches!(a, Action::Concede)));
    }

    /// OrderCards applied via submitted placements lands cards in the
    /// right library slots.
    #[test]
    fn order_cards_top_preserves_submission_order() {
        use crate::zones::Zone;
        use crate::objects::{Characteristics, GameObject};
        let mut s = GameState::new(2, 0);
        // Seed three library cards.
        let a = s.allocate_object_id();
        let b = s.allocate_object_id();
        let c = s.allocate_object_id();
        for id in [a, b, c] {
            s.objects.insert(GameObject::new(
                id, 0, Zone::Library(0), 1, Characteristics::default()));
        }
        s.player_mut(0).library_top_to_bottom = vec![a, b, c];

        // Prompt: order all three, all to Top.
        let id = s.push_pending_choice(
            0,
            ChoiceContext::ResolvingStack(crate::objects::NULL_OBJECT_ID),
            ChoiceKind::OrderCards {
                cards: vec![a, b, c],
                allowed: vec![CardDestination::TopOfLibrary, CardDestination::BottomOfLibrary],
            },
        );
        // Submit reversed order on Top: c first → c becomes topmost,
        // then b, then a.
        apply_resolution_choice(
            &mut s, id,
            ChoiceResponse::OrderCards { placements: vec![
                (c, CardDestination::TopOfLibrary),
                (b, CardDestination::TopOfLibrary),
                (a, CardDestination::TopOfLibrary),
            ] });
        assert_eq!(s.player(0).library_top_to_bottom, vec![c, b, a]);
    }

    // =========================================================================
    // Scry migration — end-to-end yield/submit cycles.
    //
    // These drive `Effect::Scry` through the framework and confirm the
    // library ends up as the agent specified. Each seeds a 5-card
    // library of ids [l0, l1, l2, l3, l4] from top to bottom.
    // =========================================================================

    fn seed_library_with_objects(
        s: &mut GameState, owner: PlayerId, n: usize,
    ) -> Vec<ObjectId> {
        use crate::zones::Zone;
        use crate::objects::{Characteristics, GameObject};
        let mut ids = Vec::with_capacity(n);
        for _ in 0..n {
            let id = s.allocate_object_id();
            s.objects.insert(GameObject::new(
                id, owner, Zone::Library(owner), 1,
                Characteristics::default()));
            ids.push(id);
        }
        s.player_mut(owner).library_top_to_bottom = ids.clone();
        ids
    }

    #[test]
    fn scry_keep_all_on_top_preserves_library() {
        let mut s = GameState::new(2, 0);
        let ids = seed_library_with_objects(&mut s, 0, 5);
        s.currently_resolving = Some(999);
        crate::effects::Effect::Scry { player: 0, count: 2 }.execute(&mut s);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::OrderCards { placements: vec![
                (ids[0], CardDestination::TopOfLibrary),
                (ids[1], CardDestination::TopOfLibrary),
            ] });

        assert!(s.pending_choice.is_none());
        assert_eq!(s.player(0).library_top_to_bottom, ids,
            "keep-on-top should leave the library unchanged");
    }

    #[test]
    fn scry_bottom_both_puts_them_at_the_bottom_in_submitted_order() {
        let mut s = GameState::new(2, 0);
        let ids = seed_library_with_objects(&mut s, 0, 5);
        s.currently_resolving = Some(999);
        crate::effects::Effect::Scry { player: 0, count: 2 }.execute(&mut s);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        // Submit l0 first to Bottom, then l1 to Bottom — submitted
        // order = placement order, so l0 ends up just above l1 at the
        // very bottom (l1 is the bottom-most card).
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::OrderCards { placements: vec![
                (ids[0], CardDestination::BottomOfLibrary),
                (ids[1], CardDestination::BottomOfLibrary),
            ] });

        // New order: [l2, l3, l4, l0, l1].
        assert_eq!(s.player(0).library_top_to_bottom,
            vec![ids[2], ids[3], ids[4], ids[0], ids[1]]);
    }

    #[test]
    fn scry_reorder_top_swaps_topmost_cards() {
        let mut s = GameState::new(2, 0);
        let ids = seed_library_with_objects(&mut s, 0, 5);
        s.currently_resolving = Some(999);
        crate::effects::Effect::Scry { player: 0, count: 2 }.execute(&mut s);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        // Submit l1 first to Top, then l0 to Top. First-submitted =
        // topmost, so l1 ends on top, l0 second.
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::OrderCards { placements: vec![
                (ids[1], CardDestination::TopOfLibrary),
                (ids[0], CardDestination::TopOfLibrary),
            ] });

        assert_eq!(s.player(0).library_top_to_bottom,
            vec![ids[1], ids[0], ids[2], ids[3], ids[4]]);
    }

    #[test]
    fn scry_mixed_top_and_bottom() {
        let mut s = GameState::new(2, 0);
        let ids = seed_library_with_objects(&mut s, 0, 5);
        s.currently_resolving = Some(999);
        crate::effects::Effect::Scry { player: 0, count: 2 }.execute(&mut s);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        // Keep l0 on top, send l1 to the bottom.
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::OrderCards { placements: vec![
                (ids[0], CardDestination::TopOfLibrary),
                (ids[1], CardDestination::BottomOfLibrary),
            ] });

        assert_eq!(s.player(0).library_top_to_bottom,
            vec![ids[0], ids[2], ids[3], ids[4], ids[1]]);
    }

    /// Scry with an empty library emits Scry(0) and does not park a
    /// resolution — the next decision after "clearing" is whatever the
    /// caller's next step is (here: nothing — no stack entry was
    /// actually pushed, so we just verify no pending slot is left).
    #[test]
    fn scry_on_empty_library_does_not_yield() {
        let mut s = GameState::new(2, 0);
        s.currently_resolving = Some(999);
        crate::effects::Effect::Scry { player: 0, count: 3 }.execute(&mut s);
        assert!(s.pending_choice.is_none());
        assert!(s.pending_resolution.is_none());
        assert!(s.event_log.iter().any(|e|
            matches!(e, crate::events::GameEvent::Scry { player: 0, count: 0 })));
    }

    // =========================================================================
    // Legend rule (CR 704.5j) — SBA context, PickCards.
    //
    // The SBA loop pushes a PickCards for the first conflicting group;
    // agent picks the keeper; handler sacrifices the rest and re-enters
    // SBAs. A second conflicting group will push another choice on the
    // next settle pass.
    // =========================================================================

    fn put_legendary_creature(
        s: &mut GameState, controller: PlayerId, name_key: u32,
    ) -> ObjectId {
        use crate::zones::Zone;
        use crate::objects::{Characteristics, GameObject};
        use crate::mana::ManaCost;
        let id = s.allocate_object_id();
        let mut supertypes = SupertypeSet::default();
        supertypes = supertypes.with(SupertypeSet::LEGENDARY);
        let chars = Characteristics {
            name: name_key as crate::types::SmallString,
            mana_cost: Some(ManaCost::parse("{W}").unwrap()),
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            supertypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        };
        let mut obj = GameObject::new(id, controller, Zone::Battlefield, 1, chars);
        obj.controller = controller;
        s.objects.insert(obj);
        id
    }

    #[test]
    fn legend_rule_agent_picks_keeper_rest_sacrificed() {
        use crate::zones::Zone;
        let mut s = GameState::new(2, 0);
        let _a = put_legendary_creature(&mut s, 0, 42);
        let b = put_legendary_creature(&mut s, 0, 42);
        let _c = put_legendary_creature(&mut s, 0, 42);

        crate::sba::apply_state_based_actions(&mut s);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        // Agent picks `b` as the keeper — NOT the oldest.
        apply_resolution_choice(
            &mut s, pc_id,
            ChoiceResponse::PickCards { picked: vec![b] });

        assert!(s.pending_choice.is_none());
        // `b` didn't move, so its id is stable. `a` and `c` re-id'd
        // on the way to the graveyard.
        assert_eq!(s.objects.get(b).unwrap().zone, Zone::Battlefield,
            "chosen keeper stays in play");
        assert_eq!(s.zone_count(Zone::Battlefield), 1);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 2);
    }

    /// Two simultaneous legend conflicts (different names) produce two
    /// sequential PickCards prompts — one per pass of the SBA loop.
    #[test]
    fn legend_rule_handles_multiple_groups_sequentially() {
        use crate::zones::Zone;
        let mut s = GameState::new(2, 0);
        // Two groups of duplicates: name=1 and name=2.
        let a1 = put_legendary_creature(&mut s, 0, 1);
        let a2 = put_legendary_creature(&mut s, 0, 1);
        let b1 = put_legendary_creature(&mut s, 0, 2);
        let b2 = put_legendary_creature(&mut s, 0, 2);

        crate::sba::apply_state_based_actions(&mut s);

        // First prompt: name=1 (BTreeMap iteration is sorted by key).
        let pc = s.pending_choice.as_ref().unwrap();
        assert!(matches!(pc.context, ChoiceContext::Sba));
        let first_id = pc.id;
        // Keep a1.
        apply_resolution_choice(
            &mut s, first_id,
            ChoiceResponse::PickCards { picked: vec![a1] });

        // The handler re-ran SBAs, which should have pushed the second
        // group's prompt.
        let pc = s.pending_choice.as_ref()
            .expect("second conflicting group should push a second choice");
        let second_id = pc.id;
        assert!(second_id > first_id, "new choice gets a fresh id");
        match &pc.kind {
            ChoiceKind::PickCards { candidates, .. } => {
                assert_eq!(candidates, &vec![b1, b2]);
            }
            other => panic!("expected PickCards, got {other:?}"),
        }
        // Keep b2.
        apply_resolution_choice(
            &mut s, second_id,
            ChoiceResponse::PickCards { picked: vec![b2] });

        assert!(s.pending_choice.is_none());
        // Survivors (a1, b2) did not move → ids stable. a2 and b1
        // re-id'd on their way to their graveyards.
        assert_eq!(s.objects.get(a1).unwrap().zone, Zone::Battlefield);
        assert_eq!(s.objects.get(b2).unwrap().zone, Zone::Battlefield);
        assert!(s.objects.get(a2).is_none());
        assert!(s.objects.get(b1).is_none());
        assert_eq!(s.zone_count(Zone::Battlefield), 2);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 2);
    }

    /// No duplicates → no choice pushed, SBAs settle cleanly.
    #[test]
    fn legend_rule_no_conflict_does_not_yield() {
        let mut s = GameState::new(2, 0);
        let _a = put_legendary_creature(&mut s, 0, 1);
        let _b = put_legendary_creature(&mut s, 0, 2);
        crate::sba::apply_state_based_actions(&mut s);
        assert!(s.pending_choice.is_none());
    }

    // =========================================================================
    // Surveil migration — end-to-end yield/submit cycles.
    //
    // Destinations are Top and Graveyard (no Bottom). Common patterns:
    // "mill all" (every card to Graveyard), "keep all" (every card to
    // Top), and mixed — plus a reorder-top case.
    // =========================================================================

    #[test]
    fn surveil_mill_all_sends_cards_to_graveyard_in_order() {
        use crate::zones::Zone;
        let mut s = GameState::new(2, 0);
        let ids = seed_library_with_objects(&mut s, 0, 5);
        s.currently_resolving = Some(999);
        crate::effects::Effect::Surveil { player: 0, count: 3 }.execute(&mut s);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::OrderCards { placements: vec![
                (ids[0], CardDestination::Graveyard),
                (ids[1], CardDestination::Graveyard),
                (ids[2], CardDestination::Graveyard),
            ] });

        // The three milled cards are re-id'd into the graveyard; the
        // old ids no longer resolve in the arena. Bottom two stay in
        // library with their original ids.
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 3);
        for id in &ids[..3] {
            assert!(s.objects.get(*id).is_none(),
                "old library id should be gone after re-id to graveyard");
        }
        assert_eq!(s.player(0).library_top_to_bottom, ids[3..].to_vec());
    }

    #[test]
    fn surveil_keep_all_leaves_library_intact() {
        let mut s = GameState::new(2, 0);
        let ids = seed_library_with_objects(&mut s, 0, 5);
        s.currently_resolving = Some(999);
        crate::effects::Effect::Surveil { player: 0, count: 3 }.execute(&mut s);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::OrderCards { placements: vec![
                (ids[0], CardDestination::TopOfLibrary),
                (ids[1], CardDestination::TopOfLibrary),
                (ids[2], CardDestination::TopOfLibrary),
            ] });

        assert_eq!(s.player(0).library_top_to_bottom, ids);
    }

    #[test]
    fn surveil_mixed_mills_some_keeps_rest_in_chosen_order() {
        use crate::zones::Zone;
        let mut s = GameState::new(2, 0);
        let ids = seed_library_with_objects(&mut s, 0, 5);
        s.currently_resolving = Some(999);
        crate::effects::Effect::Surveil { player: 0, count: 3 }.execute(&mut s);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        // Mill ids[1]; keep ids[2] on top (topmost), ids[0] second.
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::OrderCards { placements: vec![
                (ids[2], CardDestination::TopOfLibrary),
                (ids[0], CardDestination::TopOfLibrary),
                (ids[1], CardDestination::Graveyard),
            ] });

        // Milled card is re-id'd; kept-in-library cards preserve their
        // ids (never went through a zone transition).
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert!(s.objects.get(ids[1]).is_none());
        assert_eq!(s.player(0).library_top_to_bottom,
            vec![ids[2], ids[0], ids[3], ids[4]]);
    }

    // =========================================================================
    // Tutor / Search migration — end-to-end yield/submit cycles.
    // =========================================================================

    fn put_creature_in_zone(
        s: &mut GameState, owner: PlayerId, zone: crate::zones::Zone,
        p: i32, t: i32,
    ) -> ObjectId {
        use crate::mana::ManaCost;
        use crate::objects::{Characteristics, GameObject};
        let id = s.allocate_object_id();
        let chars = Characteristics {
            mana_cost: Some(ManaCost::parse("{G}").unwrap()),
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(p)),
            toughness: Some(PtValue::Fixed(t)),
            ..Default::default()
        };
        let mut obj = GameObject::new(id, owner, zone, 1, chars);
        obj.controller = owner;
        s.objects.insert(obj);
        id
    }

    #[test]
    fn tutor_to_hand_moves_picked_card_and_shuffles() {
        use crate::zones::Zone;
        let mut s = GameState::new(2, 0);
        let _filler = put_creature_in_zone(&mut s, 0, Zone::Library(0), 1, 1);
        let beast = put_creature_in_zone(&mut s, 0, Zone::Library(0), 4, 4);
        s.player_mut(0).library_top_to_bottom = vec![_filler, beast];
        s.currently_resolving = Some(999);

        crate::effects::Effect::TutorToHand {
            player: 0,
            filter: crate::targets::ObjectFilter::creature(),
            reveal: false,
        }.execute(&mut s);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::PickCards { picked: vec![beast] });

        assert!(s.pending_choice.is_none());
        // Beast re-id'd when it moved to hand.
        assert!(s.objects.get(beast).is_none());
        assert_eq!(s.zone_count(Zone::Hand(0)), 1);
        assert!(s.event_log.iter().any(|e|
            matches!(e, crate::events::GameEvent::LibraryShuffled { player: 0 })));
    }

    #[test]
    fn tutor_to_hand_declining_with_empty_indices_still_shuffles() {
        use crate::zones::Zone;
        let mut s = GameState::new(2, 0);
        let beast = put_creature_in_zone(&mut s, 0, Zone::Library(0), 4, 4);
        s.player_mut(0).library_top_to_bottom = vec![beast];
        s.currently_resolving = Some(999);

        crate::effects::Effect::TutorToHand {
            player: 0,
            filter: crate::targets::ObjectFilter::creature(),
            reveal: false,
        }.execute(&mut s);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        // min=0, so picking nothing is valid.
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::PickCards { picked: vec![] });

        // Card stays in library; library got shuffled.
        assert_eq!(s.objects.get(beast).unwrap().zone, Zone::Library(0));
        assert!(s.event_log.iter().any(|e|
            matches!(e, crate::events::GameEvent::LibraryShuffled { player: 0 })));
    }

    #[test]
    fn tutor_to_hand_with_reveal_marks_chosen_card_known_to_all() {
        use crate::zones::Zone;
        let mut s = GameState::new(2, 0);
        let beast = put_creature_in_zone(&mut s, 0, Zone::Library(0), 4, 4);
        s.player_mut(0).library_top_to_bottom = vec![beast];
        s.currently_resolving = Some(999);

        crate::effects::Effect::TutorToHand {
            player: 0,
            filter: crate::targets::ObjectFilter::creature(),
            reveal: true,
        }.execute(&mut s);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::PickCards { picked: vec![beast] });

        // The revealed card re-id'd on move; its NEW id (the card now
        // in hand) should be in every player's known_cards.
        let hand_id = s.objects.objects_in_zone(Zone::Hand(0))
            .next().unwrap().id;
        assert!(s.player(0).known_cards.contains(&hand_id));
        assert!(s.player(1).known_cards.contains(&hand_id));
    }

    #[test]
    fn tutor_to_battlefield_picks_one_puts_onto_battlefield_tapped() {
        use crate::zones::Zone;
        let mut s = GameState::new(2, 0);
        let beast = put_creature_in_zone(&mut s, 0, Zone::Library(0), 4, 4);
        s.player_mut(0).library_top_to_bottom = vec![beast];
        s.currently_resolving = Some(999);

        crate::effects::Effect::TutorToBattlefield {
            player: 0,
            filter: crate::targets::ObjectFilter::creature(),
            tapped: true,
        }.execute(&mut s);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::PickCards { picked: vec![beast] });

        assert!(s.objects.get(beast).is_none());
        let bf_obj = s.objects.objects_in_zone(Zone::Battlefield)
            .next().unwrap();
        assert!(bf_obj.is_tapped());
        assert!(s.event_log.iter().any(|e|
            matches!(e, crate::events::GameEvent::LibraryShuffled { player: 0 })));
    }

    #[test]
    fn reanimate_puts_chosen_graveyard_card_onto_battlefield_under_controller() {
        use crate::zones::Zone;
        let mut s = GameState::new(2, 0);
        // Opponent's graveyard creature — reanimator (0) can pull it.
        let beast = put_creature_in_zone(&mut s, 1, Zone::Graveyard(1), 3, 3);
        s.currently_resolving = Some(999);

        crate::effects::Effect::Reanimate {
            player: 0,
            filter: crate::targets::ObjectFilter::creature(),
            from_zone: Zone::Graveyard(1),
        }.execute(&mut s);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::PickCards { picked: vec![beast] });

        assert!(s.objects.get(beast).is_none(), "graveyard id consumed by re-id");
        let bf_obj = s.objects.objects_in_zone(Zone::Battlefield)
            .next().unwrap();
        assert_eq!(bf_obj.controller, 0,
            "reanimated permanent comes under the reanimator's control");
        assert_eq!(bf_obj.owner, 1, "owner stays the opponent");
    }

    #[test]
    fn discard_controller_end_to_end_cycle() {
        use crate::zones::Zone;
        use crate::objects::{Characteristics, GameObject};
        let mut s = GameState::new(2, 0);
        let h1 = s.allocate_object_id();
        let h2 = s.allocate_object_id();
        for id in [h1, h2] {
            s.objects.insert(GameObject::new(
                id, 0, Zone::Hand(0), 1, Characteristics::default()));
        }
        s.currently_resolving = Some(999);

        crate::effects::Effect::Discard {
            player: 0, count: 1,
            choice: crate::effects::DiscardChoice::ControllerChooses,
        }.execute(&mut s);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        // Discard h2 specifically.
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::PickCards { picked: vec![h2] });

        // h1 stayed put; h2 re-id'd into graveyard.
        assert_eq!(s.objects.get(h1).unwrap().zone, Zone::Hand(0));
        assert!(s.objects.get(h2).is_none());
        assert_eq!(s.zone_count(Zone::Hand(0)), 1);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        // Discarded event carries the pre-move id.
        assert!(s.event_log.iter().any(|e|
            matches!(e, crate::events::GameEvent::Discarded { object_id, .. }
                if *object_id == h2)));
    }

    #[test]
    fn discard_opponent_end_to_end_cycle() {
        use crate::zones::Zone;
        use crate::objects::{Characteristics, GameObject};
        let mut s = GameState::new(2, 0);
        let h1 = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            h1, 0, Zone::Hand(0), 1, Characteristics::default()));
        s.currently_resolving = Some(999);

        crate::effects::Effect::Discard {
            player: 0, count: 1,
            choice: crate::effects::DiscardChoice::OpponentChooses,
        }.execute(&mut s);

        let pc = s.pending_choice.as_ref().unwrap();
        assert_eq!(pc.choosing_player, 1);
        let pc_id = pc.id;
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::PickCards { picked: vec![h1] });

        assert!(s.objects.get(h1).is_none());
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
    }

    #[test]
    fn sacrifice_player_picks_which_permanent_to_sacrifice() {
        use crate::zones::Zone;
        let mut s = GameState::new(2, 0);
        let a = put_creature_in_zone(&mut s, 0, Zone::Battlefield, 1, 1);
        let b = put_creature_in_zone(&mut s, 0, Zone::Battlefield, 5, 5);
        s.currently_resolving = Some(999);

        crate::effects::Effect::Sacrifice {
            player: 0,
            filter: crate::targets::ObjectFilter::creature(),
            count: 1,
        }.execute(&mut s);

        // Agent sacrifices the 5/5, not the 1/1 — exercising the
        // agent-choice (not first-match) semantic.
        let pc_id = s.pending_choice.as_ref().unwrap().id;
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::PickCards { picked: vec![b] });

        // a stayed put, b re-id'd into the graveyard.
        assert_eq!(s.objects.get(a).unwrap().zone, Zone::Battlefield);
        assert!(s.objects.get(b).is_none());
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        // Sacrifice event carries the pre-move id.
        assert!(s.event_log.iter().any(|e|
            matches!(e, crate::events::GameEvent::Sacrifice { object_id, .. }
                if *object_id == b)));
    }

    // =========================================================================
    // Ward migration — end-to-end pay/decline cycles.
    //
    // Phase 2-A stopgap: the Ward prompt is resolved at spell resolution
    // (just before effects run) rather than as an independently-stacked
    // trigger. A properly-stacked Ward (CR 702.21a) could be Stifled
    // before it resolves; see `stifle_ward_interaction_is_phase_2b` for
    // the pinned gap.
    // =========================================================================

    fn noop_target_effect(
        _state: &GameState,
        _entry: &crate::stack::StackEntry,
        _reg: &CardRegistry,
    ) -> Vec<crate::effects::Effect> {
        Vec::new()
    }

    /// Register a single-target instant in `registry` that does
    /// nothing on resolution. Returns the allocated card id so tests
    /// can cast spells of this card for Ward / target-recheck paths.
    fn register_noop_target_instant(registry: &mut CardRegistry) -> crate::types::CardId {
        use crate::mana::ManaCost;
        use crate::registry::{CardDefinition, SpellAbilityDef};
        use crate::targets::TargetRequirement;
        let name = registry.interner_mut().intern("Noop-Target");
        let def = CardDefinition::new(name, Characteristics {
            mana_cost: Some(ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        }).with_spell_ability(SpellAbilityDef {
            text: "Does nothing to target creature.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            effect: noop_target_effect,
        });
        registry.register(def)
    }

    fn put_target_spell_on_stack(
        s: &mut GameState,
        controller: PlayerId,
        card_id: crate::types::CardId,
        target: ObjectId,
    ) -> ObjectId {
        use crate::stack::StackEntry;
        use crate::targets::{TargetChoice, TargetSelection};
        use crate::mana::ManaCost;
        use crate::objects::{Characteristics, GameObject};
        use crate::zones::Zone;
        let spell_id = s.allocate_object_id();
        let chars = Characteristics {
            mana_cost: Some(ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        };
        s.objects.insert(GameObject::new(
            spell_id, controller, Zone::Stack, card_id, chars.clone()));
        let mut targets = TargetSelection::new();
        targets.targets.push(TargetChoice::Object(target));
        let entry = StackEntry::new_spell(
            spell_id, controller, card_id, chars,
            targets, vec![], None);
        s.push_stack_entry(entry);
        spell_id
    }

    fn pay_cost_creature(
        s: &mut GameState, owner: PlayerId, ward: crate::mana::ManaCost,
    ) -> ObjectId {
        use crate::effects::KeywordAbility;
        let id = put_creature_in_zone(
            s, owner, crate::zones::Zone::Battlefield, 2, 2);
        s.objects.get_mut(id).unwrap().characteristics.keywords
            .push(KeywordAbility::Ward(ward));
        id
    }

    #[test]
    fn ward_paid_lets_spell_resolve() {
        use crate::mana::ManaCost;
        use crate::types::ManaColor;
        let mut s = GameState::new(2, 0);
        let mut registry = CardRegistry::new();
        let card = register_noop_target_instant(&mut registry);
        // Caster = player 0, target's controller = player 1.
        let victim = pay_cost_creature(&mut s, 1, ManaCost::parse("{2}").unwrap());
        // Give player 0 two colorless mana in pool to pay ward.
        s.player_mut(0).mana_pool.add(crate::mana::ManaUnit::plain(ManaColor::Red, 0));
        s.player_mut(0).mana_pool.add(crate::mana::ManaUnit::plain(ManaColor::Red, 0));
        let _spell = put_target_spell_on_stack(&mut s, 0, card, victim);

        resolve_top_of_stack(&mut s, &registry);

        // A Ward prompt is pending for the caster (player 0).
        let pc = s.pending_choice.as_ref().unwrap();
        assert_eq!(pc.choosing_player, 0);
        assert!(matches!(pc.kind, ChoiceKind::PayOrDecline { .. }));
        let pc_id = pc.id;
        let before_pool = s.player(0).mana_pool.total();
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::PayOrDecline { pay: true });

        // Pool debited by 2 generic.
        assert_eq!(s.player(0).mana_pool.total(), before_pool - 2);
        // Spell finalized (spell object moves off stack to graveyard —
        // instant goes to GY after resolving).
        assert!(s.pending_choice.is_none());
        assert!(s.pending_resolution.is_none());
        // Stack is empty.
        assert!(s.stack_is_empty());
    }

    #[test]
    fn ward_declined_counters_spell() {
        use crate::mana::ManaCost;
        let mut s = GameState::new(2, 0);
        let mut registry = CardRegistry::new();
        let card = register_noop_target_instant(&mut registry);
        let victim = pay_cost_creature(&mut s, 1, ManaCost::parse("{2}").unwrap());
        let spell = put_target_spell_on_stack(&mut s, 0, card, victim);

        resolve_top_of_stack(&mut s, &registry);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::PayOrDecline { pay: false });

        // Spell countered: ends up in caster's graveyard with a
        // SpellCountered event that still references the stack id.
        assert!(s.objects.get(spell).is_none(),
            "stack id is consumed on re-id into graveyard");
        assert_eq!(s.zone_count(crate::zones::Zone::Graveyard(0)), 1);
        assert!(s.event_log.iter().any(|e|
            matches!(e, crate::events::GameEvent::SpellCountered { object_id }
                if *object_id == spell)));
        assert!(s.pending_choice.is_none());
        assert!(s.pending_resolution.is_none());
    }

    #[test]
    fn ward_does_not_trigger_for_controller_own_spell() {
        use crate::mana::ManaCost;
        let mut s = GameState::new(2, 0);
        let mut registry = CardRegistry::new();
        let card = register_noop_target_instant(&mut registry);
        // Caster 0 targets their own Ward creature — no trigger.
        let own = pay_cost_creature(&mut s, 0, ManaCost::parse("{2}").unwrap());
        let _spell = put_target_spell_on_stack(&mut s, 0, card, own);

        resolve_top_of_stack(&mut s, &registry);

        assert!(s.pending_choice.is_none(),
            "Ward never triggers for the controller's own spells");
    }

    fn register_noop_two_target_instant(registry: &mut CardRegistry)
        -> crate::types::CardId
    {
        use crate::mana::ManaCost;
        use crate::registry::{CardDefinition, SpellAbilityDef};
        use crate::targets::TargetRequirement;
        let name = registry.interner_mut().intern("Noop-TwoTargets");
        let def = CardDefinition::new(name, Characteristics {
            mana_cost: Some(ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        }).with_spell_ability(SpellAbilityDef {
            text: "Does nothing to two target creatures.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement::target_creature(),
            ],
            effect: noop_target_effect,
        });
        registry.register(def)
    }

    /// A spell targeting two different Ward creatures produces two
    /// sequential PayOrDecline prompts. Paying both lets the spell
    /// resolve; declining the second still counters it.
    #[test]
    fn ward_multiple_targets_emit_sequential_prompts() {
        use crate::mana::ManaCost;
        use crate::types::ManaColor;
        use crate::stack::StackEntry;
        use crate::targets::{TargetChoice, TargetSelection};
        use crate::objects::{Characteristics, GameObject};
        use crate::zones::Zone;
        let mut s = GameState::new(2, 0);
        let mut registry = CardRegistry::new();
        let card = register_noop_two_target_instant(&mut registry);
        let a = pay_cost_creature(&mut s, 1, ManaCost::parse("{1}").unwrap());
        let b = pay_cost_creature(&mut s, 1, ManaCost::parse("{1}").unwrap());
        // Caster needs 2 mana to pay both Wards.
        for _ in 0..2 {
            s.player_mut(0).mana_pool.add(crate::mana::ManaUnit::plain(ManaColor::Red, 0));
        }
        // Two-target spell.
        let spell_id = s.allocate_object_id();
        let chars = Characteristics {
            mana_cost: Some(ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        };
        s.objects.insert(GameObject::new(
            spell_id, 0, Zone::Stack, card, chars.clone()));
        let mut targets = TargetSelection::new();
        targets.targets.push(TargetChoice::Object(a));
        targets.targets.push(TargetChoice::Object(b));
        s.push_stack_entry(StackEntry::new_spell(
            spell_id, 0, card, chars, targets, vec![], None));

        resolve_top_of_stack(&mut s, &registry);

        // First prompt pushed.
        let pc_id = s.pending_choice.as_ref().unwrap().id;
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::PayOrDecline { pay: true });

        // Second prompt should now be live.
        let second = s.pending_choice.as_ref()
            .expect("second Ward prompt");
        assert!(second.id > pc_id);
        let second_id = second.id;
        apply_resolution_choice(&mut s, second_id,
            ChoiceResponse::PayOrDecline { pay: true });

        assert!(s.pending_choice.is_none());
        assert!(s.pending_resolution.is_none());
        assert!(s.stack_is_empty());
    }

    /// Gap pin: in real MTG, a Ward trigger sits on the stack as its
    /// own object and can be Stifled (countered) before resolving,
    /// letting the original spell through without payment. Our Phase
    /// 2-A stopgap resolves Ward inline inside the spell's resolution,
    /// so there's no intermediate stack object for Stifle to hit.
    /// Promote this to Phase 2-B when triggered-ability stack routing
    /// lands.
    #[test]
    #[ignore = "Phase 2-B: Ward is a real trigger; see TODO(phase-2b) in engine::collect_ward_queue"]
    fn stifle_ward_interaction_is_phase_2b() {
        // Setup: spell targets Ward creature; Stifle responds to the
        // Ward trigger. Expected: Ward trigger countered, original
        // spell resolves without payment. Current behavior: we never
        // push a separate Ward trigger to the stack, so there's
        // nothing for Stifle to target.
        panic!("deliberately unimplemented — see #[ignore] reason");
    }

    #[test]
    fn surveil_rejects_bottom_destination() {
        let mut s = GameState::new(2, 0);
        let ids = seed_library_with_objects(&mut s, 0, 5);
        s.currently_resolving = Some(999);
        crate::effects::Effect::Surveil { player: 0, count: 2 }.execute(&mut s);
        let pc_id = s.pending_choice.as_ref().unwrap().id;

        // Submitting Bottom should panic — it's not in the allowed set.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            apply_resolution_choice(&mut s, pc_id,
                ChoiceResponse::OrderCards { placements: vec![
                    (ids[0], CardDestination::BottomOfLibrary),
                    (ids[1], CardDestination::TopOfLibrary),
                ] });
        }));
        assert!(result.is_err(),
            "Bottom must be rejected by apply_order_cards");
    }
}

#[cfg(test)]
mod prowess_tests {
    use super::*;
    use crate::effects::KeywordAbility;
    use crate::events::GameEvent;
    use crate::mana::ManaCost;
    use crate::objects::{Characteristics, GameObject};
    use crate::zones::Zone;
    use crate::types::*;

    fn put_creature(s: &mut GameState, owner: PlayerId, p: i32, t: i32) -> ObjectId {
        let id = s.allocate_object_id();
        let chars = Characteristics {
            mana_cost: Some(ManaCost::parse("{G}").unwrap()),
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(p)),
            toughness: Some(PtValue::Fixed(t)),
            ..Default::default()
        };
        let mut obj = GameObject::new(id, owner, Zone::Battlefield, 1, chars);
        obj.controller = owner;
        s.objects.insert(obj);
        id
    }

    #[test]
    fn prowess_pumps_when_controller_casts_noncreature_spell() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.objects.get_mut(c).unwrap().characteristics.keywords
            .push(KeywordAbility::Prowess);

        // Fire a SpellCast event for a noncreature spell controlled by 0.
        let spell_id = s.allocate_object_id();
        let instant_chars = Characteristics {
            mana_cost: Some(ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        };
        s.objects.insert(GameObject::new(
            spell_id, 0, Zone::Stack, 1, instant_chars));

        let event = GameEvent::SpellCast {
            object_id: spell_id,
            card_id: 1,
            controller: 0,
            targets: crate::targets::TargetSelection::new(),
        };
        apply_prowess_on_cast(&mut s, &event);

        assert_eq!(s.computed_power(c), Some(3));
        assert_eq!(s.computed_toughness(c), Some(3));
    }

    #[test]
    fn prowess_does_not_fire_on_creature_spell() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.objects.get_mut(c).unwrap().characteristics.keywords
            .push(KeywordAbility::Prowess);

        let spell_id = s.allocate_object_id();
        let creature_chars = Characteristics {
            mana_cost: Some(ManaCost::parse("{G}").unwrap()),
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            ..Default::default()
        };
        s.objects.insert(GameObject::new(
            spell_id, 0, Zone::Stack, 1, creature_chars));

        let event = GameEvent::SpellCast {
            object_id: spell_id,
            card_id: 1,
            controller: 0,
            targets: crate::targets::TargetSelection::new(),
        };
        apply_prowess_on_cast(&mut s, &event);

        assert_eq!(s.computed_power(c), Some(2));
    }

    #[test]
    fn prowess_does_not_fire_for_opponents_cast() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.objects.get_mut(c).unwrap().characteristics.keywords
            .push(KeywordAbility::Prowess);

        let spell_id = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            spell_id, 1, Zone::Stack, 1, Characteristics {
                types: TypeLine::INSTANT.into(), ..Default::default()
            }));

        let event = GameEvent::SpellCast {
            object_id: spell_id, card_id: 1,
            controller: 1,  // opponent
            targets: crate::targets::TargetSelection::new(),
        };
        apply_prowess_on_cast(&mut s, &event);

        // Untouched.
        assert_eq!(s.computed_power(c), Some(2));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mana::ManaCost;
    use crate::objects::Characteristics;
    use crate::types::*;

    // --- helpers ------------------------------------------------------------

    fn reg() -> CardRegistry { CardRegistry::new() }

    fn land_chars(name: &str, _color: ManaColor) -> Characteristics {
        // Lands have no mana cost; the untap step makes them available
        // for mana abilities. We don't yet model mana-tap abilities on
        // registered lands (Task #21) — tests that need mana add it
        // to the pool directly.
        let _ = name;
        Characteristics {
            mana_cost: None,
            types: TypeLine::LAND.into(),
            ..Default::default()
        }
    }

    fn creature_chars(power: i32, toughness: i32) -> Characteristics {
        Characteristics {
            mana_cost: Some(ManaCost::parse("{1}{G}").unwrap()),
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(power)),
            toughness: Some(PtValue::Fixed(toughness)),
            ..Default::default()
        }
    }

    fn instant_chars() -> Characteristics {
        Characteristics {
            mana_cost: Some(ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        }
    }

    fn simple_deck() -> Vec<Characteristics> {
        let mut d: Vec<Characteristics> = Vec::new();
        for _ in 0..30 {
            d.push(land_chars("Mountain", ManaColor::Red));
        }
        for _ in 0..20 {
            d.push(creature_chars(2, 2));
        }
        d
    }

    fn start(seed: u64) -> (GameState, EngineYield) {
        new_game_from_characteristics(vec![simple_deck(), simple_deck()], seed)
    }

    // --- EngineYield --------------------------------------------------------

    #[test]
    fn engine_yield_predicates() {
        let y = EngineYield::GameOver(GameResult::Draw);
        assert!(y.is_game_over());
        assert!(!y.is_pending());
    }

    // --- new_game -----------------------------------------------------------

    #[test]
    fn new_game_creates_two_players_with_seven_cards_each() {
        let (state, _yld) = start(42);
        assert_eq!(state.num_players(), 2);
        for p in 0..2 {
            let hand = state.objects.count_in_zone(Zone::Hand(p));
            assert_eq!(hand, 7, "player {p} should have 7 cards in hand");
            // 50 cards total, 7 in hand → 43 in library.
            let lib = state.player(p).library_top_to_bottom.len();
            assert_eq!(lib, 43);
        }
    }

    #[test]
    fn new_game_yields_mulligan_decision_for_active_player() {
        let (state, yld) = start(42);
        match yld {
            EngineYield::PendingDecision { player, context, legal_actions: _ } => {
                assert_eq!(player, state.active_player());
                assert!(matches!(context, DecisionContext::Mulligan));
            }
            _ => panic!("expected PendingDecision/Mulligan"),
        }
    }

    #[test]
    fn new_game_is_deterministic_given_seed() {
        let (a, _) = start(42);
        let (b, _) = start(42);
        // Library orders should be identical for identical seeds.
        assert_eq!(
            a.player(0).library_top_to_bottom.len(),
            b.player(0).library_top_to_bottom.len(),
        );
        let a_top = a.player(0).library_top_to_bottom[0];
        let b_top = b.player(0).library_top_to_bottom[0];
        // The arena id is the same because allocation is deterministic;
        // more interesting: the same card ends up on top.
        let a_name = &a.objects.get(a_top).unwrap().characteristics.types;
        let b_name = &b.objects.get(b_top).unwrap().characteristics.types;
        assert_eq!(a_name.0, b_name.0);
    }

    // --- mulligan flow ------------------------------------------------------

    #[test]
    fn mulligan_keep_by_both_players_begins_turn_1_priority_window() {
        let (mut state, _y) = start(7);
        // Each player keeps in APNAP order.
        let (s, _y) = step(state.clone(), Action::MulliganKeep, &reg());
        state = s;
        let (s, yld) = step(state.clone(), Action::MulliganKeep, &reg());
        state = s;

        match yld {
            EngineYield::PendingDecision { context, player, .. } => {
                assert!(matches!(context, DecisionContext::Priority));
                assert_eq!(player, state.active_player());
            }
            _ => panic!("expected Priority after both players keep"),
        }
        // State machine has advanced to a priority-granting step of
        // turn 1 (Upkeep is the first — Untap grants no priority so
        // settle() ran through it).
        assert_eq!(state.turn.turn_number, 1);
        assert!(!state.priority.in_special_action());
        assert!(matches!(state.turn.step,
            Step::Upkeep | Step::Draw | Step::Main));
    }

    // --- trigger collection loop ------------------------------------------

    /// Register a simple card whose ETB trigger deals 1 damage to
    /// player 0. Used to verify the engine's trigger scanner fires
    /// registered triggered abilities through the normal step loop.
    fn register_etb_pinger(
        registry: &mut crate::registry::CardRegistry,
    ) -> crate::types::CardId {
        use crate::triggers::{TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
        use crate::effects::Effect;
        use crate::events::DamageTarget;
        let name = registry.interner_mut().intern("ETB Pinger");
        let chars = Characteristics {
            name,
            mana_cost: Some(crate::mana::ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            ..Default::default()
        };
        fn pinger_effect(
            _s: &GameState,
            pt: &crate::triggers::PendingTrigger,
            _: &(),
        ) -> Vec<Effect> {
            vec![Effect::DealDamage {
                source: pt.source,
                target: DamageTarget::Player(0),
                amount: 1,
            }]
        }
        let mut def = crate::registry::CardDefinition::new(name, chars);
        def.triggered_abilities.push(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: pinger_effect,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
        });
        registry.register(def)
    }

    #[test]
    fn etb_trigger_fires_via_engine_scanner() {
        // Put an ETB-pinger on the battlefield via direct ETB
        // simulation — the goal is to exercise the engine's
        // triggered-ability collector, not the cast pipeline.
        let mut registry = crate::registry::CardRegistry::new();
        let card = register_etb_pinger(&mut registry);
        let mut state = GameState::new(2, 0);

        // Drop the card directly onto the battlefield and emit the
        // ETB event. `move_object_to_zone` handles that.
        let id = state.allocate_object_id();
        let def = registry.get(card).unwrap();
        state.objects.insert(GameObject::new(
            id, 0, Zone::Hand(0), card,
            def.base_characteristics.clone()));
        // Re-id on the hand→battlefield move; capture the new id so
        // the trigger-source assertion can compare against the id
        // currently on the battlefield.
        let battlefield_id = state.move_object_to_zone(
            id, Zone::Battlefield, MoveCause::SpellResolution).unwrap();

        // Run the scanner. Should push the ETB trigger onto the stack.
        run_sba_and_triggers(&mut state, &registry);

        assert_eq!(state.stack_size(), 1,
            "scanner should have pushed the ETB trigger");
        let entry = state.top_of_stack().unwrap();
        assert!(entry.is_triggered());
        assert_eq!(entry.source, battlefield_id);
    }

    #[test]
    fn trigger_cursor_prevents_refiring_old_events() {
        // Register the pinger, push an ETB-on-battlefield event into
        // the log by hand, scan once → one trigger on stack. Scan
        // again → no new trigger (cursor advanced past the event).
        let mut registry = crate::registry::CardRegistry::new();
        let card = register_etb_pinger(&mut registry);
        let mut state = GameState::new(2, 0);
        let id = state.allocate_object_id();
        let def = registry.get(card).unwrap();
        state.objects.insert(GameObject::new(
            id, 0, Zone::Battlefield, card,
            def.base_characteristics.clone()));
        state.emit(GameEvent::EntersBattlefield {
            object_id: id,
            from_zone: Zone::Hand(0),
            was_cast: false,
        });

        run_sba_and_triggers(&mut state, &registry);
        assert_eq!(state.stack_size(), 1);

        // Second scan: no new events since last cursor → stack
        // unchanged.
        run_sba_and_triggers(&mut state, &registry);
        assert_eq!(state.stack_size(), 1,
            "re-scan should not double-fire the same event");
    }

    #[test]
    fn delayed_trigger_fires_on_matching_event() {
        // Register a one-shot delayed trigger. Emit a matching event.
        // Scanner should fire it and remove it from delayed_triggers.
        use crate::triggers::{DelayedTrigger, TriggerCondition};
        use crate::effects::Effect;
        use crate::events::DamageTarget;

        let registry = crate::registry::CardRegistry::new();
        let mut state = GameState::new(2, 0);

        fn delayed_effect(
            _: &GameState,
            pt: &crate::triggers::PendingTrigger,
            _: &(),
        ) -> Vec<Effect> {
            vec![Effect::DealDamage {
                source: pt.source,
                target: DamageTarget::Player(1),
                amount: 2,
            }]
        }
        state.register_delayed_trigger(DelayedTrigger::one_shot(
            /*source=*/ 1,
            /*controller=*/ 0,
            TriggerCondition::StepBegins {
                step: crate::turn::Step::End,
                whose: crate::targets::ControllerConstraint::Any,
            },
            delayed_effect,
        ));
        assert_eq!(state.delayed_triggers.len(), 1);

        state.emit(GameEvent::StepBegins { step: crate::turn::Step::End });

        run_sba_and_triggers(&mut state, &registry);
        assert_eq!(state.delayed_triggers.len(), 0,
            "one-shot delayed trigger should be consumed");
        assert_eq!(state.stack_size(), 1,
            "delayed trigger should be on the stack");
    }

    // --- advanced mana / additional costs ---------------------------------

    #[test]
    fn spend_mana_plan_taps_convoke_creatures() {
        let mut s = GameState::new(2, 0);
        // Put a creature the caster controls onto the battlefield.
        let creature = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            creature, 0, Zone::Battlefield, 0, creature_chars(2, 2)));
        s.objects.get_mut(creature).unwrap().status.summoning_sick = false;

        let plan = crate::actions::ManaPaymentPlan {
            assignments: Vec::new(),
            convoke_creatures: vec![creature],
            delve_cards: Vec::new(),
            phyrexian_life_payments: Vec::new(),
        };
        spend_mana_plan(&mut s, 0, &plan);

        assert!(s.objects.get(creature).unwrap().is_tapped());
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::Tapped { object_id } if *object_id == creature)));
    }

    #[test]
    fn spend_mana_plan_exiles_delve_cards() {
        let mut s = GameState::new(2, 0);
        let gy_card = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            gy_card, 0, Zone::Graveyard(0), 0,
            Characteristics::default()));

        let plan = crate::actions::ManaPaymentPlan {
            assignments: Vec::new(),
            convoke_creatures: Vec::new(),
            delve_cards: vec![gy_card],
            phyrexian_life_payments: Vec::new(),
        };
        spend_mana_plan(&mut s, 0, &plan);

        assert_eq!(s.zone_count(Zone::Exile), 1);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 0);
    }

    #[test]
    fn spend_mana_plan_phyrexian_emits_life_lost() {
        let mut s = GameState::new(2, 0);
        let plan = crate::actions::ManaPaymentPlan {
            assignments: Vec::new(),
            convoke_creatures: Vec::new(),
            delve_cards: Vec::new(),
            phyrexian_life_payments: vec![0, 1], // two Phyrexian pips paid
        };
        let life_before = s.player(0).life;
        spend_mana_plan(&mut s, 0, &plan);
        assert_eq!(s.player(0).life, life_before - 4);
        let events = s.event_log.iter()
            .filter(|e| matches!(e, GameEvent::LifeLost { player: 0, amount: 2 }))
            .count();
        assert_eq!(events, 2);
    }

    #[test]
    #[should_panic(expected = "convoke creature")]
    fn convoke_with_tapped_creature_panics() {
        let mut s = GameState::new(2, 0);
        let creature = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            creature, 0, Zone::Battlefield, 0, creature_chars(2, 2)));
        s.objects.get_mut(creature).unwrap().status.summoning_sick = false;
        s.objects.get_mut(creature).unwrap().tap();

        let plan = crate::actions::ManaPaymentPlan {
            assignments: Vec::new(),
            convoke_creatures: vec![creature],
            delve_cards: Vec::new(),
            phyrexian_life_payments: Vec::new(),
        };
        spend_mana_plan(&mut s, 0, &plan);
    }

    #[test]
    fn additional_costs_sacrifice_moves_to_graveyard() {
        let mut s = GameState::new(2, 0);
        let perm = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            perm, 0, Zone::Battlefield, 0, creature_chars(1, 1)));
        let costs = vec![crate::actions::AdditionalCostPayment::Sacrifice(perm)];
        apply_additional_costs(&mut s, 0, &costs);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert_eq!(s.zone_count(Zone::Battlefield), 0);
    }

    #[test]
    fn additional_costs_discard_emits_event() {
        let mut s = GameState::new(2, 0);
        let card = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            card, 0, Zone::Hand(0), 0, Characteristics::default()));
        let costs = vec![crate::actions::AdditionalCostPayment::Discard(card)];
        apply_additional_costs(&mut s, 0, &costs);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::Discarded { player: 0, object_id }
                if *object_id == card)));
    }

    #[test]
    fn additional_costs_pay_life_deducts_and_emits() {
        let mut s = GameState::new(2, 0);
        let life_before = s.player(0).life;
        let costs = vec![crate::actions::AdditionalCostPayment::PayLife(3)];
        apply_additional_costs(&mut s, 0, &costs);
        assert_eq!(s.player(0).life, life_before - 3);
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::LifeLost { player: 0, amount: 3 })));
    }

    #[test]
    fn additional_costs_remove_counters_decrements() {
        let mut s = GameState::new(2, 0);
        let perm = s.allocate_object_id();
        let mut obj = GameObject::new(
            perm, 0, Zone::Battlefield, 0, creature_chars(1, 1));
        obj.add_counters(crate::types::CounterKind::Charge, 3);
        s.objects.insert(obj);
        let costs = vec![crate::actions::AdditionalCostPayment::RemoveCounters {
            source: perm,
            kind: crate::types::CounterKind::Charge,
            count: 2,
        }];
        apply_additional_costs(&mut s, 0, &costs);
        assert_eq!(
            s.objects.get(perm).unwrap().count_counters(
                crate::types::CounterKind::Charge),
            1,
        );
    }

    #[test]
    fn choose_first_player_sets_active_player_and_enters_mulligan() {
        let mut s = GameState::new(2, 0);
        s.priority.begin_special_action(SpecialAction::ChooseFirstPlayer, 0);
        let (s, yld) = step(
            s, Action::MakeChoice(ChoiceAction::ChoosePlayer(1)), &reg());

        assert_eq!(s.turn.active_player, 1);
        // Settled into mulligan for player 1.
        match yld {
            EngineYield::PendingDecision { player, context, .. } => {
                assert_eq!(player, 1);
                assert!(matches!(context, DecisionContext::Mulligan));
            }
            _ => panic!("expected Mulligan for P1 after ChooseFirstPlayer"),
        }
    }

    #[test]
    fn mulligan_keep_after_mulligans_requires_bottoming() {
        // Take a mulligan, then keep → should enter the bottom-cards
        // window rather than immediately advancing the game.
        let (state, _y) = start(11);
        let (state, _y) = step(state, Action::MulliganAgain, &reg());
        assert_eq!(state.player(0).mulligans_taken, 1);

        let (state, yld) = step(state, Action::MulliganKeep, &reg());
        // Engine should now be asking for bottom cards from player 0.
        match yld {
            EngineYield::PendingDecision { player, context, .. } => {
                assert_eq!(player, 0);
                assert!(matches!(context,
                    DecisionContext::BottomCards { count: 1 }));
            }
            _ => panic!("expected BottomCards/1 after mulligan+keep"),
        }
        assert_eq!(
            state.priority.special_action,
            Some(SpecialAction::LondonMulliganBottomCards(1)),
        );
    }

    #[test]
    fn bottom_cards_moves_owed_cards_and_advances_mulligan() {
        // Take a mulligan → keep → bottom 1 card → player 1's turn
        // to decide.
        let (state, _y) = start(13);
        let (state, _y) = step(state, Action::MulliganAgain, &reg());
        let (state, _y) = step(state, Action::MulliganKeep, &reg());

        // Pick any card from player 0's hand to bottom.
        let hand_ids: Vec<_> = state.objects
            .ids_in_zone_sorted(Zone::Hand(0));
        let pick = hand_ids[0];
        let hand_size_before = hand_ids.len();
        let lib_size_before = state.player(0).library_top_to_bottom.len();

        let (state, yld) = step(
            state, Action::BottomCards(vec![pick]), &reg());

        // Card moved hand → library bottom. Re-id means the library
        // bottom carries a new id, but hand/library counts still add
        // up and the old id is gone from the arena.
        assert!(state.objects.get(pick).is_none());
        assert_eq!(
            state.objects.count_in_zone(Zone::Hand(0)),
            hand_size_before - 1,
        );
        assert_eq!(
            state.player(0).library_top_to_bottom.len(),
            lib_size_before + 1,
        );
        let bottom_id = *state.player(0).library_top_to_bottom.last().unwrap();
        assert_eq!(state.objects.get(bottom_id).unwrap().zone, Zone::Library(0));

        // Player 1 should now be the one deciding mulligans.
        match yld {
            EngineYield::PendingDecision { player, context, .. } => {
                assert_eq!(player, 1);
                assert!(matches!(context, DecisionContext::Mulligan));
            }
            _ => panic!("expected Mulligan for P1"),
        }
    }

    #[test]
    #[should_panic(expected = "submitted")]
    fn bottom_cards_panics_on_wrong_length() {
        let (state, _y) = start(17);
        let (state, _y) = step(state, Action::MulliganAgain, &reg());
        let (state, _y) = step(state, Action::MulliganKeep, &reg());
        // 1 card is owed; submit 2 — should panic.
        let hand_ids: Vec<_> = state.objects.ids_in_zone_sorted(Zone::Hand(0));
        let picks = vec![hand_ids[0], hand_ids[1]];
        let _ = step(state, Action::BottomCards(picks), &reg());
    }

    #[test]
    fn mulligan_again_shuffles_hand_and_draws_seven() {
        let (state, _y) = start(99);
        let hand_before: Vec<_> = state.objects
            .objects_in_zone(Zone::Hand(0)).map(|o| o.id).collect();
        let (state, _y) = step(state, Action::MulliganAgain, &reg());
        assert_eq!(state.player(0).mulligans_taken, 1);
        // Same count in hand, but the set of ids may (and in a shuffle
        // almost always will) differ.
        let hand_after: Vec<_> = state.objects
            .objects_in_zone(Zone::Hand(0)).map(|o| o.id).collect();
        assert_eq!(hand_after.len(), 7);
        // We can't assert hand_after != hand_before for every seed
        // (the RNG could cycle), but we *do* know the library is the
        // same total size.
        let _ = hand_before;
    }

    // --- concede ------------------------------------------------------------

    #[test]
    fn concede_ends_game_with_opponent_winning() {
        let (state, _y) = start(1);
        // Skip mulligans.
        let (state, _y) = step(state, Action::MulliganKeep, &reg());
        let (state, _y) = step(state, Action::MulliganKeep, &reg());

        // Active player concedes.
        let (state, yld) = step(state, Action::Concede, &reg());
        match yld {
            EngineYield::GameOver(GameResult::Win(w)) => {
                assert_eq!(w, 1, "opponent (player 1) should win");
            }
            _ => panic!("expected GameOver/Win(1), got {yld:?}"),
        }
        assert!(state.player(0).has_lost);
        assert!(state.player(0).has_conceded);
    }

    // --- play land ----------------------------------------------------------

    #[test]
    fn play_land_moves_card_to_battlefield_and_decrements_plays() {
        // Skip mulligans so we're in turn 1 main phase.
        let (s, _) = start(1);
        let (s, _) = step(s, Action::MulliganKeep, &reg());
        let (state, _) = step(s, Action::MulliganKeep, &reg());

        // Find a land in player 0's hand.
        let land_id = state.objects
            .objects_in_zone(Zone::Hand(0))
            .find(|o| o.is_land())
            .map(|o| o.id)
            .expect("simple_deck has lands");

        assert_eq!(state.player(0).land_plays_remaining, 1);
        let (state_after, _) = step(
            state.clone(),
            Action::PlayLand { object_id: land_id },
            &reg(),
        );
        // Re-id on the hand→battlefield move; locate the new land.
        assert!(state_after.objects.get(land_id).is_none());
        let bf_land_count = state_after.objects.objects_in_zone(Zone::Battlefield)
            .filter(|o| o.is_land()).count();
        assert_eq!(bf_land_count, 1);
        assert_eq!(state_after.player(0).land_plays_remaining, 0);

        // Original untouched.
        state.objects.get(land_id).unwrap().zone.is_hand().then_some(()).unwrap();
    }

    // --- phase progression --------------------------------------------------

    #[test]
    fn passing_priority_in_main_phase_advances_through_combat_to_post_main() {
        let (s, _) = start(1);
        let (s, _) = step(s, Action::MulliganKeep, &reg());
        let (mut state, _) = step(s, Action::MulliganKeep, &reg());

        // Repeatedly pass priority; we should cycle through:
        // PreCombatMain → Combat (with attacker declaration) → ...
        // In turn 1 there's nothing to attack with (summoning-sick
        // lands only), so DeclareAttackers yields the empty-attacker
        // option. We'll pick that when prompted.
        for _ in 0..40 {
            let yld = compute_next_decision(&state, &reg());
            match yld {
                EngineYield::GameOver(_) => break,
                EngineYield::PendingDecision { context, legal_actions, .. } => {
                    let action = match context {
                        DecisionContext::DeclareAttackers => {
                            Action::DeclareAttackers { attackers: Vec::new() }
                        }
                        DecisionContext::DeclareBlockers => {
                            Action::DeclareBlockers { blockers: Vec::new() }
                        }
                        _ => {
                            // Pick PassPriority if available, else
                            // first action.
                            legal_actions.iter().find(|a| a.is_pass())
                                .cloned()
                                .unwrap_or_else(|| legal_actions[0].clone())
                        }
                    };
                    let (s, _y) = step(state.clone(), action, &reg());
                    state = s;
                    if state.turn.turn_number >= 2 { break; }
                }
            }
        }

        // At least one turn transition occurred.
        assert!(state.turn.turn_number >= 2);
    }

    // --- cast spell stub ----------------------------------------------------

    #[test]
    fn cast_spell_moves_card_to_stack_and_emits_spell_cast() {
        let (s, _) = start(3);
        let (s, _) = step(s, Action::MulliganKeep, &reg());
        let (mut state, _) = step(s, Action::MulliganKeep, &reg());

        // Inject a bolt into player 0's hand (bypassing deck since
        // our simple_deck has no instants).
        let bolt_id = state.allocate_object_id();
        state.objects.insert(GameObject::new(
            bolt_id, 0, Zone::Hand(0), 0, instant_chars()));
        // Give player 0 one red mana.
        state.player_mut(0).mana_pool.add_mana(ManaColor::Red, 1, 0);

        let cast = Action::CastSpell {
            object_id: bolt_id,
            targets: crate::targets::TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan {
                assignments: vec![crate::actions::ManaAssignment {
                    pool_index: 0, cost_index: 0,
                }],
                ..Default::default()
            },
            additional_costs: Vec::new(),
            x_value: None,
        };
        let (state, _) = step(state, cast, &reg());
        // Stack has one entry or — after everyone passes — has
        // resolved. Since we didn't auto-pass, the stack should hold
        // the bolt and priority returned to P0 (the active player).
        // The stack id is a fresh re-id from the hand card.
        assert_eq!(state.stack_size(), 1);
        let stack_id = state.top_of_stack().unwrap().id;
        assert_ne!(stack_id, bolt_id);
        assert!(state.event_log.iter().any(|e|
            matches!(e, GameEvent::SpellCast { object_id, .. } if *object_id == stack_id)));

        let (state, _) = step(state, Action::PassPriority, &reg());
        let (state, _) = step(state, Action::PassPriority, &reg());
        // Stack resolved → bolt in P0's graveyard under yet another new id.
        assert_eq!(state.stack_size(), 0);
        assert_eq!(
            state.objects.objects_in_zone(Zone::Graveyard(0))
                .filter(|o| o.is_instant()).count(),
            1,
            "instant should be in its owner's graveyard post-resolve"
        );
    }

    // --- game-over short-circuit -------------------------------------------

    #[test]
    fn step_short_circuits_when_game_already_over() {
        let mut state = GameState::new(2, 0);
        state.result = Some(GameResult::Draw);
        let (_s, yld) = step(state, Action::PassPriority, &reg());
        assert!(yld.is_game_over());
    }

    // --- advance_phase pair coverage ---------------------------------------

    #[test]
    fn advance_phase_walks_a_full_turn_exactly() {
        // Skip mulligans so we're in turn 1 post-cleanup transitions
        // are testable.
        let (s, _) = start(42);
        let (s, _) = step(s, Action::MulliganKeep, &reg());
        let (state, _y) = step(s, Action::MulliganKeep, &reg());

        // From (PreCombatMain, Main) walk through every (phase, step)
        // pair in the turn. We use advance_phase directly rather than
        // priority passing so we can observe each intermediate state.
        let mut state = state;
        let mut seen: Vec<(Phase, Step)> = Vec::new();
        for _ in 0..20 {
            seen.push((state.turn.phase, state.turn.step));
            if state.turn.turn_number > 1 { break; }
            advance_phase(&mut state, &reg());
        }

        // We expect to see Combat sub-steps and the ending phase.
        assert!(seen.iter().any(|&(p, _)| p == Phase::Combat));
        assert!(seen.iter().any(|&(_, s)| s == Step::End));
        assert!(seen.iter().any(|&(_, s)| s == Step::Cleanup));
    }

    // --- extra turn queue ---------------------------------------------------

    #[test]
    fn extra_turn_queue_routes_next_turn_to_that_player() {
        let (s, _) = start(5);
        let (s, _) = step(s, Action::MulliganKeep, &reg());
        let (mut state, _) = step(s, Action::MulliganKeep, &reg());

        // Queue an extra turn for the active player.
        let ap = state.active_player();
        state.turn.queue_extra_turn(ap);
        // Drive to end of turn via advance_phase.
        while state.turn.turn_number == 1 {
            advance_phase(&mut state, &reg());
            if state.turn.turn_number == 2 { break; }
        }
        // Same player takes turn 2.
        assert_eq!(state.active_player(), ap);
    }

    // --- FormatConfig integration ------------------------------------------

    #[test]
    fn default_new_uses_standard_2026_format() {
        let s = GameState::new(2, 0);
        assert_eq!(s.format, crate::format::FormatConfig::standard_2026());
        for p in 0..s.num_players() {
            assert_eq!(s.player(p).life, 20);
        }
    }

    #[test]
    fn with_format_applies_starting_life() {
        let f = crate::format::FormatConfig::commander();
        let s = GameState::with_format(2, 0, f);
        for p in 0..s.num_players() {
            assert_eq!(s.player(p).life, 40,
                "commander preset should seat players at 40 life");
        }
    }

    fn register_mountain_deck(
        registry: &mut crate::registry::CardRegistry,
        size: u32,
    ) -> Vec<crate::types::CardId> {
        let _ = crate::sample_cards::register_mountain(registry);
        let id = registry.card_id_by_name("Mountain").unwrap();
        vec![id; size as usize]
    }

    #[test]
    fn new_game_with_format_honors_starting_hand_size() {
        // Use a custom format that opens with 5 cards instead of 7.
        let mut registry = crate::registry::CardRegistry::new();
        let deck = register_mountain_deck(&mut registry, 60);
        let mut f = crate::format::FormatConfig::standard_2026();
        f.starting_hand_size = 5;
        let (state, _yld) = new_game_with_format(
            vec![deck.clone(), deck], f, &registry, 42);
        for p in 0..2 {
            assert_eq!(
                state.objects.count_in_zone(Zone::Hand(p)), 5,
                "player {p} should open with 5 cards",
            );
        }
    }

    #[test]
    fn new_game_with_format_honors_starting_life() {
        let mut registry = crate::registry::CardRegistry::new();
        let deck = register_mountain_deck(&mut registry, 60);
        let (state, _yld) = new_game_with_format(
            vec![deck.clone(), deck],
            crate::format::FormatConfig::commander(),
            &registry,
            42,
        );
        for p in 0..2 {
            assert_eq!(state.player(p).life, 40);
        }
        assert_eq!(state.format.name, "Commander");
    }

    #[test]
    fn new_game_delegates_to_new_game_with_format() {
        // Both entry points should produce states with identical
        // formats for the same seed.
        let mut registry = crate::registry::CardRegistry::new();
        let _ = crate::sample_cards::register_mountain(&mut registry);
        let deck = vec![registry.card_id_by_name("Mountain").unwrap(); 60];
        let (a, _) = new_game(vec![deck.clone(), deck.clone()], &registry, 42);
        let (b, _) = new_game_with_format(
            vec![deck.clone(), deck],
            crate::format::FormatConfig::standard_2026(),
            &registry, 42,
        );
        assert_eq!(a.format, b.format);
        assert_eq!(a.player(0).life, b.player(0).life);
        assert_eq!(
            a.objects.count_in_zone(Zone::Hand(0)),
            b.objects.count_in_zone(Zone::Hand(0)),
        );
    }

    #[test]
    fn cleanup_discards_per_format_max_hand_size() {
        // Build a minimal state and drop 10 cards into a player's hand,
        // then run cleanup. With the default format (max_hand_size = 7)
        // we should discard 3.
        let mut state = GameState::new(2, 0);
        state.turn.phase = Phase::Ending;
        state.turn.step = Step::Cleanup;
        let ap = state.active_player();
        for _ in 0..10 {
            let id = state.allocate_object_id();
            state.objects.insert(GameObject::new(
                id, ap, Zone::Hand(ap), 0,
                Characteristics::default(),
            ));
        }
        assert_eq!(state.objects.count_in_zone(Zone::Hand(ap)), 10);

        cleanup_step(&mut state);

        assert_eq!(
            state.objects.count_in_zone(Zone::Hand(ap)), 7,
            "cleanup should discard down to max_hand_size = 7",
        );
    }

    #[test]
    fn cleanup_respects_custom_max_hand_size() {
        let mut f = crate::format::FormatConfig::standard_2026();
        f.max_hand_size = 4;
        let mut state = GameState::with_format(2, 0, f);
        state.turn.phase = Phase::Ending;
        state.turn.step = Step::Cleanup;
        let ap = state.active_player();
        for _ in 0..10 {
            let id = state.allocate_object_id();
            state.objects.insert(GameObject::new(
                id, ap, Zone::Hand(ap), 0,
                Characteristics::default(),
            ));
        }

        cleanup_step(&mut state);

        assert_eq!(
            state.objects.count_in_zone(Zone::Hand(ap)), 4,
            "custom max_hand_size = 4 should cap hand at 4",
        );
    }
}
