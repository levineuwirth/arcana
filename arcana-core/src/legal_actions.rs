//! [`legal_actions`] — the single source of truth for "what can the
//! priority-holder do right now?" Drives the AI interface: every
//! [`crate::engine::EngineYield::PendingDecision`] presents this list
//! to the agent.
//!
//! Addendum Section 7 / Listing 13, Phase 1 Task #14. Depends on
//! tasks 6 (state), 8 (actions), 9 (targets), 10 (priority),
//! 11 (stack), 12 (mana solver).
//!
//! # Dispatch
//!
//! 1. Game over → no actions.
//! 2. A pending [`SpecialAction`] (mulligan, discard-to-hand-size,
//!    choose-first-player) preempts normal priority and dictates the
//!    legal-action set.
//! 3. Combat declaration sub-steps ([`CombatPhase::DeclareAttackers`]
//!    / [`CombatPhase::DeclareBlockers`]) ask for a one-shot batch
//!    action from the appropriate player (active / defender).
//! 4. Otherwise: normal priority window.
//!    - [`Action::PassPriority`] and [`Action::Concede`] are always
//!      available.
//!    - [`Action::PlayLand`] when the active player in their main
//!      phase with an empty stack and `land_plays_remaining > 0`.
//!    - [`Action::CastSpell`] for each castable spell in hand × each
//!      valid mana-payment plan (see Task #12).
//!
//! # Scope and known gaps (Phase 1)
//!
//! Several branches are **placeholders** until later tasks land:
//!
//! - **Target enumeration, mode choices, X values** require the card
//!   registry's `TargetRequirement`s and modal-clause counts. Today,
//!   each cast is emitted with an empty `TargetSelection`, no modes,
//!   and `x_value = None`. This is semantically incomplete — the AI
//!   interface will return the right action *shape* but not the full
//!   action *space*. Registry wiring (Tasks #15–#16) fills it in.
//! - **Activated abilities** need the same registry lookup. Stubbed.
//! - **Alt-cost casts** (flashback, foretell, adventure) need card
//!   keyword inspection. Stubbed.
//! - **Full combat enumeration** (power-set of attackers × defender
//!   choices, power-set of blocker-to-attacker mappings) is
//!   exponential; for Phase 1 we emit the empty declaration plus
//!   *singleton* declarations (one attacker / one blocker at a time).
//!   Integration tests that want richer combat scenarios construct
//!   `DeclareAttackers`/`DeclareBlockers` actions manually.
//! - **Haste, Defender, Summoning sickness** — honored via
//!   [`GameState::has_keyword`]. Haste overrides summoning sickness;
//!   Defender forbids attacking.
//! - **Flash** — honored: a spell with Flash can be cast at
//!   instant speed even by its controller outside their main phase.

use crate::actions::{Action, ChoiceAction};
use crate::combat::{AttackerDeclaration, BlockerDeclaration, CombatPhase, DefendingEntity};
use crate::mana::{SpendContext, enumerate_payment_plans};
use crate::objects::ObjectId;
use crate::priority::SpecialAction;
use crate::registry::{ActivationCost, CardRegistry};
use crate::state::GameState;
use crate::targets::{TargetRequirement, TargetSelection};
use crate::types::PlayerId;
use crate::zones::Zone;

// =============================================================================
// Entry point
// =============================================================================

/// Enumerate every legal [`Action`] for the current priority-holder.
///
/// The returned vector is deterministic: object ids are iterated in
/// ascending order and action families appear in a fixed sequence
/// (pass → concede → play-land → casts). This matters for replay and
/// test reproducibility.
pub fn legal_actions(state: &GameState, registry: &CardRegistry) -> Vec<Action> {
    if state.is_game_over() {
        return Vec::new();
    }
    // A pending mid-resolution choice preempts every other action family.
    // Only `SubmitResolutionChoice` (with the matching id) and
    // `Concede` are legal.
    if state.pending_choice.is_some() {
        return legal_resolution_choice_actions(state);
    }
    let player = state.priority_player();

    if let Some(special) = &state.priority.special_action {
        return legal_special_actions(state, player, special);
    }

    if let Some(combat_actions) = legal_combat_declaration_actions(state, player) {
        return combat_actions;
    }

    legal_priority_actions(state, player, registry)
}

/// Enumerate legal responses to `state.pending_choice`. Returns a
/// single canonical answer per choice kind (combinatorial fan-outs
/// like every ordering of OrderCards are pruned to the obvious
/// default — agent layer is expected to submit its preferred answer
/// directly rather than iterate). Always includes `Concede`.
fn legal_resolution_choice_actions(state: &GameState) -> Vec<Action> {
    use crate::actions::{ChoiceResponse, ChoiceKind, CardDestination};
    let pending = state.pending_choice.as_ref().unwrap();
    let id = pending.id;

    let mut out: Vec<Action> = Vec::new();
    match &pending.kind {
        ChoiceKind::OrderCards { cards, allowed } => {
            // Canonical answer: every card → first allowed destination
            // (usually TopOfLibrary, so effectively identity).
            let dest = allowed.first().copied()
                .unwrap_or(CardDestination::TopOfLibrary);
            let placements: Vec<(ObjectId, CardDestination)> = cards.iter()
                .map(|id| (*id, dest)).collect();
            out.push(Action::SubmitResolutionChoice {
                id,
                response: ChoiceResponse::OrderCards { placements },
            });
        }
        ChoiceKind::PickCards { candidates, min, .. } => {
            // Canonical answer: pick the lowest-id `min` candidates.
            let mut sorted = candidates.clone();
            sorted.sort();
            let picked: Vec<ObjectId> = sorted.into_iter()
                .take(*min as usize).collect();
            out.push(Action::SubmitResolutionChoice {
                id,
                response: ChoiceResponse::PickCards { picked },
            });
        }
        ChoiceKind::DistributeCounters { among, total, .. } => {
            // Canonical: all to first target.
            let mut distribution: Vec<(ObjectId, u32)> = Vec::new();
            if let Some(first) = among.first() {
                distribution.push((*first, *total));
            }
            out.push(Action::SubmitResolutionChoice {
                id,
                response: ChoiceResponse::DistributeCounters { distribution },
            });
        }
        ChoiceKind::DistributeDamage { among, total, .. } => {
            let mut distribution: Vec<(ObjectId, u32)> = Vec::new();
            if let Some(first) = among.first() {
                distribution.push((*first, *total));
            }
            out.push(Action::SubmitResolutionChoice {
                id,
                response: ChoiceResponse::DistributeDamage { distribution },
            });
        }
        ChoiceKind::PayOrDecline { .. } => {
            for pay in [true, false] {
                out.push(Action::SubmitResolutionChoice {
                    id,
                    response: ChoiceResponse::PayOrDecline { pay },
                });
            }
        }
        ChoiceKind::YesNo { .. } => {
            for answer in [true, false] {
                out.push(Action::SubmitResolutionChoice {
                    id,
                    response: ChoiceResponse::YesNo { answer },
                });
            }
        }
        ChoiceKind::PickPlayer { candidates } => {
            for p in candidates {
                out.push(Action::SubmitResolutionChoice {
                    id,
                    response: ChoiceResponse::PickPlayer { picked: *p },
                });
            }
        }
        ChoiceKind::ChooseTargets { source } => {
            // Requirements live on the companion state slot (kept off
            // the ChoiceKind variant because TargetRequirement carries
            // fn-pointer filters and isn't Hash/Eq/Serialize).
            if let Some(reqs) = state.pending_target_requirements.as_ref() {
                let source_controller = state.objects.get(*source)
                    .map(|o| o.controller).unwrap_or(0);
                for selection in
                    enumerate_target_selections(reqs, state, source_controller)
                {
                    out.push(Action::SubmitResolutionChoice {
                        id,
                        response: ChoiceResponse::ChooseTargets { selection },
                    });
                }
            }
        }
    }

    // Concede is always legal (spec §41.6 R3).
    out.push(Action::Concede);
    out
}

// =============================================================================
// Special-action windows
// =============================================================================

fn legal_special_actions(
    state: &GameState,
    player: PlayerId,
    special: &SpecialAction,
) -> Vec<Action> {
    match special {
        SpecialAction::MulliganDecision => {
            vec![Action::MulliganKeep, Action::MulliganAgain]
        }
        SpecialAction::DiscardToHandSize => {
            // Enumerate each card in hand as a candidate to discard.
            // The engine collects these one at a time until hand is
            // within limit (CR 514.1).
            sorted_ids_in_zone(state, Zone::Hand(player))
                .into_iter()
                .map(|id| Action::MakeChoice(ChoiceAction::ChooseObject(id)))
                .collect()
        }
        SpecialAction::ChooseFirstPlayer => {
            (0..state.num_players())
                .map(|p| Action::MakeChoice(ChoiceAction::ChoosePlayer(p)))
                .collect()
        }
        // BottomCards asks for a Vec<ObjectId> of a specific length;
        // enumerating every ordered selection blows up combinatorially.
        // Rather than returning nothing (which would panic the engine
        // loop on "no legal actions"), emit a single canonical action
        // that picks the lowest-id cards in hand. Agents that want
        // a real choice can ignore this and build `BottomCards(...)`
        // themselves with any legal selection.
        SpecialAction::LondonMulliganBottomCards(n) => {
            let hand = sorted_ids_in_zone(state, Zone::Hand(player));
            let count = (*n as usize).min(hand.len());
            let pick: Vec<_> = hand.into_iter().take(count).collect();
            vec![Action::BottomCards(pick)]
        }
        SpecialAction::Sideboarding => Vec::new(),
    }
}

// =============================================================================
// Combat declarations
// =============================================================================

fn legal_combat_declaration_actions(
    state: &GameState,
    player: PlayerId,
) -> Option<Vec<Action>> {
    let combat = state.combat.as_ref()?;
    match combat.phase {
        CombatPhase::DeclareAttackers if player == state.active_player() => {
            Some(enumerate_attacker_declarations(state, player))
        }
        CombatPhase::DeclareBlockers if player != state.active_player() => {
            Some(enumerate_blocker_declarations(state, player))
        }
        _ => None,
    }
}

