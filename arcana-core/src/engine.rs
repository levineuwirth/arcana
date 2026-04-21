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
///
/// `Clone` is derived because `arcana-session` stores the most
/// recent yield in `GameSession::pending` and occasionally needs to
/// inspect it without consuming it (undo restore, test harnesses).
/// The AI/RL path calls [`step`] directly and moves yields by value,
/// so it pays nothing for the clone impl being available.
#[derive(Clone, Debug)]
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
            cast_modifier, cost_reductions,
        } => {
            apply_cast_spell(
                state, registry, object_id, targets, modes,
                mana_payment, additional_costs, x_value, cast_modifier,
                cost_reductions,
            );
        }

        Action::ActivateAbility {
            source, ability_index, targets, mana_payment, additional_costs,
        } => apply_activate_ability(
            state, registry, source, ability_index, targets,
            mana_payment, additional_costs,
        ),

        Action::PlayLand { object_id, mdfc_back } =>
            apply_play_land(state, registry, object_id, mdfc_back),

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

        Action::OrderBlockers { orderings } => {
            let applied = state.apply_blocker_ordering(orderings);
            if applied {
                // CR 509.2 completes; enter PostDeclareBlockers with
                // priority to the active player for the triggers +
                // response window.
                let ap = state.active_player();
                state.priority.give_to(ap);
            }
            // If invalid (applied == false), phase stays at OrderBlockers
            // and the next compute_next_decision call re-prompts.
        }

        Action::AssignCombatDamage { distributions } => {
            apply_assign_combat_damage(state, distributions);
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

// The compositional-fields approach (matching `Action::CastSpell`)
// pushes the arg count past clippy's default 7. Refactor to a grouped
// `CastInputs` struct is planned when a second cost-modifier (convoke
// or improvise) lands — see the Shape B-full note on `CastSubStep`.
#[allow(clippy::too_many_arguments)]
fn apply_cast_spell(
    state: &mut GameState,
    registry: &CardRegistry,
    object_id: ObjectId,
    targets: crate::targets::TargetSelection,
    modes: Vec<crate::stack::ModeChoice>,
    mana_payment: crate::actions::ManaPaymentPlan,
    additional_costs: Vec<crate::actions::AdditionalCostPayment>,
    x_value: Option<u32>,
    cast_modifier: crate::actions::CastModifier,
    cost_reductions: crate::actions::CostReductions,
) {
    let controller = state.priority_player();

    // Alt-cost / zone validation. Belt-and-suspenders — legal_actions
    // is expected to filter invalid combinations, but bogus agent
    // input shouldn't corrupt state. Each branch rejects silently
    // (returns early) if the zone/keyword pairing is wrong.
    match cast_modifier {
        crate::actions::CastModifier::None => {
            // Normal cast: source must be in the caster's hand (lands
            // go through PlayLand, not CastSpell).
            let from_hand = state.objects.get(object_id)
                .is_some_and(|o| o.zone == crate::zones::Zone::Hand(controller));
            if !from_hand { return; }
        }
        crate::actions::CastModifier::Flashback => {
            let from_own_yard = state.objects.get(object_id)
                .is_some_and(|o| o.zone == crate::zones::Zone::Graveyard(controller));
            if !from_own_yard { return; }
            if flashback_cost_for(state, object_id).is_none() { return; }
        }
        crate::actions::CastModifier::Madness => {
            // CR 702.34 — cast from exile via madness. Source must
            // be in Exile with madness_pending=true (i.e. entered
            // exile via the madness replacement, not some other
            // exile effect) and must still have the Madness keyword
            // (granted-madness would also be honored via layers).
            let flagged_in_exile = state.objects.get(object_id)
                .is_some_and(|o| o.zone == crate::zones::Zone::Exile
                    && o.madness_pending
                    && o.owner == controller);
            if !flagged_in_exile { return; }
            if madness_cost_for(state, object_id).is_none() { return; }
        }
        crate::actions::CastModifier::Adventure => {
            // CR 715 — cast the Adventure half from hand. The card
            // must currently be in the caster's hand and its
            // registry definition must declare an Adventure face.
            let from_hand = state.objects.get(object_id)
                .is_some_and(|o| o.zone == crate::zones::Zone::Hand(controller));
            if !from_hand { return; }
            if adventure_face_of(state, registry, object_id).is_none() {
                return;
            }
        }
        crate::actions::CastModifier::AdventureCreature => {
            // CR 715 — cast the creature half from adventure-exile.
            // Source must be in Exile with
            // `adventure_exile_pending=true` and belong to the caster.
            let flagged_in_exile = state.objects.get(object_id)
                .is_some_and(|o| o.zone == crate::zones::Zone::Exile
                    && o.adventure_exile_pending
                    && o.owner == controller);
            if !flagged_in_exile { return; }
        }
        crate::actions::CastModifier::MdfcBack => {
            // CR 712.4 — cast the back face from hand. Source must
            // be in the caster's hand and the card must declare an
            // MDFC back face. The back face must be a spell — a
            // land back goes through `Action::PlayLand` with
            // `mdfc_back = true` instead.
            let from_hand = state.objects.get(object_id)
                .is_some_and(|o| o.zone == crate::zones::Zone::Hand(controller));
            if !from_hand { return; }
            let back_is_spell = mdfc_back_face_of(state, registry, object_id)
                .is_some_and(|f| !f.characteristics.types.is_land());
            if !back_is_spell { return; }
        }
        crate::actions::CastModifier::SplitRight => {
            // CR 711 — cast the right half of a split card from
            // hand. Source must be in the caster's hand and the
            // card must declare a Split relationship.
            let from_hand = state.objects.get(object_id)
                .is_some_and(|o| o.zone == crate::zones::Zone::Hand(controller));
            if !from_hand { return; }
            if split_right_face_of(state, registry, object_id).is_none() {
                return;
            }
        }
    }

    // Modal validation (CR 700.2). A modal spell requires the caster to
    // pick between `min_modes` and `max_modes` clauses; picks must be
    // in range and the card-order invariant on `ModeChoice` must hold.
    // Non-modal spells must supply either `vec![]` or a single empty
    // ModeChoice — both are treated as "no modes." Agent input that
    // violates this is rejected silently, matching the rest of this
    // function.
    {
        let spell_ability = state.objects.get(object_id)
            .and_then(|o| registry.get(o.card_id))
            .and_then(|def| def.spell_ability.as_ref());
        match spell_ability.and_then(|sa| sa.modal.as_ref()) {
            Some(modal) => {
                // Modal spell: need exactly one ModeChoice, count in
                // [min, max], indices in range and normalized.
                if modes.len() != 1 { return; }
                let choice = &modes[0];
                let n = choice.mode_indices.len();
                if n < modal.min_modes || n > modal.max_modes { return; }
                // Indices in bounds.
                if choice.mode_indices.iter().any(|i| *i >= modal.clauses.len()) {
                    return;
                }
                // Sorted-ascending + unique invariant. `ModeChoice::new`
                // normalizes on construction, but serialized inputs or
                // hand-built tests could violate this.
                if !choice.mode_indices.windows(2).all(|w| w[0] < w[1]) {
                    return;
                }
            }
            None => {
                // Non-modal: empty or a single empty ModeChoice.
                let has_payload = modes.iter().any(|m| !m.is_empty());
                if has_payload { return; }
            }
        }
    }

    // Delve validation (CR 702.66). `Some(vec![])` is a legal "has
    // delve, chose not to use it"; `None` is "card has no delve."
    // Both reach cost payment with no exile side effect. `Some(list)`
    // must pass: card has delve, every id is in caster's graveyard,
    // ids are distinct, count ≤ the spell's printed generic component.
    let delve_exiles_vec = cost_reductions.delve_exiles.clone().unwrap_or_default();
    if !delve_exiles_vec.is_empty() {
        if !has_delve(state, object_id) { return; }
        let mut seen = crate::collections::HashSet::default();
        for &exile_id in &delve_exiles_vec {
            if !seen.insert(exile_id) { return; }
            let in_caster_yard = state.objects.get(exile_id)
                .is_some_and(|o| o.zone == crate::zones::Zone::Graveyard(controller));
            if !in_caster_yard { return; }
        }
        // Bound by the spell's generic component. For flashback casts
        // the bound is against the flashback cost; for a normal cast
        // it's against the printed cost.
        let cost_opt = match cast_modifier {
            crate::actions::CastModifier::None
            | crate::actions::CastModifier::AdventureCreature =>
                state.objects.get(object_id)
                    .and_then(|o| o.characteristics.mana_cost.clone()),
            crate::actions::CastModifier::Flashback =>
                flashback_cost_for(state, object_id),
            crate::actions::CastModifier::Madness =>
                madness_cost_for(state, object_id),
            crate::actions::CastModifier::Adventure =>
                adventure_face_of(state, registry, object_id)
                    .and_then(|f| f.characteristics.mana_cost.clone()),
            crate::actions::CastModifier::MdfcBack =>
                mdfc_back_face_of(state, registry, object_id)
                    .and_then(|f| f.characteristics.mana_cost.clone()),
            crate::actions::CastModifier::SplitRight =>
                split_right_face_of(state, registry, object_id)
                    .and_then(|f| f.characteristics.mana_cost.clone()),
        };
        let generic_total: u32 = cost_opt.as_ref()
            .map(|c| c.components.iter().filter_map(|comp|
                if let crate::mana::ManaCostComponent::Generic(n) = comp {
                    Some(*n)
                } else { None }).sum())
            .unwrap_or(0);
        if (delve_exiles_vec.len() as u32) > generic_total { return; }
    }

    // Convoke validation (CR 702.51). Each assignment's creature
    // must be: caster-controlled, on the battlefield, a creature,
    // untapped. Payment must match the creature's effective colors
    // (Generic is always legal; Color(c) requires the creature to
    // actually be c). Duplicate creature ids are rejected —
    // a creature can only tap once per cast. Summoning sickness is
    // *not* a barrier: CR 302.1 restricts tap-for-mana and combat,
    // not tap-as-cost for non-mana abilities.
    let convoke_taps_vec = cost_reductions.convoke_taps.clone().unwrap_or_default();
    if !convoke_taps_vec.is_empty() {
        if !has_convoke(state, object_id) { return; }
        let mut seen_creatures = crate::collections::HashSet::default();
        for assignment in &convoke_taps_vec {
            if !seen_creatures.insert(assignment.creature) { return; }
            let Some(creature) = state.objects.get(assignment.creature) else {
                return;
            };
            if creature.controller != controller { return; }
            if creature.zone != crate::zones::Zone::Battlefield { return; }
            if !creature.characteristics.is_creature() { return; }
            if creature.is_tapped() { return; }
            // Color check: a Color(c) payment requires the creature
            // to be that color. Layer-aware via effective colors
            // (creature's printed colors may be overridden).
            if let crate::actions::ConvokePayment::Color(mana_color) = assignment.payment {
                let Some(color) = mana_color.as_color() else {
                    // Colorless ManaColor shouldn't appear in a
                    // Color(c) variant — reject.
                    return;
                };
                if !creature.characteristics.colors.contains(color) {
                    return;
                }
            }
        }
    }

    // 1a. Exile delve payment (CR 702.66, CR 601.2f-h). This is a
    //     cost-payment sub-event alongside `spend_mana_plan` and
    //     convoke taps; all happen in the same logical step. We emit
    //     the exiles first so event log ordering is deterministic,
    //     but they belong to the same atomic "pay total cost" step
    //     semantically.
    for &exile_id in &delve_exiles_vec {
        state.move_object_to_zone(
            exile_id, crate::zones::Zone::Exile,
            crate::events::MoveCause::Cost);
    }

    // 1b. Tap convoke creatures (CR 702.51). Each tap is a cost-
    //     payment sub-event. The agent's chosen (creature, payment)
    //     pairs were already validated above; this just applies the
    //     tap and emits `Tapped`.
    for assignment in &convoke_taps_vec {
        if let Some(obj) = state.objects.get_mut(assignment.creature) {
            if obj.tap() {
                state.emit(GameEvent::Tapped {
                    object_id: assignment.creature,
                });
            }
        }
    }

    // Improvise validation (CR 702.127). Each id must be: caster-
    // controlled artifact permanent, on the battlefield, untapped,
    // distinct. Count bounded by the post-delve generic total.
    // Summoning sickness is irrelevant (artifact creatures like
    // Ornithopter with sickness can still tap for improvise — CR
    // 302.1 restricts tap-for-mana and combat only).
    let improvise_taps_vec = cost_reductions.improvise_taps.clone().unwrap_or_default();
    if !improvise_taps_vec.is_empty() {
        if !has_improvise(state, object_id) { return; }
        let mut seen_artifacts = crate::collections::HashSet::default();
        for &art_id in &improvise_taps_vec {
            if !seen_artifacts.insert(art_id) { return; }
            let Some(obj) = state.objects.get(art_id) else { return; };
            if obj.controller != controller { return; }
            if obj.zone != crate::zones::Zone::Battlefield { return; }
            if !obj.characteristics.types.is_artifact() { return; }
            if obj.is_tapped() { return; }
        }
        // Bound: improvise taps + delve exiles must not exceed the
        // spell's generic component. Flashback adjusts the base cost.
        let cost_opt = match cast_modifier {
            crate::actions::CastModifier::None
            | crate::actions::CastModifier::AdventureCreature =>
                state.objects.get(object_id)
                    .and_then(|o| o.characteristics.mana_cost.clone()),
            crate::actions::CastModifier::Flashback =>
                flashback_cost_for(state, object_id),
            crate::actions::CastModifier::Madness =>
                madness_cost_for(state, object_id),
            crate::actions::CastModifier::Adventure =>
                adventure_face_of(state, registry, object_id)
                    .and_then(|f| f.characteristics.mana_cost.clone()),
            crate::actions::CastModifier::MdfcBack =>
                mdfc_back_face_of(state, registry, object_id)
                    .and_then(|f| f.characteristics.mana_cost.clone()),
            crate::actions::CastModifier::SplitRight =>
                split_right_face_of(state, registry, object_id)
                    .and_then(|f| f.characteristics.mana_cost.clone()),
        };
        let generic_total: u32 = cost_opt.as_ref()
            .map(|c| c.components.iter().filter_map(|comp|
                if let crate::mana::ManaCostComponent::Generic(n) = comp {
                    Some(*n)
                } else { None }).sum())
            .unwrap_or(0);
        let used_generic = delve_exiles_vec.len() as u32
            + improvise_taps_vec.len() as u32;
        if used_generic > generic_total { return; }
    }

    // 1b-improvise. Tap improvise artifacts (CR 702.127). Another
    //     cost-payment sub-event alongside delve exiles and convoke
    //     taps.
    for &art_id in &improvise_taps_vec {
        if let Some(obj) = state.objects.get_mut(art_id) {
            if obj.tap() {
                state.emit(GameEvent::Tapped { object_id: art_id });
            }
        }
    }

    // Kicker validation (CR 702.32). If the agent elected Kicker, the
    // card must actually have the keyword (layer-aware) and the
    // `mana_payment` must cover printed + kicker at the solver level.
    // `legal_actions` solves the summed cost; here we just guard
    // against bogus agent input. The kicker mana cost is not spent
    // separately — it's baked into the payment plan before this
    // function runs. Only the *flag* lives in `additional_costs`.
    let elected_kicker = additional_costs.iter().any(|c|
        matches!(c, crate::actions::AdditionalCostPayment::Kicker));
    if elected_kicker && !has_kicker(state, object_id) { return; }

    // 1c. Spend the mana payment. `mana_payment` is already sized
    //     against the delve-reduced and convoke-reduced cost —
    //     legal_actions solves mana against the post-reduction cost,
    //     not the printed cost.
    spend_mana_plan(state, controller, &mana_payment);

    // 2. Pay additional costs (sacrifice, discard, life, etc.).
    apply_additional_costs(state, controller, &additional_costs);

    // 3. Announce on the stack (CR 601.2a). Moves card Hand→Stack,
    //    emits ZoneChange(Cast). Snapshots the spell's targeting
    //    requirements onto the entry so copies (storm / CopySpell)
    //    can push ChooseTargets without registry access.
    //
    //    For modal spells, the effective target requirements depend on
    //    which clauses were chosen (CR 700.2c — card order). The
    //    registry helper resolves this; non-modal spells fall back to
    //    the flat list.
    //
    //    For Adventure casts (CR 715.2) the announced object uses the
    //    Adventure face's characteristics — name, type line, mana
    //    cost, and spell ability all come from the face. We swap the
    //    hand object's characteristics to the face's before announce
    //    so `announce_spell_on_stack` reads the right sheet onto the
    //    stack entry; the pre-swap (creature-face) characteristics
    //    ride on the entry via `pre_adventure_characteristics` so the
    //    leave-the-stack path can restore them on the exile object.
    let card_def = state.objects.get(object_id)
        .and_then(|o| registry.get(o.card_id));
    let is_adventure_cast = matches!(cast_modifier,
        crate::actions::CastModifier::Adventure);
    let is_mdfc_back_cast = matches!(cast_modifier,
        crate::actions::CastModifier::MdfcBack);
    let is_split_right_cast = matches!(cast_modifier,
        crate::actions::CastModifier::SplitRight);
    // A Normal cast of a Split card resolves as the left half; it
    // still needs the combined→left swap so the stack entry carries
    // left-half chars and `pre_split_characteristics` on the entry
    // restores the combined view on leave-stack (CR 711.4).
    let is_split_left_cast = matches!(cast_modifier,
            crate::actions::CastModifier::None)
        && card_def.and_then(|d| d.alternate_face.as_ref())
            .and_then(|af| af.as_split()).is_some();
    let adventure_face = if is_adventure_cast {
        card_def.and_then(|d| d.alternate_face.as_ref())
            .and_then(|af| af.as_adventure())
    } else { None };
    let mdfc_back_face = if is_mdfc_back_cast {
        card_def.and_then(|d| d.alternate_face.as_ref())
            .and_then(|af| af.as_mdfc())
    } else { None };
    let split_right_face = if is_split_right_cast {
        card_def.and_then(|d| d.alternate_face.as_ref())
            .and_then(|af| af.as_split())
    } else { None };
    let pre_adventure_chars = if is_adventure_cast {
        let swapped_chars = adventure_face.map(|f| f.characteristics.clone());
        if let (Some(swapped), Some(obj)) = (swapped_chars,
            state.objects.get_mut(object_id))
        {
            let prior = std::mem::replace(&mut obj.characteristics, swapped);
            Some(prior)
        } else { None }
    } else { None };
    // MDFC back-face casts swap characteristics the same way as
    // Adventure — the announced object needs the chosen face's sheet
    // on the stack entry. Snapshot the pre-swap front-face chars on
    // the object so `swap_to_zone_reid` can revert per CR 712.2b when
    // the object leaves the stack or battlefield to any other zone.
    if is_mdfc_back_cast {
        let swapped_chars = mdfc_back_face.map(|f| f.characteristics.clone());
        if let (Some(swapped), Some(obj)) = (swapped_chars,
            state.objects.get_mut(object_id))
        {
            let prior = std::mem::replace(&mut obj.characteristics, swapped);
            obj.default_face_characteristics = Some(prior);
            obj.visible_face = 1;
        }
    }
    // Split casts swap the object's characteristics to the chosen
    // half while it sits on the stack, and snapshot the combined
    // off-stack view onto the stack entry (stamped below) so
    // `finalize_resolved_spell` / `counter_resolved_spell` can
    // restore CR 711.4's combined view when the card moves to the
    // graveyard. Left casts swap combined → base (left face) chars;
    // right casts swap combined → right face chars. `visible_face`
    // tracks which half is on the stack (0 = left, 1 = right).
    let pre_split_chars = if is_split_right_cast {
        let swapped_chars = split_right_face.map(|f| f.characteristics.clone());
        if let (Some(swapped), Some(obj)) = (swapped_chars,
            state.objects.get_mut(object_id))
        {
            let prior = std::mem::replace(&mut obj.characteristics, swapped);
            obj.visible_face = 1;
            Some(prior)
        } else { None }
    } else if is_split_left_cast {
        let swapped_chars = card_def.map(|d| d.base_characteristics.clone());
        if let (Some(swapped), Some(obj)) = (swapped_chars,
            state.objects.get_mut(object_id))
        {
            let prior = std::mem::replace(&mut obj.characteristics, swapped);
            obj.visible_face = 0;
            Some(prior)
        } else { None }
    } else { None };

    let (spell_ability, enters_with) = if is_adventure_cast {
        // Adventure face supplies the spell ability; enters_with
        // belongs to the creature face and does not apply.
        (adventure_face.and_then(|f| f.spell_ability.as_ref()), Vec::new())
    } else if is_mdfc_back_cast {
        // MDFC back face supplies its own spell ability. `enters_with`
        // clauses on an MDFC card are declared at the CardDefinition
        // level and don't yet split per-face; none of today's MDFC
        // seeds need per-face enters_with, so this reads the main
        // list verbatim.
        let sa = mdfc_back_face.and_then(|f| f.spell_ability.as_ref());
        let ew = card_def.map(|d| d.enters_with.clone()).unwrap_or_default();
        (sa, ew)
    } else if is_split_right_cast {
        // Split right half supplies its own spell ability. No
        // enters_with — both halves are non-permanent spells.
        (split_right_face.and_then(|f| f.spell_ability.as_ref()), Vec::new())
    } else {
        let sa = card_def.and_then(|def| def.spell_ability.as_ref());
        let ew = card_def.map(|d| d.enters_with.clone()).unwrap_or_default();
        (sa, ew)
    };
    let target_requirements = spell_ability
        .map(|sa| crate::registry::effective_target_requirements(sa, &modes))
        .unwrap_or_default();
    let entry_id = state.announce_spell_on_stack(
        object_id, controller, targets, modes, x_value, target_requirements);

    // Stamp the entry with the cast modifier so leave-the-stack paths
    // (resolve / counter / fizzle) can route Flashback casts to exile.
    // Same pattern for `enters_with`: finalize_resolved_spell reads
    // the clauses off the entry to apply CR 121.6a "enters with"
    // counters/tapped.
    let delve_count = delve_exiles_vec.len() as u32;
    if let Some(entry) = state.stack.iter_mut().find(|e| e.id == entry_id) {
        entry.cast_modifier = cast_modifier;
        entry.enters_with = enters_with;
        entry.delve_count = delve_count;
        entry.kicked = elected_kicker;
        entry.pre_adventure_characteristics = pre_adventure_chars;
        entry.pre_split_characteristics = pre_split_chars;
    }

    // 4. Emit SpellCast (CR 601.2e) — triggers pick this up.
    state.emit_spell_cast(entry_id);

    // CR 115 — emit BecomesTarget for each object target, in the order
    // they were declared. Ward (CR 702.21a) and other "becomes the
    // target" triggers pick these up during the next settle pass.
    emit_becomes_target_events(state, entry_id, controller);

    // 5. Record the action (priority retained, pass counter reset).
    state.priority.record_action();
}

/// Emit [`GameEvent::BecomesTarget`] for each `TargetChoice::Object`
/// (and object variants of `ObjectOrPlayer`) on the stack entry
/// identified by `stack_entry_id`. Silently no-ops for non-existent
/// entries or entries without object targets. Used by
/// [`apply_cast_spell`] and [`apply_activate_ability`] alike.
fn emit_becomes_target_events(
    state: &mut GameState,
    stack_entry_id: ObjectId,
    controller: PlayerId,
) {
    use crate::targets::{ObjectOrPlayer, TargetChoice};
    let target_ids: Vec<ObjectId> = {
        let Some(entry) = state.stack.iter().find(|e| e.id == stack_entry_id)
            else { return; };
        entry.targets.targets.iter().filter_map(|choice| match choice {
            TargetChoice::Object(id) => Some(*id),
            TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => Some(*id),
            TargetChoice::Player(_)
            | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(_)) => None,
        }).collect()
    };
    for target in target_ids {
        state.emit(crate::events::GameEvent::BecomesTarget {
            target, source: stack_entry_id, controller,
        });
    }
}