/// Emit the empty declaration (no attacks) plus one declaration per
/// eligible attacker targeting each possible defender. Multi-attacker
/// combinations are deferred (see module docs).
fn enumerate_attacker_declarations(state: &GameState, active: PlayerId) -> Vec<Action> {
    let mut out = Vec::new();
    out.push(Action::DeclareAttackers { attackers: Vec::new() });

    let opponents: Vec<PlayerId> = state.opponents_of(active).collect();
    if opponents.is_empty() { return out; }

    let mut eligible: Vec<ObjectId> = state.objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == active && can_attack(state, o))
        .map(|o| o.id)
        .collect();
    eligible.sort();

    for atk in eligible {
        // CR 701.38a — a goaded creature can't attack any player who
        // is goading it. Planeswalker defenders are still legal (Goad
        // restricts only the "choose the defending *player*" branch).
        let goaders = state.goaders_of(atk);
        // Also enumerate planeswalker defenders currently on the
        // battlefield (each could be attacked instead of its
        // controller).
        for &opp in &opponents {
            if !goaders.contains(&opp) {
                out.push(Action::DeclareAttackers {
                    attackers: vec![AttackerDeclaration {
                        attacker: atk,
                        defending: DefendingEntity::Player(opp),
                    }],
                });
            }

            let mut pw_defenders: Vec<ObjectId> = state.objects
                .objects_in_zone(Zone::Battlefield)
                .filter(|o| o.controller == opp && o.is_planeswalker())
                .map(|o| o.id)
                .collect();
            pw_defenders.sort();
            for pw in pw_defenders {
                out.push(Action::DeclareAttackers {
                    attackers: vec![AttackerDeclaration {
                        attacker: atk,
                        defending: DefendingEntity::Planeswalker(pw),
                    }],
                });
            }
        }
    }
    out
}

/// Emit the empty declaration (no blocks) plus one declaration per
/// (eligible blocker, attacker being blocked) pair. Multi-blocker
/// combinations are deferred.
fn enumerate_blocker_declarations(state: &GameState, defender: PlayerId) -> Vec<Action> {
    let mut out = Vec::new();
    out.push(Action::DeclareBlockers { blockers: Vec::new() });

    let combat = match state.combat.as_ref() {
        Some(c) => c,
        None => return out,
    };

    let mut eligible_blockers: Vec<ObjectId> = state.objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == defender && can_block(state, o))
        .map(|o| o.id)
        .collect();
    eligible_blockers.sort();

    let mut attackers: Vec<ObjectId> = combat.attackers.iter()
        .map(|a| a.object_id).collect();
    attackers.sort();

    for blk in eligible_blockers {
        for &atk in &attackers {
            if !can_block_attacker(state, blk, atk) { continue; }
            out.push(Action::DeclareBlockers {
                blockers: vec![BlockerDeclaration {
                    blocker: blk,
                    blocking: atk,
                }],
            });
        }
    }
    out
}

/// Per-pair block-legality check. Enforces evergreen pair restrictions
/// (Flying/Reach, Menace). Singleton-only for Phase 1: since our
/// enumerator only emits one-blocker declarations, Menace always
/// rejects here — any attacker with Menace requires two blockers. Tests
/// that want to set up legal Menace blocks construct the
/// `DeclareBlockers` action manually.
fn can_block_attacker(state: &GameState, blocker: ObjectId, attacker: ObjectId) -> bool {
    use crate::effects::KeywordAbility;
    // CR 702.9a — Flying: a creature with flying can be blocked only
    // by creatures with flying and/or reach.
    if state.has_keyword(attacker, &KeywordAbility::Flying)
        && !state.has_keyword(blocker, &KeywordAbility::Flying)
        && !state.has_keyword(blocker, &KeywordAbility::Reach)
    {
        return false;
    }
    // CR 702.110a — Menace: a creature with menace can't be blocked
    // except by two or more creatures. Singleton enumerator => reject.
    if state.has_keyword(attacker, &KeywordAbility::Menace) {
        return false;
    }
    // CR 702.16b — Protection: attacker can't be blocked by a creature
    // matching its Protection quality, and vice versa.
    if let Some(blk_chars) = state.compute_characteristics(blocker) {
        if state.is_protected_from(attacker, &blk_chars) {
            return false;
        }
    }
    if let Some(atk_chars) = state.compute_characteristics(attacker) {
        if state.is_protected_from(blocker, &atk_chars) {
            return false;
        }
    }
    true
}

fn can_attack(state: &GameState, obj: &crate::objects::GameObject) -> bool {
    use crate::effects::KeywordAbility;
    if !obj.is_creature() || !obj.zone.is_battlefield() || obj.is_tapped() {
        return false;
    }
    // CR 702.3b — Defender: can't attack.
    if state.has_keyword(obj.id, &KeywordAbility::Defender) {
        return false;
    }
    // Pacifism-style "can't attack" restriction.
    if state.cant_attack(obj.id) {
        return false;
    }
    // CR 302.1 summoning sickness is overridden by Haste (CR 702.10b).
    if obj.status.summoning_sick
        && !state.has_keyword(obj.id, &KeywordAbility::Haste)
    {
        return false;
    }
    true
}

fn can_block(_state: &GameState, obj: &crate::objects::GameObject) -> bool {
    // TODO(keywords): honor Flying/Reach, Menace, Shadow, Protection
    // From, Fear, Landwalk, etc. (Flying/Reach/Menace handled in the
    // per-pairing filter below.)
    obj.is_creature() && obj.zone.is_battlefield() && !obj.is_tapped()
}

// =============================================================================
// Normal priority window
// =============================================================================

fn legal_priority_actions(
    state: &GameState,
    player: PlayerId,
    registry: &CardRegistry,
) -> Vec<Action> {
    let mut actions = Vec::with_capacity(16);

    // Always-legal: pass priority, concede.
    actions.push(Action::PassPriority);
    actions.push(Action::Concede);

    // Play a land.
    if can_play_land_now(state, player) {
        for id in sorted_ids_in_zone(state, Zone::Hand(player)) {
            let obj = state.objects.get(id).unwrap();
            if obj.is_land() {
                actions.push(Action::PlayLand { object_id: id });
            }
        }
    }

    // Cast spells from hand.
    let sorcery_speed_ok = player == state.active_player()
        && state.turn.is_main_phase()
        && state.stack_is_empty();

    for id in sorted_ids_in_zone(state, Zone::Hand(player)) {
        let obj = state.objects.get(id).unwrap();
        if obj.is_land() { continue; } // not a cast
        // CR 702.8 — Flash lets a non-instant be cast any time its
        // controller could cast an instant.
        let is_instant_speed = obj.is_instant()
            || state.has_keyword(id, &crate::effects::KeywordAbility::Flash);
        if !is_instant_speed && !sorcery_speed_ok { continue; }

        let Some(printed_cost) = obj.characteristics.mana_cost.clone() else { continue; };

        let ctx = SpendContext::for_spell(
            obj.characteristics.types, obj.characteristics.colors);

        // Target requirements and mode combinations. Modal spells
        // (CR 700.2) generate `C(N,k)` distinct mode subsets for each
        // `k` in `[min_modes, max_modes]`; each subset has its own
        // effective target list. Non-modal spells produce a single
        // subset (the empty mode choice).
        //
        // DEBT: many-target modal cards (Aminatou's Augury, etc.) will
        // hit the Cartesian blow-up of (mode_combo) × (target_selection);
        // reuse the dedup-by-characteristic-equivalence helper from
        // delve/improvise enumeration when that regime is reached.
        let spell_ability = registry.get(obj.card_id)
            .and_then(|def| def.spell_ability.as_ref());
        let mode_combos: Vec<Vec<crate::stack::ModeChoice>> =
            match spell_ability.and_then(|sa| sa.modal.as_ref()) {
                None => vec![Vec::new()],
                Some(modal) => enumerate_mode_combinations(modal)
                    .into_iter().map(|c| vec![c]).collect(),
            };

        // X-value enumeration (CR 107.3 / 601.2b). If the cost has
        // `{X}`, the caster picks a non-negative integer at cast
        // time. Upper bound: total mana in pool (a safe over-
        // approximation — the mana solver filters infeasible
        // expansions when the colored fixed-portion leaves
        // insufficient generic room). Each X value becomes a
        // distinct emitted action — X=5 and X=6 produce different
        // game outcomes since effects reference the X variable.
        let has_x = printed_cost.x_count() > 0;
        let x_values: Vec<u32> = if has_x {
            let max_x = state.player(player).mana_pool.total() as u32;
            (0..=max_x).collect()
        } else {
            vec![0]  // sentinel; x_value field set to None below
        };

        // Cost reductions: delve (CR 702.66), convoke (CR 702.51),
        // improvise (CR 702.127). These compose at the rules level
        // but are enumerated as independent tracks for v1 — no
        // printed card has more than one, and joint-enumeration
        // would combinatorially cross-product.
        //
        //   each track emitted when its keyword is present
        //   no joint (delve+convoke / delve+improvise / convoke+
        //   improvise / all-three) enumeration in v1
        //
        // When a real card combines keywords, extend here to either
        // emit joint products with careful dedup, or switch to the
        // Shape B-full substep pipeline.
        let delve_available = crate::engine::has_delve(state, id);
        let convoke_available = crate::engine::has_convoke(state, id);
        let improvise_available = crate::engine::has_improvise(state, id);
        // Kicker availability (CR 702.32). If present, we enumerate an
        // unkicked and a kicked variant for each (mode, x, reduction)
        // tuple. The kicker mana cost is concatenated onto the base
        // cost so the mana solver sizes the plan against the combined
        // total; the `AdditionalCostPayment::Kicker` flag on the
        // emitted action is a snapshot marker only (the actual mana
        // is in `mana_payment`).
        let kicker_cost_opt = crate::engine::kicker_cost_for(state, id);

        for modes in &mode_combos {
            // Effective target requirements for this mode combo.
            // Non-modal spells' `modes` is empty — the helper returns
            // the flat `target_requirements`. Modal spells concatenate
            // the chosen clauses' requirements in card order.
            let effective_reqs: Vec<TargetRequirement> = match spell_ability {
                Some(sa) => crate::registry::effective_target_requirements(sa, modes),
                None => Vec::new(),
            };
            let target_selections =
                enumerate_target_selections(&effective_reqs, state, player);
            // Mode combo has a clause with no legal targets → skip.
            if target_selections.is_empty() { continue; }

        for &x in &x_values {
            // Expand X into Generic(x) before cost-reduction tracks
            // see the cost. Delve / improvise reduce generic; X
            // expansion creates generic to reduce, so the ordering
            // is X-first-then-reductions.
            let base_cost = if has_x {
                printed_cost.with_x_expanded(x)
            } else {
                printed_cost.clone()
            };
            let x_value = if has_x { Some(x) } else { None };

            // Kicker fork. For a kickable spell we enumerate both the
            // unkicked and the kicked track; the kicked variant adds
            // the kicker cost's components to the base cost and
            // stamps `AdditionalCostPayment::Kicker` into the emitted
            // action so the stack entry can be flagged at apply time.
            let mut kicker_variants: Vec<(
                crate::mana::ManaCost,
                Vec<crate::actions::AdditionalCostPayment>,
            )> = vec![(base_cost.clone(), Vec::new())];
            if let Some(ref kc) = kicker_cost_opt {
                let mut kicked_cost = base_cost.clone();
                kicked_cost.components.extend(kc.components.iter().copied());
                kicker_variants.push((
                    kicked_cost,
                    vec![crate::actions::AdditionalCostPayment::Kicker],
                ));
            }

            for (cost, kicker_additional) in &kicker_variants {
            let gen_cap = generic_total(cost);

            // --- Delve track (only generic pips reducible) ----------
            let delve_subsets: Vec<Vec<ObjectId>> =
                if delve_available && gen_cap > 0 {
                    enumerate_delve_subsets(state, player, gen_cap as usize)
                } else {
                    vec![Vec::new()]
                };
            for subset in &delve_subsets {
                let reduced_cost = if subset.is_empty() {
                    cost.clone()
                } else {
                    reduce_generic_cost(cost, subset.len() as u32)
                };
                let plans = enumerate_payment_plans(
                    &reduced_cost, &state.player(player).mana_pool, None, &ctx);
                for plan in plans {
                    for targets in &target_selections {
                        actions.push(Action::CastSpell {
                            object_id: id,
                            targets: targets.clone(),
                            modes: modes.clone(),
                            mana_payment: plan.clone(),
                            additional_costs: kicker_additional.clone(),
                            x_value,
                            cast_modifier: crate::actions::CastModifier::None,
                            cost_reductions: crate::actions::CostReductions {
                                delve_exiles: if delve_available {
                                    Some(subset.clone())
                                } else {
                                    None
                                },
                                convoke_taps: None,
                                improvise_taps: None,
                            },
                        });
                    }
                }
            }

            // --- Convoke track --------------------------------------
            if convoke_available && !delve_available && !improvise_available {
                let pip_cap = total_pips(cost) as usize;
                let convoke_subsets = if pip_cap > 0 {
                    enumerate_convoke_subsets(state, player, pip_cap)
                } else {
                    vec![Vec::new()]
                };
                for c_subset in &convoke_subsets {
                    let assignments = enumerate_convoke_assignments(state, c_subset);
                    for assignment in assignments {
                        let Some(reduced_cost) =
                            reduce_cost_by_convoke(cost, &assignment)
                        else { continue; };
                        let plans = enumerate_payment_plans(
                            &reduced_cost, &state.player(player).mana_pool,
                            None, &ctx);
                        if plans.is_empty() { continue; }
                        let convoke_taps: Vec<crate::actions::ConvokeAssignment> =
                            c_subset.iter().zip(assignment.iter())
                                .map(|(&creature, &payment)|
                                    crate::actions::ConvokeAssignment {
                                        creature, payment,
                                    })
                                .collect();
                        for plan in &plans {
                            for targets in &target_selections {
                                actions.push(Action::CastSpell {
                                    object_id: id,
                                    targets: targets.clone(),
                                    modes: modes.clone(),
                                    mana_payment: plan.clone(),
                                    additional_costs: kicker_additional.clone(),
                                    x_value,
                                    cast_modifier:
                                        crate::actions::CastModifier::None,
                                    cost_reductions:
                                        crate::actions::CostReductions {
                                            delve_exiles: None,
                                            convoke_taps: Some(
                                                convoke_taps.clone()),
                                            improvise_taps: None,
                                        },
                                });
                            }
                        }
                    }
                }
            }

            // --- Improvise track ------------------------------------
            if improvise_available && !delve_available && !convoke_available {
                let improvise_subsets = if gen_cap > 0 {
                    enumerate_improvise_subsets(state, player, gen_cap as usize)
                } else {
                    vec![Vec::new()]
                };
                for subset in &improvise_subsets {
                    let reduced_cost = if subset.is_empty() {
                        cost.clone()
                    } else {
                        reduce_generic_cost(cost, subset.len() as u32)
                    };
                    let plans = enumerate_payment_plans(
                        &reduced_cost, &state.player(player).mana_pool, None, &ctx);
                    for plan in plans {
                        for targets in &target_selections {
                            actions.push(Action::CastSpell {
                                object_id: id,
                                targets: targets.clone(),
                                modes: modes.clone(),
                                mana_payment: plan.clone(),
                                additional_costs: kicker_additional.clone(),
                                x_value,
                                cast_modifier: crate::actions::CastModifier::None,
                                cost_reductions: crate::actions::CostReductions {
                                    delve_exiles: None,
                                    convoke_taps: None,
                                    improvise_taps: Some(subset.clone()),
                                },
                            });
                        }
                    }
                }
            }
            }
        }
        }
    }

    // Activated abilities of controlled permanents.
    actions.extend(enumerate_activation_actions(state, player, registry));

    // Flashback casts from the player's own graveyard (CR 702.33).
    // Timing reuses the same sorcery/instant check as the hand path —
    // flashback does not grant flash.
    for id in sorted_ids_in_zone(state, Zone::Graveyard(player)) {
        let obj = state.objects.get(id).unwrap();
        if obj.is_land() { continue; }
        let is_instant_speed = obj.is_instant()
            || state.has_keyword(id, &crate::effects::KeywordAbility::Flash);
        if !is_instant_speed && !sorcery_speed_ok { continue; }

        // Layer-aware lookup: honors Snapcaster-style granted flashback.
        // If a card ever has multiple flashback keywords (grants stack,
        // CR 702.33c), enumerate each separately.
        let flashback_costs = crate::engine::all_flashback_costs_for(state, id);
        if flashback_costs.is_empty() { continue; }

        let reqs: Vec<TargetRequirement> = registry.get(obj.card_id)
            .and_then(|def| def.spell_ability.as_ref())
            .map(|sa| sa.target_requirements.clone())
            .unwrap_or_default();
        let target_selections = enumerate_target_selections(&reqs, state, player);

        for printed_fb_cost in flashback_costs {
            // TODO(delve-on-flashback): no card in current Standard has
            // both flashback and delve, but composition is legal per
            // the CR. When a card arrives, enumerate delve subsets
            // here — the subtle part is excluding the cast-source
            // itself from delve candidates (its zone at cost-payment
            // time in our atomic pipeline is still Graveyard, not
            // Stack).
            let ctx = SpendContext::for_spell(
                obj.characteristics.types, obj.characteristics.colors);

            // Flashback-cost X enumeration mirrors the hand-cast
            // block. The granted flashback cost may carry its own
            // `{X}` (uncommon but legal — e.g. hypothetical granted
            // flashback on an X-cost spell via Snapcaster-style
            // effects).
            let has_x = printed_fb_cost.x_count() > 0;
            let x_values: Vec<u32> = if has_x {
                let max_x = state.player(player).mana_pool.total() as u32;
                (0..=max_x).collect()
            } else {
                vec![0]
            };

            for &x in &x_values {
                let cost = if has_x {
                    printed_fb_cost.with_x_expanded(x)
                } else {
                    printed_fb_cost.clone()
                };
                let x_value = if has_x { Some(x) } else { None };
                let plans = enumerate_payment_plans(
                    &cost, &state.player(player).mana_pool, None, &ctx);
                for plan in plans {
                    for targets in &target_selections {
                        actions.push(Action::CastSpell {
                            object_id: id,
                            targets: targets.clone(),
                            modes: Vec::new(),
                            mana_payment: plan.clone(),
                            additional_costs: Vec::new(),
                            x_value,
                            cast_modifier: crate::actions::CastModifier::Flashback,
                            cost_reductions: crate::actions::CostReductions::default(),
                        });
                    }
                }
            }
        }
    }

    // Madness casts from exile (CR 702.34). Walks the exile zone for
    // cards this player owns that were routed there via the madness
    // discard-replacement (indicated by `madness_pending=true`).
    // Each emits a CastSpell with `CastModifier::Madness` using the
    // card's madness cost.
    for id in sorted_ids_in_zone(state, Zone::Exile) {
        let Some(obj) = state.objects.get(id) else { continue; };
        if obj.owner != player { continue; }
        if !obj.madness_pending { continue; }
        if obj.is_land() { continue; }
        // Madness-cast obeys the same sorcery/instant speed rule as
        // the printed card — an instant's madness can be paid any
        // time, a sorcery's madness is still sorcery-speed.
        let is_instant_speed = obj.is_instant()
            || state.has_keyword(id, &crate::effects::KeywordAbility::Flash);
        if !is_instant_speed && !sorcery_speed_ok { continue; }

        let Some(madness_cost) = crate::engine::madness_cost_for(state, id)
            else { continue; };

        let reqs: Vec<TargetRequirement> = registry.get(obj.card_id)
            .and_then(|def| def.spell_ability.as_ref())
            .map(|sa| sa.target_requirements.clone())
            .unwrap_or_default();
        let target_selections = enumerate_target_selections(&reqs, state, player);
        if target_selections.is_empty() { continue; }

        let ctx = SpendContext::for_spell(
            obj.characteristics.types, obj.characteristics.colors);

        let has_x = madness_cost.x_count() > 0;
        let x_values: Vec<u32> = if has_x {
            let max_x = state.player(player).mana_pool.total() as u32;
            (0..=max_x).collect()
        } else {
            vec![0]
        };

        for &x in &x_values {
            let cost = if has_x {
                madness_cost.with_x_expanded(x)
            } else {
                madness_cost.clone()
            };
            let x_value = if has_x { Some(x) } else { None };
            let plans = enumerate_payment_plans(
                &cost, &state.player(player).mana_pool, None, &ctx);
            for plan in plans {
                for targets in &target_selections {
                    actions.push(Action::CastSpell {
                        object_id: id,
                        targets: targets.clone(),
                        modes: Vec::new(),
                        mana_payment: plan.clone(),
                        additional_costs: Vec::new(),
                        x_value,
                        cast_modifier: crate::actions::CastModifier::Madness,
                        cost_reductions: crate::actions::CostReductions::default(),
                    });
                }
            }
        }
    }

    // Adventure casts from hand (CR 715). Walks the hand for cards
    // whose registry definition carries an Adventure face. The
    // Adventure face's own mana cost, target requirements, and
    // type/color drive the cast (not the creature half); on
    // resolution the card routes to exile with
    // `adventure_exile_pending=true`, which opens the second
    // enumeration track below (creature cast from adventure-exile).
    //
    // Timing: the Adventure face's printed type line governs speed.
    // An Adventure instant is castable at instant speed; a sorcery
    // Adventure (uncommon but legal — e.g. "Giant's Reply") is
    // sorcery-speed. Flash on the creature face does NOT carry to
    // the Adventure half.
    for id in sorted_ids_in_zone(state, Zone::Hand(player)) {
        let Some(obj) = state.objects.get(id) else { continue; };
        let Some(def) = registry.get(obj.card_id) else { continue; };
        let Some(face) = def.alternate_face.as_ref()
            .and_then(|af| af.as_adventure()) else { continue; };
        let Some(face_cost) = face.characteristics.mana_cost.clone() else {
            continue;
        };
        let face_is_instant_speed = face.characteristics.types.is_instant();
        if !face_is_instant_speed && !sorcery_speed_ok { continue; }

        let reqs: Vec<TargetRequirement> = face.spell_ability.as_ref()
            .map(|sa| sa.target_requirements.clone())
            .unwrap_or_default();
        let target_selections = enumerate_target_selections(&reqs, state, player);
        if target_selections.is_empty() { continue; }

        let ctx = SpendContext::for_spell(
            face.characteristics.types, face.characteristics.colors);

        let has_x = face_cost.x_count() > 0;
        let x_values: Vec<u32> = if has_x {
            let max_x = state.player(player).mana_pool.total() as u32;
            (0..=max_x).collect()
        } else {
            vec![0]
        };

        for &x in &x_values {
            let cost = if has_x {
                face_cost.with_x_expanded(x)
            } else {
                face_cost.clone()
            };
            let x_value = if has_x { Some(x) } else { None };
            let plans = enumerate_payment_plans(
                &cost, &state.player(player).mana_pool, None, &ctx);
            for plan in plans {
                for targets in &target_selections {
                    actions.push(Action::CastSpell {
                        object_id: id,
                        targets: targets.clone(),
                        modes: Vec::new(),
                        mana_payment: plan.clone(),
                        additional_costs: Vec::new(),
                        x_value,
                        cast_modifier: crate::actions::CastModifier::Adventure,
                        cost_reductions: crate::actions::CostReductions::default(),
                    });
                }
            }
        }
    }

    // Adventure-creature casts from exile (CR 715). Walks exile for
    // flagged cards this player owns and emits a normal-cost cast of
    // the creature half. The exile object already carries the
    // creature-face characteristics (the resolution/counter path
    // restored them when routing to exile), so the printed mana cost
    // here is the main-face cost directly.
    for id in sorted_ids_in_zone(state, Zone::Exile) {
        let Some(obj) = state.objects.get(id) else { continue; };
        if obj.owner != player { continue; }
        if !obj.adventure_exile_pending { continue; }
        if obj.is_land() { continue; }

        let is_instant_speed = obj.is_instant()
            || state.has_keyword(id, &crate::effects::KeywordAbility::Flash);
        if !is_instant_speed && !sorcery_speed_ok { continue; }

        let Some(printed_cost) = obj.characteristics.mana_cost.clone()
            else { continue; };

        let reqs: Vec<TargetRequirement> = registry.get(obj.card_id)
            .and_then(|def| def.spell_ability.as_ref())
            .map(|sa| sa.target_requirements.clone())
            .unwrap_or_default();
        let target_selections = enumerate_target_selections(&reqs, state, player);
        if target_selections.is_empty() { continue; }

        let ctx = SpendContext::for_spell(
            obj.characteristics.types, obj.characteristics.colors);

        let has_x = printed_cost.x_count() > 0;
        let x_values: Vec<u32> = if has_x {
            let max_x = state.player(player).mana_pool.total() as u32;
            (0..=max_x).collect()
        } else {
            vec![0]
        };

        for &x in &x_values {
            let cost = if has_x {
                printed_cost.with_x_expanded(x)
            } else {
                printed_cost.clone()
            };
            let x_value = if has_x { Some(x) } else { None };
            let plans = enumerate_payment_plans(
                &cost, &state.player(player).mana_pool, None, &ctx);
            for plan in plans {
                for targets in &target_selections {
                    actions.push(Action::CastSpell {
                        object_id: id,
                        targets: targets.clone(),
                        modes: Vec::new(),
                        mana_payment: plan.clone(),
                        additional_costs: Vec::new(),
                        x_value,
                        cast_modifier:
                            crate::actions::CastModifier::AdventureCreature,
                        cost_reductions: crate::actions::CostReductions::default(),
                    });
                }
            }
        }
    }

    actions
}