/// Return the currently-granted flashback cost for `object_id`, if any.
/// Walks [`GameState::effective_keywords`] so that layer-6 grants
/// (Snapcaster Mage-style "gains flashback until end of turn") are
/// honored — `base_characteristics.keywords` alone would miss them.
///
/// Cost reductions are a Phase 2-B concern; this helper returns the
/// keyword's carried cost verbatim for now. When cost-reduction
/// effects land, apply them here so `legal_actions` and
/// `apply_cast_spell` stay in sync (both go through this helper).
///
/// Returns the *first* Flashback keyword found; if a card ends up
/// with multiple (multiple grants stack, per CR 702.33c) the caller
/// is responsible for enumerating each via
/// [`all_flashback_costs_for`].
pub(crate) fn flashback_cost_for(
    state: &GameState,
    object_id: ObjectId,
) -> Option<crate::mana::ManaCost> {
    state.effective_keywords(object_id).into_iter()
        .find_map(|kw| match kw {
            crate::effects::KeywordAbility::Flashback(cost) => Some(cost),
            _ => None,
        })
}

/// Enumerate every granted flashback cost on `object_id`. Usually a
/// one-element Vec (printed flashback) or empty (none). A card
/// gaining a second flashback via Snapcaster-on-a-flashback-card is
/// rare but legal; CR 702.33c says the caster picks one.
pub(crate) fn all_flashback_costs_for(
    state: &GameState,
    object_id: ObjectId,
) -> Vec<crate::mana::ManaCost> {
    state.effective_keywords(object_id).into_iter()
        .filter_map(|kw| match kw {
            crate::effects::KeywordAbility::Flashback(cost) => Some(cost),
            _ => None,
        })
        .collect()
}

/// Layer-aware check: does `object_id` currently have the Delve
/// keyword (CR 702.66)? Walks [`GameState::effective_keywords`] so
/// granted delve (Snapcaster-style, were any such card to exist) is
/// honored alongside the printed keyword.
pub(crate) fn has_delve(state: &GameState, object_id: ObjectId) -> bool {
    state.effective_keywords(object_id).into_iter()
        .any(|kw| matches!(kw, crate::effects::KeywordAbility::Delve))
}

/// Layer-aware check: does `object_id` currently have the Convoke
/// keyword (CR 702.51)? Walks [`GameState::effective_keywords`] so
/// granted convoke is honored alongside the printed keyword.
pub(crate) fn has_convoke(state: &GameState, object_id: ObjectId) -> bool {
    state.effective_keywords(object_id).into_iter()
        .any(|kw| matches!(kw, crate::effects::KeywordAbility::Convoke))
}

/// Layer-aware check: does `object_id` currently have the Improvise
/// keyword (CR 702.127)? Walks [`GameState::effective_keywords`]
/// so granted improvise is honored alongside the printed keyword.
pub(crate) fn has_improvise(state: &GameState, object_id: ObjectId) -> bool {
    state.effective_keywords(object_id).into_iter()
        .any(|kw| matches!(kw, crate::effects::KeywordAbility::Improvise))
}

/// Return the currently-granted kicker cost for `object_id`, if any
/// (CR 702.32). Walks [`GameState::effective_keywords`] so layer-6
/// grants of `Kicker` are honored. Returns the first Kicker keyword
/// found; multikicker (multiple Kicker keywords on one card) is not
/// yet modeled — when it lands, enumerate each via a sibling
/// `all_kicker_costs_for` helper like flashback does.
pub(crate) fn kicker_cost_for(
    state: &GameState,
    object_id: ObjectId,
) -> Option<crate::mana::ManaCost> {
    state.effective_keywords(object_id).into_iter()
        .find_map(|kw| match kw {
            crate::effects::KeywordAbility::Kicker(cost) => Some(cost),
            _ => None,
        })
}

/// Layer-aware check: does `object_id` currently have a Kicker
/// keyword (CR 702.32)? Distinct from [`kicker_cost_for`] in that
/// it only reports presence — callers that need the cost should use
/// the cost helper directly.
pub(crate) fn has_kicker(state: &GameState, object_id: ObjectId) -> bool {
    state.effective_keywords(object_id).into_iter()
        .any(|kw| matches!(kw, crate::effects::KeywordAbility::Kicker(_)))
}

/// Return the currently-granted madness cost for `object_id`, if
/// any (CR 702.34). Walks [`GameState::effective_keywords`] so
/// layer-6 grants compose with printed madness. First match wins;
/// the rules don't define multiple-madness semantics and no printed
/// card has it.
pub(crate) fn madness_cost_for(
    state: &GameState,
    object_id: ObjectId,
) -> Option<crate::mana::ManaCost> {
    state.effective_keywords(object_id).into_iter()
        .find_map(|kw| match kw {
            crate::effects::KeywordAbility::Madness(cost) => Some(cost),
            _ => None,
        })
}

/// Return the Adventure face of the card backing `object_id`, if the
/// card has one (CR 715). Dispatches on the registry definition, not
/// on the object's current characteristics — the field is
/// "characteristic of the printed card" and does not participate in
/// the layer system (no card grants another card an Adventure).
///
/// Today's only multi-face relationship is Adventure;
/// [`crate::registry::AlternateFace::as_adventure`] filters out the
/// Split / MDFC / Transform variants when they land.
pub(crate) fn adventure_face_of<'r>(
    state: &GameState,
    registry: &'r crate::registry::CardRegistry,
    object_id: ObjectId,
) -> Option<&'r crate::registry::CardFace> {
    let card_id = state.objects.get(object_id)?.card_id;
    let def = registry.get(card_id)?;
    def.alternate_face.as_ref()?.as_adventure()
}

/// Return the MDFC back face of the card backing `object_id`, if the
/// card has one (CR 712.4). Sibling of [`adventure_face_of`]; MDFC
/// and Adventure relationships are mutually exclusive on a
/// [`CardDefinition`] today (each card has at most one
/// [`crate::registry::AlternateFace`]), so a match on either
/// helper's `Some` return tells dispatch code which cast path
/// applies.
pub(crate) fn mdfc_back_face_of<'r>(
    state: &GameState,
    registry: &'r crate::registry::CardRegistry,
    object_id: ObjectId,
) -> Option<&'r crate::registry::CardFace> {
    let card_id = state.objects.get(object_id)?.card_id;
    let def = registry.get(card_id)?;
    def.alternate_face.as_ref()?.as_mdfc()
}

/// Return the right half of the Split card backing `object_id`, if
/// the card has a Split relationship (CR 711). Mirrors
/// [`adventure_face_of`] and [`mdfc_back_face_of`]; the three
/// `AlternateFace` variants are mutually exclusive on a single card
/// (at most one relationship per [`CardDefinition`]).
pub(crate) fn split_right_face_of<'r>(
    state: &GameState,
    registry: &'r crate::registry::CardRegistry,
    object_id: ObjectId,
) -> Option<&'r crate::registry::CardFace> {
    let card_id = state.objects.get(object_id)?.card_id;
    let def = registry.get(card_id)?;
    def.alternate_face.as_ref()?.as_split()
}


/// Valid [`crate::actions::ConvokePayment`] options for a creature
/// with the given characteristics (CR 702.51b).
///
/// Every creature can pay `Generic`. Colored creatures additionally
/// offer `Color(c)` for each of their colors. Colorless creatures
/// return just `[Generic]`. Multicolored creatures return one
/// `Color(c)` per color in addition to `Generic`, so a G/U creature
/// returns `[Generic, Color(Green), Color(Blue)]` (order is stable
/// but not ManaColor-enum order — iterate the creature's ColorSet).
pub(crate) fn convoke_eligible_payments(
    chars: &crate::objects::Characteristics,
) -> Vec<crate::actions::ConvokePayment> {
    use crate::actions::ConvokePayment;
    let mut out = vec![ConvokePayment::Generic];
    for color in chars.colors.iter() {
        out.push(ConvokePayment::Color(color.to_mana()));
    }
    out
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
    // legal_actions shouldn't have emitted such an action. Snapshot
    // `card_id` up front so resolution-time dispatch survives the
    // source-object re-id'ing that happens when a cost like
    // sacrifice-self or discard-self moves the card before the
    // ability resolves (CR 400.7).
    let (card_id, is_mana_ability, is_loyalty_ability, tap, sacrifice, life) = {
        let Some(obj) = state.objects.get(source) else { return; };
        let Some(def) = registry.get(obj.card_id) else { return; };
        let Some(ability) = def.activated_abilities.get(ability_index) else { return; };
        (
            obj.card_id,
            ability.is_mana_ability,
            ability.is_loyalty_ability,
            ability.cost.tap,
            ability.cost.sacrifice,
            ability.cost.life,
        )
    };
    // CR 606.3 — loyalty activation: belt-and-suspenders validation in
    // addition to legal_actions gating. Rejects controller mismatch,
    // non-empty stack, or an already-activated PW. `legal_actions`
    // filters these upstream; this mirrors the style of the rest of
    // apply_activate_ability (silent rejection on bogus input).
    if is_loyalty_ability {
        let Some(obj) = state.objects.get(source) else { return; };
        if obj.controller != controller { return; }
        if !state.stack_is_empty() { return; }
        if state.loyalty_activated_this_turn.contains(&source) { return; }
    }

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
        // using the stack. Dispatch the effect directly via the
        // snapshotted `card_id` so the dispatch is robust against
        // the source object having moved zones (won't happen for
        // mana abilities in practice, but the snapshot is harmless
        // and keeps the activation-from-non-battlefield story
        // internally consistent).
        let ctx = crate::registry::ActivationContext {
            source,
            controller,
            ability_index,
            targets,
            x_value: None,
        };
        let effects: Vec<crate::effects::Effect> = match registry.get(card_id) {
            Some(d) => d.activated_abilities.get(ability_index)
                .map(|a| (a.effect)(state, &ctx, registry))
                .unwrap_or_default(),
            None => Vec::new(),
        };
        for effect in effects {
            effect.execute(state);
        }
    } else {
        // Non-mana activated abilities go on the stack (CR 602.2).
        // `card_id` + `ability_index` together look the ability back
        // up at resolution time, independent of whether the source
        // object still exists at `source` (it may have moved to
        // graveyard as part of the activation cost — cycling,
        // sacrifice-self abilities).
        let entry_id = state.allocate_object_id();
        let entry = crate::stack::StackEntry::new_activated_ability(
            entry_id,
            source,
            controller,
            card_id,
            ability_index as crate::types::AbilityId,
            /*text=*/ String::new(),
            targets,
            Vec::new(),
            None,
        );
        state.push_stack_entry(entry);
        // CR 115 — activated abilities that target emit BecomesTarget
        // after landing on the stack, same hook as cast spells. Ward
        // treats spells and activated abilities uniformly.
        emit_becomes_target_events(state, entry_id, controller);
    }

    // CR 606.3 — mark the PW as having had a loyalty ability activated
    // this turn. Done after costs clear and the ability is launched so
    // rejected activations don't burn the once-per-turn allowance.
    if is_loyalty_ability {
        state.loyalty_activated_this_turn.insert(source);
    }

    state.priority.record_action();
}