/// Cartesian product of legal target choices across the requirement
/// list. Returns `[TargetSelection::new()]` (single empty selection)
/// if there are no requirements. For a single-target requirement
/// with count=Exactly(1), yields one selection per legal choice.
///
/// Phase 1 limit: only `TargetCount::Exactly(1)` and `Exactly(0)`
/// are enumerated per clause. Multi-target (`Exactly(2)`, `UpTo`,
/// `Any`, `X`) fall back to a single empty selection so we never
/// emit over-combinatorial action sets.
fn enumerate_target_selections(
    requirements: &[TargetRequirement],
    state: &GameState,
    source_controller: PlayerId,
) -> Vec<TargetSelection> {
    if requirements.is_empty() {
        return vec![TargetSelection::new()];
    }

    let mut partials: Vec<Vec<crate::targets::TargetChoice>> = vec![Vec::new()];
    for req in requirements {
        match req.count {
            crate::targets::TargetCount::Exactly(0) => {
                // Nothing to add.
            }
            crate::targets::TargetCount::Exactly(1) => {
                let choices = req.filter.enumerate_legal(state, source_controller);
                let mut next = Vec::new();
                for partial in &partials {
                    for choice in &choices {
                        // Re-check the outer controller constraint —
                        // `enumerate_legal` doesn't apply it.
                        if !req.matches_choice(choice, state, source_controller) {
                            continue;
                        }
                        let mut extended = partial.clone();
                        extended.push(choice.clone());
                        next.push(extended);
                    }
                }
                if next.is_empty() {
                    // No legal target for this clause → no legal
                    // cast (spell requires at least one target).
                    return Vec::new();
                }
                partials = next;
            }
            // Multi-target / variable-count requirements are deferred
            // for the AI flattener (Phase 2) which does proper
            // subset enumeration. The engine accepts hand-crafted
            // actions either way.
            _ => return vec![TargetSelection::new()],
        }
    }

    partials.into_iter()
        .map(|targets| TargetSelection { targets })
        .collect()
}

/// Enumerate every sorted mode subset for a modal spell whose size
/// lies in `[min_modes, max_modes]`. Bitmask walk over `2^N` subsets;
/// cheap for every real card (Cryptic Command: N=4 → 16 subsets,
/// 6 of size 2; Kolaghan's Command: N=4 → 6 of size 2). `ModalSpec`
/// construction is expected to bound `N` to a reasonable clause count.
///
/// Each returned `ModeChoice` is sorted ascending and deduplicated
/// (the bitmask walk produces this order naturally), so the
/// `ModeChoice::new` normalization is idempotent here.
fn enumerate_mode_combinations(
    modal: &crate::registry::ModalSpec,
) -> Vec<crate::stack::ModeChoice> {
    let n = modal.clauses.len();
    // Upper guardrail — a 32-bit mask covers ModalSpec sizes we'd ever
    // see on a printed card. If someone lands a card with >32 modes,
    // switch to a recursive combinations generator.
    assert!(n <= 32, "enumerate_mode_combinations: >32 modes unsupported");
    let mut out = Vec::new();
    for mask in 0u32..(1u32 << n) {
        let bits = mask.count_ones() as usize;
        if bits < modal.min_modes || bits > modal.max_modes { continue; }
        let indices: Vec<usize> = (0..n)
            .filter(|i| (mask >> i) & 1 == 1)
            .collect();
        out.push(crate::stack::ModeChoice { mode_indices: indices });
    }
    out
}

/// Enumerate every distinct subset of `candidates` of size ≤ `max_size`,
/// deduplicated by equivalence `key`. Two candidates producing the
/// same key are considered interchangeable — the enumerator emits
/// one representative subset per (key, count) multiset rather than
/// expanding every permutation. This is the difference between
/// delve-subset enumeration exploding as `C(n,k)` and as the number
/// of distinct *multisets* of equivalence classes.
///
/// Intended for cost-modifier enumeration (delve today, convoke /
/// improvise when they land — each will project to a different key
/// while reusing this shape).
///
/// Emits the empty subset first, then extends greedily through the
/// groups. Enumeration order is stable given stable iteration over
/// `candidates`.
pub(crate) fn enumerate_equivalence_subsets<T, K, F>(
    candidates: &[T],
    max_size: usize,
    mut key: F,
) -> Vec<Vec<T>>
where
    T: Copy,
    K: std::hash::Hash + Eq,
    F: FnMut(&T) -> K,
{
    let mut groups: Vec<Vec<T>> = Vec::new();
    let mut index: crate::collections::HashMap<K, usize> =
        crate::collections::HashMap::default();
    for item in candidates {
        let k = key(item);
        match index.get(&k) {
            Some(&i) => groups[i].push(*item),
            None => {
                index.insert(k, groups.len());
                groups.push(vec![*item]);
            }
        }
    }

    let mut out = Vec::new();
    let mut current = Vec::new();
    enumerate_groups(&groups, max_size, 0, &mut current, &mut out);
    out
}

fn enumerate_groups<T: Copy>(
    groups: &[Vec<T>],
    remaining: usize,
    gidx: usize,
    current: &mut Vec<T>,
    out: &mut Vec<Vec<T>>,
) {
    if gidx == groups.len() {
        out.push(current.clone());
        return;
    }
    let group = &groups[gidx];
    let max_take = group.len().min(remaining);
    for take in 0..=max_take {
        for item in group.iter().take(take) {
            current.push(*item);
        }
        enumerate_groups(groups, remaining - take, gidx + 1, current, out);
        for _ in 0..take {
            current.pop();
        }
    }
}

/// Equivalence key for a graveyard object from the perspective of
/// cost-modifier dedup. Two objects with the same key are
/// interchangeable for delve / convoke / improvise purposes.
///
/// Uses `(card_id, effective keywords)` — card_id covers the common
/// case, effective_keywords covers layer-granted variations (the
/// Snapcaster-grants-delve hypothetical). Broader characteristic
/// differences between two copies of the same card in a graveyard
/// don't exist in current Standard, but if they do in the future
/// this key is where to extend.
fn object_equivalence_key(
    state: &GameState,
    object_id: ObjectId,
) -> (crate::types::CardId, Vec<crate::effects::KeywordAbility>) {
    let card_id = state.objects.get(object_id)
        .map(|o| o.card_id).unwrap_or(0);
    let mut kws = state.effective_keywords(object_id);
    // Sort for determinism — effective_keywords' ordering is a layer
    // implementation detail, but dedup must be position-stable.
    kws.sort_by_key(|k| format!("{k:?}"));
    (card_id, kws)
}

/// Return a copy of `cost` with its generic component reduced by
/// `by`. Drains Generic pips left-to-right, consuming whole pips
/// first, then partially consuming one if needed. Colored, hybrid,
/// Phyrexian, X, Colorless, and Snow components are untouched —
/// delve / convoke / improvise cannot reduce non-generic pips.
///
/// If `by` exceeds the total generic, the result has zero generic.
/// Caller is responsible for bounding `by`; this helper clamps
/// silently (the bound check lives in [`enumerate_delve_subsets`]).
fn reduce_generic_cost(cost: &crate::mana::ManaCost, by: u32) -> crate::mana::ManaCost {
    let mut out = cost.clone();
    let mut remaining = by;
    out.components.retain_mut(|c| {
        if remaining == 0 { return true; }
        if let crate::mana::ManaCostComponent::Generic(n) = c {
            if *n <= remaining {
                remaining -= *n;
                false
            } else {
                *n -= remaining;
                remaining = 0;
                true
            }
        } else {
            true
        }
    });
    out
}

/// Sum of `Generic(n)` components in `cost`. Used as the upper bound
/// on delve exile count (CR 702.66a: each exiled card pays for `{1}`
/// generic; you cannot delve colored pips).
fn generic_total(cost: &crate::mana::ManaCost) -> u32 {
    cost.components.iter().filter_map(|c| match c {
        crate::mana::ManaCostComponent::Generic(n) => Some(*n),
        _ => None,
    }).sum()
}

/// Enumerate delve-exile subsets for a given caster's graveyard.
///
/// Bounded by `max_generic` (the spell's generic mana requirement —
/// delve can only reduce generic, never colored). Always includes the
/// empty subset (zero-delve cast). Applies characteristic-equivalence
/// dedup so N copies of the same card produce N+1 distinct counts
/// rather than 2^N subsets.
///
/// Returns `Vec::new()` if the graveyard is empty; the single-empty-
/// subset case (`vec![vec![]]`) means "delve is legal with zero
/// exiles only," useful when the spell has zero generic cost.
fn enumerate_delve_subsets(
    state: &GameState,
    player: PlayerId,
    max_generic: usize,
) -> Vec<Vec<ObjectId>> {
    let candidates: Vec<ObjectId> =
        sorted_ids_in_zone(state, Zone::Graveyard(player));
    enumerate_equivalence_subsets(
        &candidates, max_generic,
        |&id| object_equivalence_key(state, id),
    )
}

// ----- Convoke (CR 702.51) enumeration helpers -----------------------

/// Candidate creatures for convoke: the caster's untapped creature
/// permanents. Sorted by object id for stable enumeration.
fn convoke_candidate_creatures(
    state: &GameState,
    player: PlayerId,
) -> Vec<ObjectId> {
    state.objects.ids_in_zone_sorted(Zone::Battlefield)
        .into_iter()
        .filter(|&id| {
            state.objects.get(id).is_some_and(|obj|
                obj.controller == player
                && obj.characteristics.is_creature()
                && !obj.is_tapped())
        })
        .collect()
}

/// Equivalence key for a battlefield creature from the perspective
/// of convoke dedup. Two creatures with the same key are
/// interchangeable for tap-as-cost: they offer the same payment
/// options and leave equivalent game state when tapped.
///
/// Key = (card_id, colors, sorted effective keywords). Does NOT
/// include counters or attachments — two creatures that differ only
/// in counters/attachments still collapse in this key, which
/// over-dedups in edge cases. **Phase 2 limitation**: when a counter-
/// or aura-mattering card lands, extend this key.
fn convoke_creature_key(
    state: &GameState,
    id: ObjectId,
) -> (crate::types::CardId, Vec<crate::types::Color>, Vec<crate::effects::KeywordAbility>) {
    let obj = state.objects.get(id);
    let card_id = obj.map(|o| o.card_id).unwrap_or(0);
    let mut colors: Vec<crate::types::Color> = obj
        .map(|o| o.characteristics.colors.iter().collect())
        .unwrap_or_default();
    colors.sort_by_key(|c| format!("{c:?}"));
    let mut kws = state.effective_keywords(id);
    kws.sort_by_key(|k| format!("{k:?}"));
    (card_id, colors, kws)
}

/// Enumerate creature subsets for convoke, bounded by `max_pips`
/// (total pips in the spell's cost — convoke can never tap more
/// creatures than there are pips). Uses characteristic-equivalence
/// dedup, so identical creatures collapse to a per-count axis.
fn enumerate_convoke_subsets(
    state: &GameState,
    player: PlayerId,
    max_pips: usize,
) -> Vec<Vec<ObjectId>> {
    let candidates = convoke_candidate_creatures(state, player);
    enumerate_equivalence_subsets(
        &candidates, max_pips,
        |&id| convoke_creature_key(state, id),
    )
}

/// Enumerate every payment assignment over `subset`. Each creature
/// in the subset independently chooses one of its eligible payment
/// options (Generic, or Color(c) for each of its colors). Returns
/// the full Cartesian product — the caller filters by pip-coherence.
///
/// A multicolored creature contributes multiple options, which is
/// the AI's real decision ("save the multicolor for flexibility vs.
/// pay the colored pip with it"). Do NOT canonicalize here.
fn enumerate_convoke_assignments(
    state: &GameState,
    subset: &[ObjectId],
) -> Vec<Vec<crate::actions::ConvokePayment>> {
    use crate::actions::ConvokePayment;
    let per_creature: Vec<Vec<ConvokePayment>> = subset.iter()
        .map(|&id| {
            state.objects.get(id)
                .map(|o| crate::engine::convoke_eligible_payments(&o.characteristics))
                .unwrap_or_else(|| vec![ConvokePayment::Generic])
        })
        .collect();

    let mut out: Vec<Vec<ConvokePayment>> = vec![Vec::new()];
    for options in &per_creature {
        let mut next = Vec::with_capacity(out.len() * options.len());
        for partial in &out {
            for opt in options {
                let mut extended = partial.clone();
                extended.push(*opt);
                next.push(extended);
            }
        }
        out = next;
    }
    out
}

/// Subtract a convoke assignment's payments from `cost`. Returns
/// `None` if the assignment over-pays (pays more of some color than
/// the cost has, or more generic than the cost has). Otherwise
/// returns the post-convoke cost, which still needs mana-solving.
///
/// Phase 2 limit: only simple costs (Generic + Colored) are
/// supported. Hybrid / Phyrexian / monohybrid costs return `None` —
/// the convoke-hybrid case is flagged as a Phase 2-B follow-up.
fn reduce_cost_by_convoke(
    cost: &crate::mana::ManaCost,
    assignment: &[crate::actions::ConvokePayment],
) -> Option<crate::mana::ManaCost> {
    use crate::actions::ConvokePayment;
    use crate::mana::ManaCostComponent;

    let mut generic_paid: u32 = 0;
    let mut color_paid: crate::collections::HashMap<crate::types::ManaColor, u32> =
        crate::collections::HashMap::default();
    for p in assignment {
        match p {
            ConvokePayment::Generic => generic_paid += 1,
            ConvokePayment::Color(c) => {
                *color_paid.entry(*c).or_insert(0) += 1;
            }
        }
    }

    let mut out = cost.clone();
    // Reduce generic first.
    let mut remaining_generic = generic_paid;
    out.components.retain_mut(|c| {
        if remaining_generic == 0 { return true; }
        if let ManaCostComponent::Generic(n) = c {
            if *n <= remaining_generic {
                remaining_generic -= *n;
                false
            } else {
                *n -= remaining_generic;
                remaining_generic = 0;
                true
            }
        } else {
            true
        }
    });
    if remaining_generic > 0 { return None; }

    // Reduce colored pips per color.
    for (mana_color, count) in &color_paid {
        let mut to_remove = *count;
        let target_color = mana_color.as_color()?;
        out.components.retain_mut(|c| {
            if to_remove == 0 { return true; }
            if let ManaCostComponent::Colored(cc) = c {
                if *cc == target_color {
                    to_remove -= 1;
                    return false;
                }
            }
            true
        });
        if to_remove > 0 { return None; }
    }

    Some(out)
}

/// Total non-X pip count in `cost`. Used as upper bound on convoke
/// subset size.
fn total_pips(cost: &crate::mana::ManaCost) -> u32 {
    cost.components.iter().map(|c| match c {
        crate::mana::ManaCostComponent::Generic(n) => *n,
        crate::mana::ManaCostComponent::Colored(_) => 1,
        // Other variants don't cleanly accept convoke; conservative
        // bound as 1-per-component.
        _ => 1,
    }).sum()
}

// ----- Improvise (CR 702.127) enumeration helpers ---------------------

/// Candidate artifacts for improvise: the caster's untapped artifact
/// permanents (artifact creatures qualify — artifact-ness is what
/// matters, not creature-ness). Sorted by object id.
fn improvise_candidate_artifacts(
    state: &GameState,
    player: PlayerId,
) -> Vec<ObjectId> {
    state.objects.ids_in_zone_sorted(Zone::Battlefield)
        .into_iter()
        .filter(|&id| {
            state.objects.get(id).is_some_and(|obj|
                obj.controller == player
                && obj.characteristics.types.is_artifact()
                && !obj.is_tapped())
        })
        .collect()
}

/// Enumerate improvise artifact subsets for a caster, bounded by
/// `max_generic` (improvise can only reduce generic pips, same
/// constraint as delve). Uses the same `object_equivalence_key`
/// dedup as delve — artifacts with same (card_id, effective
/// keywords) collapse to per-count axis.
fn enumerate_improvise_subsets(
    state: &GameState,
    player: PlayerId,
    max_generic: usize,
) -> Vec<Vec<ObjectId>> {
    let candidates = improvise_candidate_artifacts(state, player);
    enumerate_equivalence_subsets(
        &candidates, max_generic,
        |&id| object_equivalence_key(state, id),
    )
}