fn apply_play_land(
    state: &mut GameState,
    registry: &CardRegistry,
    object_id: ObjectId,
    mdfc_back: bool,
) {
    let controller = state.priority_player();

    // CR 712.4 — MDFC land back face. Before moving to the
    // battlefield, swap the object's characteristics to the back
    // face so the arriving permanent is the land with its own type
    // line, color (typically none for MDFC lands), and printed mana
    // abilities. The registry is the source of truth for the back
    // face; the arena object holds the live copy. Snapshot the
    // pre-swap front-face chars on the object for the CR 712.2b
    // revert in `swap_to_zone_reid` when this land eventually
    // leaves the battlefield.
    if mdfc_back {
        let back_chars = state.objects.get(object_id)
            .and_then(|o| registry.get(o.card_id))
            .and_then(|def| def.alternate_face.as_ref())
            .and_then(|af| af.as_mdfc())
            .map(|f| f.characteristics.clone());
        let Some(back_chars) = back_chars else {
            // Agent requested an MDFC land play on a card with no
            // MDFC back face — reject silently, matching the rest of
            // this function.
            return;
        };
        if !back_chars.types.is_land() { return; }
        if let Some(obj) = state.objects.get_mut(object_id) {
            let prior = std::mem::replace(&mut obj.characteristics, back_chars);
            obj.default_face_characteristics = Some(prior);
            obj.visible_face = 1;
        }
    }

    state.player_mut(controller).land_plays_remaining =
        state.player(controller).land_plays_remaining.saturating_sub(1);
    // Land play is NOT a spell — no stack entry. Direct ETB. Re-id
    // on the move means we must address the post-move object via the
    // returned new id.
    let Some(new_id) = state.move_object_to_zone(
        object_id, Zone::Battlefield, MoveCause::PlayLand) else {
        state.priority.record_action();
        return;
    };
    // CR 305.2 — playing a land gives control of it to the player
    // who played it.
    if let Some(obj) = state.objects.get_mut(new_id) {
        obj.controller = controller;
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

        // --- YesNo mid-resolution: cascade may-cast (CR 702.85) ------
        (
            ChoiceKind::YesNo { .. },
            ChoiceContext::ResolvingStack(_),
            ChoiceResponse::YesNo { answer },
        ) if state.pending_cascade.is_some() => {
            let pc = state.pending_cascade.take().unwrap();
            if *answer {
                // Yes: cast the hit for free; other_exiled goes to
                // the bottom in random order.
                cast_cascade_hit(state, pc.controller, pc.hit);
                crate::effects::cascade_shuffle_to_bottom(
                    state, pc.controller, pc.other_exiled);
            } else {
                // No: hit joins the rest on the bottom.
                let mut all = pc.other_exiled;
                all.push(pc.hit);
                crate::effects::cascade_shuffle_to_bottom(
                    state, pc.controller, all);
            }
        }

        // --- ChooseTargets mid-resolution (storm copies, CopySpell) --
        // The pushing effect stashed an
        // `ApplyTargetsToStackEntry` follow-up naming the entry
        // whose targets to overwrite. Clears the requirements slot.
        (
            ChoiceKind::ChooseTargets { .. },
            ChoiceContext::ResolvingStack(_),
            ChoiceResponse::ChooseTargets { selection },
        ) => {
            state.pending_target_requirements = None;
            let follow_up = state.pending_choice_follow_up.take()
                .expect("apply_resolution_choice: ChooseTargets needs \
                         a pending_choice_follow_up");
            match follow_up {
                crate::actions::ChoiceFollowUp::ApplyTargetsToStackEntry {
                    entry_id,
                } => {
                    if let Some(entry) =
                        state.stack.iter_mut().find(|e| e.id == entry_id)
                    {
                        entry.targets = selection.clone();
                    }
                }
                other => panic!(
                    "ChooseTargets dispatch: unexpected follow-up {other:?}"),
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

/// Cast `hit` from exile for free (CR 702.85 cascade). No targets or
/// modes are selected (Phase 2-A simplification, same as
/// `cast_from_zone_free`); a targeted cascaded spell resolves with
/// empty targets and gets rules-countered. When resolution-time
/// ChooseTargets lands for the cascade cast path, thread it through
/// here.
fn cast_cascade_hit(state: &mut GameState, player: PlayerId, hit: ObjectId) {
    let Some(obj) = state.objects.get(hit) else { return; };
    if obj.zone != crate::zones::Zone::Exile { return; }
    if obj.is_land() { return; }
    // Limitation: the cascade may-cast path doesn't currently stamp
    // `enters_with` on the stack entry, so a cascaded permanent with
    // fixed "enters with N counters" won't receive them. X-keyed
    // enters_with is still safe — cascade casts pay no mana, so
    // `x_value` is `None` either way. Fixing this requires threading
    // `&CardRegistry` through `apply_resolution_choice`; deferred.
    let entry_id = state.announce_spell_on_stack(
        hit, player, crate::targets::TargetSelection::new(),
        Vec::new(), None, Vec::new());
    state.emit_spell_cast(entry_id);
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

/// Apply the decline consequence from a PayOrDecline. For Ward
/// (CR 702.21a) the decline path counters the targeting spell/ability
/// whose stack-entry id was stamped into
/// [`DeclineConsequence::CounterStackEntry`] at prompt-push time.
///
/// The Ward trigger itself (the stack entry currently resolving)
/// drains normally via [`resume_parked_resolution`] — its
/// `remaining_effects` are empty, so `execute_effects_or_park`
/// finalizes immediately after this returns.
fn apply_decline_consequence(
    state: &mut GameState,
    consequence: crate::actions::DeclineConsequence,
) {
    use crate::actions::DeclineConsequence;
    match consequence {
        DeclineConsequence::CounterStackEntry(entry_id) => {
            let Some(entry) = state.remove_stack_entry_by_id(entry_id)
                else { return; };
            if entry.is_spell() {
                state.counter_resolved_spell(entry);
            } else {
                state.counter_resolved_ability(entry);
            }
        }
        DeclineConsequence::SkipEffect => {
            // "May" effects: no cleanup needed; resume_parked_resolution
            // will advance to the next effect.
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
                // Madness-aware discard. The pre-move id is what
                // `Discarded` carries (so trigger filters that
                // compare by id resolve via LKI the same way whether
                // the destination was graveyard or exile).
                let _ = state.discard_object(player, *id, MoveCause::Cost);
            }
        }
        ChoiceFollowUp::ApplyTargetsToStackEntry { .. } => {
            // PickCards path can never produce this follow-up (it pairs
            // with ChooseTargets responses, dispatched separately).
            panic!("apply_choice_follow_up: ApplyTargetsToStackEntry is for \
                    ChooseTargets, not PickCards");
        }
        ChoiceFollowUp::GrantFlashbackEqualToOwnManaCost { source, duration } => {
            // Per picked card, read its printed mana cost from the
            // card's base characteristics (not computed, since
            // flashback's granted cost is "equal to its mana cost"
            // referring to the printed value) and install a layer-6
            // grant of Flashback(cost) keyed on the picked ObjectId.
            // If the card leaves the graveyard and re-enters as a
            // new object per CR 400.7, the grant's target ObjectId
            // no longer resolves to it — which is the CR-faithful
            // behavior.
            for id in chosen {
                let Some(obj) = state.objects.get(*id) else { continue; };
                let Some(printed_cost) = obj.characteristics.mana_cost.clone()
                    else { continue; };
                state.add_continuous_effect(
                    crate::layers::ContinuousEffect::grant_keyword(
                        source, *id,
                        crate::effects::KeywordAbility::Flashback(printed_cost),
                        duration,
                    ));
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

        // 3. Synthesized Ward triggers (CR 702.21a). Ward lives on
        //    [`crate::objects::Characteristics::keywords`] rather than as a
        //    registered [`crate::triggers::TriggeredAbilityDef`], so we
        //    synthesize a PendingTrigger here for any battlefield object
        //    with Ward whose id matches this `BecomesTarget` event and
        //    whose controller is opposed to the event's caster. Dispatch
        //    at resolution time uses the [`WARD_TRIGGER_ID`] sentinel.
        if let crate::events::GameEvent::BecomesTarget { target, controller: caster, .. } = event {
            if let Some(ward_obj) = state.objects.get(*target) {
                if ward_obj.zone.is_battlefield()
                    && ward_obj.controller != *caster
                {
                    let has_ward = state.effective_keywords(*target).iter()
                        .any(|kw| matches!(kw,
                            crate::effects::KeywordAbility::Ward(_)));
                    if has_ward {
                        pending.push(crate::triggers::PendingTrigger {
                            source: *target,
                            trigger_id: WARD_TRIGGER_ID,
                            controller: ward_obj.controller,
                            trigger_event: event.clone(),
                        });
                    }
                }
            }
        }

        // 4. Auto-applied keyword triggers that bypass the stack in
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
            // CR 510.1c — if any attacker needs a player-chosen
            // distribution in the first-strike pass, pause for the
            // active player to submit it; otherwise deal immediately.
            if state.needs_damage_assignment(
                crate::combat::PendingDamagePass::FirstStrike)
            {
                let combat = state.combat.as_mut().unwrap();
                combat.damage_assignments.clear();
                combat.pending_damage_assignment =
                    Some(crate::combat::PendingDamagePass::FirstStrike);
                // Don't advance the step; compute_next_decision will
                // yield a DistributeDamage decision. Step advances
                // inside the AssignCombatDamage handler.
            } else {
                state.deal_first_strike_damage();
                state.turn.step = Step::CombatDamageRegular;
                emit_step_begins(state);
            }
        }
        (Phase::Combat, Step::CombatDamageRegular) => {
            if state.needs_damage_assignment(
                crate::combat::PendingDamagePass::Regular)
            {
                let combat = state.combat.as_mut().unwrap();
                combat.damage_assignments.clear();
                combat.pending_damage_assignment =
                    Some(crate::combat::PendingDamagePass::Regular);
            } else {
                state.deal_combat_damage();
                state.turn.step = Step::EndCombat;
                emit_step_begins(state);
            }
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

/// CR 510.1c handler: validate + post the player-chosen damage
/// distributions, run the damage pass, advance the step. Rejects
/// (no-op, stays pending) if any distribution is illegal or required
/// attackers are missing, so the agent can re-submit.
fn apply_assign_combat_damage(
    state: &mut GameState,
    distributions: Vec<crate::combat::DamageAssignment>,
) {
    let Some(combat) = state.combat.as_ref() else { return; };
    let Some(pass) = combat.pending_damage_assignment else { return; };

    // Every attacker requiring CR 510.1c assignment must be present.
    let required: Vec<ObjectId> = state
        .attackers_needing_damage_assignment(pass);
    let provided: crate::collections::HashSet<ObjectId> =
        distributions.iter().map(|d| d.attacker).collect();
    if !required.iter().all(|a| provided.contains(a)) { return; }

    // Validate every distribution before posting any — keeps the
    // reject path atomic.
    for d in &distributions {
        if !state.is_legal_damage_assignment(d.attacker, &d.distribution) {
            return;
        }
    }
    for d in distributions {
        state.set_damage_assignment(d);
    }

    // Deal the pass.
    match pass {
        crate::combat::PendingDamagePass::FirstStrike =>
            state.deal_first_strike_damage(),
        crate::combat::PendingDamagePass::Regular =>
            state.deal_combat_damage(),
    }

    // Advance step + clear pending state.
    if let Some(combat) = state.combat.as_mut() {
        combat.pending_damage_assignment = None;
        combat.damage_assignments.clear();
    }
    state.turn.step = match pass {
        crate::combat::PendingDamagePass::FirstStrike => Step::CombatDamageRegular,
        crate::combat::PendingDamagePass::Regular => Step::EndCombat,
    };
    emit_step_begins(state);
    grant_priority_for_current_step(state);
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
    // CR 702.40a: storm count resets between turns.
    state.storm_count = 0;
    // CR 606.3 — loyalty-activation ledger is per-turn; clear it so
    // each PW's controller may activate exactly one loyalty ability
    // next turn.
    state.loyalty_activated_this_turn.clear();
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
    let requirements = resolution_target_requirements(state, &entry, registry);
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
            execute_effects_or_park(state, entry, effects, is_spell);
        }
    }
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
    execute_effects_or_park(
        state, parked.entry, parked.remaining_effects, parked.is_spell);
}

/// Look up the target-requirement vector for a stack entry from the
/// registry. Returns an empty vector for unregistered cards or
/// abilities that don't target.
fn resolution_target_requirements(
    state: &GameState,
    entry: &crate::stack::StackEntry,
    registry: &CardRegistry,
) -> Vec<crate::targets::TargetRequirement> {
    match &entry.kind {
        crate::stack::StackEntryKind::Spell { card_id, .. } => {
            // Modal-aware: CR 608.2b rechecks each chosen target against
            // the requirement declared on the clause that summoned it.
            // Non-modal spells fall back to the flat list.
            //
            // Adventure casts dispatch on the Adventure face's spell
            // ability rather than the main (creature) one — modal
            // adventures aren't printed today, but routing through
            // `effective_target_requirements` still degrades correctly.
            let def = registry.get(*card_id);
            let sa = match entry.cast_modifier {
                crate::actions::CastModifier::Adventure =>
                    def.and_then(|d| d.alternate_face.as_ref())
                        .and_then(|af| af.as_adventure())
                        .and_then(|f| f.spell_ability.as_ref()),
                crate::actions::CastModifier::MdfcBack =>
                    def.and_then(|d| d.alternate_face.as_ref())
                        .and_then(|af| af.as_mdfc())
                        .and_then(|f| f.spell_ability.as_ref()),
                crate::actions::CastModifier::SplitRight =>
                    def.and_then(|d| d.alternate_face.as_ref())
                        .and_then(|af| af.as_split())
                        .and_then(|f| f.spell_ability.as_ref()),
                _ => def.and_then(|d| d.spell_ability.as_ref()),
            };
            sa.map(|sa| crate::registry::effective_target_requirements(sa, &entry.modes))
                .unwrap_or_default()
        }
        crate::stack::StackEntryKind::ActivatedAbility {
            card_id, ability_id, ..
        } => {
            // Look up the ability via the stack-entry-snapshotted
            // card_id so CR 608.2b recheck works even if the source
            // object moved zones as part of the activation cost
            // (sacrifice-self, cycling's discard-self). The
            // requirements themselves are on the card def, not the
            // object, so this is layer-independent.
            let _ = state;
            registry.get(*card_id)
                .and_then(|def| def.activated_abilities
                    .get(*ability_id as usize))
                .map(|a| a.target_requirements.clone())
                .unwrap_or_default()
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
            // Adventure and MDFC back casts resolve via the
            // alternate face's spell ability (CR 715.2, CR 712.4).
            // Everything else uses the main face.
            let sa = match entry.cast_modifier {
                crate::actions::CastModifier::Adventure =>
                    def.alternate_face.as_ref()
                        .and_then(|af| af.as_adventure())
                        .and_then(|f| f.spell_ability.as_ref()),
                crate::actions::CastModifier::MdfcBack =>
                    def.alternate_face.as_ref()
                        .and_then(|af| af.as_mdfc())
                        .and_then(|f| f.spell_ability.as_ref()),
                crate::actions::CastModifier::SplitRight =>
                    def.alternate_face.as_ref()
                        .and_then(|af| af.as_split())
                        .and_then(|f| f.spell_ability.as_ref()),
                _ => def.spell_ability.as_ref(),
            };
            let Some(sa) = sa else { return Vec::new(); };
            (sa.effect)(state, entry, registry)
        }
        crate::stack::StackEntryKind::ActivatedAbility {
            card_id, ability_id, ..
        } => {
            // Re-dispatch through the registry via the stack-entry-
            // snapshotted `card_id`. `ability_id` encodes the
            // activated-ability index directly. The source object's
            // id is kept in `entry.source` for effects that reference
            // the source (damage-from-source etc.) — even if the
            // object has moved zones, the id is the stable handle
            // the effect was authored against.
            let Some(def) = registry.get(*card_id) else { return Vec::new(); };
            let idx = *ability_id as usize;
            let Some(ability) = def.activated_abilities.get(idx) else { return Vec::new(); };
            let ctx = crate::registry::ActivationContext {
                source: entry.source,
                controller: entry.controller,
                ability_index: idx,
                targets: entry.targets.clone(),
                x_value: entry.x_value,
            };
            (ability.effect)(state, &ctx, registry)
        }
        crate::stack::StackEntryKind::TriggeredAbility {
            trigger_id, trigger_event, ..
        } => {
            // Ward (CR 702.21a) is a keyword-born trigger; its stack
            // entry carries the [`WARD_TRIGGER_ID`] sentinel and no
            // per-card [`TriggeredAbilityDef`] exists. Dispatch to
            // the built-in handler.
            if *trigger_id == WARD_TRIGGER_ID {
                return ward_trigger_resolve(state, entry.source, trigger_event);
            }
            let source = entry.source;
            let Some(obj) = state.objects.get(source)
                .or_else(|| state.lki.get(&source))
                else { return Vec::new(); };
            let Some(def) = registry.get(obj.card_id) else { return Vec::new(); };
            let Some(ability) = def.triggered_abilities.iter()
                .find(|a| a.id == *trigger_id)
                else { return Vec::new(); };
            let pt = crate::triggers::PendingTrigger {
                source,
                trigger_id: *trigger_id,
                controller: entry.controller,
                trigger_event: trigger_event.clone(),
            };
            (ability.effect)(state, &pt, registry)
        }
    }
}

/// Resolution effect list for a synthesized Ward trigger. Reads the
/// Ward cost from the source's effective keywords (layers-aware, so
/// granted-Ward is honored) and returns a single
/// [`Effect::WardPrompt`] keyed to the targeting spell/ability's
/// stack entry. Returns an empty list if the source no longer has
/// Ward (e.g. granted via a temporary layer that expired between
/// trigger-announcement and resolution).
fn ward_trigger_resolve(
    state: &GameState,
    source: ObjectId,
    trigger_event: &crate::events::GameEvent,
) -> Vec<crate::effects::Effect> {
    use crate::effects::KeywordAbility;
    let crate::events::GameEvent::BecomesTarget {
        source: target_entry, controller: caster, ..
    } = trigger_event else { return Vec::new(); };
    // Re-read Ward cost at resolution time (layers may have changed).
    let cost = state.effective_keywords(source).into_iter()
        .find_map(|kw| match kw {
            KeywordAbility::Ward(c) => Some(c),
            _ => None,
        });
    let Some(cost) = cost else { return Vec::new(); };
    vec![crate::effects::Effect::WardPrompt {
        caster: *caster,
        cost,
        counter_target: *target_entry,
    }]
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
                // Route through the Madness-aware helper (CR 702.34a):
                // a card with Madness on its characteristics goes to
                // exile with `madness_pending=true` instead of
                // graveyard, opening the madness-cast window.
                state.discard_object(player, *id, MoveCause::Cost);
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
            A::AddCounters { source, kind, count } => {
                // Route through place_counters so Doubling Season and
                // friends intercept — `+1` activations should stack
                // exactly like ETB counter-placement does.
                state.place_counters(
                    crate::replacement::CounterTarget::Object(*source),
                    *kind,
                    *count);
            }
            A::RevealCard(_id) => {
                // Reveals are informational; no state change until
                // Phase 2 wires the observer/UI path. The known_cards
                // HashSet on PlayerState is where a reveal would
                // ultimately get recorded.
            }
            A::Kicker => {
                // Kicker's cost is mana — already paid as part of the
                // pre-summed `mana_payment` at the call site (CR
                // 702.32a). The flag's observable effect is downstream:
                // `apply_cast_spell` stamps `StackEntry::kicked` from
                // the presence of this variant so resolution-time
                // effect fns can branch on the kicked rider.
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

/// Sentinel [`crate::types::TriggerId`] used for the synthesized Ward
/// trigger (CR 702.21a). Ward lives on an object's
/// [`crate::objects::Characteristics::keywords`] list rather than as
/// a per-card [`crate::triggers::TriggeredAbilityDef`], so the
/// collector fabricates [`crate::triggers::PendingTrigger`] entries
/// with this id and [`resolution_effects`] dispatches on the sentinel
/// to the built-in Ward handler. `u32::MAX` chosen so real card
/// trigger ids (assigned from 1 upward per card) never collide.
pub(crate) const WARD_TRIGGER_ID: crate::types::TriggerId = u32::MAX;

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
                card_id, def.initial_characteristics().clone());
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
        // CR 510.1c — a pending distribution decision preempts the
        // normal combat-phase yield shape.
        if let Some(pass) = combat.pending_damage_assignment {
            let attackers = state.attackers_needing_damage_assignment(pass);
            return EngineYield::PendingDecision {
                player: state.active_player(),
                legal_actions: legal_actions(state, registry),
                context: DecisionContext::DistributeDamage { attackers },
            };
        }
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
            CombatPhase::OrderBlockers => {
                // CR 509.2 — active player orders each multi-blocked
                // attacker's blockers before priority opens in
                // PostDeclareBlockers.
                let multi: Vec<ObjectId> = combat.attackers.iter()
                    .filter(|a| a.blocked_by.len() >= 2)
                    .map(|a| a.object_id)
                    .collect();
                return EngineYield::PendingDecision {
                    player: state.active_player(),
                    legal_actions: legal_actions(state, registry),
                    context: DecisionContext::OrderBlockers { attackers: multi },
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
    // Ward — CR 702.21a, Phase 2-B.
    //
    // Ward fires as a synthesized triggered ability ([`WARD_TRIGGER_ID`])
    // collected from [`GameEvent::BecomesTarget`] events that
    // [`apply_cast_spell`] and [`apply_activate_ability`] emit after
    // their target-bearing stack entry lands. The Ward trigger sits on
    // the stack above the targeting entry and can be countered
    // independently (Stifle). Resolution pushes PayOrDecline to the
    // caster; a decline counters the original stack entry.
    //
    // Tests below drive this without going through Action::CastSpell:
    // we manually put a spell on the stack, emit BecomesTarget, run
    // `run_sba_and_triggers` to push the Ward trigger, then resolve
    // and answer.
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
            modal: None,
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

    /// Simulate the "spell targets Ward creature" event chain without
    /// going through Action::CastSpell: caller has already pushed the
    /// targeting stack entry. Emits BecomesTarget for the target and
    /// runs the SBA/trigger loop so the Ward trigger lands on top of
    /// the original entry. Returns when the synthesized Ward trigger
    /// is in place (or, if Ward doesn't fire, when the settle loop
    /// reaches a no-op).
    fn emit_becomes_target_and_settle(
        s: &mut GameState,
        registry: &CardRegistry,
        target: ObjectId,
        source: ObjectId,
        controller: PlayerId,
    ) {
        s.emit(crate::events::GameEvent::BecomesTarget {
            target, source, controller,
        });
        run_sba_and_triggers(s, registry);
    }

    #[test]
    fn ward_paid_lets_spell_resolve() {
        use crate::mana::ManaCost;
        use crate::types::ManaColor;
        let mut s = GameState::new(2, 0);
        let mut registry = CardRegistry::new();
        let card = register_noop_target_instant(&mut registry);
        let victim = pay_cost_creature(&mut s, 1, ManaCost::parse("{2}").unwrap());
        // Give player 0 two colorless mana in pool to pay ward.
        s.player_mut(0).mana_pool.add(crate::mana::ManaUnit::plain(ManaColor::Red, 0));
        s.player_mut(0).mana_pool.add(crate::mana::ManaUnit::plain(ManaColor::Red, 0));
        let spell = put_target_spell_on_stack(&mut s, 0, card, victim);
        emit_becomes_target_and_settle(&mut s, &registry, victim, spell, 0);
        assert_eq!(s.stack_size(), 2,
            "Ward trigger landed on top of the targeting spell");

        // Resolve the Ward trigger — it pushes PayOrDecline on the caster.
        resolve_top_of_stack(&mut s, &registry);
        let pc = s.pending_choice.as_ref().unwrap();
        assert_eq!(pc.choosing_player, 0, "caster pays or declines");
        assert!(matches!(pc.kind, ChoiceKind::PayOrDecline { .. }));
        let pc_id = pc.id;
        let before_pool = s.player(0).mana_pool.total();
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::PayOrDecline { pay: true });

        // Pool debited by 2 generic; Ward trigger drained and
        // finalized. The original spell is still on the stack — the
        // caller is responsible for resolving it (in a real game,
        // that's the next priority round).
        assert_eq!(s.player(0).mana_pool.total(), before_pool - 2);
        assert!(s.pending_choice.is_none());
        assert!(s.pending_resolution.is_none());
        assert_eq!(s.stack_size(), 1,
            "original spell remains on the stack after Ward resolves");
    }

    #[test]
    fn ward_declined_counters_spell() {
        use crate::mana::ManaCost;
        let mut s = GameState::new(2, 0);
        let mut registry = CardRegistry::new();
        let card = register_noop_target_instant(&mut registry);
        let victim = pay_cost_creature(&mut s, 1, ManaCost::parse("{2}").unwrap());
        let spell = put_target_spell_on_stack(&mut s, 0, card, victim);
        emit_becomes_target_and_settle(&mut s, &registry, victim, spell, 0);

        resolve_top_of_stack(&mut s, &registry);

        let pc_id = s.pending_choice.as_ref().unwrap().id;
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::PayOrDecline { pay: false });

        // Spell countered via CounterStackEntry; Ward trigger itself
        // drained and finalized. Stack is empty.
        assert!(s.objects.get(spell).is_none(),
            "stack id is consumed on re-id into graveyard");
        assert_eq!(s.zone_count(crate::zones::Zone::Graveyard(0)), 1);
        assert!(s.event_log.iter().any(|e|
            matches!(e, crate::events::GameEvent::SpellCountered { object_id }
                if *object_id == spell)));
        assert!(s.pending_choice.is_none());
        assert!(s.pending_resolution.is_none());
        assert!(s.stack_is_empty());
    }

    #[test]
    fn ward_does_not_trigger_for_controller_own_spell() {
        use crate::mana::ManaCost;
        let mut s = GameState::new(2, 0);
        let mut registry = CardRegistry::new();
        let card = register_noop_target_instant(&mut registry);
        // Caster 0 targets their own Ward creature — no trigger.
        let own = pay_cost_creature(&mut s, 0, ManaCost::parse("{2}").unwrap());
        let spell = put_target_spell_on_stack(&mut s, 0, card, own);
        emit_becomes_target_and_settle(&mut s, &registry, own, spell, 0);

        assert_eq!(s.stack_size(), 1,
            "no Ward trigger pushed — stack holds only the original spell");
        assert!(s.pending_choice.is_none());
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
            modal: None,
            effect: noop_target_effect,
        });
        registry.register(def)
    }

    /// A spell targeting two different Ward creatures produces two
    /// Ward triggers, each its own stack object. The active-player
    /// (triggerer's controller) resolves LIFO: the second trigger
    /// pushed lands on top and resolves first. Paying both lets the
    /// spell through; declining either counters it.
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
        // Two-target spell on the stack.
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

        // Emit both BecomesTarget events, then settle: two Ward
        // triggers land on the stack (one per target).
        s.emit(crate::events::GameEvent::BecomesTarget {
            target: a, source: spell_id, controller: 0,
        });
        s.emit(crate::events::GameEvent::BecomesTarget {
            target: b, source: spell_id, controller: 0,
        });
        run_sba_and_triggers(&mut s, &registry);
        assert_eq!(s.stack_size(), 3,
            "spell + two Ward triggers on the stack");

        // Resolve first (top) Ward trigger; pay.
        resolve_top_of_stack(&mut s, &registry);
        let pc_id = s.pending_choice.as_ref().unwrap().id;
        apply_resolution_choice(&mut s, pc_id,
            ChoiceResponse::PayOrDecline { pay: true });

        // Resolve second Ward trigger; pay.
        resolve_top_of_stack(&mut s, &registry);
        let pc2_id = s.pending_choice.as_ref()
            .expect("second Ward prompt").id;
        assert!(pc2_id > pc_id);
        apply_resolution_choice(&mut s, pc2_id,
            ChoiceResponse::PayOrDecline { pay: true });

        assert!(s.pending_choice.is_none());
        assert!(s.pending_resolution.is_none());
        assert_eq!(s.stack_size(), 1,
            "both Ward triggers drained; original spell remains");
    }

    /// CR 702.21a + Stifle (CR 701.47 / printed as "counter target
    /// activated or triggered ability"): the Ward trigger is its own
    /// stack object; countering it with Stifle skips the payment and
    /// the targeting spell resolves unimpeded. Simulated here by
    /// directly countering the Ward trigger via the stack API (we
    /// don't register a Stifle card — the gap pinned is the trigger
    /// *shape*, not Stifle the spell).
    #[test]
    fn ward_trigger_countered_by_stifle_bypasses_payment() {
        use crate::mana::ManaCost;
        let mut s = GameState::new(2, 0);
        let mut registry = CardRegistry::new();
        let card = register_noop_target_instant(&mut registry);
        let victim = pay_cost_creature(&mut s, 1, ManaCost::parse("{2}").unwrap());
        // Caster 0 deliberately has NO mana for the Ward — if the
        // trigger resolves without being Stifled, the decline path
        // would counter the spell. Stifle intervenes first.
        let spell = put_target_spell_on_stack(&mut s, 0, card, victim);
        emit_becomes_target_and_settle(&mut s, &registry, victim, spell, 0);
        assert_eq!(s.stack_size(), 2,
            "Ward trigger landed on top of the targeting spell");

        // "Stifle" the Ward trigger: remove it from the stack and
        // counter it. This mirrors what resolving a real Stifle spell
        // targeting the trigger would do via
        // `apply_decline_consequence` / its counter-resolved path.
        let ward_trigger_id = s.stack.last().unwrap().id;
        let entry = s.remove_stack_entry_by_id(ward_trigger_id)
            .expect("Ward trigger on stack");
        s.counter_resolved_ability(entry);

        // Ward trigger gone, original spell intact.
        assert_eq!(s.stack_size(), 1);
        assert!(s.pending_choice.is_none());
        assert!(s.event_log.iter().any(|e|
            matches!(e, crate::events::GameEvent::SpellCountered { object_id }
                if *object_id == ward_trigger_id)));

        // Resolve the original spell — it lands without any Ward
        // payment prompt because the Ward trigger no longer exists.
        resolve_top_of_stack(&mut s, &registry);
        assert!(s.pending_choice.is_none(),
            "no Ward prompt: trigger was countered before it resolved");
        assert!(s.stack_is_empty(),
            "spell resolved cleanly");
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
            _: &crate::registry::CardRegistry,
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
            _: &crate::registry::CardRegistry,
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
            Action::PlayLand { object_id: land_id, mdfc_back: false },
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
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions::default(),
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

    /// Register a stub land and return `size` copies of its CardId.
    /// The tests that use this don't care about the card's abilities —
    /// only that a registered card can be used to build a deck.
    fn register_mountain_deck(
        registry: &mut crate::registry::CardRegistry,
        size: u32,
    ) -> Vec<crate::types::CardId> {
        let id = register_stub_land(registry);
        vec![id; size as usize]
    }

    fn register_stub_land(
        registry: &mut crate::registry::CardRegistry,
    ) -> crate::types::CardId {
        let name = registry.interner_mut().intern("Mountain");
        let chars = crate::objects::Characteristics {
            name,
            types: crate::types::TypeLine::LAND.into(),
            ..Default::default()
        };
        registry.register(
            crate::registry::CardDefinition::new(name, chars))
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
        let id = register_stub_land(&mut registry);
        let deck = vec![id; 60];
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

    // =====================================================================
    // Storm & Cascade (CR 702.40 / 702.85)
    // =====================================================================

    /// Register a Grapeshot-shape spell: {R} instant, "deal 1 damage to
    /// target player" + storm. (We use target-player rather than
    /// "any target" to keep the test on the canonical Player branch of
    /// the target filter.)
    fn register_grapeshot(registry: &mut CardRegistry) -> crate::types::CardId {
        use crate::mana::ManaCost;
        use crate::registry::{CardDefinition, SpellAbilityDef};
        use crate::targets::TargetRequirement;
        fn grapeshot_effect(
            _: &GameState,
            entry: &crate::stack::StackEntry,
            _: &CardRegistry,
        ) -> Vec<crate::effects::Effect> {
            let Some(t) = entry.targets.targets.first() else { return vec![]; };
            let target = match t {
                crate::targets::TargetChoice::Object(id) =>
                    crate::events::DamageTarget::Object(*id),
                crate::targets::TargetChoice::Player(p) =>
                    crate::events::DamageTarget::Player(*p),
                crate::targets::TargetChoice::ObjectOrPlayer(op) => match op {
                    crate::targets::ObjectOrPlayer::Object(id) =>
                        crate::events::DamageTarget::Object(*id),
                    crate::targets::ObjectOrPlayer::Player(p) =>
                        crate::events::DamageTarget::Player(*p),
                },
            };
            vec![crate::effects::Effect::DealDamage {
                source: entry.source, target, amount: 1,
            }]
        }
        let name = registry.interner_mut().intern("Grapeshot");
        let mut def = CardDefinition::new(name, Characteristics {
            mana_cost: Some(ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        }).with_spell_ability(SpellAbilityDef {
            text: "Deal 1 damage to target player. Storm.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: grapeshot_effect,
        });
        def.triggered_abilities.push(
            crate::keywords::storm_trigger_def(1));
        def.base_characteristics.keywords.push(
            crate::effects::KeywordAbility::Storm);
        registry.register(def)
    }

    /// Register a Tendrils-shape spell: {B}{B} sorcery, "target player
    /// loses 2 life" + storm. Single-target storm.
    fn register_tendrils(registry: &mut CardRegistry) -> crate::types::CardId {
        use crate::mana::ManaCost;
        use crate::registry::{CardDefinition, SpellAbilityDef};
        use crate::targets::TargetRequirement;
        fn tendrils_effect(
            _: &GameState,
            entry: &crate::stack::StackEntry,
            _: &CardRegistry,
        ) -> Vec<crate::effects::Effect> {
            let Some(t) = entry.targets.targets.first() else { return vec![]; };
            let player = match t {
                crate::targets::TargetChoice::Player(p) => *p,
                _ => return vec![],
            };
            vec![crate::effects::Effect::LoseLife { player, amount: 2 }]
        }
        let name = registry.interner_mut().intern("Tendrils");
        let mut def = CardDefinition::new(name, Characteristics {
            mana_cost: Some(ManaCost::parse("{B}{B}").unwrap()),
            colors: ColorSet::black(),
            types: TypeLine::SORCERY.into(),
            ..Default::default()
        }).with_spell_ability(SpellAbilityDef {
            text: "Target player loses 2 life. Storm.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: tendrils_effect,
        });
        def.triggered_abilities.push(
            crate::keywords::storm_trigger_def(1));
        def.base_characteristics.keywords.push(
            crate::effects::KeywordAbility::Storm);
        registry.register(def)
    }

    /// Harness: put a spell on the stack as if cast at storm count N-1.
    /// Primes `state.storm_count = prior_spells`, then announces the
    /// spell (incrementing storm_count to prior_spells+1 and snapshotting
    /// prior_spells on the entry).
    fn announce_as_nth_spell(
        state: &mut GameState,
        registry: &CardRegistry,
        controller: PlayerId,
        card_id: crate::types::CardId,
        targets: crate::targets::TargetSelection,
        prior_spells: u32,
    ) -> ObjectId {
        state.storm_count = prior_spells;
        let obj_id = state.allocate_object_id();
        let chars = registry.get(card_id).unwrap().base_characteristics.clone();
        state.objects.insert(crate::objects::GameObject::new(
            obj_id, controller,
            crate::zones::Zone::Hand(controller),
            card_id, chars,
        ));
        let reqs = registry.get(card_id).unwrap().spell_ability.as_ref()
            .map(|sa| sa.target_requirements.clone()).unwrap_or_default();
        state.announce_spell_on_stack(
            obj_id, controller, targets, vec![], None, reqs)
    }

    #[test]
    fn storm_trigger_creates_n_copies_on_stack() {
        use crate::targets::{TargetChoice, TargetSelection};
        let mut registry = CardRegistry::new();
        let card = register_tendrils(&mut registry);
        let mut s = GameState::new(2, 0);
        // Prior storm count = 3 (three spells already cast).
        let mut targets = TargetSelection::new();
        targets.targets.push(TargetChoice::Player(1));
        let cast_id = announce_as_nth_spell(
            &mut s, &registry, 0, card, targets, 3);
        s.emit_spell_cast(cast_id);
        run_sba_and_triggers(&mut s, &registry);
        // Resolve the storm trigger, which enqueues 3 CopySpell
        // effects. The first one pushes ChooseTargets and parks.
        resolve_top_of_stack(&mut s, &registry);
        assert!(s.pending_choice.is_some(),
            "first copy's ChooseTargets should be pending");
    }

    #[test]
    fn storm_copies_resolve_per_copy_targets_and_deal_right_damage() {
        use crate::targets::{TargetChoice, TargetSelection};
        let mut registry = CardRegistry::new();
        let card = register_grapeshot(&mut registry);
        let mut s = GameState::new(2, 0);
        // Cast Grapeshot targeting player 1. Prior = 3.
        let mut targets = TargetSelection::new();
        targets.targets.push(TargetChoice::Player(1));
        let cast_id = announce_as_nth_spell(
            &mut s, &registry, 0, card, targets, 3);
        let p1_start = s.player(1).life;
        s.emit_spell_cast(cast_id);
        run_sba_and_triggers(&mut s, &registry);

        // Resolve storm trigger: enqueues 3 CopySpell effects, each
        // parking for ChooseTargets.
        resolve_top_of_stack(&mut s, &registry);

        // Drive three per-copy target choices. All targets = player 1.
        for _ in 0..3 {
            let pending = s.pending_choice.as_ref()
                .expect("expected per-copy ChooseTargets").clone();
            let mut sel = TargetSelection::new();
            sel.targets.push(TargetChoice::Player(1));
            apply_resolution_choice(&mut s, pending.id,
                crate::actions::ChoiceResponse::ChooseTargets { selection: sel });
        }

        // Resolve 3 copies + the original = 4 damage events.
        while !s.stack_is_empty() {
            resolve_top_of_stack(&mut s, &registry);
            run_sba_and_triggers(&mut s, &registry);
        }

        assert_eq!(p1_start - s.player(1).life, 4,
            "3 storm copies + original Grapeshot = 4 damage to player 1");
    }

    #[test]
    fn storm_copies_do_not_increment_storm_count_for_later_spells() {
        use crate::targets::{TargetChoice, TargetSelection};
        let mut registry = CardRegistry::new();
        let card = register_tendrils(&mut registry);
        let mut s = GameState::new(2, 0);
        let mut targets = TargetSelection::new();
        targets.targets.push(TargetChoice::Player(1));
        let cast_id = announce_as_nth_spell(
            &mut s, &registry, 0, card, targets, 2);
        // After announce, storm_count should be 3 (2 prior + this cast).
        assert_eq!(s.storm_count, 3);
        s.emit_spell_cast(cast_id);
        run_sba_and_triggers(&mut s, &registry);
        // Resolve storm trigger -> 2 copies (prior=2 before cast).
        resolve_top_of_stack(&mut s, &registry);

        // After trigger resolution, copies are on the stack + original.
        // state.storm_count must still be 3 — copies don't increment.
        assert_eq!(s.storm_count, 3,
            "copies on the stack don't go through announce_spell_on_stack \
             and must not increment storm_count");
    }

    #[test]
    fn effect_copy_spell_pushes_choose_targets_when_original_has_targets() {
        use crate::targets::{TargetChoice, TargetSelection};
        let mut registry = CardRegistry::new();
        let card = register_grapeshot(&mut registry);
        let mut s = GameState::new(2, 0);
        let mut targets = TargetSelection::new();
        targets.targets.push(TargetChoice::Player(1));
        let cast_id = announce_as_nth_spell(
            &mut s, &registry, 0, card, targets, 0);
        crate::effects::Effect::CopySpell { target: cast_id }.execute(&mut s);
        assert!(s.pending_choice.is_some(),
            "CopySpell on a targeted spell must push a ChooseTargets prompt");
        assert!(s.pending_target_requirements.is_some());
    }

    // -------------------- Cascade --------------------

    /// Build a simple cascade card: {3}{R} sorcery, "deal 1 to any target".
    /// MV=4, so cascade hits a nonland with MV<4.
    fn register_cascade_card(registry: &mut CardRegistry) -> crate::types::CardId {
        use crate::mana::ManaCost;
        use crate::registry::{CardDefinition, SpellAbilityDef};
        use crate::targets::TargetRequirement;
        fn nada(
            _: &GameState,
            _: &crate::stack::StackEntry,
            _: &CardRegistry,
        ) -> Vec<crate::effects::Effect> { vec![] }
        let name = registry.interner_mut().intern("CascadeCard");
        let mut def = CardDefinition::new(name, Characteristics {
            mana_cost: Some(ManaCost::parse("{3}{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::SORCERY.into(),
            ..Default::default()
        }).with_spell_ability(SpellAbilityDef {
            text: "Cascade. (No other effect for this test card.)".into(),
            target_requirements: vec![TargetRequirement::any_target()],
            modal: None,
            effect: nada,
        });
        def.triggered_abilities.push(
            crate::keywords::cascade_trigger_def(1));
        def.base_characteristics.keywords.push(
            crate::effects::KeywordAbility::Cascade);
        registry.register(def)
    }

    /// Seed `player`'s library top-to-bottom with cards of the given
    /// characteristics (index 0 = top).
    fn seed_library(
        state: &mut GameState,
        player: PlayerId,
        cards: Vec<Characteristics>,
    ) -> Vec<ObjectId> {
        let mut ids = Vec::new();
        for chars in cards {
            let id = state.allocate_object_id();
            state.objects.insert(crate::objects::GameObject::new(
                id, player, crate::zones::Zone::Library(player), 0, chars));
            state.player_mut(player).library_top_to_bottom.push(id);
            ids.push(id);
        }
        ids
    }

    fn basic_land_chars() -> Characteristics {
        Characteristics {
            mana_cost: None,
            types: TypeLine::LAND.into(),
            ..Default::default()
        }
    }

    fn cmc1_instant_chars() -> Characteristics {
        use crate::mana::ManaCost;
        Characteristics {
            mana_cost: Some(ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        }
    }

    #[test]
    fn cascade_no_valid_hit_sends_lands_to_bottom() {
        use crate::targets::{TargetChoice, TargetSelection};
        let mut registry = CardRegistry::new();
        let card = register_cascade_card(&mut registry);
        let mut s = GameState::new(2, 0);
        // Library top-to-bottom: 2 lands, then empty. No valid hit.
        seed_library(&mut s, 0, vec![basic_land_chars(), basic_land_chars()]);
        let mut targets = TargetSelection::new();
        targets.targets.push(TargetChoice::Player(1));
        let cast_id = announce_as_nth_spell(
            &mut s, &registry, 0, card, targets, 0);
        s.emit_spell_cast(cast_id);
        run_sba_and_triggers(&mut s, &registry);

        // Resolve cascade trigger.
        resolve_top_of_stack(&mut s, &registry);

        // No pending choice (no valid hit → no may-cast prompt).
        assert!(s.pending_choice.is_none());
        assert!(s.pending_cascade.is_none());
        assert_eq!(s.player(0).library_top_to_bottom.len(), 2,
            "both lands returned to bottom");
    }

    #[test]
    fn cascade_may_cast_yes_casts_the_hit_for_free() {
        use crate::targets::{TargetChoice, TargetSelection};
        let mut registry = CardRegistry::new();
        let card = register_cascade_card(&mut registry);
        let mut s = GameState::new(2, 0);
        // Top: land, then MV-1 instant. Cascade (MV 4) hits the instant.
        seed_library(&mut s, 0,
            vec![basic_land_chars(), cmc1_instant_chars()]);
        let mut targets = TargetSelection::new();
        targets.targets.push(TargetChoice::Player(1));
        let cast_id = announce_as_nth_spell(
            &mut s, &registry, 0, card, targets, 0);
        s.emit_spell_cast(cast_id);
        run_sba_and_triggers(&mut s, &registry);

        // Resolve cascade trigger → exile land, exile instant, prompt.
        resolve_top_of_stack(&mut s, &registry);
        assert!(s.pending_choice.is_some(), "may-cast YesNo should be pending");

        // Answer yes.
        let pid = s.pending_choice.as_ref().unwrap().id;
        apply_resolution_choice(&mut s, pid,
            crate::actions::ChoiceResponse::YesNo { answer: true });

        // The instant should now be on the stack as a cast spell.
        let has_instant_on_stack = s.stack.iter()
            .any(|e| matches!(e.kind,
                crate::stack::StackEntryKind::Spell { .. }
                if e.id != cast_id));
        assert!(has_instant_on_stack,
            "hit must be cast (on stack) after may-cast yes");
        // Land goes to bottom.
        assert_eq!(s.player(0).library_top_to_bottom.len(), 1);
    }

    #[test]
    fn cascade_may_cast_no_sends_hit_to_bottom_with_rest() {
        use crate::targets::{TargetChoice, TargetSelection};
        let mut registry = CardRegistry::new();
        let card = register_cascade_card(&mut registry);
        let mut s = GameState::new(2, 0);
        seed_library(&mut s, 0,
            vec![basic_land_chars(), cmc1_instant_chars()]);
        let mut targets = TargetSelection::new();
        targets.targets.push(TargetChoice::Player(1));
        let cast_id = announce_as_nth_spell(
            &mut s, &registry, 0, card, targets, 0);
        s.emit_spell_cast(cast_id);
        run_sba_and_triggers(&mut s, &registry);
        resolve_top_of_stack(&mut s, &registry);

        let pid = s.pending_choice.as_ref().unwrap().id;
        apply_resolution_choice(&mut s, pid,
            crate::actions::ChoiceResponse::YesNo { answer: false });

        // Nothing should be on the stack from cascade (aside from the
        // original cascade spell — which still sits where it was).
        let stack_spells: Vec<_> = s.stack.iter()
            .filter(|e| matches!(e.kind,
                crate::stack::StackEntryKind::Spell { .. }))
            .collect();
        assert_eq!(stack_spells.len(), 1,
            "only the original cascade spell remains on the stack");
        // Library now has both exiled cards back at the bottom.
        assert_eq!(s.player(0).library_top_to_bottom.len(), 2);
    }

    #[test]
    fn cascade_is_deterministic_with_same_seed() {
        // Two identical games, same seed, same cascade action sequence
        // — the bottom-shuffle order must be identical.
        use crate::targets::{TargetChoice, TargetSelection};
        fn run(seed: u64) -> Vec<ObjectId> {
            let mut registry = CardRegistry::new();
            let card = register_cascade_card(&mut registry);
            let mut s = GameState::new(2, seed);
            seed_library(&mut s, 0, vec![
                basic_land_chars(), basic_land_chars(), basic_land_chars(),
                cmc1_instant_chars(),
            ]);
            let mut targets = TargetSelection::new();
            targets.targets.push(TargetChoice::Player(1));
            let cast_id = announce_as_nth_spell(
                &mut s, &registry, 0, card, targets, 0);
            s.emit_spell_cast(cast_id);
            run_sba_and_triggers(&mut s, &registry);
            resolve_top_of_stack(&mut s, &registry);
            // Decline the may-cast so all 4 cards land at the bottom.
            let pid = s.pending_choice.as_ref().unwrap().id;
            apply_resolution_choice(&mut s, pid,
                crate::actions::ChoiceResponse::YesNo { answer: false });
            s.player(0).library_top_to_bottom.clone()
        }
        assert_eq!(run(42), run(42),
            "same seed → same cascade bottom order");
    }

    // =====================================================================
    // Flashback (CR 702.33)
    // =====================================================================

    /// Register a flashback test card: {R} instant, "deal 1 damage to
    /// target player", flashback {2}{R}.
    fn register_flashback_bolt(registry: &mut CardRegistry) -> crate::types::CardId {
        use crate::mana::ManaCost;
        use crate::registry::{CardDefinition, SpellAbilityDef};
        use crate::targets::TargetRequirement;
        fn fb_bolt_effect(
            _: &GameState,
            entry: &crate::stack::StackEntry,
            _: &CardRegistry,
        ) -> Vec<crate::effects::Effect> {
            let Some(t) = entry.targets.targets.first() else { return vec![]; };
            let player = match t {
                crate::targets::TargetChoice::Player(p) => *p,
                _ => return vec![],
            };
            vec![crate::effects::Effect::DealDamage {
                source: entry.source,
                target: crate::events::DamageTarget::Player(player),
                amount: 1,
            }]
        }
        let name = registry.interner_mut().intern("Flashback-Bolt");
        let mut def = CardDefinition::new(name, Characteristics {
            mana_cost: Some(ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        }).with_spell_ability(SpellAbilityDef {
            text: "Deal 1 damage to target player. Flashback {2}{R}.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: fb_bolt_effect,
        });
        def.base_characteristics.keywords.push(
            crate::effects::KeywordAbility::Flashback(
                ManaCost::parse("{2}{R}").unwrap()));
        registry.register(def)
    }

    fn register_flashback_sorcery(registry: &mut CardRegistry) -> crate::types::CardId {
        use crate::mana::ManaCost;
        use crate::registry::{CardDefinition, SpellAbilityDef};
        use crate::targets::TargetRequirement;
        fn fb_sorcery_effect(
            _: &GameState,
            _: &crate::stack::StackEntry,
            _: &CardRegistry,
        ) -> Vec<crate::effects::Effect> { vec![] }
        let name = registry.interner_mut().intern("Flashback-Sorcery");
        let mut def = CardDefinition::new(name, Characteristics {
            mana_cost: Some(ManaCost::parse("{1}{B}").unwrap()),
            colors: ColorSet::black(),
            types: TypeLine::SORCERY.into(),
            ..Default::default()
        }).with_spell_ability(SpellAbilityDef {
            text: "Flashback {3}{B}.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: fb_sorcery_effect,
        });
        def.base_characteristics.keywords.push(
            crate::effects::KeywordAbility::Flashback(
                crate::mana::ManaCost::parse("{3}{B}").unwrap()));
        registry.register(def)
    }

    /// Put a card into `player`'s graveyard as if it had been cast and
    /// gone to yard normally.
    fn put_in_graveyard(
        state: &mut GameState,
        registry: &CardRegistry,
        player: PlayerId,
        card_id: crate::types::CardId,
    ) -> ObjectId {
        let obj_id = state.allocate_object_id();
        let chars = registry.get(card_id).unwrap().base_characteristics.clone();
        state.objects.insert(crate::objects::GameObject::new(
            obj_id, player, Zone::Graveyard(player), card_id, chars));
        obj_id
    }

    fn give_mana(state: &mut GameState, player: PlayerId, cost: &str) {
        let cost = crate::mana::ManaCost::parse(cost).unwrap();
        for c in cost.components.iter() {
            use crate::types::ManaColor;
            match c {
                crate::mana::ManaCostComponent::Colored(color) => {
                    let mc = match color {
                        Color::White => ManaColor::White,
                        Color::Blue => ManaColor::Blue,
                        Color::Black => ManaColor::Black,
                        Color::Red => ManaColor::Red,
                        Color::Green => ManaColor::Green,
                    };
                    state.player_mut(player).mana_pool.add_mana(mc, 1, 0);
                }
                crate::mana::ManaCostComponent::Generic(n) => {
                    state.player_mut(player).mana_pool
                        .add_mana(ManaColor::Red, *n, 0);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn flashback_cast_pays_flashback_cost_and_resolves() {
        let mut registry = CardRegistry::new();
        let card = register_flashback_bolt(&mut registry);
        let mut s = GameState::new(2, 0);
        let gy_id = put_in_graveyard(&mut s, &registry, 0, card);
        give_mana(&mut s, 0, "{2}{R}");
        let p1_start = s.player(1).life;

        // Look up the flashback action from legal_actions.
        s.priority.give_to(0);
        // Set phase to main so sorcery-speed check passes. (It's an
        // instant, but we mirror the normal cast flow.)
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;
        let actions = crate::legal_actions::legal_actions(&s, &registry);
        // Pick the flashback action that targets player 1 specifically
        // (enumerate_target_selections yields one action per legal
        // target; we want the one aimed at the opponent).
        let fb_action = actions.iter().find(|a| matches!(a,
            Action::CastSpell { object_id, cast_modifier, targets, .. }
            if *object_id == gy_id
                && matches!(cast_modifier, crate::actions::CastModifier::Flashback)
                && targets.targets.first()
                    == Some(&crate::targets::TargetChoice::Player(1))))
            .expect("legal_actions should offer the flashback cast at player 1")
            .clone();

        let (s, _) = step(s, fb_action, &registry);
        // Bolt is on the stack. Resolve it.
        let mut s = s;
        let pending_targets = s.pending_choice.is_some();
        assert!(!pending_targets, "flashback bolt with target already chosen");
        while !s.stack_is_empty() {
            resolve_top_of_stack(&mut s, &registry);
            run_sba_and_triggers(&mut s, &registry);
        }

        // 1 damage dealt.
        assert_eq!(p1_start - s.player(1).life, 1);
        // Card is in Exile, not Graveyard.
        assert_eq!(s.zone_count(Zone::Exile), 1,
            "flashback spell should land in exile");
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 0);
    }

    #[test]
    fn flashback_counter_sends_card_to_exile() {
        let mut registry = CardRegistry::new();
        let card = register_flashback_bolt(&mut registry);
        let mut s = GameState::new(2, 0);
        let gy_id = put_in_graveyard(&mut s, &registry, 0, card);

        // Directly build the cast action with cast_modifier = Flashback.
        let mut targets = crate::targets::TargetSelection::new();
        targets.targets.push(crate::targets::TargetChoice::Player(1));
        let cast = Action::CastSpell {
            object_id: gy_id, targets,
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::Flashback,
            cost_reductions: crate::actions::CostReductions::default(),
        };
        give_mana(&mut s, 0, "{2}{R}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;
        let (s, _) = step(s, cast, &registry);
        let mut s = s;
        let stack_id = s.top_of_stack().unwrap().id;
        let entry = s.remove_stack_entry_by_id(stack_id).unwrap();
        s.counter_resolved_spell(entry);
        assert_eq!(s.zone_count(Zone::Exile), 1,
            "countered flashback spell must go to exile");
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 0);
    }

    #[test]
    fn flashback_fizzle_sends_card_to_exile() {
        // Cast flashback bolt targeting player 1, then remove player 1
        // from the game… not supported. Simulate by tampering: set the
        // chosen target to an invalid player id. The resolution-time
        // recheck will rules-counter the spell and route via
        // counter_resolved_spell — which respects the flashback flag.
        let mut registry = CardRegistry::new();
        let card = register_flashback_bolt(&mut registry);
        let mut s = GameState::new(2, 0);
        let gy_id = put_in_graveyard(&mut s, &registry, 0, card);
        give_mana(&mut s, 0, "{2}{R}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let mut targets = crate::targets::TargetSelection::new();
        targets.targets.push(crate::targets::TargetChoice::Player(1));
        let cast = Action::CastSpell {
            object_id: gy_id, targets,
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::Flashback,
            cost_reductions: crate::actions::CostReductions::default(),
        };
        let (s, _) = step(s, cast, &registry);
        let mut s = s;
        // Invalidate the target: mark player 1 as lost.
        s.player_mut(1).has_lost = true;
        // Resolve — recheck should classify as CounteredIllegalTargets.
        resolve_top_of_stack(&mut s, &registry);
        // Card goes to exile via counter_resolved_spell.
        assert_eq!(s.zone_count(Zone::Exile), 1,
            "fizzled flashback spell must go to exile");
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 0);
    }

    #[test]
    fn legal_actions_does_not_offer_flashback_from_exile() {
        let mut registry = CardRegistry::new();
        let card = register_flashback_bolt(&mut registry);
        let mut s = GameState::new(2, 0);
        let obj_id = s.allocate_object_id();
        let chars = registry.get(card).unwrap().base_characteristics.clone();
        s.objects.insert(crate::objects::GameObject::new(
            obj_id, 0, Zone::Exile, card, chars));
        give_mana(&mut s, 0, "{2}{R}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;
        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let fb_actions = actions.iter().filter(|a| matches!(a,
            Action::CastSpell { cast_modifier, .. }
                if matches!(cast_modifier,
                    crate::actions::CastModifier::Flashback))).count();
        assert_eq!(fb_actions, 0,
            "flashback offered only from graveyard, not exile");
    }

    #[test]
    fn legal_actions_does_not_offer_flashback_without_mana() {
        let mut registry = CardRegistry::new();
        let card = register_flashback_bolt(&mut registry);
        let mut s = GameState::new(2, 0);
        put_in_graveyard(&mut s, &registry, 0, card);
        // Only one red mana — not enough for {2}{R}.
        give_mana(&mut s, 0, "{R}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;
        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let fb_actions = actions.iter().filter(|a| matches!(a,
            Action::CastSpell { cast_modifier, .. }
                if matches!(cast_modifier,
                    crate::actions::CastModifier::Flashback))).count();
        assert_eq!(fb_actions, 0, "no mana → no flashback enumeration");
    }

    #[test]
    fn legal_actions_respects_sorcery_speed_for_flashback_sorcery() {
        let mut registry = CardRegistry::new();
        let card = register_flashback_sorcery(&mut registry);
        let mut s = GameState::new(2, 0);
        put_in_graveyard(&mut s, &registry, 0, card);
        give_mana(&mut s, 0, "{3}{B}");
        // Opponent's turn: sorcery speed is denied even with mana.
        s.turn.active_player = 1;
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;
        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let fb_actions = actions.iter().filter(|a| matches!(a,
            Action::CastSpell { cast_modifier, .. }
                if matches!(cast_modifier,
                    crate::actions::CastModifier::Flashback))).count();
        assert_eq!(fb_actions, 0,
            "flashback sorcery on opponent's turn must not be legal");
    }

    #[test]
    fn flashback_cast_modifier_none_does_not_pay_flashback_cost() {
        // Mutual-exclusion pin: if the agent passes CastModifier::None
        // on a card in the graveyard, apply_cast_spell does not treat
        // it as a flashback cast and — since no non-flashback cast
        // path exists from graveyard — silently rejects. (A hand cast
        // with the same cast_modifier=None would succeed; this test
        // specifically asserts "from graveyard with None is a no-op".)
        let mut registry = CardRegistry::new();
        let card = register_flashback_bolt(&mut registry);
        let mut s = GameState::new(2, 0);
        let gy_id = put_in_graveyard(&mut s, &registry, 0, card);
        give_mana(&mut s, 0, "{R}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let mut targets = crate::targets::TargetSelection::new();
        targets.targets.push(crate::targets::TargetChoice::Player(1));
        let cast = Action::CastSpell {
            object_id: gy_id, targets,
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions::default(),
        };
        let (s, _) = step(s, cast, &registry);
        // Cast modifier = None but source is graveyard — nothing
        // should move to the stack.
        assert!(s.stack_is_empty(),
            "None modifier shouldn't let a graveyard cast sneak through");
    }

    #[test]
    fn snapcaster_style_granted_flashback_is_offered_by_legal_actions() {
        // Test fixture: a non-flashback instant gains flashback via a
        // layer-6 continuous effect (stand-in for Snapcaster Mage's
        // "target instant or sorcery gains flashback until end of
        // turn"). legal_actions should discover it via
        // state.effective_keywords and offer the flashback cast.
        use crate::layers::{ContinuousEffect, Duration};
        let mut registry = CardRegistry::new();
        // Plain instant with NO printed flashback.
        let name = registry.interner_mut().intern("Plain-Bolt");
        let def = crate::registry::CardDefinition::new(name, Characteristics {
            mana_cost: Some(crate::mana::ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        });
        let card = registry.register(def);

        let mut s = GameState::new(2, 0);
        let gy_id = put_in_graveyard(&mut s, &registry, 0, card);

        // Grant flashback via a continuous effect until end of turn.
        let granted_cost = crate::mana::ManaCost::parse("{2}{R}").unwrap();
        s.add_continuous_effect(ContinuousEffect::grant_keyword(
            /*source=*/ 999, gy_id,
            crate::effects::KeywordAbility::Flashback(granted_cost),
            Duration::EndOfTurn,
        ));
        give_mana(&mut s, 0, "{2}{R}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;
        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let fb_actions = actions.iter().filter(|a| matches!(a,
            Action::CastSpell { cast_modifier, .. }
                if matches!(cast_modifier,
                    crate::actions::CastModifier::Flashback))).count();
        assert!(fb_actions >= 1,
            "granted-flashback card must be offered via effective_keywords");
    }

    // =========================================================================
    // Delve (CR 702.66)
    // =========================================================================

    /// Register a delve spell costing `printed_cost` that does nothing
    /// on resolution. Keyword ability is set; target list is empty so
    /// target enumeration stays trivial.
    fn register_delve_spell(
        registry: &mut CardRegistry,
        name: &str,
        printed_cost: &str,
    ) -> crate::types::CardId {
        use crate::mana::ManaCost;
        use crate::registry::{CardDefinition, SpellAbilityDef};
        fn noop_effect(
            _: &GameState, _: &crate::stack::StackEntry, _: &CardRegistry,
        ) -> Vec<crate::effects::Effect> { vec![] }
        let cost = ManaCost::parse(printed_cost).unwrap();
        let interned = registry.interner_mut().intern(name);
        let mut def = CardDefinition::new(interned, Characteristics {
            mana_cost: Some(cost.clone()),
            colors: cost.colors(),
            types: TypeLine::SORCERY.into(),
            ..Default::default()
        }).with_spell_ability(SpellAbilityDef {
            text: "Delve. Does nothing.".into(),
            target_requirements: vec![],
            modal: None,
            effect: noop_effect,
        });
        def.base_characteristics.keywords.push(
            crate::effects::KeywordAbility::Delve);
        registry.register(def)
    }

    /// Put a card into `player`'s hand (analog of `put_in_graveyard`).
    fn put_in_hand(
        state: &mut GameState,
        registry: &CardRegistry,
        player: PlayerId,
        card_id: crate::types::CardId,
    ) -> ObjectId {
        let obj_id = state.allocate_object_id();
        let chars = registry.get(card_id).unwrap().base_characteristics.clone();
        state.objects.insert(crate::objects::GameObject::new(
            obj_id, player, crate::zones::Zone::Hand(player), card_id, chars));
        obj_id
    }

    /// Helper: count how many cast actions have exactly `k` delve
    /// exiles among `actions`, filtering to a specific source id.
    fn count_delve_actions_with_k(
        actions: &[Action], source: ObjectId, k: usize,
    ) -> usize {
        // Count only `Some(list)` entries: a delve-enabled cast with
        // `list.len() == k`. `None` means "card has no delve" and is
        // a different action shape entirely — never counted here.
        actions.iter().filter(|a| match a {
            Action::CastSpell {
                object_id,
                cost_reductions: crate::actions::CostReductions {
                    delve_exiles: Some(list), ..
                },
                ..
            } if *object_id == source => list.len() == k,
            _ => false,
        }).count()
    }

    #[test]
    fn delve_exiles_graveyard_cards_pays_generic() {
        // Delve spell {3}{U}, graveyard has 4 cards, mana pool has
        // {U}. Agent delves 3, pays {U}. Spell resolves; 3 cards
        // moved to exile; source moved to stack (then resolved).
        let mut registry = CardRegistry::new();
        let delve_card = register_delve_spell(&mut registry, "Delve-Sorc", "{3}{U}");
        let filler = register_delve_spell(&mut registry, "Filler", "{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, delve_card);
        let g1 = put_in_graveyard(&mut s, &registry, 0, filler);
        let g2 = put_in_graveyard(&mut s, &registry, 0, filler);
        let g3 = put_in_graveyard(&mut s, &registry, 0, filler);
        let _g4 = put_in_graveyard(&mut s, &registry, 0, filler);
        give_mana(&mut s, 0, "{U}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
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
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: Some(vec![g1, g2, g3]),
                convoke_taps: None,
                improvise_taps: None,
            },
        };
        let (s, _) = step(s, cast, &registry);
        // 3 cards went to exile, source is on stack.
        assert_eq!(s.zone_count(Zone::Exile), 3,
            "3 delve cards must be exiled");
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1,
            "1 filler card remaining in graveyard");
        assert!(!s.stack_is_empty(),
            "delve spell must be on stack");
    }

    #[test]
    fn delve_cannot_pay_colored_requirement() {
        // Delve spell {3}{U}{U}, graveyard has 5 cards, no mana.
        // Agent tries to delve 5 (would cover everything) — reject:
        // colored requirement can't be delved.
        let mut registry = CardRegistry::new();
        let delve_card = register_delve_spell(&mut registry, "Delve-Big", "{3}{U}{U}");
        let filler = register_delve_spell(&mut registry, "Filler", "{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, delve_card);
        let gids: Vec<ObjectId> = (0..5).map(|_|
            put_in_graveyard(&mut s, &registry, 0, filler)).collect();
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
            targets: crate::targets::TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: Some(gids),
                convoke_taps: None,
                improvise_taps: None,
            },
        };
        let (s, _) = step(s, cast, &registry);
        // Exceeds generic (3) — apply_cast_spell bails, no exile,
        // no stack entry.
        assert!(s.stack_is_empty(),
            "over-generic delve must be rejected");
        assert_eq!(s.zone_count(Zone::Exile), 0);
    }

    #[test]
    fn delve_zero_exiles_equivalent_to_normal_cast() {
        // Delve card, graveyard has cards, but agent delves zero.
        // Mana pool covers the full printed cost.
        let mut registry = CardRegistry::new();
        let delve_card = register_delve_spell(&mut registry, "Delve-Sorc", "{2}{U}");
        let filler = register_delve_spell(&mut registry, "Filler", "{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, delve_card);
        put_in_graveyard(&mut s, &registry, 0, filler);
        give_mana(&mut s, 0, "{2}{U}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let zero_delve = actions.iter().find(|a| matches!(a,
            Action::CastSpell {
                object_id,
                cost_reductions: crate::actions::CostReductions {
                    delve_exiles: Some(v), ..
                },
                ..
            } if *object_id == src && v.is_empty()))
            .expect("zero-delve cast must be among legal actions")
            .clone();
        let (s, _) = step(s, zero_delve, &registry);
        assert_eq!(s.zone_count(Zone::Exile), 0,
            "zero-delve cast must not exile anything");
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1,
            "graveyard untouched");
        assert!(!s.stack_is_empty());
    }

    #[test]
    fn legal_actions_enumerates_delve_subsets_multiple_counts() {
        // Delve spell {2}{U}, graveyard has 3 distinct cards, mana
        // covers every reduced cost from {2}{U} down to {U}. Expect
        // delve counts 0, 1, 2 all enumerated (bounded by generic=2).
        let mut registry = CardRegistry::new();
        let delve_card = register_delve_spell(&mut registry, "Delve-Sorc", "{2}{U}");
        // Three distinct filler card ids so each has its own equivalence
        // class — dedup won't collapse them.
        let f1 = register_delve_spell(&mut registry, "F1", "{U}");
        let f2 = register_delve_spell(&mut registry, "F2", "{U}");
        let f3 = register_delve_spell(&mut registry, "F3", "{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, delve_card);
        put_in_graveyard(&mut s, &registry, 0, f1);
        put_in_graveyard(&mut s, &registry, 0, f2);
        put_in_graveyard(&mut s, &registry, 0, f3);
        give_mana(&mut s, 0, "{2}{U}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let k0 = count_delve_actions_with_k(&actions, src, 0);
        let k1 = count_delve_actions_with_k(&actions, src, 1);
        let k2 = count_delve_actions_with_k(&actions, src, 2);
        // Cannot delve 3 — generic is only 2.
        let k3 = count_delve_actions_with_k(&actions, src, 3);
        assert!(k0 >= 1, "k=0 (normal cast) must be enumerated");
        assert!(k1 >= 1, "k=1 must be enumerated");
        assert!(k2 >= 1, "k=2 must be enumerated");
        assert_eq!(k3, 0, "k=3 exceeds generic=2, must not be enumerated");
    }

    #[test]
    fn delve_dedup_identical_cards() {
        // Delve spell {3}{U}, graveyard has 3 identical cards (same
        // card_id). Equivalence-class dedup collapses choosing "any
        // one of them" to a single action, not 3.
        let mut registry = CardRegistry::new();
        let delve_card = register_delve_spell(&mut registry, "Delve-Sorc", "{3}{U}");
        let filler = register_delve_spell(&mut registry, "Filler", "{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, delve_card);
        put_in_graveyard(&mut s, &registry, 0, filler);
        put_in_graveyard(&mut s, &registry, 0, filler);
        put_in_graveyard(&mut s, &registry, 0, filler);
        give_mana(&mut s, 0, "{3}{U}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let actions = crate::legal_actions::legal_actions(&s, &registry);
        // For k=1, three identical cards collapse to ONE enumerated
        // subset (not three). Same for k=2, k=3.
        let k1 = count_delve_actions_with_k(&actions, src, 1);
        let k2 = count_delve_actions_with_k(&actions, src, 2);
        let k3 = count_delve_actions_with_k(&actions, src, 3);
        assert_eq!(k1, 1, "identical cards must dedup at k=1 (not 3)");
        assert_eq!(k2, 1, "identical cards must dedup at k=2 (not 3)");
        assert_eq!(k3, 1, "identical cards must dedup at k=3 (not 1)");
    }

    #[test]
    fn legal_actions_does_not_offer_delve_without_keyword() {
        // Non-delve card with the same cost and graveyard as a delve
        // card test — no delve-subset action should appear. Every
        // enumerated cast action must have `delve_exiles: None`.
        let mut registry = CardRegistry::new();
        let plain_card = {
            use crate::mana::ManaCost;
            use crate::registry::{CardDefinition, SpellAbilityDef};
            fn noop(
                _: &GameState, _: &crate::stack::StackEntry, _: &CardRegistry,
            ) -> Vec<crate::effects::Effect> { vec![] }
            let name = registry.interner_mut().intern("Plain-Sorc");
            let def = CardDefinition::new(name, Characteristics {
                mana_cost: Some(ManaCost::parse("{2}{U}").unwrap()),
                colors: ColorSet::blue(),
                types: TypeLine::SORCERY.into(),
                ..Default::default()
            }).with_spell_ability(SpellAbilityDef {
                text: "Does nothing.".into(),
                target_requirements: vec![],
            modal: None,
                effect: noop,
            });
            registry.register(def)
        };
        let filler = register_delve_spell(&mut registry, "Filler", "{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, plain_card);
        put_in_graveyard(&mut s, &registry, 0, filler);
        give_mana(&mut s, 0, "{2}{U}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;
        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let has_delve_field = actions.iter().any(|a| matches!(a,
            Action::CastSpell {
                object_id,
                cost_reductions: crate::actions::CostReductions {
                    delve_exiles: Some(_), ..
                },
                ..
            }
                if *object_id == src));
        assert!(!has_delve_field,
            "card without delve must never get Some(delve_exiles)");
    }

    #[test]
    fn snapcaster_style_granted_delve_is_offered_by_legal_actions() {
        // Non-delve card gains Delve via layer-6 continuous effect.
        // legal_actions must discover it through effective_keywords.
        use crate::layers::{ContinuousEffect, Duration};
        let mut registry = CardRegistry::new();
        let plain_card = {
            use crate::mana::ManaCost;
            use crate::registry::{CardDefinition, SpellAbilityDef};
            fn noop(
                _: &GameState, _: &crate::stack::StackEntry, _: &CardRegistry,
            ) -> Vec<crate::effects::Effect> { vec![] }
            let name = registry.interner_mut().intern("Plain-Granted");
            let def = CardDefinition::new(name, Characteristics {
                mana_cost: Some(ManaCost::parse("{2}{U}").unwrap()),
                colors: ColorSet::blue(),
                types: TypeLine::SORCERY.into(),
                ..Default::default()
            }).with_spell_ability(SpellAbilityDef {
                text: "Does nothing.".into(),
                target_requirements: vec![],
            modal: None,
                effect: noop,
            });
            registry.register(def)
        };
        let filler = register_delve_spell(&mut registry, "Filler", "{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, plain_card);
        put_in_graveyard(&mut s, &registry, 0, filler);
        // Grant Delve until end of turn.
        s.add_continuous_effect(ContinuousEffect::grant_keyword(
            /*source=*/ 999, src,
            crate::effects::KeywordAbility::Delve,
            Duration::EndOfTurn,
        ));
        give_mana(&mut s, 0, "{2}{U}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;
        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let k1 = count_delve_actions_with_k(&actions, src, 1);
        assert!(k1 >= 1,
            "granted-delve card must offer delve actions via effective_keywords");
    }

    #[test]
    fn delve_exile_happens_during_cost_payment() {
        // Emission-order check: the delve-exile events must precede
        // the SpellCast event but happen within the same cast call.
        // Verified via event log: Exiled events come before the
        // stack entry lands (priority re-check).
        let mut registry = CardRegistry::new();
        let delve_card = register_delve_spell(&mut registry, "Delve-Sorc", "{2}{U}");
        let filler = register_delve_spell(&mut registry, "Filler", "{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, delve_card);
        let g1 = put_in_graveyard(&mut s, &registry, 0, filler);
        let g2 = put_in_graveyard(&mut s, &registry, 0, filler);
        give_mana(&mut s, 0, "{U}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
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
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: Some(vec![g1, g2]),
                convoke_taps: None,
                improvise_taps: None,
            },
        };
        let (s, _) = step(s, cast, &registry);
        // Both delve cards ended up exiled. (Zone changes re-id
        // objects per CR 400.7 — lookups by original id will miss,
        // so assert via zone_count instead.)
        assert_eq!(s.zone_count(Zone::Exile), 2,
            "both delve cards exiled as cost payment");
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 0,
            "graveyard emptied by delve");
        // Source moved to stack.
        assert!(!s.stack_is_empty());
        // Mana pool drained (the {U} paid the colored pip).
        assert_eq!(s.player(0).mana_pool.total(), 0,
            "mana spent during cost payment");
    }

    #[test]
    fn delve_with_nonself_duplicate_exile_rejected() {
        // Agent passes the same card_id twice in delve_exiles — must
        // reject (can't double-exile the same object).
        let mut registry = CardRegistry::new();
        let delve_card = register_delve_spell(&mut registry, "Delve-Sorc", "{2}{U}");
        let filler = register_delve_spell(&mut registry, "Filler", "{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, delve_card);
        let g1 = put_in_graveyard(&mut s, &registry, 0, filler);
        give_mana(&mut s, 0, "{U}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
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
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: Some(vec![g1, g1]),
                convoke_taps: None,
                improvise_taps: None,
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(s.stack_is_empty(),
            "duplicate delve-exile id must be rejected");
        assert_eq!(s.zone_count(Zone::Exile), 0);
    }

    // =========================================================================
    // Convoke (CR 702.51)
    // =========================================================================

    /// Register a convoke spell with the given printed cost.
    fn register_convoke_spell(
        registry: &mut CardRegistry,
        name: &str,
        printed_cost: &str,
    ) -> crate::types::CardId {
        use crate::mana::ManaCost;
        use crate::registry::{CardDefinition, SpellAbilityDef};
        fn noop_effect(
            _: &GameState, _: &crate::stack::StackEntry, _: &CardRegistry,
        ) -> Vec<crate::effects::Effect> { vec![] }
        let cost = ManaCost::parse(printed_cost).unwrap();
        let interned = registry.interner_mut().intern(name);
        let mut def = CardDefinition::new(interned, Characteristics {
            mana_cost: Some(cost.clone()),
            colors: cost.colors(),
            types: TypeLine::SORCERY.into(),
            ..Default::default()
        }).with_spell_ability(SpellAbilityDef {
            text: "Convoke. Does nothing.".into(),
            target_requirements: vec![],
            modal: None,
            effect: noop_effect,
        });
        def.base_characteristics.keywords.push(
            crate::effects::KeywordAbility::Convoke);
        registry.register(def)
    }

    /// Put a simple creature on the battlefield for a given player
    /// with the given colors. Returns the new object id.
    fn put_creature(
        state: &mut GameState,
        player: PlayerId,
        colors: crate::types::ColorSet,
    ) -> ObjectId {
        let obj_id = state.allocate_object_id();
        let chars = Characteristics {
            colors,
            types: TypeLine::CREATURE.into(),
            power: Some(crate::types::PtValue::Fixed(1)),
            toughness: Some(crate::types::PtValue::Fixed(1)),
            ..Default::default()
        };
        state.objects.insert(crate::objects::GameObject::new(
            obj_id, player, Zone::Battlefield, 0, chars));
        obj_id
    }

    #[test]
    fn convoke_taps_creatures_pays_generic() {
        // Convoke spell {3}{U}, 3 white creatures on battlefield
        // (each pays Generic), {U} in mana pool. Cast with convoke
        // covering all 3 generic pips.
        let mut registry = CardRegistry::new();
        let convoke_card = register_convoke_spell(
            &mut registry, "Convoke-Sorc", "{3}{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, convoke_card);
        let c1 = put_creature(&mut s, 0, crate::types::ColorSet::white());
        let c2 = put_creature(&mut s, 0, crate::types::ColorSet::white());
        let c3 = put_creature(&mut s, 0, crate::types::ColorSet::white());
        give_mana(&mut s, 0, "{U}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
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
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: None,
                convoke_taps: Some(vec![
                    crate::actions::ConvokeAssignment {
                        creature: c1,
                        payment: crate::actions::ConvokePayment::Generic,
                    },
                    crate::actions::ConvokeAssignment {
                        creature: c2,
                        payment: crate::actions::ConvokePayment::Generic,
                    },
                    crate::actions::ConvokeAssignment {
                        creature: c3,
                        payment: crate::actions::ConvokePayment::Generic,
                    },
                ]),
                improvise_taps: None,
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(!s.stack_is_empty(), "convoke spell must be on stack");
        assert!(s.objects.get(c1).unwrap().is_tapped(),
            "c1 must be tapped");
        assert!(s.objects.get(c2).unwrap().is_tapped());
        assert!(s.objects.get(c3).unwrap().is_tapped());
    }

    #[test]
    fn convoke_colored_creature_pays_colored_pip() {
        // Convoke spell {U}, one blue creature. Tap it for blue to
        // cover the colored pip.
        let mut registry = CardRegistry::new();
        let convoke_card = register_convoke_spell(
            &mut registry, "Convoke-U", "{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, convoke_card);
        let blue_creature = put_creature(
            &mut s, 0, crate::types::ColorSet::blue());
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
            targets: crate::targets::TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: None,
                convoke_taps: Some(vec![
                    crate::actions::ConvokeAssignment {
                        creature: blue_creature,
                        payment: crate::actions::ConvokePayment::Color(
                            crate::types::ManaColor::Blue),
                    },
                ]),
                improvise_taps: None,
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(!s.stack_is_empty(),
            "convoke blue creature for blue pip must succeed");
        assert!(s.objects.get(blue_creature).unwrap().is_tapped());
    }

    #[test]
    fn convoke_cannot_pay_mismatched_color() {
        // Green creature attempts to pay {U} — reject.
        let mut registry = CardRegistry::new();
        let convoke_card = register_convoke_spell(
            &mut registry, "Convoke-U", "{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, convoke_card);
        let green_creature = put_creature(
            &mut s, 0, crate::types::ColorSet::green());
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
            targets: crate::targets::TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: None,
                convoke_taps: Some(vec![
                    crate::actions::ConvokeAssignment {
                        creature: green_creature,
                        payment: crate::actions::ConvokePayment::Color(
                            crate::types::ManaColor::Blue),
                    },
                ]),
                improvise_taps: None,
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(s.stack_is_empty(),
            "green creature paying {{U}} must be rejected");
        assert!(!s.objects.get(green_creature).unwrap().is_tapped());
    }

    #[test]
    fn convoke_cannot_use_tapped_creature() {
        let mut registry = CardRegistry::new();
        let convoke_card = register_convoke_spell(
            &mut registry, "Convoke-Sorc", "{1}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, convoke_card);
        let creature = put_creature(&mut s, 0, crate::types::ColorSet::white());
        // Tap the creature before the cast.
        s.objects.get_mut(creature).unwrap().tap();
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
            targets: crate::targets::TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: None,
                convoke_taps: Some(vec![
                    crate::actions::ConvokeAssignment {
                        creature,
                        payment: crate::actions::ConvokePayment::Generic,
                    },
                ]),
                improvise_taps: None,
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(s.stack_is_empty(),
            "already-tapped creature must be rejected");
    }

    #[test]
    fn convoke_cannot_use_opponent_creature() {
        let mut registry = CardRegistry::new();
        let convoke_card = register_convoke_spell(
            &mut registry, "Convoke-Sorc", "{1}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, convoke_card);
        // Opponent's creature.
        let opp_creature = put_creature(
            &mut s, /*player=*/ 1, crate::types::ColorSet::white());
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
            targets: crate::targets::TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: None,
                convoke_taps: Some(vec![
                    crate::actions::ConvokeAssignment {
                        creature: opp_creature,
                        payment: crate::actions::ConvokePayment::Generic,
                    },
                ]),
                improvise_taps: None,
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(s.stack_is_empty(),
            "opponent's creature must be rejected");
        assert!(!s.objects.get(opp_creature).unwrap().is_tapped());
    }

    #[test]
    fn convoke_duplicate_creature_rejected() {
        // Agent passes the same creature id twice — reject.
        let mut registry = CardRegistry::new();
        let convoke_card = register_convoke_spell(
            &mut registry, "Convoke-Sorc", "{2}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, convoke_card);
        let c1 = put_creature(&mut s, 0, crate::types::ColorSet::white());
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
            targets: crate::targets::TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: None,
                convoke_taps: Some(vec![
                    crate::actions::ConvokeAssignment {
                        creature: c1,
                        payment: crate::actions::ConvokePayment::Generic,
                    },
                    crate::actions::ConvokeAssignment {
                        creature: c1,
                        payment: crate::actions::ConvokePayment::Generic,
                    },
                ]),
                improvise_taps: None,
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(s.stack_is_empty(),
            "duplicate creature id must be rejected");
    }

    #[test]
    fn convoke_zero_creatures_equivalent_to_normal_cast() {
        let mut registry = CardRegistry::new();
        let convoke_card = register_convoke_spell(
            &mut registry, "Convoke-Sorc", "{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, convoke_card);
        put_creature(&mut s, 0, crate::types::ColorSet::blue());
        give_mana(&mut s, 0, "{U}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let zero_convoke = actions.iter().find(|a| matches!(a,
            Action::CastSpell {
                object_id,
                cost_reductions: crate::actions::CostReductions {
                    convoke_taps: Some(v), ..
                },
                ..
            } if *object_id == src && v.is_empty()))
            .expect("zero-convoke cast must be among legal actions")
            .clone();
        let (s, _) = step(s, zero_convoke, &registry);
        assert!(!s.stack_is_empty());
    }

    #[test]
    fn convoke_multicolor_creature_pays_either_color() {
        // G/U creature casting a spell with a {G} pip OR a {U} pip.
        // Both (multicolor, pay G) and (multicolor, pay U) must be
        // enumerable as distinct actions.
        let mut registry = CardRegistry::new();
        // Two separate spell cards, one per color, so we can verify
        // the multicolor creature covers either.
        let gu_spell = register_convoke_spell(
            &mut registry, "Convoke-GU", "{G}{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, gu_spell);
        // One G/U creature. With 2 pips in cost it taps once,
        // covering either G or U.
        let mut colors = crate::types::ColorSet::green();
        colors = colors | crate::types::ColorSet::blue();
        put_creature(&mut s, 0, colors);
        // Give exactly one U in mana pool so the OTHER pip must come
        // from convoke. This forces the enumerator to produce two
        // actions: (creature pays G, mana pays U) AND (creature pays
        // U, mana pays G). The second is infeasible (no G mana), so
        // only the first should appear — verify that.
        give_mana(&mut s, 0, "{U}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let convoke_actions: Vec<_> = actions.iter().filter(|a| matches!(a,
            Action::CastSpell {
                object_id,
                cost_reductions: crate::actions::CostReductions {
                    convoke_taps: Some(v), ..
                },
                ..
            } if *object_id == src && !v.is_empty())).collect();
        // Creature pays G; mana pays U → 1 action.
        // Creature pays U; mana pays G → 0 actions (no G mana).
        // Creature pays Generic: no generic pips; → 0 actions.
        assert_eq!(convoke_actions.len(), 1,
            "exactly one convoke action for (creature=G, mana=U)");
        let (cast_payment, ) = if let Action::CastSpell {
            cost_reductions: crate::actions::CostReductions {
                convoke_taps: Some(v), ..
            }, ..
        } = convoke_actions[0] {
            (v[0].payment,)
        } else { panic!() };
        assert_eq!(cast_payment,
            crate::actions::ConvokePayment::Color(
                crate::types::ManaColor::Green),
            "creature must be paying Green (the pip not covered by mana)");
    }

    #[test]
    fn convoke_colorless_creature_pays_generic_only() {
        // Colorless creature: convoke_eligible_payments → [Generic].
        // Attempting Color(W) must be rejected.
        let mut registry = CardRegistry::new();
        let convoke_card = register_convoke_spell(
            &mut registry, "Convoke-W", "{W}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, convoke_card);
        // Colorless creature.
        let colorless = put_creature(
            &mut s, 0, crate::types::ColorSet::new());
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
            targets: crate::targets::TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: None,
                convoke_taps: Some(vec![
                    crate::actions::ConvokeAssignment {
                        creature: colorless,
                        payment: crate::actions::ConvokePayment::Color(
                            crate::types::ManaColor::White),
                    },
                ]),
                improvise_taps: None,
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(s.stack_is_empty(),
            "colorless creature can't pay colored pip");
    }

    #[test]
    fn convoke_with_summoning_sick_creature() {
        // Creature with summoning sickness: convoke tap-as-cost is
        // not a mana ability and not combat (CR 302.1), so sickness
        // is irrelevant.
        let mut registry = CardRegistry::new();
        let convoke_card = register_convoke_spell(
            &mut registry, "Convoke-Sorc", "{1}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, convoke_card);
        let creature = put_creature(&mut s, 0, crate::types::ColorSet::white());
        // Explicitly mark the creature as summoning-sick.
        s.objects.get_mut(creature).unwrap().status.summoning_sick = true;
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
            targets: crate::targets::TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: None,
                convoke_taps: Some(vec![
                    crate::actions::ConvokeAssignment {
                        creature,
                        payment: crate::actions::ConvokePayment::Generic,
                    },
                ]),
                improvise_taps: None,
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(!s.stack_is_empty(),
            "sickness must not block convoke tap-as-cost");
        assert!(s.objects.get(creature).unwrap().is_tapped());
    }

    #[test]
    fn convoke_with_delve_not_enumerated_jointly() {
        // A card hypothetically with both delve AND convoke: v1 does
        // not enumerate joint (delve+convoke) actions. We check that
        // no legal action carries both Some(non-empty-delve) AND
        // Some(non-empty-convoke).
        let mut registry = CardRegistry::new();
        let spell_id = {
            use crate::mana::ManaCost;
            use crate::registry::{CardDefinition, SpellAbilityDef};
            fn noop(
                _: &GameState, _: &crate::stack::StackEntry, _: &CardRegistry,
            ) -> Vec<crate::effects::Effect> { vec![] }
            let name = registry.interner_mut().intern("Delve+Convoke");
            let mut def = CardDefinition::new(name, Characteristics {
                mana_cost: Some(ManaCost::parse("{2}{U}").unwrap()),
                colors: ColorSet::blue(),
                types: TypeLine::SORCERY.into(),
                ..Default::default()
            }).with_spell_ability(SpellAbilityDef {
                text: "Delve. Convoke.".into(),
                target_requirements: vec![],
            modal: None,
                effect: noop,
            });
            def.base_characteristics.keywords.push(
                crate::effects::KeywordAbility::Delve);
            def.base_characteristics.keywords.push(
                crate::effects::KeywordAbility::Convoke);
            registry.register(def)
        };
        let filler = register_delve_spell(&mut registry, "Filler", "{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, spell_id);
        put_in_graveyard(&mut s, &registry, 0, filler);
        put_creature(&mut s, 0, crate::types::ColorSet::blue());
        give_mana(&mut s, 0, "{2}{U}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;
        let actions = crate::legal_actions::legal_actions(&s, &registry);
        // For every CastSpell on this source, assert we don't have
        // both non-empty delve AND non-empty convoke.
        for a in &actions {
            if let Action::CastSpell {
                object_id,
                cost_reductions: crate::actions::CostReductions {
                    delve_exiles,
                    convoke_taps,
                    ..
                },
                ..
            } = a {
                if *object_id != src { continue; }
                let delve_non_empty = delve_exiles.as_ref()
                    .is_some_and(|v| !v.is_empty());
                let convoke_non_empty = convoke_taps.as_ref()
                    .is_some_and(|v| !v.is_empty());
                assert!(!(delve_non_empty && convoke_non_empty),
                    "v1 must not enumerate joint delve+convoke actions");
            }
        }
    }

    #[test]
    fn legal_actions_enumerates_convoke_subsets() {
        // Convoke spell {1}{U}, 2 untapped blue creatures, no mana.
        // Expect multiple convoke actions covering different
        // assignments (both generic, one generic + one U, both U).
        let mut registry = CardRegistry::new();
        let convoke_card = register_convoke_spell(
            &mut registry, "Convoke-1U", "{1}{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, convoke_card);
        // Two distinct-card-id blue creatures so they don't dedup.
        let c1_id = {
            use crate::registry::CardDefinition;
            let name = registry.interner_mut().intern("Distinct-A");
            let def = CardDefinition::new(name, Characteristics {
                colors: crate::types::ColorSet::blue(),
                types: TypeLine::CREATURE.into(),
                power: Some(crate::types::PtValue::Fixed(1)),
                toughness: Some(crate::types::PtValue::Fixed(1)),
                ..Default::default()
            });
            registry.register(def)
        };
        let c2_id = {
            use crate::registry::CardDefinition;
            let name = registry.interner_mut().intern("Distinct-B");
            let def = CardDefinition::new(name, Characteristics {
                colors: crate::types::ColorSet::blue(),
                types: TypeLine::CREATURE.into(),
                power: Some(crate::types::PtValue::Fixed(1)),
                toughness: Some(crate::types::PtValue::Fixed(1)),
                ..Default::default()
            });
            registry.register(def)
        };
        for card_id in [c1_id, c2_id] {
            let obj = state_allocate_creature(&mut s, 0, card_id, &registry);
            let _ = obj;
        }
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let convoke_non_empty: Vec<_> = actions.iter().filter(|a| matches!(a,
            Action::CastSpell {
                object_id,
                cost_reductions: crate::actions::CostReductions {
                    convoke_taps: Some(v), ..
                },
                ..
            } if *object_id == src && !v.is_empty())).collect();
        assert!(convoke_non_empty.len() >= 2,
            "multiple convoke actions must be enumerated (got {})",
            convoke_non_empty.len());
    }

    fn state_allocate_creature(
        state: &mut GameState,
        player: PlayerId,
        card_id: crate::types::CardId,
        registry: &CardRegistry,
    ) -> ObjectId {
        let obj_id = state.allocate_object_id();
        let chars = registry.get(card_id).unwrap().base_characteristics.clone();
        state.objects.insert(crate::objects::GameObject::new(
            obj_id, player, Zone::Battlefield, card_id, chars));
        obj_id
    }

    #[test]
    fn convoke_dedup_identical_creatures() {
        // Three identical white creatures (same card_id). Convoke
        // spell {3}. At k=1, dedup collapses to 1 action (not 3).
        let mut registry = CardRegistry::new();
        let convoke_card = register_convoke_spell(
            &mut registry, "Convoke-3", "{3}");
        // Build a proper creature card with a card_id so dedup sees
        // identical keys across the three copies.
        let creature_card = {
            use crate::registry::CardDefinition;
            let name = registry.interner_mut().intern("Identical-W");
            let def = CardDefinition::new(name, Characteristics {
                colors: crate::types::ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                power: Some(crate::types::PtValue::Fixed(1)),
                toughness: Some(crate::types::PtValue::Fixed(1)),
                ..Default::default()
            });
            registry.register(def)
        };
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, convoke_card);
        for _ in 0..3 {
            state_allocate_creature(&mut s, 0, creature_card, &registry);
        }
        // Mana to cover the remaining generic after convoke reduces
        // it by up to 3. Without this, k=1 / k=2 are infeasible at
        // mana-solve time and get filtered out.
        give_mana(&mut s, 0, "{2}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let actions = crate::legal_actions::legal_actions(&s, &registry);
        // k=1 actions: one convoke tap (Generic, since no colored
        // pips in cost). Three identical creatures → 1 action.
        let k1: usize = actions.iter().filter(|a| matches!(a,
            Action::CastSpell {
                object_id,
                cost_reductions: crate::actions::CostReductions {
                    convoke_taps: Some(v), ..
                },
                ..
            } if *object_id == src && v.len() == 1)).count();
        assert_eq!(k1, 1, "3 identical creatures must dedup at k=1");
    }

    #[test]
    fn snapcaster_style_granted_convoke() {
        use crate::layers::{ContinuousEffect, Duration};
        let mut registry = CardRegistry::new();
        let plain_card = {
            use crate::mana::ManaCost;
            use crate::registry::{CardDefinition, SpellAbilityDef};
            fn noop(
                _: &GameState, _: &crate::stack::StackEntry, _: &CardRegistry,
            ) -> Vec<crate::effects::Effect> { vec![] }
            let name = registry.interner_mut().intern("Plain-Convoke-Grant");
            let def = CardDefinition::new(name, Characteristics {
                mana_cost: Some(ManaCost::parse("{2}").unwrap()),
                colors: crate::types::ColorSet::new(),
                types: TypeLine::SORCERY.into(),
                ..Default::default()
            }).with_spell_ability(SpellAbilityDef {
                text: "Does nothing.".into(),
                target_requirements: vec![],
            modal: None,
                effect: noop,
            });
            registry.register(def)
        };
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, plain_card);
        put_creature(&mut s, 0, crate::types::ColorSet::white());
        s.add_continuous_effect(ContinuousEffect::grant_keyword(
            /*source=*/ 999, src,
            crate::effects::KeywordAbility::Convoke,
            Duration::EndOfTurn,
        ));
        give_mana(&mut s, 0, "{1}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let convoke_actions: Vec<_> = actions.iter().filter(|a| matches!(a,
            Action::CastSpell {
                object_id,
                cost_reductions: crate::actions::CostReductions {
                    convoke_taps: Some(v), ..
                },
                ..
            } if *object_id == src && !v.is_empty())).collect();
        assert!(!convoke_actions.is_empty(),
            "granted-convoke card must offer convoke actions");
    }

    // =========================================================================
    // Improvise (CR 702.127)
    // =========================================================================

    /// Register an improvise spell with the given printed cost.
    fn register_improvise_spell(
        registry: &mut CardRegistry,
        name: &str,
        printed_cost: &str,
    ) -> crate::types::CardId {
        use crate::mana::ManaCost;
        use crate::registry::{CardDefinition, SpellAbilityDef};
        fn noop_effect(
            _: &GameState, _: &crate::stack::StackEntry, _: &CardRegistry,
        ) -> Vec<crate::effects::Effect> { vec![] }
        let cost = ManaCost::parse(printed_cost).unwrap();
        let interned = registry.interner_mut().intern(name);
        let mut def = CardDefinition::new(interned, Characteristics {
            mana_cost: Some(cost.clone()),
            colors: cost.colors(),
            types: TypeLine::SORCERY.into(),
            ..Default::default()
        }).with_spell_ability(SpellAbilityDef {
            text: "Improvise. Does nothing.".into(),
            target_requirements: vec![],
            modal: None,
            effect: noop_effect,
        });
        def.base_characteristics.keywords.push(
            crate::effects::KeywordAbility::Improvise);
        registry.register(def)
    }

    /// Put an artifact on the battlefield for a player. Colors set
    /// to empty (artifacts are typically colorless). Returns object id.
    fn put_artifact(
        state: &mut GameState,
        player: PlayerId,
    ) -> ObjectId {
        let obj_id = state.allocate_object_id();
        let chars = Characteristics {
            colors: crate::types::ColorSet::new(),
            types: TypeLine::ARTIFACT.into(),
            ..Default::default()
        };
        state.objects.insert(crate::objects::GameObject::new(
            obj_id, player, Zone::Battlefield, 0, chars));
        obj_id
    }

    /// Put an artifact creature on the battlefield (for sickness and
    /// convoke+improvise joint-eligibility tests).
    fn put_artifact_creature(
        state: &mut GameState,
        player: PlayerId,
    ) -> ObjectId {
        let obj_id = state.allocate_object_id();
        let chars = Characteristics {
            colors: crate::types::ColorSet::new(),
            types: crate::types::TypeLine(
                TypeLine::ARTIFACT | TypeLine::CREATURE),
            power: Some(crate::types::PtValue::Fixed(1)),
            toughness: Some(crate::types::PtValue::Fixed(1)),
            ..Default::default()
        };
        state.objects.insert(crate::objects::GameObject::new(
            obj_id, player, Zone::Battlefield, 0, chars));
        obj_id
    }

    #[test]
    fn improvise_taps_artifacts_pays_generic() {
        // {3}{U} spell, 3 artifacts + {U} mana.
        let mut registry = CardRegistry::new();
        let imp_card = register_improvise_spell(
            &mut registry, "Imp-Sorc", "{3}{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, imp_card);
        let a1 = put_artifact(&mut s, 0);
        let a2 = put_artifact(&mut s, 0);
        let a3 = put_artifact(&mut s, 0);
        give_mana(&mut s, 0, "{U}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
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
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: None,
                convoke_taps: None,
                improvise_taps: Some(vec![a1, a2, a3]),
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(!s.stack_is_empty(),
            "improvise spell must be on stack");
        assert!(s.objects.get(a1).unwrap().is_tapped());
        assert!(s.objects.get(a2).unwrap().is_tapped());
        assert!(s.objects.get(a3).unwrap().is_tapped());
    }

    #[test]
    fn improvise_cannot_use_tapped_artifact() {
        let mut registry = CardRegistry::new();
        let imp_card = register_improvise_spell(
            &mut registry, "Imp-Sorc", "{1}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, imp_card);
        let artifact = put_artifact(&mut s, 0);
        s.objects.get_mut(artifact).unwrap().tap();
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
            targets: crate::targets::TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: None,
                convoke_taps: None,
                improvise_taps: Some(vec![artifact]),
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(s.stack_is_empty(),
            "already-tapped artifact must be rejected");
    }

    #[test]
    fn improvise_cannot_use_non_artifact() {
        // Creature (not an artifact) — reject.
        let mut registry = CardRegistry::new();
        let imp_card = register_improvise_spell(
            &mut registry, "Imp-Sorc", "{1}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, imp_card);
        let creature = put_creature(&mut s, 0, crate::types::ColorSet::white());
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
            targets: crate::targets::TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: None,
                convoke_taps: None,
                improvise_taps: Some(vec![creature]),
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(s.stack_is_empty(),
            "non-artifact creature must be rejected for improvise");
        assert!(!s.objects.get(creature).unwrap().is_tapped());
    }

    #[test]
    fn improvise_cannot_use_opponent_artifact() {
        let mut registry = CardRegistry::new();
        let imp_card = register_improvise_spell(
            &mut registry, "Imp-Sorc", "{1}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, imp_card);
        let opp_art = put_artifact(&mut s, /*player=*/ 1);
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
            targets: crate::targets::TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: None,
                convoke_taps: None,
                improvise_taps: Some(vec![opp_art]),
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(s.stack_is_empty(),
            "opponent's artifact must be rejected");
    }

    #[test]
    fn improvise_duplicate_artifact_rejected() {
        let mut registry = CardRegistry::new();
        let imp_card = register_improvise_spell(
            &mut registry, "Imp-Sorc", "{2}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, imp_card);
        let a1 = put_artifact(&mut s, 0);
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
            targets: crate::targets::TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: None,
                convoke_taps: None,
                improvise_taps: Some(vec![a1, a1]),
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(s.stack_is_empty(),
            "duplicate artifact id must be rejected");
    }

    #[test]
    fn improvise_over_generic_not_legal() {
        // Cost {1}, agent tries to tap 2 artifacts — reject.
        let mut registry = CardRegistry::new();
        let imp_card = register_improvise_spell(
            &mut registry, "Imp-Sorc", "{1}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, imp_card);
        let a1 = put_artifact(&mut s, 0);
        let a2 = put_artifact(&mut s, 0);
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
            targets: crate::targets::TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: None,
                convoke_taps: None,
                improvise_taps: Some(vec![a1, a2]),
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(s.stack_is_empty(),
            "improvise count > generic must be rejected");
    }

    #[test]
    fn improvise_zero_artifacts_equivalent_to_normal_cast() {
        let mut registry = CardRegistry::new();
        let imp_card = register_improvise_spell(
            &mut registry, "Imp-Sorc", "{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, imp_card);
        put_artifact(&mut s, 0);
        give_mana(&mut s, 0, "{U}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;
        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let zero = actions.iter().find(|a| matches!(a,
            Action::CastSpell {
                object_id,
                cost_reductions: crate::actions::CostReductions {
                    improvise_taps: Some(v), ..
                },
                ..
            } if *object_id == src && v.is_empty()))
            .expect("zero-improvise cast must be legal")
            .clone();
        let (s, _) = step(s, zero, &registry);
        assert!(!s.stack_is_empty());
    }

    #[test]
    fn improvise_with_summoning_sick_artifact_creature() {
        // Artifact creature with sickness: tap-as-cost is unaffected.
        let mut registry = CardRegistry::new();
        let imp_card = register_improvise_spell(
            &mut registry, "Imp-Sorc", "{1}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, imp_card);
        let art_creature = put_artifact_creature(&mut s, 0);
        s.objects.get_mut(art_creature).unwrap().status.summoning_sick = true;
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let cast = Action::CastSpell {
            object_id: src,
            targets: crate::targets::TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: crate::actions::ManaPaymentPlan::default(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: crate::actions::CastModifier::None,
            cost_reductions: crate::actions::CostReductions {
                delve_exiles: None,
                convoke_taps: None,
                improvise_taps: Some(vec![art_creature]),
            },
        };
        let (s, _) = step(s, cast, &registry);
        assert!(!s.stack_is_empty(),
            "sickness must not block improvise");
        assert!(s.objects.get(art_creature).unwrap().is_tapped());
    }

    #[test]
    fn legal_actions_enumerates_improvise_subsets() {
        // {2} spell, 2 distinct-id artifacts → expect k=0, 1, 2
        // actions all enumerated (k=0 from delve track baseline,
        // k=1, 2 from improvise track).
        let mut registry = CardRegistry::new();
        let imp_card = register_improvise_spell(
            &mut registry, "Imp-Sorc", "{2}");
        let a1_card = {
            use crate::registry::CardDefinition;
            let name = registry.interner_mut().intern("A1");
            let def = CardDefinition::new(name, Characteristics {
                colors: crate::types::ColorSet::new(),
                types: TypeLine::ARTIFACT.into(),
                ..Default::default()
            });
            registry.register(def)
        };
        let a2_card = {
            use crate::registry::CardDefinition;
            let name = registry.interner_mut().intern("A2");
            let def = CardDefinition::new(name, Characteristics {
                colors: crate::types::ColorSet::new(),
                types: TypeLine::ARTIFACT.into(),
                ..Default::default()
            });
            registry.register(def)
        };
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, imp_card);
        state_allocate_creature(&mut s, 0, a1_card, &registry);
        state_allocate_creature(&mut s, 0, a2_card, &registry);
        // Mana covers the post-reduction cost at every k (0, 1, 2).
        give_mana(&mut s, 0, "{2}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;

        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let imp_non_empty: Vec<_> = actions.iter().filter(|a| matches!(a,
            Action::CastSpell {
                object_id,
                cost_reductions: crate::actions::CostReductions {
                    improvise_taps: Some(v), ..
                },
                ..
            } if *object_id == src && !v.is_empty())).collect();
        // Two distinct artifacts + {2} cost → at least k=1 (two
        // separate card_ids → two subsets at k=1) + k=2 (one subset).
        assert!(imp_non_empty.len() >= 2,
            "expect multiple improvise actions (got {})", imp_non_empty.len());
    }

    #[test]
    fn improvise_dedup_identical_artifacts() {
        // 3 identical artifacts, cost {3}. k=1 → 1 action (not 3).
        let mut registry = CardRegistry::new();
        let imp_card = register_improvise_spell(
            &mut registry, "Imp-Sorc", "{3}");
        let art_card = {
            use crate::registry::CardDefinition;
            let name = registry.interner_mut().intern("Identical-Art");
            let def = CardDefinition::new(name, Characteristics {
                colors: crate::types::ColorSet::new(),
                types: TypeLine::ARTIFACT.into(),
                ..Default::default()
            });
            registry.register(def)
        };
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, imp_card);
        for _ in 0..3 {
            state_allocate_creature(&mut s, 0, art_card, &registry);
        }
        // Mana to cover the post-reduction cost so k=1 is feasible.
        give_mana(&mut s, 0, "{2}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;
        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let k1: usize = actions.iter().filter(|a| matches!(a,
            Action::CastSpell {
                object_id,
                cost_reductions: crate::actions::CostReductions {
                    improvise_taps: Some(v), ..
                },
                ..
            } if *object_id == src && v.len() == 1)).count();
        assert_eq!(k1, 1, "3 identical artifacts must dedup to k=1 single action");
    }

    #[test]
    fn snapcaster_style_granted_improvise() {
        use crate::layers::{ContinuousEffect, Duration};
        let mut registry = CardRegistry::new();
        let plain = {
            use crate::mana::ManaCost;
            use crate::registry::{CardDefinition, SpellAbilityDef};
            fn noop(
                _: &GameState, _: &crate::stack::StackEntry, _: &CardRegistry,
            ) -> Vec<crate::effects::Effect> { vec![] }
            let name = registry.interner_mut().intern("Plain-Improvise-Grant");
            let def = CardDefinition::new(name, Characteristics {
                mana_cost: Some(ManaCost::parse("{2}").unwrap()),
                colors: crate::types::ColorSet::new(),
                types: TypeLine::SORCERY.into(),
                ..Default::default()
            }).with_spell_ability(SpellAbilityDef {
                text: "Does nothing.".into(),
                target_requirements: vec![],
            modal: None,
                effect: noop,
            });
            registry.register(def)
        };
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, plain);
        put_artifact(&mut s, 0);
        s.add_continuous_effect(ContinuousEffect::grant_keyword(
            /*source=*/ 999, src,
            crate::effects::KeywordAbility::Improvise,
            Duration::EndOfTurn,
        ));
        give_mana(&mut s, 0, "{1}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;
        let actions = crate::legal_actions::legal_actions(&s, &registry);
        let imp_actions: Vec<_> = actions.iter().filter(|a| matches!(a,
            Action::CastSpell {
                object_id,
                cost_reductions: crate::actions::CostReductions {
                    improvise_taps: Some(v), ..
                },
                ..
            } if *object_id == src && !v.is_empty())).collect();
        assert!(!imp_actions.is_empty(),
            "granted-improvise card must offer improvise actions");
    }

    #[test]
    fn improvise_with_delve_not_enumerated_jointly() {
        let mut registry = CardRegistry::new();
        let dual_id = {
            use crate::mana::ManaCost;
            use crate::registry::{CardDefinition, SpellAbilityDef};
            fn noop(
                _: &GameState, _: &crate::stack::StackEntry, _: &CardRegistry,
            ) -> Vec<crate::effects::Effect> { vec![] }
            let name = registry.interner_mut().intern("Delve+Improvise");
            let mut def = CardDefinition::new(name, Characteristics {
                mana_cost: Some(ManaCost::parse("{2}").unwrap()),
                colors: crate::types::ColorSet::new(),
                types: TypeLine::SORCERY.into(),
                ..Default::default()
            }).with_spell_ability(SpellAbilityDef {
                text: "Delve. Improvise.".into(),
                target_requirements: vec![],
            modal: None,
                effect: noop,
            });
            def.base_characteristics.keywords.push(
                crate::effects::KeywordAbility::Delve);
            def.base_characteristics.keywords.push(
                crate::effects::KeywordAbility::Improvise);
            registry.register(def)
        };
        let filler = register_delve_spell(&mut registry, "Filler", "{U}");
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, dual_id);
        put_in_graveyard(&mut s, &registry, 0, filler);
        put_artifact(&mut s, 0);
        give_mana(&mut s, 0, "{2}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;
        let actions = crate::legal_actions::legal_actions(&s, &registry);
        for a in &actions {
            if let Action::CastSpell {
                object_id,
                cost_reductions: crate::actions::CostReductions {
                    delve_exiles, improvise_taps, ..
                },
                ..
            } = a {
                if *object_id != src { continue; }
                let d = delve_exiles.as_ref().is_some_and(|v| !v.is_empty());
                let i = improvise_taps.as_ref().is_some_and(|v| !v.is_empty());
                assert!(!(d && i),
                    "v1 must not enumerate joint delve+improvise actions");
            }
        }
    }

    #[test]
    fn improvise_with_convoke_not_enumerated_jointly() {
        // Artifact creature qualifies for both — but v1 enumerates
        // neither track when both keywords are present (mutual
        // exclusion across all three cost-reduction tracks).
        let mut registry = CardRegistry::new();
        let dual_id = {
            use crate::mana::ManaCost;
            use crate::registry::{CardDefinition, SpellAbilityDef};
            fn noop(
                _: &GameState, _: &crate::stack::StackEntry, _: &CardRegistry,
            ) -> Vec<crate::effects::Effect> { vec![] }
            let name = registry.interner_mut().intern("Convoke+Improvise");
            let mut def = CardDefinition::new(name, Characteristics {
                mana_cost: Some(ManaCost::parse("{2}").unwrap()),
                colors: crate::types::ColorSet::new(),
                types: TypeLine::SORCERY.into(),
                ..Default::default()
            }).with_spell_ability(SpellAbilityDef {
                text: "Convoke. Improvise.".into(),
                target_requirements: vec![],
            modal: None,
                effect: noop,
            });
            def.base_characteristics.keywords.push(
                crate::effects::KeywordAbility::Convoke);
            def.base_characteristics.keywords.push(
                crate::effects::KeywordAbility::Improvise);
            registry.register(def)
        };
        let mut s = GameState::new(2, 0);
        let src = put_in_hand(&mut s, &registry, 0, dual_id);
        put_artifact_creature(&mut s, 0);
        give_mana(&mut s, 0, "{2}");
        s.priority.give_to(0);
        s.turn.phase = crate::turn::Phase::PreCombatMain;
        s.turn.step = crate::turn::Step::Main;
        let actions = crate::legal_actions::legal_actions(&s, &registry);
        for a in &actions {
            if let Action::CastSpell {
                object_id,
                cost_reductions: crate::actions::CostReductions {
                    convoke_taps, improvise_taps, ..
                },
                ..
            } = a {
                if *object_id != src { continue; }
                let c = convoke_taps.as_ref().is_some_and(|v| !v.is_empty());
                let i = improvise_taps.as_ref().is_some_and(|v| !v.is_empty());
                assert!(!(c && i),
                    "v1 must not enumerate joint convoke+improvise actions");
            }
        }
    }
}