/// Emit one [`Action::ActivateAbility`] per (permanent, ability,
/// target combination) the priority-holder can legally activate
/// right now. Costs that cannot be paid are filtered out.
fn enumerate_activation_actions(
    state: &GameState,
    player: PlayerId,
    registry: &CardRegistry,
) -> Vec<Action> {
    let mut out = Vec::new();
    let sorcery_speed_ok = player == state.active_player()
        && state.turn.is_main_phase()
        && state.stack_is_empty();

    // Walk every zone where an activated ability could live:
    // Battlefield for permanent abilities, Hand for cycling /
    // channel, Graveyard for future dredge / unearth. Ability-
    // specific zone matching is done inside `ability_is_activatable`
    // — the outer loop is a superset so a card in hand isn't
    // invisible to enumeration just because no one thought to
    // check Hand for activations.
    let mut candidate_ids: Vec<ObjectId> = Vec::new();
    candidate_ids.extend(state.objects.ids_in_zone_sorted(Zone::Battlefield));
    candidate_ids.extend(state.objects.ids_in_zone_sorted(Zone::Hand(player)));
    candidate_ids.extend(state.objects.ids_in_zone_sorted(Zone::Graveyard(player)));

    for id in candidate_ids {
        let Some(obj) = state.objects.get(id) else { continue; };
        // Controller/owner semantics differ per zone. Battlefield
        // uses controller; Hand and Graveyard are zone-scoped to the
        // object's owner — hand-activations by the opponent aren't a
        // thing in printed cards, and owner == controller for cards
        // in non-battlefield zones in Phase 2 (no control-changing
        // effects that reach into hand/graveyard yet).
        let activating_player = if obj.zone == Zone::Battlefield {
            obj.controller
        } else {
            obj.owner
        };
        if activating_player != player { continue; }
        let Some(def) = registry.get(obj.card_id) else { continue; };

        for (i, ability) in def.activated_abilities.iter().enumerate() {
            if !ability_is_activatable(
                state, obj, ability, player, sorcery_speed_ok,
            ) {
                continue;
            }
            // Enumerate payment plans for the mana portion of the cost.
            let ctx = SpendContext::for_activated_ability();
            let pool = &state.player(player).mana_pool;
            let plans = if ability.cost.mana_cost.is_empty() {
                vec![crate::actions::ManaPaymentPlan::empty()]
            } else {
                enumerate_payment_plans(&ability.cost.mana_cost, pool, None, &ctx)
            };
            if plans.is_empty() { continue; }

            let target_selections = enumerate_target_selections(
                &ability.target_requirements, state, player);
            if target_selections.is_empty() { continue; }

            let additional = build_additional_costs(&ability.cost, id);

            for plan in &plans {
                for targets in &target_selections {
                    out.push(Action::ActivateAbility {
                        source: id,
                        ability_index: i,
                        targets: targets.clone(),
                        mana_payment: plan.clone(),
                        additional_costs: additional.clone(),
                    });
                }
            }
        }
    }
    out
}

/// Is this ability timing-legal and cost-payable right now? Covers
/// tap-cost (must be untapped), sacrifice-cost (must exist), and
/// the mana-ability-at-any-time rule vs. sorcery-speed abilities.
fn ability_is_activatable(
    state: &GameState,
    obj: &crate::objects::GameObject,
    ability: &crate::registry::ActivatedAbilityDef,
    activator: crate::types::PlayerId,
    sorcery_speed_ok: bool,
) -> bool {
    // Zone gate: the ability's declared `activation_zone` must match
    // the object's current zone (CR 113.6). Cycling (Hand) and the
    // usual permanent abilities (Battlefield) go through the same
    // helper — the only axis that varies is which zone the object
    // lives in right now.
    if !ability.activation_zone.matches(obj.zone, obj.owner) {
        return false;
    }
    if ability.cost.tap {
        if obj.is_tapped() { return false; }
        // Tapping a creature requires no summoning-sickness for
        // non-mana abilities (CR 302.1). Mana abilities from
        // creatures are still blocked by summoning sickness — the
        // rule only exempts mana abilities from the stack, not from
        // sickness. But basic lands ignore summoning sickness for
        // mana purposes (CR 305.4, they have no creature type).
        if obj.is_creature() && obj.status.summoning_sick {
            return false;
        }
    }
    // Counter-removal cost: source must have at least `count`
    // counters of the requested kind.
    if let Some((kind, count)) = ability.cost.remove_self_counter {
        if obj.count_counters(kind) < count {
            return false;
        }
    }
    // Mana abilities can be activated at any time a player has
    // priority. Non-mana activated abilities default to sorcery
    // speed; `is_instant_speed` lifts that gate (CR 702.29a
    // Cycling). Loyalty abilities ignore `is_instant_speed` — the
    // CR 606.3 sorcery-speed rule for loyalty takes precedence and
    // is checked below.
    if !ability.is_mana_ability
        && !ability.is_instant_speed
        && !sorcery_speed_ok
    {
        return false;
    }
    // CR 606 — loyalty abilities: only the PW's controller may
    // activate, only at sorcery speed (already enforced above via
    // !is_mana_ability && sorcery_speed_ok), only with stack empty,
    // and only once per turn per PW. Summoning-sickness does NOT
    // block loyalty activations (CR 114.3 — PW sickness only
    // restricts attacking).
    if ability.is_loyalty_ability {
        if obj.controller != activator { return false; }
        if !state.stack_is_empty() { return false; }
        if state.loyalty_activated_this_turn.contains(&obj.id) {
            return false;
        }
    }
    true
}

fn build_additional_costs(
    cost: &ActivationCost,
    source: ObjectId,
) -> Vec<crate::actions::AdditionalCostPayment> {
    let mut v = Vec::new();
    if cost.sacrifice {
        v.push(crate::actions::AdditionalCostPayment::Sacrifice(source));
    }
    if cost.discard_self {
        // CR 702.29a — cycling's "Discard this card" cost. Routed
        // as the generic Discard additional-cost, which the shared
        // `apply_additional_costs` moves to graveyard + emits the
        // Discarded event.
        v.push(crate::actions::AdditionalCostPayment::Discard(source));
    }
    if cost.life > 0 {
        v.push(crate::actions::AdditionalCostPayment::PayLife(cost.life));
    }
    if let Some((kind, count)) = cost.remove_self_counter {
        v.push(crate::actions::AdditionalCostPayment::RemoveCounters {
            source, kind, count,
        });
    }
    if let Some((kind, count)) = cost.add_self_counter {
        v.push(crate::actions::AdditionalCostPayment::AddCounters {
            source, kind, count,
        });
    }
    v
}

fn can_play_land_now(state: &GameState, player: PlayerId) -> bool {
    player == state.active_player()
        && state.stack_is_empty()
        && state.turn.is_main_phase()
        && state.player(player).can_play_land()
}

fn sorted_ids_in_zone(state: &GameState, zone: Zone) -> Vec<ObjectId> {
    state.objects.ids_in_zone_sorted(zone)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::{AttackerInfo, CombatState};
    use crate::mana::{ManaCost, ManaUnit};
    use crate::objects::{Characteristics, GameObject};
    use crate::state::GameResult;
    use crate::types::*;

    // --- helpers ------------------------------------------------------------

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

    fn instant_chars() -> Characteristics {
        Characteristics {
            mana_cost: Some(ManaCost::parse("{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        }
    }

    fn sorcery_chars() -> Characteristics {
        Characteristics {
            mana_cost: Some(ManaCost::parse("{U}").unwrap()),
            colors: ColorSet::blue(),
            types: TypeLine::SORCERY.into(),
            ..Default::default()
        }
    }

    fn land_chars() -> Characteristics {
        Characteristics {
            mana_cost: None,
            types: TypeLine::LAND.into(),
            ..Default::default()
        }
    }

    fn x_instant_chars() -> Characteristics {
        Characteristics {
            mana_cost: Some(ManaCost::parse("{X}{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        }
    }

    fn put(state: &mut GameState, owner: PlayerId, zone: Zone, chars: Characteristics)
        -> ObjectId
    {
        let id = state.allocate_object_id();
        let mut obj = GameObject::new(id, owner, zone, 1, chars);
        obj.controller = owner;
        state.objects.insert(obj);
        id
    }

    fn add_mana(state: &mut GameState, p: PlayerId, color: ManaColor, n: u32) {
        state.player_mut(p).mana_pool.add_mana(color, n, 0);
    }

    fn set_main_phase(state: &mut GameState) {
        state.turn.phase = crate::turn::Phase::PreCombatMain;
        state.turn.step = crate::turn::Step::Main;
    }

    // --- game over ----------------------------------------------------------

    #[test]
    fn game_over_yields_no_actions() {
        let mut s = GameState::new(2, 0);
        s.result = Some(GameResult::Win(0));
        assert!(legal_actions(&s, &CardRegistry::new()).is_empty());
    }

    // --- always-legal in priority window ------------------------------------

    #[test]
    fn priority_window_always_has_pass_and_concede() {
        let s = GameState::new(2, 0);
        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(actions.iter().any(|a| a.is_pass()));
        assert!(actions.iter().any(|a| a.is_concede()));
    }

    // --- PlayLand -----------------------------------------------------------

    #[test]
    fn can_play_land_in_main_phase_with_empty_stack() {
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let l = put(&mut s, 0, Zone::Hand(0), land_chars());
        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(actions.iter().any(|a|
            matches!(a, Action::PlayLand { object_id } if *object_id == l)));
    }

    #[test]
    fn cannot_play_land_outside_main_phase() {
        let mut s = GameState::new(2, 0);
        // Default turn state is (Beginning, Untap) — not main.
        put(&mut s, 0, Zone::Hand(0), land_chars());
        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(!actions.iter().any(|a| matches!(a, Action::PlayLand { .. })));
    }

    #[test]
    fn cannot_play_land_with_nonempty_stack() {
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        put(&mut s, 0, Zone::Hand(0), land_chars());
        let stack_card = put(&mut s, 0, Zone::Hand(0), instant_chars());
        s.announce_spell_on_stack(stack_card, 0, TargetSelection::new(), vec![], None, vec![]);
        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(!actions.iter().any(|a| matches!(a, Action::PlayLand { .. })));
    }

    #[test]
    fn cannot_play_land_when_plays_remaining_zero() {
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        put(&mut s, 0, Zone::Hand(0), land_chars());
        s.player_mut(0).land_plays_remaining = 0;
        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(!actions.iter().any(|a| matches!(a, Action::PlayLand { .. })));
    }

    #[test]
    fn only_active_player_can_play_land() {
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        // Give priority to the non-active player.
        s.priority.player = 1;
        put(&mut s, 1, Zone::Hand(1), land_chars());
        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(!actions.iter().any(|a| matches!(a, Action::PlayLand { .. })));
    }

    // --- CastSpell ----------------------------------------------------------

    #[test]
    fn can_cast_affordable_instant_outside_main_phase() {
        let mut s = GameState::new(2, 0);
        // Default state: (Beginning, Untap) — no main-phase.
        let bolt = put(&mut s, 0, Zone::Hand(0), instant_chars());
        add_mana(&mut s, 0, ManaColor::Red, 1);
        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(actions.iter().any(|a|
            matches!(a, Action::CastSpell { object_id, .. } if *object_id == bolt)));
    }

    #[test]
    fn cannot_cast_instant_without_mana() {
        let mut s = GameState::new(2, 0);
        put(&mut s, 0, Zone::Hand(0), instant_chars());
        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(!actions.iter().any(|a| matches!(a, Action::CastSpell { .. })));
    }

    #[test]
    fn sorcery_speed_requires_main_phase_empty_stack_active_player() {
        let mut s = GameState::new(2, 0);
        let sorc = put(&mut s, 0, Zone::Hand(0), sorcery_chars());
        add_mana(&mut s, 0, ManaColor::Blue, 1);

        // Not in main phase yet → no cast.
        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(!actions.iter().any(|a|
            matches!(a, Action::CastSpell { object_id, .. } if *object_id == sorc)));

        // Now in main phase → castable.
        set_main_phase(&mut s);
        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(actions.iter().any(|a|
            matches!(a, Action::CastSpell { object_id, .. } if *object_id == sorc)));
    }

    #[test]
    fn flash_creature_castable_outside_main_phase() {
        use crate::effects::KeywordAbility;
        // A Flash creature in hand is castable even when the active
        // player isn't in a main phase.
        let mut s = GameState::new(2, 0);
        let chars = Characteristics {
            mana_cost: Some(ManaCost::parse("{G}").unwrap()),
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flash],
            ..Default::default()
        };
        let flashie = put(&mut s, 0, Zone::Hand(0), chars);
        add_mana(&mut s, 0, ManaColor::Green, 1);

        // No main phase, but Flash lets us cast.
        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(actions.iter().any(|a|
            matches!(a, Action::CastSpell { object_id, .. } if *object_id == flashie)));
    }

    #[test]
    fn cast_spell_uses_real_payment_plan_from_solver() {
        let mut s = GameState::new(2, 0);
        let bolt = put(&mut s, 0, Zone::Hand(0), instant_chars());
        // Two reds — should still produce one plan for {R} (greedy).
        add_mana(&mut s, 0, ManaColor::Red, 2);
        let actions = legal_actions(&s, &CardRegistry::new());
        let casts: Vec<_> = actions.iter().filter_map(|a| match a {
            Action::CastSpell { object_id, mana_payment, .. } if *object_id == bolt =>
                Some(mana_payment),
            _ => None,
        }).collect();
        assert_eq!(casts.len(), 1);
        // The plan is non-empty (actually assigns a red to the {R} pip).
        assert_eq!(casts[0].assignments.len(), 1);
    }

    #[test]
    fn cast_spell_emits_one_action_per_payment_plan() {
        // Hybrid {W/U}: with one W and one U, there are 2 plans → 2 casts.
        let mut s = GameState::new(2, 0);
        let chars = Characteristics {
            mana_cost: Some(ManaCost::parse("{W/U}").unwrap()),
            colors: ColorSet::white() | ColorSet::blue(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        };
        put(&mut s, 0, Zone::Hand(0), chars);
        add_mana(&mut s, 0, ManaColor::White, 1);
        add_mana(&mut s, 0, ManaColor::Blue, 1);
        let actions = legal_actions(&s, &CardRegistry::new());
        let casts = actions.iter().filter(|a| matches!(a, Action::CastSpell { .. })).count();
        assert_eq!(casts, 2);
    }

    #[test]
    fn x_cost_spells_enumerate_one_action_per_x_value() {
        // Cost is {X}{R}. With 5 red in the pool, feasible X values
        // are 0..=4 (X=5 would need 6 red total). Each enumerable
        // X produces at least one emitted cast action.
        let mut s = GameState::new(2, 0);
        put(&mut s, 0, Zone::Hand(0), x_instant_chars());
        add_mana(&mut s, 0, ManaColor::Red, 5);
        let actions = legal_actions(&s, &CardRegistry::new());
        let x_values: crate::collections::HashSet<u32> = actions.iter()
            .filter_map(|a| match a {
                Action::CastSpell { x_value: Some(x), .. } => Some(*x),
                _ => None,
            }).collect();
        for x in 0..=4 {
            assert!(x_values.contains(&x),
                "expected X={x} to be among enumerated cast actions");
        }
        // X=5 not feasible: leaves no mana for the fixed {R}.
        assert!(!x_values.contains(&5),
            "X=5 infeasible with {{X}}{{R}} and 5 red total");
    }

    #[test]
    fn lands_are_not_emitted_as_cast_actions() {
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        put(&mut s, 0, Zone::Hand(0), land_chars());
        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(!actions.iter().any(|a| matches!(a, Action::CastSpell { .. })));
    }

    // --- Special-action windows --------------------------------------------

    #[test]
    fn mulligan_decision_yields_keep_and_again() {
        let mut s = GameState::new(2, 0);
        s.priority.special_action = Some(SpecialAction::MulliganDecision);
        let actions = legal_actions(&s, &CardRegistry::new());
        assert_eq!(actions.len(), 2);
        assert!(actions.contains(&Action::MulliganKeep));
        assert!(actions.contains(&Action::MulliganAgain));
    }

    #[test]
    fn discard_to_hand_size_enumerates_hand() {
        let mut s = GameState::new(2, 0);
        s.priority.special_action = Some(SpecialAction::DiscardToHandSize);
        let a = put(&mut s, 0, Zone::Hand(0), instant_chars());
        let b = put(&mut s, 0, Zone::Hand(0), instant_chars());
        let actions = legal_actions(&s, &CardRegistry::new());
        assert_eq!(actions.len(), 2);
        assert!(actions.contains(&Action::MakeChoice(ChoiceAction::ChooseObject(a))));
        assert!(actions.contains(&Action::MakeChoice(ChoiceAction::ChooseObject(b))));
    }

    #[test]
    fn choose_first_player_enumerates_players() {
        let mut s = GameState::new(3, 0);
        s.priority.special_action = Some(SpecialAction::ChooseFirstPlayer);
        let actions = legal_actions(&s, &CardRegistry::new());
        assert_eq!(actions.len(), 3);
    }

    #[test]
    fn bottom_cards_emits_one_canonical_action() {
        // Phase 1 emits a single canonical BottomCards action (picking
        // the lowest-id cards in hand); real agents can build any legal
        // selection themselves.
        let mut s = GameState::new(2, 0);
        put(&mut s, 0, Zone::Hand(0), instant_chars());
        put(&mut s, 0, Zone::Hand(0), instant_chars());
        put(&mut s, 0, Zone::Hand(0), instant_chars());
        s.priority.special_action = Some(SpecialAction::LondonMulliganBottomCards(2));
        let actions = legal_actions(&s, &CardRegistry::new());
        assert_eq!(actions.len(), 1);
        match &actions[0] {
            Action::BottomCards(ids) => assert_eq!(ids.len(), 2),
            other => panic!("expected BottomCards, got {other:?}"),
        }
    }

    #[test]
    fn special_action_suppresses_normal_priority_actions() {
        // Even though normal priority would offer PassPriority, the
        // mulligan special action preempts it.
        let mut s = GameState::new(2, 0);
        s.priority.special_action = Some(SpecialAction::MulliganDecision);
        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(!actions.iter().any(|a| a.is_pass()));
    }

    // --- Combat -------------------------------------------------------------

    #[test]
    fn declare_attackers_emits_empty_plus_singletons() {
        let mut s = GameState::new(2, 0);
        // Put the game into DeclareAttackers phase.
        s.combat = Some(CombatState {
            phase: CombatPhase::DeclareAttackers,
            ..CombatState::new()
        });
        // One eligible attacker.
        let atk = put(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        s.objects.get_mut(atk).unwrap().status.summoning_sick = false;

        let actions = legal_actions(&s, &CardRegistry::new());
        // At minimum: the empty declaration + one per (attacker, opponent).
        assert!(actions.iter().any(|a|
            matches!(a, Action::DeclareAttackers { attackers } if attackers.is_empty())));
        assert!(actions.iter().any(|a|
            matches!(a, Action::DeclareAttackers { attackers }
                if attackers.len() == 1
                && attackers[0].attacker == atk
                && matches!(attackers[0].defending, DefendingEntity::Player(1)))));
    }

    #[test]
    fn summoning_sick_creature_is_not_an_attacker() {
        let mut s = GameState::new(2, 0);
        s.combat = Some(CombatState {
            phase: CombatPhase::DeclareAttackers,
            ..CombatState::new()
        });
        let c = put(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        s.objects.get_mut(c).unwrap().status.summoning_sick = true;

        let actions = legal_actions(&s, &CardRegistry::new());
        // Only the empty declaration should appear.
        let decls = actions.iter().filter(|a|
            matches!(a, Action::DeclareAttackers { .. })).count();
        assert_eq!(decls, 1);
    }

    #[test]
    fn haste_overrides_summoning_sickness_for_attack() {
        use crate::effects::KeywordAbility;
        let mut s = GameState::new(2, 0);
        s.combat = Some(CombatState {
            phase: CombatPhase::DeclareAttackers,
            ..CombatState::new()
        });
        let atk = put(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        s.objects.get_mut(atk).unwrap().status.summoning_sick = true;
        s.objects.get_mut(atk).unwrap().characteristics.keywords
            .push(KeywordAbility::Haste);

        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(actions.iter().any(|a|
            matches!(a, Action::DeclareAttackers { attackers }
                if attackers.len() == 1 && attackers[0].attacker == atk)));
    }

    #[test]
    fn cant_attack_effect_excludes_creature_from_attackers() {
        use crate::layers::{ContinuousEffect, Duration};
        let mut s = GameState::new(2, 0);
        s.combat = Some(CombatState {
            phase: CombatPhase::DeclareAttackers,
            ..CombatState::new()
        });
        let c = put(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        s.objects.get_mut(c).unwrap().status.summoning_sick = false;
        s.add_continuous_effect(ContinuousEffect::cant_attack(
            /*source=*/ c, c, Duration::EndOfTurn,
        ));

        let actions = legal_actions(&s, &CardRegistry::new());
        let decls = actions.iter().filter(|a|
            matches!(a, Action::DeclareAttackers { .. })).count();
        // Only the empty declaration.
        assert_eq!(decls, 1);
    }

    #[test]
    fn goaded_creature_cannot_declare_attack_on_goader() {
        use crate::layers::{ContinuousEffect, Duration};
        let mut s = GameState::new(3, 0);
        s.combat = Some(CombatState {
            phase: CombatPhase::DeclareAttackers,
            ..CombatState::new()
        });
        let atk = put(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        s.objects.get_mut(atk).unwrap().status.summoning_sick = false;
        // Player 1 goads player 0's creature.
        s.add_continuous_effect(ContinuousEffect::goad(
            atk, atk, /*goader=*/ 1, Duration::UntilYourNextTurn(1),
        ));

        let actions = legal_actions(&s, &CardRegistry::new());
        // Attacks on player 1 (the goader) must be absent.
        assert!(!actions.iter().any(|a| matches!(a,
            Action::DeclareAttackers { attackers }
                if attackers.len() == 1
                && attackers[0].attacker == atk
                && matches!(attackers[0].defending, DefendingEntity::Player(1)))));
        // But attacks on player 2 are still legal.
        assert!(actions.iter().any(|a| matches!(a,
            Action::DeclareAttackers { attackers }
                if attackers.len() == 1
                && attackers[0].attacker == atk
                && matches!(attackers[0].defending, DefendingEntity::Player(2)))));
    }

    #[test]
    fn defender_creature_cannot_attack() {
        use crate::effects::KeywordAbility;
        let mut s = GameState::new(2, 0);
        s.combat = Some(CombatState {
            phase: CombatPhase::DeclareAttackers,
            ..CombatState::new()
        });
        let c = put(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        s.objects.get_mut(c).unwrap().status.summoning_sick = false;
        s.objects.get_mut(c).unwrap().characteristics.keywords
            .push(KeywordAbility::Defender);

        let actions = legal_actions(&s, &CardRegistry::new());
        let decls = actions.iter().filter(|a|
            matches!(a, Action::DeclareAttackers { .. })).count();
        // Only the empty declaration.
        assert_eq!(decls, 1);
    }

    #[test]
    fn tapped_creature_is_not_an_attacker() {
        let mut s = GameState::new(2, 0);
        s.combat = Some(CombatState {
            phase: CombatPhase::DeclareAttackers,
            ..CombatState::new()
        });
        let c = put(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        s.objects.get_mut(c).unwrap().status.summoning_sick = false;
        s.objects.get_mut(c).unwrap().tap();

        let actions = legal_actions(&s, &CardRegistry::new());
        let decls = actions.iter().filter(|a|
            matches!(a, Action::DeclareAttackers { .. })).count();
        assert_eq!(decls, 1);
    }

    #[test]
    fn declare_attackers_includes_planeswalker_as_defender() {
        let mut s = GameState::new(2, 0);
        s.combat = Some(CombatState {
            phase: CombatPhase::DeclareAttackers,
            ..CombatState::new()
        });
        let atk = put(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        s.objects.get_mut(atk).unwrap().status.summoning_sick = false;

        // Opponent's planeswalker.
        let pw_chars = Characteristics {
            types: TypeLine::PLANESWALKER.into(),
            loyalty: Some(3),
            ..Default::default()
        };
        let pw = put(&mut s, 1, Zone::Battlefield, pw_chars);

        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(actions.iter().any(|a|
            matches!(a, Action::DeclareAttackers { attackers }
                if attackers.len() == 1
                && matches!(attackers[0].defending, DefendingEntity::Planeswalker(id) if id == pw))));
    }

    #[test]
    fn declare_blockers_needs_nonactive_player() {
        let mut s = GameState::new(2, 0);
        s.priority.player = 1; // defender has priority in blocks step
        s.combat = Some(CombatState {
            phase: CombatPhase::DeclareBlockers,
            attackers: vec![AttackerInfo {
                object_id: 99,
                defending_player: 1,
                defending_planeswalker: None,
                blocked_by: vec![],
                is_blocked: false,
            }],
            ..CombatState::new()
        });
        // Put a blocker on the defender's side.
        let blk = put(&mut s, 1, Zone::Battlefield, creature_chars(2, 2));
        s.objects.get_mut(blk).unwrap().status.summoning_sick = false;

        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(actions.iter().any(|a|
            matches!(a, Action::DeclareBlockers { blockers } if blockers.is_empty())));
        assert!(actions.iter().any(|a|
            matches!(a, Action::DeclareBlockers { blockers }
                if blockers.len() == 1
                && blockers[0].blocker == blk
                && blockers[0].blocking == 99)));
    }

    #[test]
    fn combat_declare_attackers_ignored_when_not_active_player() {
        // If the priority-holder isn't the active player during
        // DeclareAttackers, the combat branch is skipped — we fall
        // through to normal priority.
        let mut s = GameState::new(2, 0);
        s.priority.player = 1;
        s.combat = Some(CombatState {
            phase: CombatPhase::DeclareAttackers,
            ..CombatState::new()
        });
        let actions = legal_actions(&s, &CardRegistry::new());
        // No DeclareAttackers actions; should just be normal priority.
        assert!(!actions.iter().any(|a| matches!(a, Action::DeclareAttackers { .. })));
        assert!(actions.iter().any(|a| a.is_pass()));
    }

    // --- Determinism --------------------------------------------------------

    #[test]
    fn cast_actions_are_in_sorted_object_order() {
        let mut s = GameState::new(2, 0);
        // Insert in non-id order to stress the sort.
        let c1 = put(&mut s, 0, Zone::Hand(0), instant_chars());
        let c2 = put(&mut s, 0, Zone::Hand(0), instant_chars());
        let c3 = put(&mut s, 0, Zone::Hand(0), instant_chars());
        add_mana(&mut s, 0, ManaColor::Red, 3);

        let actions = legal_actions(&s, &CardRegistry::new());
        let cast_ids: Vec<_> = actions.iter().filter_map(|a| match a {
            Action::CastSpell { object_id, .. } => Some(*object_id),
            _ => None,
        }).collect();
        assert_eq!(cast_ids, vec![c1, c2, c3]);
    }

    // --- Integration: a typical turn-1 opening hand -----------------------

    #[test]
    fn opening_hand_on_turn_1_main_phase_offers_land_and_nothing_else() {
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        // Hand: 1 land, 1 instant ({R}), 1 sorcery ({U}). No mana yet.
        let land = put(&mut s, 0, Zone::Hand(0), land_chars());
        put(&mut s, 0, Zone::Hand(0), instant_chars());
        put(&mut s, 0, Zone::Hand(0), sorcery_chars());

        let actions = legal_actions(&s, &CardRegistry::new());
        // Pass + concede + exactly one PlayLand, no CastSpells.
        assert!(actions.iter().any(|a|
            matches!(a, Action::PlayLand { object_id } if *object_id == land)));
        assert!(!actions.iter().any(|a| matches!(a, Action::CastSpell { .. })));
    }

    #[test]
    fn placeholder_mana_unit_field_used() {
        // Sanity: this module uses ManaUnit indirectly via the solver,
        // but the unused-warning check for this import is worth
        // pinning.
        let _unit = ManaUnit::plain(ManaColor::Red, 0);
    }

    // --- activation with counter-removal cost ----------------------------

    /// Build a permanent with a single activated ability that costs
    /// "remove N +1/+1 counters" to deal 1 damage to any target.
    /// The ability is sorcery-speed (non-mana-ability) for simplicity.
    fn register_walking_ballista_stub(
        reg: &mut CardRegistry,
        remove_count: u32,
    ) -> CardId {
        use crate::registry::{ActivatedAbilityDef, ActivationCost, CardDefinition};
        let name = reg.interner_mut().intern("Ballista Stub");
        let chars = creature_chars(0, 0);
        reg.register(
            CardDefinition::new(name, chars)
                .with_activated_ability(ActivatedAbilityDef {
                    text: format!(
                        "Remove {remove_count} +1/+1 counters: deal 1 damage."),
                    cost: ActivationCost {
                        remove_self_counter: Some((
                            CounterKind::PlusOnePlusOne, remove_count)),
                        ..ActivationCost::default()
                    },
                    target_requirements: vec![],
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: crate::registry::ActivationZone::Battlefield,
                    is_instant_speed: false,
                    effect: |_, _, _| Vec::new(),
                })
        )
    }

    #[test]
    fn remove_counter_ability_is_unavailable_without_counters() {
        let mut reg = CardRegistry::new();
        let cid = register_walking_ballista_stub(&mut reg, 1);
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let mut chars = creature_chars(0, 0);
        chars.power = Some(PtValue::Fixed(3));
        chars.toughness = Some(PtValue::Fixed(3));
        let obj = state_put_with_card(&mut s, 0, Zone::Battlefield, chars, cid);
        s.objects.get_mut(obj).unwrap().status.summoning_sick = false;

        // No counters -> ability is unavailable.
        let actions = legal_actions(&s, &reg);
        assert!(!actions.iter().any(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == obj)),
            "no remove-counter-cost activation should be legal with 0 counters");
    }

    #[test]
    fn remove_counter_ability_available_with_enough_counters() {
        let mut reg = CardRegistry::new();
        let cid = register_walking_ballista_stub(&mut reg, 1);
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let chars = creature_chars(0, 0);
        let obj = state_put_with_card(&mut s, 0, Zone::Battlefield, chars, cid);
        s.objects.get_mut(obj).unwrap().status.summoning_sick = false;
        s.objects.get_mut(obj).unwrap()
            .add_counters(CounterKind::PlusOnePlusOne, 2);

        let actions = legal_actions(&s, &reg);
        let activation = actions.iter().find(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == obj));
        let action = activation.expect("activation should be legal with counters");
        // Payment carries the RemoveCounters additional-cost entry.
        let Action::ActivateAbility { additional_costs, .. } = action else {
            unreachable!()
        };
        assert!(additional_costs.iter().any(|c| matches!(c,
            crate::actions::AdditionalCostPayment::RemoveCounters {
                source: s, kind: CounterKind::PlusOnePlusOne, count: 1,
            } if *s == obj)));
    }

    /// `put` but with a specific registered CardId so ability lookup
    /// hits `registry.get(obj.card_id)`.
    fn state_put_with_card(
        state: &mut GameState, owner: PlayerId, zone: Zone,
        chars: Characteristics, card_id: CardId,
    ) -> ObjectId {
        let id = state.allocate_object_id();
        let mut obj = GameObject::new(id, owner, zone, card_id, chars);
        obj.controller = owner;
        state.objects.insert(obj);
        id
    }
}
