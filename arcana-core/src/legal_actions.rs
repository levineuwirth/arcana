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
/// CR 601.3e — does any permanent `player` controls statically forbid casting a
/// spell with type line `types`? (e.g. Codie, Vociferous Codex's "You can't cast
/// permanent spells".) See [`crate::registry::CastRestriction`].
fn cast_restricted(
    state: &GameState, player: PlayerId, registry: &CardRegistry,
    types: crate::types::TypeLine,
) -> bool {
    state.objects.objects_in_zone(Zone::Battlefield).any(|obj| {
        obj.controller == player
            && registry.get(obj.card_id)
                .and_then(|d| d.cant_cast)
                .is_some_and(|r| r.forbids(types))
    })
}

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

    // CR 510.1c — a pending combat-damage assignment preempts every
    // priority-style action; only the AssignCombatDamage submission
    // (plus Concede) is legal.
    if state.combat.as_ref()
        .and_then(|c| c.pending_damage_assignment).is_some()
        && player == state.active_player()
    {
        let mut out = enumerate_combat_damage_assignments(state);
        out.push(Action::Concede);
        return out;
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
        ChoiceKind::PayOrDecline { cost, .. } => {
            // Decline is always available. Offer pay only if the chooser
            // can actually afford the (mana) cost — otherwise the
            // pay-branch hits the solver with no valid plan and panics
            // in `auto_pay_ward_cost`. Mirrors the `OptionalCost` gate.
            let can_pay = !crate::mana::enumerate_payment_plans(
                cost,
                &state.player(pending.choosing_player).mana_pool,
                /*x_value=*/ None,
                &crate::mana::SpendContext::unrestricted(),
            ).is_empty();
            if can_pay {
                out.push(Action::SubmitResolutionChoice {
                    id,
                    response: ChoiceResponse::PayOrDecline { pay: true },
                });
            }
            out.push(Action::SubmitResolutionChoice {
                id,
                response: ChoiceResponse::PayOrDecline { pay: false },
            });
        }
        ChoiceKind::OptionalCost { cost } => {
            // Decline is always available. Offer pay only if the
            // chooser actually has the resources — CR 119.4 / mana
            // solver legality.
            let can_pay = match cost {
                crate::actions::OptionalPaymentKind::Mana(mc) => {
                    !crate::mana::enumerate_payment_plans(
                        mc,
                        &state.player(pending.choosing_player).mana_pool,
                        /*x_value=*/ None,
                        &crate::mana::SpendContext::unrestricted(),
                    ).is_empty()
                }
                crate::actions::OptionalPaymentKind::Life(amount) => {
                    state.player(pending.choosing_player).life
                        >= *amount as i32
                }
                // Same candidate predicate as push_sacrifice_choice, so
                // "pay" is offered iff the sacrifice will find a target.
                crate::actions::OptionalPaymentKind::Sacrifice(sf) => {
                    let filter = sf.to_object_filter();
                    let chooser = pending.choosing_player;
                    state.objects.iter().any(|o| o.is_permanent_on_battlefield()
                        && o.controller == chooser
                        && filter.matches(o, state, chooser))
                }
                crate::actions::OptionalPaymentKind::Discard(n) => {
                    *n > 0 && sorted_ids_in_zone(
                        state, Zone::Hand(pending.choosing_player),
                    ).len() as u32 >= *n
                }
            };
            if can_pay {
                out.push(Action::SubmitResolutionChoice {
                    id,
                    response: ChoiceResponse::OptionalCost { pay: true },
                });
            }
            out.push(Action::SubmitResolutionChoice {
                id,
                response: ChoiceResponse::OptionalCost { pay: false },
            });
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
                    enumerate_target_selections(reqs, state, *source, source_controller)
                {
                    out.push(Action::SubmitResolutionChoice {
                        id,
                        response: ChoiceResponse::ChooseTargets { selection },
                    });
                }
            }
        }
        ChoiceKind::ChooseColor => {
            for color in crate::types::Color::all() {
                out.push(Action::SubmitResolutionChoice {
                    id,
                    response: ChoiceResponse::ChooseColor { color },
                });
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

/// Upper bound on how many actions any single combat enumerator emits.
/// The blocker-subset (2^k), damage-assignment-order (k!), and damage-
/// distribution spaces are combinatorial, and a large board can push one
/// `legal_actions` result into the millions — enough to OOM the process
/// (the random-game harness, before this cap, drove a single enumeration
/// to a 43 GB allocation; the damage-order space alone reached 9! =
/// 362880). Past this many the enumerator stops early. The canonical /
/// lowest-id selections are generated first, so a representative legal
/// action is always present, and a search/AI consumer that wants the
/// full space constructs the remaining selections itself — the same
/// contract as `BottomCards` and multi-attacker block batches.
const MAX_COMBAT_ENUM: usize = 1024;

/// Is a combat DECLARATION (attackers / blockers / damage-order) pending for
/// `player`? True only in the matching combat phase for the player who makes
/// that declaration — NOT for the priority windows that share the same combat
/// step (which key on the same `player == priority_player()` but produce a
/// normal priority action set). The single source of truth for "this is a
/// combat-declaration decision", shared by [`legal_actions`] and the engine's
/// decision/context routing (`compute_next_decision`) so the yielded player and
/// context can't disagree with the legal actions.
pub(crate) fn combat_declaration_pending(state: &GameState, player: PlayerId) -> bool {
    let Some(combat) = state.combat.as_ref() else { return false; };
    match combat.phase {
        CombatPhase::DeclareAttackers | CombatPhase::OrderBlockers =>
            player == state.active_player(),
        CombatPhase::DeclareBlockers => player != state.active_player(),
        _ => false,
    }
}

fn legal_combat_declaration_actions(
    state: &GameState,
    player: PlayerId,
) -> Option<Vec<Action>> {
    if !combat_declaration_pending(state, player) {
        return None;
    }
    let combat = state.combat.as_ref()?;
    Some(match combat.phase {
        CombatPhase::DeclareAttackers => enumerate_attacker_declarations(state, player),
        CombatPhase::DeclareBlockers => enumerate_blocker_declarations(state, player),
        CombatPhase::OrderBlockers => enumerate_blocker_orderings(state),
        _ => unreachable!("combat_declaration_pending gated the phase"),
    })
}

/// CR 509.2 — emit every legal assignment of damage-assignment order
/// across the multi-blocked attackers, as a Cartesian product of per-
/// attacker permutations.
///
/// Blows up as product of k_i! for attackers with k_i blockers each.
/// For the seed set (typical multi-block depth ≤3) the enumeration
/// stays small; deeper blocks come back as a truncation-with-policy
/// refinement when deeper board states enter the test pool.
fn enumerate_blocker_orderings(state: &GameState) -> Vec<Action> {
    let Some(combat) = state.combat.as_ref() else { return Vec::new(); };
    let per_attacker: Vec<(ObjectId, Vec<Vec<ObjectId>>)> = combat.attackers.iter()
        .filter(|a| a.blocked_by.len() >= 2)
        .map(|a| (a.object_id, permutations(&a.blocked_by)))
        .collect();
    if per_attacker.is_empty() { return Vec::new(); }

    // Cartesian product across attackers, bounded each round so the
    // running product can't explode (see [`MAX_COMBAT_ENUM`]).
    let mut acc: Vec<Vec<(ObjectId, Vec<ObjectId>)>> = vec![Vec::new()];
    for (atk, perms) in &per_attacker {
        let mut next: Vec<Vec<(ObjectId, Vec<ObjectId>)>> = Vec::new();
        'product: for partial in &acc {
            for p in perms {
                let mut extended = partial.clone();
                extended.push((*atk, p.clone()));
                next.push(extended);
                if next.len() >= MAX_COMBAT_ENUM { break 'product; }
            }
        }
        acc = next;
    }
    acc.into_iter()
        .map(|orderings| Action::OrderBlockers { orderings })
        .collect()
}

/// CR 510.1c — enumerate every legal [`Action::AssignCombatDamage`]
/// for the attackers currently needing a distribution. One entry per
/// attacker in the action; Cartesian product across attackers.
///
/// Blowup rides on per-attacker distribution count, which scales
/// polynomially with attacker power and blocker count (~O(P * k!)
/// in the worst case). Seed-set scale keeps this comfortably small;
/// a deeper-board refinement (sampling / heuristic pruning) will be
/// required when training runs surface it.
///
/// Trample (CR 702.19b) is handled via an extra "stop at the last
/// blocker and let the remainder overflow to the defender" branch
/// in [`recurse_distributions`]. Overflow requires every blocker to
/// have received ≥ lethal, matching [`is_legal_damage_assignment`].
fn enumerate_combat_damage_assignments(state: &GameState) -> Vec<Action> {
    let Some(combat) = state.combat.as_ref() else { return Vec::new(); };
    let Some(pass) = combat.pending_damage_assignment else { return Vec::new(); };
    let attackers = state.attackers_needing_damage_assignment(pass);
    if attackers.is_empty() { return Vec::new(); }

    let per_attacker: Vec<(ObjectId, Vec<Vec<(ObjectId, u32)>>)> =
        attackers.iter()
            .map(|&atk| (atk, enumerate_distributions_for_attacker(state, atk)))
            .collect();

    // Cartesian product across attackers, bounded each round (see
    // [`MAX_COMBAT_ENUM`]).
    let mut acc: Vec<Vec<crate::combat::DamageAssignment>> = vec![Vec::new()];
    for (atk, dists) in &per_attacker {
        let mut next: Vec<Vec<crate::combat::DamageAssignment>> = Vec::new();
        'product: for partial in &acc {
            for d in dists {
                let mut extended = partial.clone();
                extended.push(crate::combat::DamageAssignment {
                    attacker: *atk,
                    distribution: d.clone(),
                });
                next.push(extended);
                if next.len() >= MAX_COMBAT_ENUM { break 'product; }
            }
        }
        acc = next;
    }
    acc.into_iter()
        .map(|distributions| Action::AssignCombatDamage { distributions })
        .collect()
}

/// Enumerate every CR 510.1c–legal distribution of `attacker`'s
/// computed power across its ordered `blocked_by`. Uses a recursive
/// "terminate-here or pay-lethal-and-recurse" generator.
fn enumerate_distributions_for_attacker(
    state: &GameState,
    attacker: ObjectId,
) -> Vec<Vec<(ObjectId, u32)>> {
    let Some(_combat) = state.combat.as_ref() else { return Vec::new(); };
    let power = state.computed_power(attacker).unwrap_or(0).max(0) as u32;
    if power == 0 { return vec![Vec::new()]; }

    let has_dt = state.has_keyword(attacker, &crate::effects::KeywordAbility::Deathtouch);
    let has_trample = state.has_keyword(
        attacker, &crate::effects::KeywordAbility::Trample);
    // (id, lethal) pairs in damage-assignment order. CR 510.1c —
    // blockers that have already been dealt lethal damage are
    // "treated as though they weren't there", so they drop out of
    // the enumeration entirely. Critical in the regular pass after
    // the first-strike pass has killed earlier blockers, which
    // would otherwise let the enumerator spend all of the attacker's
    // damage on a corpse and skip the live blocker.
    let live_blockers = state.live_blockers_of(attacker);
    let ordered: Vec<(ObjectId, u32)> = live_blockers.iter().map(|&id| {
        let lethal = if has_dt { 1 } else { raw_remaining_lethal(state, id) };
        (id, lethal)
    }).collect();

    let mut out: Vec<Vec<(ObjectId, u32)>> = Vec::new();
    recurse_distributions(power, &ordered, Vec::new(), has_trample, &mut out);
    out
}

fn recurse_distributions(
    remaining: u32,
    blockers: &[(ObjectId, u32)],
    current: Vec<(ObjectId, u32)>,
    has_trample: bool,
    out: &mut Vec<Vec<(ObjectId, u32)>>,
) {
    // Bound the distribution space (it grows ~O(power^blockers)). The
    // canonical "dump all on the first blocker" is emitted before any
    // recursion, so the cap never starves the caller of a legal
    // assignment. See [`MAX_COMBAT_ENUM`].
    if out.len() >= MAX_COMBAT_ENUM { return; }
    if remaining == 0 {
        out.push(current);
        return;
    }
    if blockers.is_empty() { return; }
    let (blk, lethal) = blockers[0];
    let rest = &blockers[1..];
    // Option 1 — terminate: dump all remaining on this blocker.
    let mut term = current.clone();
    term.push((blk, remaining));
    out.push(term);
    // Option 2 — pay lethal-or-more, continue to next blocker.
    let lb = lethal.max(1);
    for k in lb..remaining {
        if out.len() >= MAX_COMBAT_ENUM { return; }
        let mut next = current.clone();
        next.push((blk, k));
        recurse_distributions(remaining - k, rest, next, has_trample, out);
    }
    // Option 3 (CR 702.19b trample) — pay ≥ lethal to this (last) blocker
    // and stop. The unassigned remainder overflows to the defender;
    // `is_legal_damage_assignment` reconstructs the overflow from
    // `power - sum(distribution)`. Only valid as the final blocker,
    // since every earlier blocker must be paid ≥ lethal before overflow
    // can exist.
    if has_trample && rest.is_empty() {
        for k in lb..remaining {
            if out.len() >= MAX_COMBAT_ENUM { return; }
            let mut next = current.clone();
            next.push((blk, k));
            out.push(next);
        }
    }
}

fn raw_remaining_lethal(state: &GameState, id: ObjectId) -> u32 {
    let Some(obj) = state.objects.get(id) else { return 0; };
    let Some(t) = obj.raw_toughness_with_counters(None) else { return 0; };
    if t <= 0 { return 0; }
    (t as u32).saturating_sub(obj.damage_marked)
}

/// Lexicographic permutations of `items`, identity-order first, capped
/// at [`MAX_COMBAT_ENUM`] total. The cap threads INTO the recursion (via
/// the shared `out`), so the intermediate sub-permutation lists never
/// blow up either — a 9-blocked attacker yields ≤1024 orderings rather
/// than 9! = 362880. The first ordering emitted is the input order, so a
/// canonical damage-assignment order is always available.
fn permutations(items: &[ObjectId]) -> Vec<Vec<ObjectId>> {
    let mut out = Vec::new();
    permute_capped(items, Vec::new(), &mut out);
    out
}

fn permute_capped(
    rest: &[ObjectId],
    prefix: Vec<ObjectId>,
    out: &mut Vec<Vec<ObjectId>>,
) {
    if out.len() >= MAX_COMBAT_ENUM { return; }
    if rest.is_empty() {
        out.push(prefix);
        return;
    }
    for i in 0..rest.len() {
        if out.len() >= MAX_COMBAT_ENUM { return; }
        let mut sub: Vec<ObjectId> = rest.to_vec();
        let picked = sub.remove(i);
        let mut next_prefix = prefix.clone();
        next_prefix.push(picked);
        permute_capped(&sub, next_prefix, out);
    }
}

/// Emit the empty declaration (no attacks) plus one declaration per
/// eligible attacker targeting each possible defender. Multi-attacker
/// combinations are deferred (see module docs).
fn enumerate_attacker_declarations(state: &GameState, active: PlayerId) -> Vec<Action> {
    // Declining to attack is always legal (CR 508.1 — declaring 0 attackers).
    let mut out = vec![Action::DeclareAttackers { attackers: Vec::new() }];

    let opponents: Vec<PlayerId> = state.opponents_of(active).collect();
    if opponents.is_empty() { return out; }

    let mut eligible_ids: Vec<ObjectId> = state.objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == active && can_attack(state, o))
        .map(|o| o.id)
        .collect();
    eligible_ids.sort();
    if eligible_ids.is_empty() { return out; }

    // Planeswalker / battle defenders (shared across attackers; CR 508.4).
    // The Siege protector-designation rule (CR 310.6) is not modeled — any
    // opponent's battle is attackable, mirroring planeswalkers.
    let mut extra_defenders: Vec<DefendingEntity> = Vec::new();
    for &opp in &opponents {
        let mut pws: Vec<ObjectId> = state.objects.objects_in_zone(Zone::Battlefield)
            .filter(|o| o.controller == opp && o.is_planeswalker()).map(|o| o.id).collect();
        pws.sort();
        extra_defenders.extend(pws.into_iter().map(DefendingEntity::Planeswalker));
        let mut battles: Vec<ObjectId> = state.objects.objects_in_zone(Zone::Battlefield)
            .filter(|o| o.controller == opp && o.characteristics.types.is_battle())
            .map(|o| o.id).collect();
        battles.sort();
        extra_defenders.extend(battles.into_iter().map(DefendingEntity::Battle));
    }

    // Each attacker's legal defenders: opponent players NOT goading it
    // (CR 701.38a — Goad only restricts the choose-the-player branch) + every
    // planeswalker/battle defender.
    let slots: Vec<(ObjectId, Vec<DefendingEntity>)> = eligible_ids.iter().filter_map(|&atk| {
        let goaders = state.goaders_of(atk);
        let mut defs: Vec<DefendingEntity> = opponents.iter()
            .filter(|opp| !goaders.contains(opp))
            .map(|&opp| DefendingEntity::Player(opp))
            .collect();
        defs.extend(extra_defenders.iter().copied());
        if defs.is_empty() { None } else { Some((atk, defs)) }
    }).collect();
    if slots.is_empty() { return out; }

    // Attack tax (Ghostly Prison / Propaganda): a declaration is legal only if
    // its TOTAL tax (summed over attackers attacking a taxed player) fits the
    // active player's FLOATED mana pool (mirrors the apply-side gate).
    let floated = state.player(active).mana_pool.total() as u32;
    let tax_ok = |decl: &[AttackerDeclaration]| -> bool {
        let total: u32 = decl.iter().map(|d| match d.defending {
            DefendingEntity::Player(opp) => state.attack_tax_total(opp),
            _ => 0,
        }).sum();
        total <= floated
    };

    // Size of the full cross product = ∏(1 + |defenders_i|) (the +1 is the
    // "this attacker declines" option; the all-decline case = the empty
    // declaration already emitted).
    let mut total: u128 = 1;
    for (_, defs) in &slots {
        total = total.saturating_mul(1 + defs.len() as u128);
        if total > MAX_COMBAT_ENUM as u128 { break; }
    }

    if total <= MAX_COMBAT_ENUM as u128 {
        // FULL multi-attacker cross product via an odometer: each attacker
        // independently declines (digit 0) or attacks one legal defender
        // (digit 1..=len). Faithful — every legal declaration is emitted.
        let radices: Vec<usize> = slots.iter().map(|(_, d)| 1 + d.len()).collect();
        let mut idx = vec![0usize; slots.len()];
        loop {
            let mut decl = Vec::new();
            for (i, &c) in idx.iter().enumerate() {
                if c == 0 { continue; }
                decl.push(AttackerDeclaration {
                    attacker: slots[i].0, defending: slots[i].1[c - 1],
                });
            }
            if !decl.is_empty() && tax_ok(&decl) {
                out.push(Action::DeclareAttackers { attackers: decl });
            }
            // Odometer increment; stop once the most-significant digit rolls over.
            let mut k = 0;
            while k < idx.len() {
                idx[k] += 1;
                if idx[k] < radices[k] { break; }
                idx[k] = 0;
                k += 1;
            }
            if k == idx.len() { break; }
        }
    } else {
        // DEGRADE (combinatorial blowup beyond the cap): emit a bounded but
        // strategically meaningful subset — every singleton (attacker ×
        // defender) plus the "all able attackers vs one defender" alpha-strike
        // for each distinct defender. Mid-size subsets are omitted; declining
        // and focused/all-in attacks stay available.
        for (atk, defs) in &slots {
            for &def in defs {
                if out.len() >= MAX_COMBAT_ENUM { return out; }
                let decl = vec![AttackerDeclaration { attacker: *atk, defending: def }];
                if tax_ok(&decl) { out.push(Action::DeclareAttackers { attackers: decl }); }
            }
        }
        let distinct: Vec<DefendingEntity> = opponents.iter().map(|&o| DefendingEntity::Player(o))
            .chain(extra_defenders.iter().copied()).collect();
        for def in distinct {
            if out.len() >= MAX_COMBAT_ENUM { return out; }
            let decl: Vec<AttackerDeclaration> = slots.iter()
                .filter(|(_, defs)| defs.contains(&def))
                .map(|(atk, _)| AttackerDeclaration { attacker: *atk, defending: def })
                .collect();
            if !decl.is_empty() && tax_ok(&decl) {
                out.push(Action::DeclareAttackers { attackers: decl });
            }
        }
    }

    // CR 508.1a — must-attack enforcement. A creature with a "must attack if
    // able" requirement (Goad, Juggernaut-style self, board-wide) that is ABLE
    // (it's a slot → has ≥1 legal defender) must be declared as an attacker.
    // Filter to declarations satisfying every such requirement; if NONE are
    // affordable (e.g. attack tax can't be paid), fall back to the unfiltered
    // set so a legal declaration always exists (the requirement is then
    // treated as un-meetable, CR 508.1a "as many as possible").
    let required: Vec<ObjectId> = slots.iter()
        .map(|(atk, _)| *atk)
        .filter(|&atk| state.must_attack(atk))
        .collect();
    if !required.is_empty() {
        let satisfying: Vec<Action> = out.iter().filter(|a| match a {
            Action::DeclareAttackers { attackers } =>
                required.iter().all(|r| attackers.iter().any(|d| d.attacker == *r)),
            _ => true,
        }).cloned().collect();
        if !satisfying.is_empty() { return satisfying; }
    }
    out
}

/// Emit the empty declaration (no blocks) plus, for each attacker,
/// every legal subset of that attacker's eligible blockers whose size
/// satisfies the attacker's [`AttackerBlockConstraints`] (CR 509.1
/// plus count-keywords). This is a constraint-driven generalization
/// of the earlier singleton-only enumerator: the default
/// `min=1, max=None` case recovers the old "every (blocker, attacker)
/// pair" shape, while Menace's `min=2` produces pair-and-larger
/// subsets and a hypothetical "can't be blocked by more than one"
/// would cap at `max=Some(1)`.
///
/// Per-attacker independence (Phase 2 scope): each emitted action
/// declares blockers on **one** attacker with the others unblocked.
/// The defender can still construct multi-attacker block batches by
/// hand. Full multi-attacker-assignment enumeration is deferred — it
/// blows up combinatorially and no current consumer pays for the
/// generality.
///
/// DEBT: subset enumeration at high eligible-blocker counts can
/// generate many equivalent declarations when multiple blockers
/// share characteristics. The
/// [`enumerate_equivalence_subsets`] helper applies
/// characteristic-equivalence dedup (same mechanism delve uses), so
/// two Grizzly Bears enter a single equivalence class and subsets of
/// the class are counted once. This is the main lever that keeps
/// Menace enumeration tractable. If a future AI-training profile
/// shows the enumeration is still pathological, revisit with a
/// richer key (counters, attachments, damage).
fn enumerate_blocker_declarations(state: &GameState, defender: PlayerId) -> Vec<Action> {
    let mut out = Vec::new();
    out.push(Action::DeclareBlockers { blockers: Vec::new() });

    let combat = match state.combat.as_ref() {
        Some(c) => c,
        None => return out,
    };

    let mut all_eligible: Vec<ObjectId> = state.objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == defender && can_block(state, o))
        .map(|o| o.id)
        .collect();
    all_eligible.sort();

    let mut attackers: Vec<ObjectId> = combat.attackers.iter()
        .map(|a| a.object_id).collect();
    attackers.sort();

    for atk in attackers {
        // Total emitted DeclareBlockers actions are bounded too: a wide
        // board with many attackers could otherwise sum to a large set
        // even with each attacker's subsets individually capped.
        if out.len() >= MAX_COMBAT_ENUM { break; }
        // Per-attacker eligibility filter (Flying/Reach, Protection).
        // Menace is now expressed via `block_constraints`, not per-
        // blocker eligibility.
        let eligible_for_atk: Vec<ObjectId> = all_eligible.iter()
            .copied()
            .filter(|&blk| can_block_attacker(state, blk, atk))
            .collect();

        let constraints = state.block_constraints(atk);
        let max_size = match constraints.max_blockers {
            Some(m) => (m as usize).min(eligible_for_atk.len()),
            None => eligible_for_atk.len(),
        };
        if (constraints.min_blockers as usize) > max_size {
            // No legal non-empty block exists on this attacker
            // (e.g. Menace with only one eligible blocker, or
            // unblockable via max=Some(0)).
            continue;
        }

        // Characteristic-equivalence dedup: subsets of size 0..=max
        // over the eligible blockers, grouped by equivalence class.
        // Filter the emitted subsets down to the constraint-allowed
        // sizes; drop the empty subset (already emitted above).
        let subsets = enumerate_equivalence_subsets(
            &eligible_for_atk, max_size, MAX_COMBAT_ENUM,
            |&id| object_equivalence_key(state, id),
        );
        for subset in subsets {
            if out.len() >= MAX_COMBAT_ENUM { break; }
            let size = subset.len() as u32;
            if size == 0 { continue; }
            if !constraints.allows_block_count(size) { continue; }
            let decls: Vec<BlockerDeclaration> = subset.into_iter()
                .map(|blk| BlockerDeclaration { blocker: blk, blocking: atk })
                .collect();
            out.push(Action::DeclareBlockers { blockers: decls });
        }
    }
    out
}

/// Per-pair blocker eligibility (CR 509.1b). Checks the per-blocker
/// restrictions: Flying/Reach evasion, Protection. Count constraints
/// (Menace, unblockable, etc.) live on
/// [`crate::combat::AttackerBlockConstraints`] and are enforced by
/// the enumerator's subset-size filter rather than here — this fn's
/// job is strictly "is this *individual* creature a legal blocker
/// for this attacker."
fn can_block_attacker(state: &GameState, blocker: ObjectId, attacker: ObjectId) -> bool {
    // Single source of truth for per-pairing evasion (Flying/Reach,
    // Protection, Fear, Intimidate, Shadow, Horsemanship, Skulk) —
    // shared with the apply-side blocker filter so the two can't drift.
    state.blocker_eligible(blocker, attacker)
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
    // CR 702.176 — suspected creatures can't block.
    obj.is_creature()
        && obj.zone.is_battlefield()
        && !obj.is_tapped()
        && !obj.status.suspected
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

    // Play a land. CR 712.4 — an MDFC whose back face is a land is
    // playable as the back face via `PlayLand { mdfc_back: true }`,
    // counting the same single land drop as any other land play.
    // Only one of the two faces can be chosen per play — the agent
    // picks which face the drop goes to.
    if can_play_land_now(state, player) {
        for id in sorted_ids_in_zone(state, Zone::Hand(player)) {
            let obj = state.objects.get(id).unwrap();
            if obj.is_land() {
                actions.push(Action::PlayLand {
                    object_id: id,
                    mdfc_back: false,
                });
            }
            if let Some(def) = registry.get(obj.card_id) {
                if let Some(back) = def.alternate_face.as_ref()
                    .and_then(|af| af.as_mdfc())
                {
                    if back.characteristics.types.is_land() {
                        actions.push(Action::PlayLand {
                            object_id: id,
                            mdfc_back: true,
                        });
                    }
                }
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

        // CR 711.4 — for a Split card the object's off-stack chars are
        // the combined view; a Normal cast dispatches the left half,
        // so cost, types, and colors come from `base_characteristics`.
        // For every other card, `base_characteristics` and
        // `obj.characteristics` agree in hand (no swap has occurred).
        let cast_chars: &crate::objects::Characteristics = registry.get(obj.card_id)
            .filter(|def| def.alternate_face.as_ref()
                .and_then(|af| af.as_split()).is_some())
            .map(|def| &def.base_characteristics)
            .unwrap_or(&obj.characteristics);

        let Some(printed_cost) = cast_chars.mana_cost.clone() else { continue; };
        // CR 601.3e — static casting restrictions a permanent imposes on its
        // controller (e.g. Codie, Vociferous Codex: "You can't cast permanent
        // spells"). The spell simply isn't enumerated as castable.
        if cast_restricted(state, player, registry, cast_chars.types) { continue; }
        // CR 601.2f — cost modifiers (Chill / Sphere of Resistance /
        // Arcane Melee class) adjust the generic component.
        let printed_cost = printed_cost.with_generic_delta(
            state.spell_cost_delta(cast_chars, player));

        let ctx = SpendContext::for_spell(
            cast_chars.types, cast_chars.colors);

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
                enumerate_target_selections(&effective_reqs, state, id, player);
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
        let target_selections = enumerate_target_selections(&reqs, state, id, player);

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
        let target_selections = enumerate_target_selections(&reqs, state, id, player);
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

    // MDFC back-face casts from hand (CR 712.4). Walks the hand for
    // cards whose registry definition declares an MDFC back face that
    // is a SPELL (non-land — land backs are played via the land-play
    // block above). Uses the back face's own mana cost, target
    // requirements, and type for speed-gating.
    //
    // No exile routing or post-cast flag: an MDFC back-face creature
    // resolves to the battlefield normally; an MDFC back-face
    // instant/sorcery resolves its effect and goes to graveyard via
    // the standard paths. The front face is NOT enumerated here —
    // the main hand-cast loop already offers it when applicable.
    for id in sorted_ids_in_zone(state, Zone::Hand(player)) {
        let Some(obj) = state.objects.get(id) else { continue; };
        let Some(def) = registry.get(obj.card_id) else { continue; };
        let Some(back) = def.alternate_face.as_ref()
            .and_then(|af| af.as_mdfc()) else { continue; };
        if back.characteristics.types.is_land() { continue; }
        let Some(back_cost) = back.characteristics.mana_cost.clone()
            else { continue; };
        let back_cost = back_cost.with_generic_delta(
            state.spell_cost_delta(&back.characteristics, player));

        let back_is_instant_speed = back.characteristics.types.is_instant();
        if !back_is_instant_speed && !sorcery_speed_ok { continue; }

        let reqs: Vec<TargetRequirement> = back.spell_ability.as_ref()
            .map(|sa| sa.target_requirements.clone())
            .unwrap_or_default();
        let target_selections = enumerate_target_selections(&reqs, state, id, player);
        if target_selections.is_empty() { continue; }

        let ctx = SpendContext::for_spell(
            back.characteristics.types, back.characteristics.colors);

        let has_x = back_cost.x_count() > 0;
        let x_values: Vec<u32> = if has_x {
            let max_x = state.player(player).mana_pool.total() as u32;
            (0..=max_x).collect()
        } else {
            vec![0]
        };

        for &x in &x_values {
            let cost = if has_x {
                back_cost.with_x_expanded(x)
            } else {
                back_cost.clone()
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
                        cast_modifier: crate::actions::CastModifier::MdfcBack,
                        cost_reductions: crate::actions::CostReductions::default(),
                    });
                }
            }
        }
    }

    // Split right-half casts from hand (CR 711). Walks the hand for
    // cards whose registry definition declares a Split relationship.
    // The right half's cost, targets, and type govern the cast; the
    // left half is already enumerated by the main hand-cast loop
    // above (via `base_characteristics`). Mirrors the MDFC back-face
    // branch structurally — the only differences are the variant
    // name and the lack of a land-back case.
    for id in sorted_ids_in_zone(state, Zone::Hand(player)) {
        let Some(obj) = state.objects.get(id) else { continue; };
        let Some(def) = registry.get(obj.card_id) else { continue; };
        let Some(right) = def.alternate_face.as_ref()
            .and_then(|af| af.as_split()) else { continue; };
        let Some(right_cost) = right.characteristics.mana_cost.clone()
            else { continue; };
        let right_cost = right_cost.with_generic_delta(
            state.spell_cost_delta(&right.characteristics, player));

        let right_is_instant_speed = right.characteristics.types.is_instant();
        if !right_is_instant_speed && !sorcery_speed_ok { continue; }

        let reqs: Vec<TargetRequirement> = right.spell_ability.as_ref()
            .map(|sa| sa.target_requirements.clone())
            .unwrap_or_default();
        let target_selections = enumerate_target_selections(&reqs, state, id, player);
        if target_selections.is_empty() { continue; }

        let ctx = SpendContext::for_spell(
            right.characteristics.types, right.characteristics.colors);

        let has_x = right_cost.x_count() > 0;
        let x_values: Vec<u32> = if has_x {
            let max_x = state.player(player).mana_pool.total() as u32;
            (0..=max_x).collect()
        } else {
            vec![0]
        };

        for &x in &x_values {
            let cost = if has_x {
                right_cost.with_x_expanded(x)
            } else {
                right_cost.clone()
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
                        cast_modifier: crate::actions::CastModifier::SplitRight,
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
        let face_cost = face_cost.with_generic_delta(
            state.spell_cost_delta(&face.characteristics, player));
        let face_is_instant_speed = face.characteristics.types.is_instant();
        if !face_is_instant_speed && !sorcery_speed_ok { continue; }

        let reqs: Vec<TargetRequirement> = face.spell_ability.as_ref()
            .map(|sa| sa.target_requirements.clone())
            .unwrap_or_default();
        let target_selections = enumerate_target_selections(&reqs, state, id, player);
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
        let printed_cost = printed_cost.with_generic_delta(
            state.spell_cost_delta(&obj.characteristics, player));

        let reqs: Vec<TargetRequirement> = registry.get(obj.card_id)
            .and_then(|def| def.spell_ability.as_ref())
            .map(|sa| sa.target_requirements.clone())
            .unwrap_or_default();
        let target_selections = enumerate_target_selections(&reqs, state, id, player);
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

    // Impulse plays from exile (CR 601.3e). Walks exile for cards this
    // player owns that carry `impulse_play_pending` (set by
    // Effect::ImpulseExile, "you may play them until end of turn") and
    // emits a normal-cost cast of each nonland spell — a pure zone
    // override, structurally identical to the adventure-creature track
    // above. Playing impulse-exiled *lands* is a documented partial.
    for id in sorted_ids_in_zone(state, Zone::Exile) {
        let Some(obj) = state.objects.get(id) else { continue; };
        if obj.owner != player { continue; }
        if !obj.impulse_play_pending { continue; }
        if obj.is_land() { continue; }

        let is_instant_speed = obj.is_instant()
            || state.has_keyword(id, &crate::effects::KeywordAbility::Flash);
        if !is_instant_speed && !sorcery_speed_ok { continue; }

        let Some(printed_cost) = obj.characteristics.mana_cost.clone()
            else { continue; };
        let printed_cost = printed_cost.with_generic_delta(
            state.spell_cost_delta(&obj.characteristics, player));

        let reqs: Vec<TargetRequirement> = registry.get(obj.card_id)
            .and_then(|def| def.spell_ability.as_ref())
            .map(|sa| sa.target_requirements.clone())
            .unwrap_or_default();
        let target_selections = enumerate_target_selections(&reqs, state, id, player);
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
                            crate::actions::CastModifier::ImpulsePlay,
                        cost_reductions: crate::actions::CostReductions::default(),
                    });
                }
            }
        }
    }

    actions
}

/// Ceiling on how many target selections one requirement-list produces.
/// The Cartesian product across single-target clauses is n^k in board
/// size; this keeps a single spell/ability's cast/activate fan-out
/// bounded (mirrors [`MAX_COMBAT_ENUM`]). 256 is far more target
/// permutations than any consumer enumerates; the lowest-id selections
/// come first, and an exact combination can always be submitted directly.
const MAX_TARGET_SELECTIONS: usize = 256;

/// Cartesian product of legal target choices across the requirement
/// list. Returns `[TargetSelection::new()]` (single empty selection)
/// if there are no requirements. For a single-target requirement
/// with count=Exactly(1), yields one selection per legal choice.
///
/// Phase 1 limit: only `TargetCount::Exactly(1)` and `Exactly(0)`
/// are enumerated per clause. Multi-target (`Exactly(2)`, `UpTo`,
/// `Any`, `X`) fall back to a single empty selection so we never
/// emit over-combinatorial action sets.
pub(crate) fn enumerate_target_selections(
    requirements: &[TargetRequirement],
    state: &GameState,
    source: crate::objects::ObjectId,
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
                let choices = req.filter.enumerate_legal(state, source, source_controller);
                let mut next = Vec::new();
                // Bound the Cartesian product across single-target clauses:
                // a k-target spell over an n-permanent board is otherwise
                // n^k. The lowest-id targets are enumerated first (choices
                // come pre-sorted), so a representative selection is always
                // present; a consumer wanting an exact combination submits
                // it directly. See [`MAX_TARGET_SELECTIONS`].
                'build: for partial in &partials {
                    for choice in &choices {
                        // Re-check the outer controller constraint —
                        // `enumerate_legal` doesn't apply it.
                        if !req.matches_choice(choice, state, source, source_controller) {
                            continue;
                        }
                        let mut extended = partial.clone();
                        extended.push(choice.clone());
                        next.push(extended);
                        if next.len() >= MAX_TARGET_SELECTIONS { break 'build; }
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
    cap: usize,
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
    enumerate_groups(&groups, max_size, 0, &mut current, &mut out, cap);
    out
}

/// `cap` bounds the number of subsets emitted. Mana-reduction callers
/// (delve / convoke / improvise) pass `usize::MAX` and are naturally
/// bounded by `max_size` (the cost's generic pips); the combat blocker
/// enumerator passes [`MAX_COMBAT_ENUM`] because its `max_size` is the
/// eligible-blocker count and the subset space is otherwise 2^k.
fn enumerate_groups<T: Copy>(
    groups: &[Vec<T>],
    remaining: usize,
    gidx: usize,
    current: &mut Vec<T>,
    out: &mut Vec<Vec<T>>,
    cap: usize,
) {
    if out.len() >= cap { return; }
    if gidx == groups.len() {
        out.push(current.clone());
        return;
    }
    let group = &groups[gidx];
    let max_take = group.len().min(remaining);
    for take in 0..=max_take {
        if out.len() >= cap { return; }
        for item in group.iter().take(take) {
            current.push(*item);
        }
        enumerate_groups(groups, remaining - take, gidx + 1, current, out, cap);
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
        &candidates, max_generic, usize::MAX,
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
        &candidates, max_pips, usize::MAX,
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
        &candidates, max_generic, usize::MAX,
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
        // Registry-backed abilities first, then intrinsic abilities
        // (used by commodity tokens whose canonical activations are
        // stashed directly on the object). Index space is flat:
        // 0..N are the registry's `activated_abilities`, N..N+M are
        // `intrinsic_activated_abilities`. The resolver inverts the
        // same way via `lookup_activated_ability_with_kind`.
        // Flat index space: 0..R registry, R..R+I intrinsic,
        // R+I.. granted-by-attachment (Aura/Equipment "enchanted
        // creature has '[cost]: …'"). `lookup_activated_ability`
        // inverts in the same order.
        let reg_count = registry.get(obj.card_id)
            .map_or(0, |d| d.activated_abilities.len());
        let granted = state.granted_activated_for(obj.id);
        let abilities = registry.get(obj.card_id)
            .map(|d| d.activated_abilities.as_slice())
            .unwrap_or(&[])
            .iter()
            .chain(obj.intrinsic_activated_abilities.iter())
            .chain(granted.iter().copied());
        for (i, ability) in abilities.enumerate() {
            // Mark which list the index falls into for the activator's
            // benefit (debug); not surfaced in the Action — `i` alone
            // is the source of truth.
            let _is_intrinsic = i >= reg_count;
            if !ability_is_activatable(
                state, obj, ability, i, player, sorcery_speed_ok, registry,
            ) {
                continue;
            }
            // Enumerate payment plans for the mana portion of the cost.
            let ctx = SpendContext::for_activated_ability();
            let pool = &state.player(player).mana_pool;
            // Training Grounds class: activated-ability cost
            // modifiers adjust the generic component (floor 0).
            // MANA ABILITIES are exempt (Suppression Field's printed
            // exception, and a +N tax on land taps would otherwise
            // deadlock mana production entirely).
            let modified_ability_cost = if ability.is_mana_ability {
                ability.cost.mana_cost.clone()
            } else {
                ability.cost.mana_cost
                    .with_generic_delta(state.ability_cost_delta(obj.id))
            };
            // Generic-{X} fan-out (CR 107.3 / 601.2b for activated costs):
            // expand {X} to each affordable value (0..=pool total, a safe
            // over-approximation the mana solver filters), tagging each
            // resulting plan with its X. The tag rides onto the activation
            // as an `ActivationX` marker so apply stamps the stack entry's
            // x_value (the mana analog of the −X loyalty path). Only loyalty
            // activations fanned out before; a normal `{X}` cost used to
            // resolve at X=0.
            let has_mana_x = modified_ability_cost.x_count() > 0;
            let mana_x_values: Vec<u32> = if has_mana_x {
                let max_x = pool.total() as u32;
                (0..=max_x).collect()
            } else {
                vec![0] // sentinel; cost used as-is, tag = None
            };
            let plans: Vec<(crate::actions::ManaPaymentPlan, Option<u32>)> = {
                let mut out = Vec::new();
                for &mx in &mana_x_values {
                    let cost = if has_mana_x {
                        modified_ability_cost.with_x_expanded(mx)
                    } else {
                        modified_ability_cost.clone()
                    };
                    let tag = if has_mana_x { Some(mx) } else { None };
                    if cost.is_empty() {
                        out.push((crate::actions::ManaPaymentPlan::empty(), tag));
                    } else {
                        for plan in enumerate_payment_plans(&cost, pool, None, &ctx) {
                            out.push((plan, tag));
                        }
                    }
                }
                out
            };
            if plans.is_empty() { continue; }

            let target_selections = enumerate_target_selections(
                &ability.target_requirements, state, id, player);
            if target_selections.is_empty() { continue; }

            let additional = build_additional_costs(&ability.cost, id);

            // Choice-bearing additional costs: "sacrifice a [filtered
            // permanent]" / "discard a [filtered card]". Each enumerates
            // one payment per matching object the activator owns,
            // excluding the source itself (matches the common
            // "sacrifice/discard ANOTHER ~" wording). If a required
            // choice has zero candidates the ability isn't activatable.
            let sac_choices = enumerate_cost_sacrifices(
                state, ability, player, id);
            if sac_choices.is_empty() { continue; }
            let discard_choices = enumerate_cost_discards(
                state, ability, player, id);
            if discard_choices.is_empty() { continue; }
            let tap_choices = enumerate_cost_taps(
                state, ability, player, id);
            if tap_choices.is_empty() { continue; }
            // "Exile [filter] card(s) from your graveyard" cost.
            let exile_gy_choices = enumerate_cost_exile_graveyard(
                state, ability, player);
            if exile_gy_choices.is_empty() { continue; }
            // "Discard N at random" gates on hand size but is NOT enumerated
            // (the cards are chosen by the engine RNG in apply, so no
            // per-card fan-out and no baked payment). Skip if too few cards.
            if ability.cost.discard_random > 0 {
                let hand = state.objects.objects_in_zone(Zone::Hand(player)).count();
                if hand < ability.cost.discard_random as usize { continue; }
            }

            // CR 606.5 "−X" loyalty: fan out one activation per X in
            // 1..=current loyalty, each removing X Loyalty counters. The
            // chosen X rides in the RemoveCounters payment; apply derives
            // the stack entry's x_value from it. (Empty when loyalty is
            // 0 — can't pay any X ≥ 1.) `None` = the ordinary fixed-cost
            // path.
            let x_loyalties: Vec<Option<u32>> = if ability.cost.remove_loyalty_x {
                let loy = obj.count_counters(crate::types::CounterKind::Loyalty);
                (1..=loy).map(Some).collect()
            } else {
                vec![None]
            };
            for (plan, mana_x) in &plans {
                for targets in &target_selections {
                    for sac in &sac_choices {
                        for disc in &discard_choices {
                            for taps in &tap_choices {
                                for eg in &exile_gy_choices {
                                for x in &x_loyalties {
                                let mut costs = additional.clone();
                                if let Some(mx) = mana_x {
                                    costs.push(
                                        crate::actions::AdditionalCostPayment::ActivationX(*mx));
                                }
                                if !eg.is_empty() {
                                    costs.push(
                                        crate::actions::AdditionalCostPayment::ExileFromGraveyard(
                                            eg.clone()));
                                }
                                if let Some(n) = x {
                                    costs.push(
                                        crate::actions::AdditionalCostPayment::RemoveCounters {
                                            source: id,
                                            kind: crate::types::CounterKind::Loyalty,
                                            count: *n,
                                        });
                                }
                                for s in sac {
                                    costs.push(
                                        crate::actions::AdditionalCostPayment::Sacrifice(*s));
                                }
                                for d in disc {
                                    costs.push(
                                        crate::actions::AdditionalCostPayment::Discard(*d));
                                }
                                if !taps.is_empty() {
                                    costs.push(
                                        crate::actions::AdditionalCostPayment::TapCreatures(
                                            taps.clone()));
                                }
                                out.push(Action::ActivateAbility {
                                    source: id,
                                    ability_index: i,
                                    targets: targets.clone(),
                                    mana_payment: plan.clone(),
                                    additional_costs: costs,
                                });
                                } // x_loyalties
                                } // exile_gy_choices
                            }
                        }
                    }
                }
            }
        }
    }
    out
}

/// Candidate permanents the activator can sacrifice to pay a
/// `sacrifice_other` cost. Each inner `Vec` is one complete payment (the
/// permanents to sacrifice for that activation). Returns `vec![vec![]]`
/// (one no-op payment) when the ability has no such cost, so the
/// enumeration loop runs exactly once; an empty outer `Vec` means the
/// cost exists but can't be satisfied (ability not activatable). The
/// source object is always excluded. `sacrifice_other_count` (default 1)
/// gives one payment per N-permanent combination — mirrors
/// [`enumerate_cost_discards`].
/// Candidate graveyard cards the activator can exile to pay an
/// `exile_graveyard_other` cost (mirror of [`enumerate_cost_sacrifices`],
/// but the activator's GRAVEYARD). `vec![vec![]]` = no such cost (loop runs
/// once); empty outer Vec = cost exists but unpayable (ability not legal).
fn enumerate_cost_exile_graveyard(
    state: &GameState,
    ability: &crate::registry::ActivatedAbilityDef,
    player: crate::types::PlayerId,
) -> Vec<Vec<ObjectId>> {
    let Some(filter) = ability.cost.exile_graveyard_other.as_ref() else {
        return vec![vec![]];
    };
    let candidates: Vec<ObjectId> = state.objects
        .objects_in_zone(Zone::Graveyard(player))
        .filter(|o| filter.matches(o, state, player))
        .map(|o| o.id)
        .collect();
    let n = ability.cost.exile_graveyard_count.max(1) as usize;
    if candidates.len() < n { return Vec::new(); }
    combinations(&candidates, n)
}

fn enumerate_cost_sacrifices(
    state: &GameState,
    ability: &crate::registry::ActivatedAbilityDef,
    player: crate::types::PlayerId,
    source: ObjectId,
) -> Vec<Vec<ObjectId>> {
    let Some(filter) = ability.cost.sacrifice_other.as_ref() else {
        return vec![vec![]];
    };
    let candidates: Vec<ObjectId> = state.objects.objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == player && o.id != source)
        .filter(|o| filter.matches(o, state, player))
        .map(|o| o.id)
        .collect();
    let n = ability.cost.sacrifice_other_count.max(1) as usize;
    if candidates.len() < n { return Vec::new(); }
    combinations(&candidates, n)
}

/// Untapped permanents the activator can tap to pay a `tap_other`
/// cost. Same contract as [`enumerate_cost_sacrifices`]:
/// `vec![vec![]]` = no such cost; empty outer Vec = cost exists but
/// unpayable; the source is always excluded.
fn enumerate_cost_taps(
    state: &GameState,
    ability: &crate::registry::ActivatedAbilityDef,
    player: crate::types::PlayerId,
    source: ObjectId,
) -> Vec<Vec<ObjectId>> {
    let Some(filter) = ability.cost.tap_other.as_ref() else {
        return vec![vec![]];
    };
    let candidates: Vec<ObjectId> = state.objects.objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == player && o.id != source && !o.is_tapped())
        .filter(|o| filter.matches(o, state, player))
        .map(|o| o.id)
        .collect();
    let n = ability.cost.tap_other_count.max(1) as usize;
    if candidates.len() < n { return Vec::new(); }
    combinations(&candidates, n)
}

/// Card-sets the activator can discard to pay the discard cost. Each
/// inner `Vec` is one complete payment (the cards to discard for that
/// activation). Returns `vec![vec![]]` (one no-op payment) when there
/// is no discard cost, so the enumeration loop runs exactly once; an
/// empty outer `Vec` means the cost exists but can't be satisfied
/// (ability not activatable). The source is always excluded.
///
/// - `discard_hand`: one payment discarding the activator's whole hand
///   (always satisfiable — an empty hand discards zero cards).
/// - `discard_other` (count N, default 1): one payment per N-card
///   combination of matching hand cards.
fn enumerate_cost_discards(
    state: &GameState,
    ability: &crate::registry::ActivatedAbilityDef,
    player: crate::types::PlayerId,
    source: ObjectId,
) -> Vec<Vec<ObjectId>> {
    if ability.cost.discard_hand {
        let hand: Vec<ObjectId> = state.objects.objects_in_zone(Zone::Hand(player))
            .filter(|o| o.id != source)
            .map(|o| o.id)
            .collect();
        return vec![hand];
    }
    let Some(filter) = ability.cost.discard_other.as_ref() else {
        return vec![vec![]];
    };
    let candidates: Vec<ObjectId> = state.objects.objects_in_zone(Zone::Hand(player))
        .filter(|o| o.id != source)
        .filter(|o| filter.matches(o, state, player))
        .map(|o| o.id)
        .collect();
    let n = ability.cost.discard_other_count.max(1) as usize;
    if candidates.len() < n { return Vec::new(); }
    combinations(&candidates, n)
}

/// All `k`-element combinations of `items` (order-independent), as
/// owned `Vec`s. `k == 0` yields a single empty combination. Used for
/// multi-card discard-cost enumeration; hands are small so this stays
/// cheap.
fn combinations(items: &[ObjectId], k: usize) -> Vec<Vec<ObjectId>> {
    if k == 0 { return vec![vec![]]; }
    if k > items.len() { return Vec::new(); }
    let mut out = Vec::new();
    // Index-based: pick item[i], then k-1 from the suffix after i.
    for i in 0..=items.len() - k {
        for mut tail in combinations(&items[i + 1..], k - 1) {
            let mut combo = Vec::with_capacity(k);
            combo.push(items[i]);
            combo.append(&mut tail);
            out.push(combo);
        }
    }
    out
}

/// The mana `player` could produce **right now** — their current floating pool
/// plus everything their mana abilities can still make — as a concrete
/// [`crate::mana::ManaPool`]. The engine primitive behind a UI "available mana"
/// gauge and "can I afford it if I tap out" checks (compose with
/// [`crate::mana::can_afford`] over the returned pool).
///
/// Computed by simulating activations on a clone: CR 605 mana abilities resolve
/// immediately, so the loop re-enumerates the player's activatable abilities
/// against the *growing* pool and applies one mana ability per pass until none
/// remain. Re-enumerating each pass is what makes chained sources correct — a
/// filter land ("{U}, {T}: Add {G}{G}") only becomes activatable once a basic
/// has been tapped for its {U} input. It reuses the real cost-gated enumeration
/// and activation path, so every cost (tap / mana / sacrifice / counters / …) is
/// honored; this never mutates the caller's `state`.
///
/// LIMITATION: a flexible "any color" source resolves its color via the engine's
/// deterministic fallback (mana abilities can't post a player choice, CR 605.3),
/// so it contributes its *default* color rather than every option. Fixed sources
/// (basic lands, mono-color producers, filter chains) are exact.
pub fn available_mana(
    state: &GameState,
    player: PlayerId,
    registry: &CardRegistry,
) -> crate::mana::ManaPool {
    let mut sim = state.clone();
    // Each pass activates one distinct (source, ability) and a source either
    // taps or is consumed, so progress is bounded by the permanent count; `+ 8`
    // covers the rarer hand/graveyard mana activations.
    let guard = sim.objects.objects_in_zone(Zone::Battlefield).count() + 8;
    let mut done: std::collections::HashSet<(ObjectId, usize)> =
        std::collections::HashSet::new();

    for _ in 0..guard {
        // `apply_activate_ability` pays/produces as the priority player; retarget
        // priority so we measure `player` (its board is public, so either seat is
        // fair game). Re-asserted each pass for robustness.
        sim.priority.give_to(player);
        let next = enumerate_activation_actions(&sim, player, registry)
            .into_iter()
            .find(|a| match a {
                Action::ActivateAbility { source, ability_index, .. } => {
                    !done.contains(&(*source, *ability_index))
                        && sim.objects.get(*source)
                            .and_then(|o| lookup_activated_ability(
                                o, registry, &sim, *ability_index))
                            .map_or(false, |ab| ab.is_mana_ability)
                }
                _ => false,
            });
        let Some(action) = next else { break };
        if let Action::ActivateAbility { source, ability_index, .. } = &action {
            done.insert((*source, *ability_index));
        }
        crate::engine::apply_action(&mut sim, action, registry);
    }
    sim.player(player).mana_pool.clone()
}

/// `legal_actions` for `player` as if they had first produced all their
/// [`available_mana`] — i.e., the actions that *would* be legal once they tap
/// out. This is the basis for a "what can I play this turn" castability
/// highlight and an auto-pass heuristic, both of which must see through the
/// engine's manual-tap model (casting checks the floating pool, so nothing looks
/// castable until mana is floated). Real timing/zone/stack rules still apply —
/// only the pool is hypothetical — so a sorcery stays unplayable on the
/// opponent's turn. Clones; never mutates `state`.
pub fn potential_actions(
    state: &GameState,
    player: PlayerId,
    registry: &CardRegistry,
) -> Vec<Action> {
    let avail = available_mana(state, player, registry);
    let mut sim = state.clone();
    sim.priority.give_to(player);
    sim.player_mut(player).mana_pool = avail;
    legal_actions(&sim, registry)
}

/// Object ids of the cards `player` could cast or play this turn if they tapped
/// out — the `CastSpell` / `PlayLand` entries of [`potential_actions`]. Drives a
/// "playable" highlight on the hand.
pub fn playable_cards(
    state: &GameState,
    player: PlayerId,
    registry: &CardRegistry,
) -> Vec<ObjectId> {
    potential_actions(state, player, registry)
        .iter()
        .filter_map(|a| match a {
            Action::CastSpell { object_id, .. } => Some(*object_id),
            Action::PlayLand { object_id, .. } => Some(*object_id),
            _ => None,
        })
        .collect()
}

/// True if `player` has any non-trivial play available now or after tapping out
/// — a `CastSpell`, a `PlayLand`, or a *non-mana* activated ability appears in
/// [`potential_actions`]. False ⇒ the only options are passing or tapping mana
/// with nothing to spend it on, so a UI may safely auto-pass this priority
/// window (CR: a mana ability produces mana that empties at end of step, so
/// floating it with nothing to cast accomplishes nothing). An unrecognized
/// activated ability counts as meaningful, so the check never hides a real play.
pub fn has_meaningful_play(
    state: &GameState,
    player: PlayerId,
    registry: &CardRegistry,
) -> bool {
    potential_actions(state, player, registry).iter().any(|a| match a {
        Action::PassPriority | Action::Concede => false,
        Action::ActivateAbility { source, ability_index, .. } => {
            state.objects.get(*source)
                .and_then(|o| lookup_activated_ability(o, registry, state, *ability_index))
                .map_or(true, |ab| !ab.is_mana_ability)
        }
        _ => true,
    })
}

// --- auto-tap ---------------------------------------------------------------

const ALL_MANA_COLORS: [crate::types::ManaColor; 6] = {
    use crate::types::ManaColor::*;
    [White, Blue, Black, Red, Green, Colorless]
};

fn mana_color_index(c: crate::types::ManaColor) -> usize {
    use crate::types::ManaColor::*;
    match c { White => 0, Blue => 1, Black => 2, Red => 3, Green => 4, Colorless => 5 }
}

fn pool_color_counts(pool: &crate::mana::ManaPool) -> [u32; 6] {
    let mut c = [0u32; 6];
    for u in pool.iter() { c[mana_color_index(u.color)] += 1; }
    c
}

/// Does `action` cast or play the hand card `target`?
fn action_plays_card(action: &Action, target: ObjectId) -> bool {
    matches!(action,
        Action::CastSpell { object_id, .. } | Action::PlayLand { object_id, .. }
            if *object_id == target)
}

fn is_mana_activation(state: &GameState, action: &Action, registry: &CardRegistry) -> bool {
    matches!(action, Action::ActivateAbility { source, ability_index, .. }
        if state.objects.get(*source)
            .and_then(|o| lookup_activated_ability(o, registry, state, *ability_index))
            .map_or(false, |ab| ab.is_mana_ability))
}

/// The mana colors `action` (a mana-ability activation) nets for `player`,
/// measured by applying it on a clone — the effect is an opaque fn pointer, and
/// measuring the per-color delta handles filters (which spend then produce).
fn mana_production_colors(
    state: &GameState, player: PlayerId, action: &Action, registry: &CardRegistry,
) -> Vec<crate::types::ManaColor> {
    let before = pool_color_counts(&state.player(player).mana_pool);
    let mut sim = state.clone();
    crate::engine::apply_action(&mut sim, action.clone(), registry);
    let after = pool_color_counts(&sim.player(player).mana_pool);
    ALL_MANA_COLORS.iter().copied()
        .filter(|&c| after[mana_color_index(c)] > before[mana_color_index(c)])
        .collect()
}

/// Colors a cost still needs that the current pool can't cover (colored pips
/// only; generic/colorless are handled by raw mana count). A heuristic for
/// choosing which source to tap next.
fn unmet_cost_colors(
    cost: Option<&crate::mana::ManaCost>, pool: &crate::mana::ManaPool,
) -> Vec<crate::types::ManaColor> {
    let Some(cost) = cost else { return Vec::new(); };
    let mut need = [0i32; 6];
    for comp in &cost.components {
        if let crate::mana::ManaCostComponent::Colored(c) = comp {
            need[mana_color_index(c.to_mana())] += 1;
        }
    }
    let have = pool_color_counts(pool);
    ALL_MANA_COLORS.iter().copied()
        .filter(|&c| need[mana_color_index(c)] > have[mana_color_index(c)] as i32)
        .collect()
}

/// Pick the next mana source to tap toward affording `cost`: prefer one that
/// supplies a still-unmet colored pip, and among those the least flexible (fewest
/// colors), so dual/any-color sources are preserved for when they're needed.
fn choose_mana_source(
    state: &GameState, player: PlayerId, candidates: &[Action],
    cost: Option<&crate::mana::ManaCost>, registry: &CardRegistry,
) -> Action {
    let unmet = unmet_cost_colors(cost, &state.player(player).mana_pool);
    let mut best: Option<&Action> = None;
    let mut best_key = (false, usize::MAX); // (supplies an unmet color, #colors)
    for c in candidates {
        let prod = mana_production_colors(state, player, c, registry);
        let key = (prod.iter().any(|col| unmet.contains(col)), prod.len());
        let better = best.is_none()
            || (key.0 && !best_key.0)
            || (key.0 == best_key.0 && key.1 < best_key.1);
        if better { best = Some(c); best_key = key; }
    }
    best.cloned().unwrap_or_else(|| candidates[0].clone())
}

/// Plan an "auto-tap and cast" for the hand card `target`: the sequence of
/// mana-ability activations that make it castable/playable, followed by the
/// cast/play itself **iff** there is exactly one way to do it (no target / mode /
/// X choice to make — otherwise the taps are returned alone so the caller can
/// surface the now-available choices). Returns `None` if `target` can't be made
/// castable even tapping out (i.e. it isn't playable).
///
/// Planned on a clone via [`crate::engine::step`] — the same path the caller will
/// replay the sequence through — so applying it reproduces the plan exactly. The
/// engine's payment solver fills each `CastSpell`'s `ManaPaymentPlan`, so all the
/// hard cost assignment stays engine-side; this only decides which sources to tap.
/// Shares [`available_mana`]'s flexible-source caveat (a source's color is its
/// deterministic default).
pub fn auto_tap_sequence(
    state: &GameState, registry: &CardRegistry, player: PlayerId, target: ObjectId,
) -> Option<Vec<Action>> {
    let cost = state.objects.get(target)
        .and_then(|o| o.characteristics.mana_cost.clone());
    auto_tap_for(state, registry, player, cost.as_ref(),
        |a| action_plays_card(a, target))
}

/// Like [`auto_tap_sequence`] but the "play" is ACTIVATING a (non-mana) ability of
/// `source` — e.g. Codie's `{4},{T}: …`. Taps mana until the activation is legal,
/// then appends it (single ability/target) or leaves the taps so the caller
/// surfaces the ability/target choice. Mana abilities of `source` itself are NOT
/// the target (they're the tap sources); only mana-COSTING abilities are auto-paid.
pub fn auto_tap_activate_sequence(
    state: &GameState, registry: &CardRegistry, player: PlayerId, source: ObjectId,
) -> Option<Vec<Action>> {
    auto_tap_for(state, registry, player, None, |a| {
        matches!(a, Action::ActivateAbility { source: s, ability_index, .. }
            if *s == source
                && state.objects.get(source)
                    .and_then(|o| lookup_activated_ability(o, registry, state, *ability_index))
                    .is_some_and(|ab| !ab.is_mana_ability))
    })
}

/// Shared driver for [`auto_tap_sequence`] / [`auto_tap_activate_sequence`]: float
/// mana (tapping mana abilities, color-aware via `cost`) until some action passes
/// `is_play`, then return the taps plus that action when it's unambiguous (else
/// just the taps, for the caller to surface the remaining choice). `None` if mana
/// can't be produced and no play ever becomes legal.
fn auto_tap_for(
    state: &GameState, registry: &CardRegistry, player: PlayerId,
    cost: Option<&crate::mana::ManaCost>, is_play: impl Fn(&Action) -> bool,
) -> Option<Vec<Action>> {
    let mut sim = state.clone();
    sim.priority.give_to(player);
    let mut seq: Vec<Action> = Vec::new();
    let guard = sim.objects.objects_in_zone(Zone::Battlefield).count() + 4;

    for _ in 0..guard {
        let legal = legal_actions(&sim, registry);
        let plays: Vec<Action> = legal.iter().filter(|a| is_play(a)).cloned().collect();
        if !plays.is_empty() {
            if plays.len() == 1 {
                seq.push(plays.into_iter().next().unwrap());
            }
            return Some(seq);
        }
        let candidates: Vec<Action> = legal.into_iter()
            .filter(|a| is_mana_activation(&sim, a, registry)).collect();
        if candidates.is_empty() {
            return None; // can't produce more mana, and the play isn't legal
        }
        let pick = choose_mana_source(&sim, player, &candidates, cost, registry);
        seq.push(pick.clone());
        let (next, _yld) = crate::engine::step(sim, pick, registry);
        sim = next;
    }
    None
}

/// Look up an activated ability by flat index — registry abilities
/// first, then [`crate::objects::GameObject::intrinsic_activated_abilities`].
/// Returns the ability and a tag identifying which list it came from
/// (debug / dispatch hint; the engine treats them uniformly).
pub(crate) fn lookup_activated_ability<'a>(
    obj: &'a crate::objects::GameObject,
    registry: &'a crate::registry::CardRegistry,
    state: &'a GameState,
    index: usize,
) -> Option<&'a crate::registry::ActivatedAbilityDef> {
    let reg_count = registry.get(obj.card_id)
        .map_or(0, |d| d.activated_abilities.len());
    let intrinsic_count = obj.intrinsic_activated_abilities.len();
    if index < reg_count {
        registry.get(obj.card_id)
            .and_then(|d| d.activated_abilities.get(index))
    } else if index < reg_count + intrinsic_count {
        obj.intrinsic_activated_abilities.get(index - reg_count)
    } else {
        // Granted-by-attachment tier — same `continuous_effects` order
        // as the enumerator (`granted_activated_for`).
        state.granted_activated_for(obj.id)
            .get(index - reg_count - intrinsic_count)
            .copied()
    }
}

/// Is this ability timing-legal and cost-payable right now? Covers
/// tap-cost (must be untapped), sacrifice-cost (must exist), and
/// the mana-ability-at-any-time rule vs. sorcery-speed abilities.
fn ability_is_activatable(
    state: &GameState,
    obj: &crate::objects::GameObject,
    ability: &crate::registry::ActivatedAbilityDef,
    ability_index: usize,
    activator: crate::types::PlayerId,
    sorcery_speed_ok: bool,
    reg: &crate::registry::CardRegistry,
) -> bool {
    // Zone gate: the ability's declared `activation_zone` must match
    // the object's current zone (CR 113.6). Cycling (Hand) and the
    // usual permanent abilities (Battlefield) go through the same
    // helper — the only axis that varies is which zone the object
    // lives in right now.
    if !ability.activation_zone.matches(obj.zone, obj.owner) {
        return false;
    }
    // Face gate (CR 712): multi-face cards can have abilities that
    // only apply on a specific face. The shared `activated_abilities`
    // list holds all faces' abilities; this gate filters out ones
    // that don't belong to the object's current `visible_face`.
    // Single-face cards skip this check (`face_gate` defaults to
    // `None`, ability applies on any face).
    if let Some(required_face) = ability.face_gate {
        if obj.visible_face != required_face {
            return false;
        }
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
    // Pure-precondition counter gate (CR 717.5b — Class level-up,
    // among other things): source must have at least `count` counters
    // of `kind`, but they are NOT removed by activation. Distinct
    // from `remove_self_counter` above (which both gates and consumes).
    if let Some((kind, count)) = ability.cost.min_self_counters {
        if obj.count_counters(kind) < count {
            return false;
        }
    }
    // Pure board/zone/player precondition (CR 602.5b — "you may activate
    // this ability only if <condition>"): a predicate over game state
    // that gates legality without paying anything. Resolves subtype/card
    // names via the registry (e.g. "only if you control a Swamp").
    if let Some(cond) = ability.cost.activation_condition {
        if !cond(state, obj.id, activator, reg) {
            return false;
        }
    }
    // CR 602.5d — "Activate only once each turn" (Boast). The ledger
    // is recorded by apply_activate_ability after costs clear; here we
    // only gate legality.
    if ability.cost.once_per_turn
        && state.abilities_activated_this_turn.contains(&(obj.id, ability_index))
    {
        return false;
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
    if cost.exile_self {
        // CR 702.41a — Scavenge's "Exile this card from your
        // graveyard" cost. ExileFromGraveyard moves the source to
        // exile via the shared additional-cost path.
        v.push(crate::actions::AdditionalCostPayment::ExileFromGraveyard(
            vec![source]));
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
            matches!(a, Action::PlayLand { object_id, .. } if *object_id == l)));
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

    /// CR 601.3e — a permanent imposing `CastRestriction::Permanents` (Codie,
    /// Vociferous Codex) makes permanent spells uncastable while it's in play,
    /// but instants/sorceries stay castable.
    #[test]
    fn cast_restriction_blocks_permanent_spells() {
        use crate::registry::{CardDefinition, CastRestriction};
        let mut reg = CardRegistry::new();
        let nm = reg.interner_mut().intern("TestCodex");
        let codex_chars = Characteristics { types: TypeLine::ARTIFACT.into(), ..Default::default() };
        let codex_id = reg.register(
            CardDefinition::new(nm, codex_chars.clone())
                .with_cant_cast(CastRestriction::Permanents));

        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        // The restricting permanent on the battlefield (controller 0).
        let oid = s.allocate_object_id();
        let mut codex = GameObject::new(oid, 0, Zone::Battlefield, codex_id, codex_chars);
        codex.controller = 0;
        s.objects.insert(codex);
        // A creature (permanent spell) and an instant in hand; plenty of mana.
        let creature = put(&mut s, 0, Zone::Hand(0), creature_chars(2, 2));
        let inst = put(&mut s, 0, Zone::Hand(0), instant_chars());
        add_mana(&mut s, 0, ManaColor::Green, 5);
        add_mana(&mut s, 0, ManaColor::Red, 5);

        let actions = legal_actions(&s, &reg);
        let castable = |q: ObjectId| actions.iter().any(|a|
            matches!(a, Action::CastSpell { object_id, .. } if *object_id == q));
        assert!(!castable(creature), "permanent spell must be uncastable under the restriction");
        assert!(castable(inst), "instant must remain castable");
    }

    // --- PayOrDecline (Ward) affordability gate ----------------------------

    /// Build a state with a pending Ward-style PayOrDecline on player 0.
    fn pending_pay_or_decline(cost: &str) -> GameState {
        use crate::actions::{ChoiceContext, ChoiceKind, DeclineConsequence,
                             PendingChoice};
        let mut s = GameState::new(2, 0);
        let target = put(&mut s, 0, Zone::Battlefield, creature_chars(1, 1));
        s.pending_choice = Some(PendingChoice {
            id: 7,
            choosing_player: 0,
            context: ChoiceContext::ResolvingStack(target),
            kind: ChoiceKind::PayOrDecline {
                cost: ManaCost::parse(cost).unwrap(),
                on_decline: DeclineConsequence::CounterStackEntry(target),
            },
        });
        s
    }

    #[test]
    fn pay_or_decline_hides_pay_when_unaffordable() {
        // Regression (found by the random-game harness): with no mana the
        // engine must NOT offer the pay branch — it would hit the solver
        // with no valid plan and panic in auto_pay_ward_cost.
        let s = pending_pay_or_decline("{3}");
        let actions = legal_actions(&s, &CardRegistry::new());
        let pays = |pay: bool| Action::SubmitResolutionChoice {
            id: 7, response: crate::actions::ChoiceResponse::PayOrDecline { pay } };
        assert!(!actions.contains(&pays(true)),
            "pay must be hidden when the chooser cannot afford the cost");
        assert!(actions.contains(&pays(false)),
            "decline is always available");
    }

    #[test]
    fn pay_or_decline_offers_pay_when_affordable() {
        let mut s = pending_pay_or_decline("{2}");
        add_mana(&mut s, 0, ManaColor::Red, 2);
        let actions = legal_actions(&s, &CardRegistry::new());
        let pays = |pay: bool| Action::SubmitResolutionChoice {
            id: 7, response: crate::actions::ChoiceResponse::PayOrDecline { pay } };
        assert!(actions.contains(&pays(true)), "pay must be offered when affordable");
        assert!(actions.contains(&pays(false)), "decline is always available");
    }

    // --- Combat enumeration caps (anti-OOM) --------------------------------

    #[test]
    fn permutations_capped_and_identity_first() {
        // 9 blockers => 9! = 362880 orderings uncapped. The cap bounds it,
        // and the identity (input) order is emitted first so a canonical
        // OrderBlockers is always available.
        let items: Vec<ObjectId> = (1..=9).collect();
        let perms = permutations(&items);
        assert!(perms.len() <= MAX_COMBAT_ENUM,
            "permutations must be capped, got {}", perms.len());
        assert_eq!(perms[0], items, "identity ordering must be emitted first");
        // Small inputs stay fully exhaustive (3! = 6).
        assert_eq!(permutations(&[1, 2, 3]).len(), 6);
    }

    #[test]
    fn target_selections_capped_on_multi_target_spell() {
        // Two single-target clauses over a 40-creature board would be
        // 40*40 = 1600 selections uncapped; the cap bounds it.
        let mut s = GameState::new(2, 0);
        for _ in 0..40 {
            put(&mut s, 0, Zone::Battlefield, creature_chars(1, 1));
        }
        let tc = || TargetRequirement::target_creature();
        let sels = enumerate_target_selections(&[tc(), tc()], &s, crate::objects::NULL_OBJECT_ID, 0);
        assert!(!sels.is_empty(), "at least one selection");
        assert!(sels.len() <= MAX_TARGET_SELECTIONS,
            "target selections must be capped, got {}", sels.len());
        // A single-target clause over a 40-creature board is just 40 — well
        // under the cap, so it stays fully enumerated.
        let one = enumerate_target_selections(&[tc()], &s, crate::objects::NULL_OBJECT_ID, 0);
        assert_eq!(one.len(), 40);
    }

    #[test]
    fn equivalence_subsets_respect_cap() {
        // 30 distinct keys => 2^30 subsets uncapped (would OOM). The cap
        // keeps it bounded; the empty subset (canonical) is generated first.
        let items: Vec<u32> = (0..30).collect();
        let subs = enumerate_equivalence_subsets(&items, 30, MAX_COMBAT_ENUM, |&x| x);
        assert!(subs.len() <= MAX_COMBAT_ENUM,
            "subset enumeration must be capped, got {}", subs.len());
        assert!(subs.iter().any(|s| s.is_empty()), "empty subset present");
        // Mana callers pass usize::MAX and stay bounded by max_size instead:
        // size<=2 over 5 distinct keys is small and fully enumerated.
        let bounded = enumerate_equivalence_subsets(&[1u32, 2, 3, 4, 5], 2, usize::MAX, |&x| x);
        assert_eq!(bounded.len(), 1 + 5 + 10); // C(5,0)+C(5,1)+C(5,2)
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
    fn declare_attackers_enumerates_multi_attacker_declarations() {
        // Two eligible attackers, one opponent, no planeswalkers/battles → the
        // full cross product is empty + {a} + {b} + {a,b} (4 declarations). The
        // "attack with both" option was impossible under the old single-
        // attacker enumerator.
        let mut s = GameState::new(2, 0);
        s.combat = Some(CombatState {
            phase: CombatPhase::DeclareAttackers, ..CombatState::new() });
        let a = put(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        let b = put(&mut s, 0, Zone::Battlefield, creature_chars(3, 3));
        s.objects.get_mut(a).unwrap().status.summoning_sick = false;
        s.objects.get_mut(b).unwrap().status.summoning_sick = false;

        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(actions.iter().any(|act| matches!(act,
            Action::DeclareAttackers { attackers }
                if attackers.len() == 2
                && attackers.iter().any(|d| d.attacker == a)
                && attackers.iter().any(|d| d.attacker == b)
                && attackers.iter().all(|d|
                    matches!(d.defending, DefendingEntity::Player(1))))),
            "multi-attacker (both) declaration is enumerated");
        let decls = actions.iter()
            .filter(|act| matches!(act, Action::DeclareAttackers { .. })).count();
        assert_eq!(decls, 4, "empty + {{a}} + {{b}} + {{a,b}}");
    }

    #[test]
    fn must_attack_creature_forces_declaration() {
        use crate::layers::{ContinuousEffect, Duration};
        // CR 508.1a — a creature that "attacks each combat if able" must be in
        // every legal declaration; declining (empty) becomes illegal.
        let mut s = GameState::new(2, 0);
        s.combat = Some(CombatState {
            phase: CombatPhase::DeclareAttackers, ..CombatState::new() });
        let a = put(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        let b = put(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        s.objects.get_mut(a).unwrap().status.summoning_sick = false;
        s.objects.get_mut(b).unwrap().status.summoning_sick = false;
        s.add_continuous_effect(ContinuousEffect::must_attack(a, a, Duration::EndOfTurn));

        let actions = legal_actions(&s, &CardRegistry::new());
        let decls: Vec<_> = actions.iter()
            .filter(|act| matches!(act, Action::DeclareAttackers { .. })).collect();
        assert!(!decls.is_empty());
        assert!(decls.iter().all(|act| matches!(act,
            Action::DeclareAttackers { attackers }
                if attackers.iter().any(|d| d.attacker == a))),
            "every declaration includes the must-attack creature");
        assert!(!actions.iter().any(|act| matches!(act,
            Action::DeclareAttackers { attackers } if attackers.is_empty())),
            "declining is illegal when a creature must attack");
    }

    #[test]
    fn goaded_creature_must_attack_if_able() {
        use crate::layers::{ContinuousEffect, Duration};
        // CR 701.38a — Goad both forbids attacking the goader AND requires the
        // creature to attack if able. 3 players so a non-goader defender exists.
        let mut s = GameState::new(3, 0);
        s.combat = Some(CombatState {
            phase: CombatPhase::DeclareAttackers, ..CombatState::new() });
        let atk = put(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        s.objects.get_mut(atk).unwrap().status.summoning_sick = false;
        s.add_continuous_effect(ContinuousEffect::goad(
            atk, atk, /*goader=*/ 1, Duration::UntilYourNextTurn(1)));

        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(!actions.iter().any(|act| matches!(act,
            Action::DeclareAttackers { attackers } if attackers.is_empty())),
            "a goaded creature must attack (it can attack the non-goader)");
        // And it still can't attack the goader (player 1).
        assert!(!actions.iter().any(|act| matches!(act,
            Action::DeclareAttackers { attackers }
                if attackers.iter().any(|d|
                    matches!(d.defending, DefendingEntity::Player(1))))));
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
    fn declare_attackers_includes_battle_as_defender() {
        let mut s = GameState::new(2, 0);
        s.combat = Some(CombatState {
            phase: CombatPhase::DeclareAttackers,
            ..CombatState::new()
        });
        let atk = put(&mut s, 0, Zone::Battlefield, creature_chars(2, 2));
        s.objects.get_mut(atk).unwrap().status.summoning_sick = false;

        // Opponent's battle (CR 508.4 — attackable like a planeswalker).
        let battle_chars = Characteristics {
            types: TypeLine::BATTLE.into(),
            ..Default::default()
        };
        let battle = put(&mut s, 1, Zone::Battlefield, battle_chars);

        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(actions.iter().any(|a|
            matches!(a, Action::DeclareAttackers { attackers }
                if attackers.len() == 1
                && matches!(attackers[0].defending, DefendingEntity::Battle(id) if id == battle))),
            "an opponent's battle must be offered as an attack target");
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
            matches!(a, Action::PlayLand { object_id, .. } if *object_id == land)));
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
                    face_gate: None,
                    effect: |_, _, _| Vec::new(),
                })
        )
    }

    // --- available_mana -----------------------------------------------------

    fn register_mana_source(
        reg: &mut CardRegistry, name: &str, chars: Characteristics,
        cost: ActivationCost, effect: crate::registry::ActivatedEffectFn,
    ) -> CardId {
        use crate::registry::{ActivatedAbilityDef, CardDefinition};
        let nm = reg.interner_mut().intern(name);
        reg.register(CardDefinition::new(nm, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "test mana ability".into(),
                cost,
                target_requirements: vec![],
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: crate::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect,
            }))
    }
    fn add_one_green(
        _: &GameState, ctx: &crate::registry::ActivationContext, _: &CardRegistry,
    ) -> Vec<crate::effects::Effect> {
        vec![crate::effects::Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
        }]
    }
    // a filter land: "{G}, {T}: Add {R}{R}"
    fn add_two_red(
        _: &GameState, ctx: &crate::registry::ActivationContext, _: &CardRegistry,
    ) -> Vec<crate::effects::Effect> {
        vec![crate::effects::Effect::AddMana {
            player: ctx.controller,
            mana: vec![
                ManaUnit::plain(ManaColor::Red, ctx.source),
                ManaUnit::plain(ManaColor::Red, ctx.source),
            ],
        }]
    }

    #[test]
    fn available_mana_sums_untapped_sources_and_pool() {
        let mut reg = CardRegistry::new();
        let forest = register_mana_source(
            &mut reg, "Test Forest", land_chars(), ActivationCost::tap_only(), add_one_green);
        let mut s = GameState::new(2, 0);
        let a = state_put_with_card(&mut s, 0, Zone::Battlefield, land_chars(), forest);
        let _b = state_put_with_card(&mut s, 0, Zone::Battlefield, land_chars(), forest);

        let pool = available_mana(&s, 0, &reg);
        assert_eq!(pool.total(), 2, "two untapped green sources -> 2 mana");
        assert!(pool.iter().all(|u| u.color == ManaColor::Green));

        // the opponent controls no sources
        assert_eq!(available_mana(&s, 1, &reg).total(), 0);

        // tapping a source drops availability by one
        s.objects.get_mut(a).unwrap().tap();
        assert_eq!(available_mana(&s, 0, &reg).total(), 1);

        // current floating mana is counted on top of producible mana
        add_mana(&mut s, 0, ManaColor::Red, 1);
        let pool = available_mana(&s, 0, &reg);
        assert_eq!(pool.total(), 2, "1 producible green + 1 floating red");
        assert_eq!(pool.iter().filter(|u| u.color == ManaColor::Red).count(), 1);
        assert_eq!(pool.iter().filter(|u| u.color == ManaColor::Green).count(), 1);

        // the probe never mutates the caller's state
        assert_eq!(s.player(0).mana_pool.total(), 1, "caller pool unchanged");
        assert!(s.objects.get(_b).unwrap().is_tapped() == false, "caller's land untouched");
    }

    #[test]
    fn available_mana_chains_filter_through_growing_pool() {
        // A filter ("{G},{T}: Add {R}{R}") is only affordable once the basic has
        // been tapped for its {G} — the fixpoint must re-enumerate to find it.
        let mut reg = CardRegistry::new();
        let forest = register_mana_source(
            &mut reg, "Test Forest", land_chars(), ActivationCost::tap_only(), add_one_green);
        let filter_cost = ActivationCost {
            mana_cost: ManaCost::parse("{G}").unwrap(),
            tap: true,
            ..ActivationCost::default()
        };
        let filter = register_mana_source(
            &mut reg, "Test Filter", land_chars(), filter_cost, add_two_red);
        let mut s = GameState::new(2, 0);
        state_put_with_card(&mut s, 0, Zone::Battlefield, land_chars(), forest);
        state_put_with_card(&mut s, 0, Zone::Battlefield, land_chars(), filter);

        // green is spent into the filter, netting {R}{R}.
        let pool = available_mana(&s, 0, &reg);
        assert_eq!(pool.total(), 2, "filter chain nets two mana");
        assert!(pool.iter().all(|u| u.color == ManaColor::Red),
            "the green was consumed paying the filter's input");
    }

    #[test]
    fn playable_cards_and_meaningful_play_reflect_tap_out_castability() {
        use crate::registry::CardDefinition;
        let mut reg = CardRegistry::new();
        let forest = register_mana_source(
            &mut reg, "Test Forest", land_chars(), ActivationCost::tap_only(), add_one_green);
        // a vanilla {G} creature (sorcery-speed cast)
        let bear_def = {
            let nm = reg.interner_mut().intern("Test Bear");
            reg.register(CardDefinition::new(nm, creature_chars(2, 2)))
        };

        // Board: one untapped green source; the {G} bear in hand. The pool is
        // empty, so nothing is castable *right now* — but it is playable if the
        // forest is tapped, which is exactly what the highlight should reflect.
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        state_put_with_card(&mut s, 0, Zone::Battlefield, land_chars(), forest);
        let bear = state_put_with_card(&mut s, 0, Zone::Hand(0), creature_chars(2, 2), bear_def);

        assert!(legal_actions(&s, &reg).iter().all(|a|
            !matches!(a, Action::CastSpell { .. })),
            "nothing is castable from an empty pool (manual-tap model)");
        assert!(playable_cards(&s, 0, &reg).contains(&bear),
            "the bear is playable once the forest is tapped");
        assert!(has_meaningful_play(&s, 0, &reg), "a castable spell is a meaningful play");

        // Tap the only source: now even tapping out yields no mana, so the bear
        // is not playable and the window has nothing to do but pass.
        let land = s.objects.objects_in_zone(Zone::Battlefield)
            .find(|o| o.controller == 0).map(|o| o.id).unwrap();
        s.objects.get_mut(land).unwrap().tap();
        assert!(!playable_cards(&s, 0, &reg).contains(&bear),
            "no mana available -> bear not playable");
        assert!(!has_meaningful_play(&s, 0, &reg),
            "only pass + a now-useless mana source -> safe to auto-pass");
    }

    fn add_one_red(
        _: &GameState, ctx: &crate::registry::ActivationContext, _: &CardRegistry,
    ) -> Vec<crate::effects::Effect> {
        vec![crate::effects::Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
        }]
    }

    fn red_creature_def(reg: &mut CardRegistry) -> (CardId, Characteristics) {
        use crate::registry::CardDefinition;
        let mut chars = creature_chars(3, 3);
        chars.mana_cost = Some(ManaCost::parse("{2}{R}").unwrap());
        chars.colors = ColorSet::red();
        let nm = reg.interner_mut().intern("Test Ogre");
        (reg.register(CardDefinition::new(nm, chars.clone())), chars)
    }

    #[test]
    fn auto_tap_sequence_taps_minimally_and_casts() {
        let mut reg = CardRegistry::new();
        let forest = register_mana_source(
            &mut reg, "Test Forest", land_chars(), ActivationCost::tap_only(), add_one_green);
        let mountain = register_mana_source(
            &mut reg, "Test Mountain", land_chars(), ActivationCost::tap_only(), add_one_red);
        let (ogre_def, ogre_chars) = red_creature_def(&mut reg);

        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        // 3 Forests + 1 Mountain — a naive tap order would burn all 3 Forests
        // before the Mountain (4 taps); the color-aware planner takes the R
        // source first, so a {2}{R} spell needs exactly 3.
        for _ in 0..3 { state_put_with_card(&mut s, 0, Zone::Battlefield, land_chars(), forest); }
        state_put_with_card(&mut s, 0, Zone::Battlefield, land_chars(), mountain);
        let ogre = state_put_with_card(&mut s, 0, Zone::Hand(0), ogre_chars, ogre_def);

        let seq = auto_tap_sequence(&s, &reg, 0, ogre).expect("castable after tapping out");
        let taps = seq.iter().filter(|a| matches!(a, Action::ActivateAbility { .. })).count();
        assert_eq!(taps, 3, "exactly 3 sources tapped for a 3-cost spell (no over-tap)");

        // Replaying the plan (taps, plus the cast if it was bundled) takes the
        // ogre out of hand. If the cast was left out (ambiguous), finish it.
        let mut sim = s.clone();
        for act in seq { let (n, _) = crate::engine::step(sim, act, &reg); sim = n; }
        if sim.objects.get(ogre).map_or(false, |o| o.zone == Zone::Hand(0)) {
            let cast = legal_actions(&sim, &reg).into_iter()
                .find(|a| action_plays_card(a, ogre)).expect("cast now legal");
            let (n, _) = crate::engine::step(sim, cast, &reg); sim = n;
        }
        assert!(sim.objects.get(ogre).map_or(true, |o| o.zone != Zone::Hand(0)),
            "the ogre left the hand via the auto-tap cast");
    }

    #[test]
    fn auto_tap_sequence_none_when_unaffordable() {
        let mut reg = CardRegistry::new();
        let (ogre_def, ogre_chars) = red_creature_def(&mut reg);
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let ogre = state_put_with_card(&mut s, 0, Zone::Hand(0), ogre_chars, ogre_def);
        assert!(auto_tap_sequence(&s, &reg, 0, ogre).is_none(), "no mana sources -> no plan");
    }

    #[test]
    fn auto_tap_sequence_for_land_is_just_play() {
        let mut reg = CardRegistry::new();
        let forest = register_mana_source(
            &mut reg, "Test Forest", land_chars(), ActivationCost::tap_only(), add_one_green);
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let land = state_put_with_card(&mut s, 0, Zone::Hand(0), land_chars(), forest);
        let seq = auto_tap_sequence(&s, &reg, 0, land).expect("a land is playable in main");
        assert!(matches!(seq.as_slice(),
            [Action::PlayLand { object_id, .. }] if *object_id == land),
            "a land's plan is a single PlayLand, no taps");
    }

    /// A non-mana activated ability with a mana cost ({4},{T} — Codie's shape) is
    /// not legal until mana is floated; auto_tap_activate_sequence taps lands for
    /// the cost and appends the activation.
    #[test]
    fn auto_tap_activate_sequence_taps_then_activates() {
        use crate::registry::{ActivatedAbilityDef, ActivationZone, CardDefinition};
        let mut reg = CardRegistry::new();
        let forest = register_mana_source(
            &mut reg, "Test Forest", land_chars(), ActivationCost::tap_only(), add_one_green);
        let dev_chars = Characteristics { types: TypeLine::ARTIFACT.into(), ..Default::default() };
        let dnm = reg.interner_mut().intern("Test Device");
        let dev = reg.register(CardDefinition::new(dnm, dev_chars.clone()).with_activated_ability(
            ActivatedAbilityDef {
                text: "{4}, {T}: do a thing".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").unwrap(), tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_one_green,
            }));

        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let device = state_put_with_card(&mut s, 0, Zone::Battlefield, dev_chars, dev);
        for _ in 0..4 { state_put_with_card(&mut s, 0, Zone::Battlefield, land_chars(), forest); }

        // Not legal with an empty pool.
        assert!(!legal_actions(&s, &reg).iter().any(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == device)));

        let seq = auto_tap_activate_sequence(&s, &reg, 0, device)
            .expect("activatable after tapping out");
        let taps = seq.iter().filter(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source != device)).count();
        assert_eq!(taps, 4, "4 lands tapped for {{4}}");
        assert!(seq.iter().any(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == device)),
            "the device activation is appended (unambiguous)");
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

    #[test]
    fn exile_from_graveyard_activation_cost() {
        use crate::registry::{ActivatedAbilityDef, ActivationCost, CardDefinition};
        let mut reg = CardRegistry::new();
        let name = reg.interner_mut().intern("GY Exiler");
        let chars = creature_chars(1, 1);
        let cid = reg.register(
            CardDefinition::new(name, chars)
                .with_activated_ability(ActivatedAbilityDef {
                    text: "Exile a creature card from your graveyard: draw.".into(),
                    cost: ActivationCost {
                        exile_graveyard_other: Some(crate::targets::ObjectFilter::creature()),
                        ..ActivationCost::default()
                    },
                    target_requirements: vec![],
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: crate::registry::ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: |_, _, _| Vec::new(),
                }));
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let obj = state_put_with_card(&mut s, 0, Zone::Battlefield,
            creature_chars(1, 1), cid);
        s.objects.get_mut(obj).unwrap().status.summoning_sick = false;

        // No creature in graveyard → ability unavailable.
        assert!(!legal_actions(&s, &reg).iter().any(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == obj)));

        // Put a creature card in the controller's graveyard → now payable.
        let gy = state_put_with_card(&mut s, 0, Zone::Graveyard(0),
            creature_chars(2, 2), cid);
        let act = legal_actions(&s, &reg).into_iter().find(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == obj));
        let Some(Action::ActivateAbility { additional_costs, .. }) = act
            else { panic!("activation should be legal with a GY creature") };
        assert!(additional_costs.iter().any(|c| matches!(c,
            crate::actions::AdditionalCostPayment::ExileFromGraveyard(ids)
                if ids == &vec![gy])));
    }

    #[test]
    fn generic_x_activated_ability_fans_out_x_values() {
        use crate::registry::{ActivatedAbilityDef, ActivationCost, CardDefinition};
        use crate::mana::ManaCost;
        let mut reg = CardRegistry::new();
        let name = reg.interner_mut().intern("X Pinger");
        let chars = creature_chars(0, 0);
        let cid = reg.register(
            CardDefinition::new(name, chars)
                .with_activated_ability(ActivatedAbilityDef {
                    text: "{X}: reads X".into(),
                    cost: ActivationCost {
                        mana_cost: ManaCost::parse("{X}").expect("valid"),
                        ..ActivationCost::default()
                    },
                    target_requirements: vec![],
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: crate::registry::ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: |_, _, _| Vec::new(),
                }));
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let chars = creature_chars(0, 0);
        let obj = state_put_with_card(&mut s, 0, Zone::Battlefield, chars, cid);
        s.objects.get_mut(obj).unwrap().status.summoning_sick = false;
        add_mana(&mut s, 0, crate::types::ManaColor::Colorless, 3);

        // One activation per X in 0..=3, each carrying its ActivationX marker.
        let xs: Vec<u32> = legal_actions(&s, &reg).iter().filter_map(|a| match a {
            Action::ActivateAbility { source, additional_costs, .. } if *source == obj =>
                additional_costs.iter().find_map(|c| match c {
                    crate::actions::AdditionalCostPayment::ActivationX(x) => Some(*x),
                    _ => None,
                }),
            _ => None,
        }).collect();
        for expected in 0..=3u32 {
            assert!(xs.contains(&expected),
                "X={expected} activation should be enumerated; got {xs:?}");
        }
        assert!(!xs.contains(&4), "X cannot exceed available mana (3)");
    }

    // --- min_self_counters precondition (Class level-up CR 717.5b) -------

    /// Build a permanent with a single activated ability whose only
    /// gate is a `min_self_counters: Some((Level, 1))` precondition —
    /// "Level 2" on a Class enchantment. The activation has no cost
    /// and a trivial effect; we only care about the legality filter.
    fn register_class_level_2_stub(reg: &mut CardRegistry) -> CardId {
        use crate::registry::{ActivatedAbilityDef, ActivationCost, CardDefinition};
        let name = reg.interner_mut().intern("Class Stub");
        let chars = creature_chars(0, 0);
        reg.register(
            CardDefinition::new(name, chars)
                .with_activated_ability(ActivatedAbilityDef {
                    text: "Level 2".into(),
                    cost: ActivationCost {
                        min_self_counters: Some((CounterKind::Level, 1)),
                        ..ActivationCost::default()
                    },
                    target_requirements: vec![],
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: crate::registry::ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: |_, _, _| Vec::new(),
                })
        )
    }

    #[test]
    fn once_per_turn_gate_blocks_after_activation_and_resets() {
        use crate::registry::{ActivatedAbilityDef, ActivationCost, CardDefinition};
        let mut reg = CardRegistry::new();
        let name = reg.interner_mut().intern("Boast Stub");
        let cid = reg.register(
            CardDefinition::new(name, creature_chars(1, 1))
                .with_activated_ability(ActivatedAbilityDef {
                    text: "Boast stub".into(),
                    cost: ActivationCost {
                        once_per_turn: true,
                        ..ActivationCost::default()
                    },
                    target_requirements: vec![],
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: crate::registry::ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: |_, _, _| Vec::new(),
                }),
        );
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let obj = state_put_with_card(&mut s, 0, Zone::Battlefield, creature_chars(1, 1), cid);
        s.objects.get_mut(obj).unwrap().status.summoning_sick = false;

        // Fresh turn: offered.
        let actions = legal_actions(&s, &reg);
        assert!(actions.iter().any(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == obj)));

        // Already activated this turn (ledger entry): filtered out.
        s.abilities_activated_this_turn.insert((obj, 0));
        let actions = legal_actions(&s, &reg);
        assert!(!actions.iter().any(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == obj)),
            "once-per-turn ability must be illegal after activation");

        // Turn boundary clears the ledger: offered again.
        s.abilities_activated_this_turn.clear();
        let actions = legal_actions(&s, &reg);
        assert!(actions.iter().any(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == obj)));
    }

    #[test]
    fn min_self_counters_gate_blocks_when_count_too_low() {
        let mut reg = CardRegistry::new();
        let cid = register_class_level_2_stub(&mut reg);
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let chars = creature_chars(0, 0);
        let obj = state_put_with_card(&mut s, 0, Zone::Battlefield, chars, cid);
        s.objects.get_mut(obj).unwrap().status.summoning_sick = false;

        // No Level counters → the Level-2 activation is gated out.
        let actions = legal_actions(&s, &reg);
        assert!(!actions.iter().any(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == obj)),
            "Level-2 activation must be illegal with 0 Level counters");
    }

    #[test]
    fn min_self_counters_gate_passes_at_or_above_threshold() {
        let mut reg = CardRegistry::new();
        let cid = register_class_level_2_stub(&mut reg);
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let chars = creature_chars(0, 0);
        let obj = state_put_with_card(&mut s, 0, Zone::Battlefield, chars, cid);
        s.objects.get_mut(obj).unwrap().status.summoning_sick = false;
        s.objects.get_mut(obj).unwrap()
            .add_counters(CounterKind::Level, 1);

        let actions = legal_actions(&s, &reg);
        let activation = actions.iter().find(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == obj));
        let action = activation.expect("Level-2 activation should be legal at level 1");
        // Crucially: the precondition does NOT translate into a
        // RemoveCounters / AddCounters additional-cost — the counter
        // stays put, only the legality gate consults it.
        let Action::ActivateAbility { additional_costs, .. } = action else {
            unreachable!()
        };
        assert!(!additional_costs.iter().any(|c| matches!(c,
            crate::actions::AdditionalCostPayment::RemoveCounters { .. }
                | crate::actions::AdditionalCostPayment::AddCounters { .. })),
            "min_self_counters is a precondition, not a cost — \
             no counter payment should be emitted");
    }

    #[test]
    fn dynamic_x_loyalty_fans_out_one_activation_per_x() {
        use crate::registry::{ActivatedAbilityDef, ActivationCost, CardDefinition};
        let mut reg = CardRegistry::new();
        let name = reg.interner_mut().intern("X PW Stub");
        let cid = {
            let chars = creature_chars(0, 0);
            reg.register(CardDefinition::new(name, chars).with_activated_ability(
                ActivatedAbilityDef {
                    text: "−X: do X".into(),
                    cost: ActivationCost { remove_loyalty_x: true, ..ActivationCost::default() },
                    target_requirements: vec![],
                    is_mana_ability: false,
                    is_loyalty_ability: true,
                    activation_zone: crate::registry::ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: |_, _, _| Vec::new(),
                }))
        };
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let pw = state_put_with_card(&mut s, 0, Zone::Battlefield, creature_chars(0, 0), cid);
        s.objects.get_mut(pw).unwrap().add_counters(CounterKind::Loyalty, 3);

        let acts: Vec<_> = legal_actions(&s, &reg).into_iter().filter(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == pw)).collect();
        // One activation per X in 1..=3, each removing X loyalty.
        assert_eq!(acts.len(), 3, "fans out X = 1,2,3");
        let mut xs: Vec<u32> = acts.iter().filter_map(|a| {
            let Action::ActivateAbility { additional_costs, .. } = a else { return None; };
            additional_costs.iter().find_map(|c| match c {
                crate::actions::AdditionalCostPayment::RemoveCounters {
                    kind: CounterKind::Loyalty, count, .. } => Some(*count),
                _ => None,
            })
        }).collect();
        xs.sort();
        assert_eq!(xs, vec![1, 2, 3]);
    }

    // --- activation_condition precondition (CR 602.5b) ------------------

    /// A permanent with a costless activated ability gated only by an
    /// `activation_condition`: "you may activate this only if an opponent
    /// has a card in their graveyard". Exercises the predicate end-to-end
    /// through the legal-action enumerator.
    fn register_opponent_gy_gated_stub(reg: &mut CardRegistry) -> CardId {
        use crate::registry::{ActivatedAbilityDef, ActivationCost, CardDefinition};
        let name = reg.interner_mut().intern("Opp-GY Gated Stub");
        reg.register(
            CardDefinition::new(name, creature_chars(0, 0))
                .with_activated_ability(ActivatedAbilityDef {
                    text: "Only if an opponent has a card in their graveyard: …".into(),
                    cost: ActivationCost {
                        activation_condition: Some(|s, _src, you, _reg|
                            crate::conditions::an_opponent_graveyard_at_least(s, you, 1)),
                        ..ActivationCost::default()
                    },
                    target_requirements: vec![],
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: crate::registry::ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: |_, _, _| Vec::new(),
                })
        )
    }

    #[test]
    fn activation_condition_gates_legality() {
        let mut reg = CardRegistry::new();
        let cid = register_opponent_gy_gated_stub(&mut reg);
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let obj = state_put_with_card(&mut s, 0, Zone::Battlefield, creature_chars(0, 0), cid);
        s.objects.get_mut(obj).unwrap().status.summoning_sick = false;

        // Opponent's graveyard empty → the activation is gated out.
        assert!(!legal_actions(&s, &reg).iter().any(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == obj)),
            "activation must be illegal while the precondition is false");

        // Put a card in the opponent's graveyard → now legal.
        let _ = state_put_with_card(&mut s, 1, Zone::Graveyard(1), creature_chars(0, 0), cid);
        assert!(legal_actions(&s, &reg).iter().any(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == obj)),
            "activation must become legal once the precondition holds");
    }

    // --- choice-bearing additional costs (sacrifice-other / discard) ----

    /// A creature with `{T}, Sacrifice another creature: …` — the
    /// sacrifice cost is a chosen OTHER creature, not the source.
    fn register_sac_another_creature_stub(reg: &mut CardRegistry) -> CardId {
        use crate::registry::{ActivatedAbilityDef, ActivationCost, CardDefinition};
        use crate::targets::ObjectFilter;
        let name = reg.interner_mut().intern("Carrion Feeder Stub");
        reg.register(
            CardDefinition::new(name, creature_chars(1, 1))
                .with_activated_ability(ActivatedAbilityDef {
                    text: "Sacrifice another creature: +1/+1.".into(),
                    cost: ActivationCost {
                        sacrifice_other: Some(ObjectFilter {
                            types: Some(TypeLine::CREATURE.into()),
                            ..ObjectFilter::default()
                        }),
                        ..ActivationCost::default()
                    },
                    target_requirements: vec![],
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: crate::registry::ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: |_, _, _| Vec::new(),
                })
        )
    }

    /// `{T}, Tap an untapped creature you control: ...` (Springleaf
    /// Drum) — the tap cost is a chosen OTHER untapped creature.
    fn register_tap_another_creature_stub(reg: &mut CardRegistry) -> CardId {
        use crate::registry::{ActivatedAbilityDef, ActivationCost, CardDefinition};
        use crate::targets::ObjectFilter;
        let name = reg.interner_mut().intern("Springleaf Stub");
        reg.register(
            CardDefinition::new(name, creature_chars(0, 1))
                .with_activated_ability(ActivatedAbilityDef {
                    text: "{T}, Tap an untapped creature you control: ...".into(),
                    cost: ActivationCost {
                        tap: true,
                        tap_other: Some(ObjectFilter {
                            types: Some(TypeLine::CREATURE.into()),
                            ..ObjectFilter::default()
                        }),
                        ..ActivationCost::default()
                    },
                    target_requirements: vec![],
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: crate::registry::ActivationZone::Battlefield,
                    is_instant_speed: true,
                    face_gate: None,
                    effect: |_, _, _| Vec::new(),
                })
        )
    }

    #[test]
    fn tap_other_enumerates_untapped_candidates_and_gates_on_none() {
        use crate::actions::AdditionalCostPayment;
        let mut reg = CardRegistry::new();
        let cid = register_tap_another_creature_stub(&mut reg);
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let src = state_put_with_card(&mut s, 0, Zone::Battlefield, creature_chars(0,1), cid);
        s.objects.get_mut(src).unwrap().status.summoning_sick = false;

        // No other creature -> cost unpayable -> not offered.
        assert!(!legal_actions(&s, &reg).iter().any(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == src)));

        // One untapped + one tapped creature -> exactly the untapped
        // one is enumerated as the payment.
        let untapped = state_put_with_card(&mut s, 0, Zone::Battlefield, creature_chars(2, 2), cid);
        let tapped = state_put_with_card(&mut s, 0, Zone::Battlefield, creature_chars(2, 2), cid);
        s.objects.get_mut(tapped).unwrap().tap();
        let actions: Vec<_> = legal_actions(&s, &reg).into_iter()
            .filter(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == src))
            .collect();
        // The two non-source creatures are themselves copies of the stub
        // card; only the SOURCE's activation is filtered here, and only
        // the untapped candidate can pay (the others' own activations
        // need their own other-untapped candidates).
        assert_eq!(actions.len(), 1, "one payment per untapped candidate");
        let Action::ActivateAbility { additional_costs, .. } = &actions[0] else {
            unreachable!()
        };
        assert!(additional_costs.iter().any(|c| matches!(c,
            AdditionalCostPayment::TapCreatures(ids) if ids == &vec![untapped])));
    }

    #[test]
    fn sacrifice_other_enumerates_one_action_per_candidate_excluding_source() {
        let mut reg = CardRegistry::new();
        let cid = register_sac_another_creature_stub(&mut reg);
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let src = state_put_with_card(&mut s, 0, Zone::Battlefield, creature_chars(1,1), cid);
        s.objects.get_mut(src).unwrap().status.summoning_sick = false;
        // Two other creatures controlled by the activator → two sac choices.
        let other_a = put(&mut s, 0, Zone::Battlefield, creature_chars(2,2));
        let other_b = put(&mut s, 0, Zone::Battlefield, creature_chars(3,3));
        // An opponent creature must NOT be a sacrifice candidate.
        let _opp = put(&mut s, 1, Zone::Battlefield, creature_chars(4,4));

        let acts: Vec<_> = legal_actions(&s, &reg).into_iter()
            .filter(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == src))
            .collect();
        assert_eq!(acts.len(), 2, "one activation per other creature controlled");
        let mut sacrificed: Vec<ObjectId> = acts.iter().filter_map(|a| {
            let Action::ActivateAbility { additional_costs, .. } = a else { return None; };
            additional_costs.iter().find_map(|c| match c {
                crate::actions::AdditionalCostPayment::Sacrifice(s) => Some(*s),
                _ => None,
            })
        }).collect();
        sacrificed.sort();
        let mut want = vec![other_a, other_b]; want.sort();
        assert_eq!(sacrificed, want, "the source is never a sac candidate; opp creatures excluded");
    }

    #[test]
    fn sacrifice_other_is_illegal_with_no_other_candidates() {
        let mut reg = CardRegistry::new();
        let cid = register_sac_another_creature_stub(&mut reg);
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let src = state_put_with_card(&mut s, 0, Zone::Battlefield, creature_chars(1,1), cid);
        s.objects.get_mut(src).unwrap().status.summoning_sick = false;
        // Only the source is on the battlefield → nothing to sacrifice.
        let acts: Vec<_> = legal_actions(&s, &reg).into_iter()
            .filter(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == src))
            .collect();
        assert!(acts.is_empty(),
            "sacrifice-another with no other creature is not activatable");
    }

    #[test]
    fn discard_other_enumerates_one_action_per_hand_card() {
        use crate::registry::{ActivatedAbilityDef, ActivationCost, CardDefinition};
        use crate::targets::ObjectFilter;
        let mut reg = CardRegistry::new();
        let name = reg.interner_mut().intern("Discard Engine Stub");
        let cid = reg.register(
            CardDefinition::new(name, creature_chars(1, 1))
                .with_activated_ability(ActivatedAbilityDef {
                    text: "Discard a card: +1/+1.".into(),
                    cost: ActivationCost {
                        discard_other: Some(ObjectFilter::default()),
                        ..ActivationCost::default()
                    },
                    target_requirements: vec![],
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: crate::registry::ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: |_, _, _| Vec::new(),
                })
        );
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let src = state_put_with_card(&mut s, 0, Zone::Battlefield, creature_chars(1,1), cid);
        s.objects.get_mut(src).unwrap().status.summoning_sick = false;
        // Two cards in hand → two discard choices.
        let h1 = put(&mut s, 0, Zone::Hand(0), creature_chars(2,2));
        let h2 = put(&mut s, 0, Zone::Hand(0), creature_chars(3,3));

        let acts: Vec<_> = legal_actions(&s, &reg).into_iter()
            .filter(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == src))
            .collect();
        assert_eq!(acts.len(), 2, "one activation per discardable hand card");
        let mut discarded: Vec<ObjectId> = acts.iter().filter_map(|a| {
            let Action::ActivateAbility { additional_costs, .. } = a else { return None; };
            additional_costs.iter().find_map(|c| match c {
                crate::actions::AdditionalCostPayment::Discard(d) => Some(*d),
                _ => None,
            })
        }).collect();
        discarded.sort();
        let mut want = vec![h1, h2]; want.sort();
        assert_eq!(discarded, want);
    }

    #[test]
    fn discard_two_enumerates_one_action_per_pair() {
        use crate::registry::{ActivatedAbilityDef, ActivationCost, CardDefinition};
        use crate::targets::ObjectFilter;
        let mut reg = CardRegistry::new();
        let name = reg.interner_mut().intern("Discard Two Stub");
        let cid = reg.register(
            CardDefinition::new(name, creature_chars(1, 1))
                .with_activated_ability(ActivatedAbilityDef {
                    text: "Discard two cards: draw a card.".into(),
                    cost: ActivationCost {
                        discard_other: Some(ObjectFilter::default()),
                        discard_other_count: 2,
                        ..ActivationCost::default()
                    },
                    target_requirements: vec![],
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: crate::registry::ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: |_, _, _| Vec::new(),
                })
        );
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let src = state_put_with_card(&mut s, 0, Zone::Battlefield, creature_chars(1,1), cid);
        s.objects.get_mut(src).unwrap().status.summoning_sick = false;
        // Three cards in hand → C(3,2) = 3 two-card combinations.
        put(&mut s, 0, Zone::Hand(0), creature_chars(2,2));
        put(&mut s, 0, Zone::Hand(0), creature_chars(3,3));
        put(&mut s, 0, Zone::Hand(0), creature_chars(4,4));

        let acts: Vec<_> = legal_actions(&s, &reg).into_iter()
            .filter(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == src))
            .collect();
        assert_eq!(acts.len(), 3, "one activation per 2-card combination of 3 hand cards");
        for a in &acts {
            let Action::ActivateAbility { additional_costs, .. } = a else { unreachable!() };
            let n_discard = additional_costs.iter().filter(|c|
                matches!(c, crate::actions::AdditionalCostPayment::Discard(_))).count();
            assert_eq!(n_discard, 2, "each activation discards exactly two cards");
        }
    }

    #[test]
    fn sacrifice_two_enumerates_one_action_per_pair() {
        use crate::registry::{ActivatedAbilityDef, ActivationCost, CardDefinition};
        use crate::targets::ObjectFilter;
        let mut reg = CardRegistry::new();
        let name = reg.interner_mut().intern("Sacrifice Two Stub");
        let cid = reg.register(
            CardDefinition::new(name, creature_chars(1, 1))
                .with_activated_ability(ActivatedAbilityDef {
                    text: "Sacrifice two creatures: ...".into(),
                    cost: ActivationCost {
                        sacrifice_other: Some(ObjectFilter {
                            types: Some(TypeLine::CREATURE.into()),
                            ..ObjectFilter::default()
                        }),
                        sacrifice_other_count: 2,
                        ..ActivationCost::default()
                    },
                    target_requirements: vec![],
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: crate::registry::ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: |_, _, _| Vec::new(),
                })
        );
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let src = state_put_with_card(&mut s, 0, Zone::Battlefield, creature_chars(1,1), cid);
        s.objects.get_mut(src).unwrap().status.summoning_sick = false;
        // Three OTHER creatures → C(3,2) = 3 two-permanent sacrifices.
        put(&mut s, 0, Zone::Battlefield, creature_chars(2,2));
        put(&mut s, 0, Zone::Battlefield, creature_chars(3,3));
        put(&mut s, 0, Zone::Battlefield, creature_chars(4,4));

        let acts: Vec<_> = legal_actions(&s, &reg).into_iter()
            .filter(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == src))
            .collect();
        assert_eq!(acts.len(), 3, "one activation per 2-creature combination (source excluded)");
        for a in &acts {
            let Action::ActivateAbility { additional_costs, .. } = a else { unreachable!() };
            let n = additional_costs.iter().filter(|c|
                matches!(c, crate::actions::AdditionalCostPayment::Sacrifice(_))).count();
            assert_eq!(n, 2, "each activation sacrifices exactly two permanents");
        }
    }

    #[test]
    fn discard_random_offers_single_activation_gated_on_hand() {
        use crate::registry::{ActivatedAbilityDef, ActivationCost, CardDefinition};
        let mut reg = CardRegistry::new();
        let name = reg.interner_mut().intern("Discard Random Stub");
        let cid = reg.register(
            CardDefinition::new(name, creature_chars(1, 1))
                .with_activated_ability(ActivatedAbilityDef {
                    text: "Discard a card at random: ...".into(),
                    cost: ActivationCost { discard_random: 1, ..ActivationCost::default() },
                    target_requirements: vec![],
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: crate::registry::ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: |_, _, _| Vec::new(),
                })
        );
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let src = state_put_with_card(&mut s, 0, Zone::Battlefield, creature_chars(1,1), cid);
        s.objects.get_mut(src).unwrap().status.summoning_sick = false;
        let n_acts = |s: &GameState| legal_actions(s, &reg).into_iter()
            .filter(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == src))
            .count();
        // Empty hand → not activatable.
        assert_eq!(n_acts(&s), 0, "no random-discard activation with an empty hand");
        // Cards in hand → exactly ONE activation, NO baked Discard payment
        // (the card is chosen by the engine RNG in apply, not enumerated).
        put(&mut s, 0, Zone::Hand(0), creature_chars(2,2));
        put(&mut s, 0, Zone::Hand(0), creature_chars(3,3));
        let acts: Vec<_> = legal_actions(&s, &reg).into_iter()
            .filter(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == src))
            .collect();
        assert_eq!(acts.len(), 1, "random discard is one activation, not one-per-card");
        let Action::ActivateAbility { additional_costs, .. } = &acts[0] else { unreachable!() };
        assert!(additional_costs.iter().all(|c|
            !matches!(c, crate::actions::AdditionalCostPayment::Discard(_))),
            "no specific card baked into the action — chosen at random in apply");
    }

    #[test]
    fn discard_two_not_activatable_with_one_card() {
        use crate::registry::{ActivatedAbilityDef, ActivationCost, CardDefinition};
        use crate::targets::ObjectFilter;
        let mut reg = CardRegistry::new();
        let name = reg.interner_mut().intern("Discard Two Stub2");
        let cid = reg.register(
            CardDefinition::new(name, creature_chars(1, 1))
                .with_activated_ability(ActivatedAbilityDef {
                    text: "Discard two cards: draw a card.".into(),
                    cost: ActivationCost {
                        discard_other: Some(ObjectFilter::default()),
                        discard_other_count: 2,
                        ..ActivationCost::default()
                    },
                    target_requirements: vec![],
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: crate::registry::ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: |_, _, _| Vec::new(),
                })
        );
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let src = state_put_with_card(&mut s, 0, Zone::Battlefield, creature_chars(1,1), cid);
        s.objects.get_mut(src).unwrap().status.summoning_sick = false;
        put(&mut s, 0, Zone::Hand(0), creature_chars(2,2)); // only one card

        let acts: Vec<_> = legal_actions(&s, &reg).into_iter()
            .filter(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == src))
            .collect();
        assert!(acts.is_empty(), "discard-two with one card in hand is not activatable");
    }

    #[test]
    fn discard_hand_discards_every_card() {
        use crate::registry::{ActivatedAbilityDef, ActivationCost, CardDefinition};
        let mut reg = CardRegistry::new();
        let name = reg.interner_mut().intern("Discard Hand Stub");
        let cid = reg.register(
            CardDefinition::new(name, creature_chars(1, 1))
                .with_activated_ability(ActivatedAbilityDef {
                    text: "Discard your hand: draw three cards.".into(),
                    cost: ActivationCost {
                        discard_hand: true,
                        ..ActivationCost::default()
                    },
                    target_requirements: vec![],
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: crate::registry::ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: |_, _, _| Vec::new(),
                })
        );
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        let src = state_put_with_card(&mut s, 0, Zone::Battlefield, creature_chars(1,1), cid);
        s.objects.get_mut(src).unwrap().status.summoning_sick = false;
        let h1 = put(&mut s, 0, Zone::Hand(0), creature_chars(2,2));
        let h2 = put(&mut s, 0, Zone::Hand(0), creature_chars(3,3));

        let acts: Vec<_> = legal_actions(&s, &reg).into_iter()
            .filter(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == src))
            .collect();
        assert_eq!(acts.len(), 1, "discard-your-hand is a single deterministic payment");
        let Action::ActivateAbility { additional_costs, .. } = &acts[0] else { unreachable!() };
        let mut discarded: Vec<ObjectId> = additional_costs.iter().filter_map(|c| match c {
            crate::actions::AdditionalCostPayment::Discard(d) => Some(*d),
            _ => None,
        }).collect();
        discarded.sort();
        let mut want = vec![h1, h2]; want.sort();
        assert_eq!(discarded, want, "the whole hand is discarded");
    }

    // --- intrinsic activations on tokens (commodity-token plumbing) -----

    #[test]
    fn intrinsic_activated_abilities_show_up_in_legal_actions() {
        use crate::effects::{CommodityToken, Effect};
        let reg = CardRegistry::new();
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        // Mint a Clue (an intrinsic-ability token: card_id=0).
        Effect::CreateCommodityToken {
            controller: 0, kind: CommodityToken::Clue, count: 1,
        }.execute(&mut s);
        // Give player 0 the {2} needed and clear summoning sickness.
        let token_id = s.objects.iter()
            .find(|o| o.zone.is_battlefield())
            .map(|o| o.id).expect("clue minted");
        s.objects.get_mut(token_id).unwrap().status.summoning_sick = false;
        let pool = &mut s.player_mut(0).mana_pool;
        // Two colorless mana, enough for {2}.
        pool.add(crate::mana::ManaUnit::plain(ManaColor::Colorless, 0));
        pool.add(crate::mana::ManaUnit::plain(ManaColor::Colorless, 0));

        let actions = legal_actions(&s, &reg);
        assert!(actions.iter().any(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == token_id)),
            "Clue's intrinsic {{2}}, Sac: Draw activation should be enumerated");
    }

    #[test]
    fn granted_activated_ability_from_attachment_shows_up_on_host() {
        use crate::layers::{ContinuousEffect, Duration};
        use crate::registry::{
            ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
        };
        use crate::effects::Effect;
        let reg = CardRegistry::new();
        let mut s = GameState::new(2, 0);
        set_main_phase(&mut s);
        // Host creature (player 0), able to tap.
        let host = {
            let id = s.allocate_object_id();
            let mut o = GameObject::new(id, 0, Zone::Battlefield, 0, creature_chars(2, 2));
            o.controller = 0;
            o.status.summoning_sick = false;
            s.objects.insert(o);
            id
        };
        // Aura attached to the host.
        let aura = {
            let id = s.allocate_object_id();
            let mut o = GameObject::new(id, 0, Zone::Battlefield, 0,
                Characteristics { types: TypeLine::ENCHANTMENT.into(), ..Default::default() });
            o.controller = 0;
            o.attached_to = Some(host);
            s.objects.insert(o);
            id
        };
        s.objects.get_mut(host).unwrap().attachments.push(aura);

        fn granted_effect(_s: &GameState, _c: &ActivationContext, _r: &CardRegistry)
            -> Vec<Effect> { Vec::new() }
        let ability = ActivatedAbilityDef {
            text: "{T}: granted".into(),
            cost: ActivationCost { tap: true, ..ActivationCost::default() },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: granted_effect,
        };
        s.add_continuous_effect(ContinuousEffect::attached_activated(
            aura, ability, Duration::WhileSourceOnBattlefield));

        // The HOST can activate the Aura-granted "{T}: …" ability.
        let actions = legal_actions(&s, &reg);
        assert!(actions.iter().any(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == host)),
            "Aura-granted activated ability should be enumerated on the host");

        // Detach the Aura → the grant vanishes (auto-expiry).
        s.objects.get_mut(aura).unwrap().attached_to = None;
        let actions2 = legal_actions(&s, &reg);
        assert!(!actions2.iter().any(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == host)),
            "an unattached Aura grants nothing");
    }

    #[test]
    fn enumerate_blocker_orderings_emits_all_permutations() {
        use crate::combat::{
            AttackerDeclaration, BlockerDeclaration, DefendingEntity,
        };
        let reg = CardRegistry::new();
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = {
            let id = s.allocate_object_id();
            let chars = creature_chars(5, 5);
            let mut obj = GameObject::new(id, 0, Zone::Battlefield, 0, chars);
            obj.controller = 0;
            obj.status.summoning_sick = false;
            s.objects.insert(obj);
            id
        };
        let b1 = {
            let id = s.allocate_object_id();
            let chars = creature_chars(2, 2);
            let mut obj = GameObject::new(id, 1, Zone::Battlefield, 0, chars);
            obj.controller = 1;
            obj.status.summoning_sick = false;
            s.objects.insert(obj);
            id
        };
        let b2 = {
            let id = s.allocate_object_id();
            let chars = creature_chars(1, 4);
            let mut obj = GameObject::new(id, 1, Zone::Battlefield, 0, chars);
            obj.controller = 1;
            obj.status.summoning_sick = false;
            s.objects.insert(obj);
            id
        };
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        s.apply_declared_blockers(vec![
            BlockerDeclaration { blocker: b1, blocking: atk },
            BlockerDeclaration { blocker: b2, blocking: atk },
        ]);
        // Active player (0) gets the OrderBlockers decision.
        let actions = legal_actions(&s, &reg);
        let orderings: Vec<_> = actions.iter().filter_map(|a| match a {
            Action::OrderBlockers { orderings } => Some(orderings.clone()),
            _ => None,
        }).collect();
        assert_eq!(orderings.len(), 2, "both permutations emitted");
        assert!(orderings.iter().any(|o|
            o == &vec![(atk, vec![b1, b2])]));
        assert!(orderings.iter().any(|o|
            o == &vec![(atk, vec![b2, b1])]));
    }

    #[test]
    fn enumerate_combat_damage_assignments_covers_legal_distributions() {
        use crate::combat::{
            AttackerDeclaration, BlockerDeclaration, DamageAssignment,
            DefendingEntity, PendingDamagePass,
        };
        let reg = CardRegistry::new();
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = {
            let id = s.allocate_object_id();
            let chars = creature_chars(5, 5);
            let mut obj = GameObject::new(id, 0, Zone::Battlefield, 0, chars);
            obj.controller = 0;
            obj.status.summoning_sick = false;
            s.objects.insert(obj);
            id
        };
        let b1 = {
            let id = s.allocate_object_id();
            let chars = creature_chars(1, 2);
            let mut obj = GameObject::new(id, 1, Zone::Battlefield, 0, chars);
            obj.controller = 1;
            obj.status.summoning_sick = false;
            s.objects.insert(obj);
            id
        };
        let b2 = {
            let id = s.allocate_object_id();
            let chars = creature_chars(1, 4);
            let mut obj = GameObject::new(id, 1, Zone::Battlefield, 0, chars);
            obj.controller = 1;
            obj.status.summoning_sick = false;
            s.objects.insert(obj);
            id
        };
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        s.apply_declared_blockers(vec![
            BlockerDeclaration { blocker: b1, blocking: atk },
            BlockerDeclaration { blocker: b2, blocking: atk },
        ]);
        s.apply_blocker_ordering(vec![(atk, vec![b1, b2])]);
        s.combat.as_mut().unwrap().pending_damage_assignment
            = Some(PendingDamagePass::Regular);

        let actions = legal_actions(&s, &reg);
        let dists: Vec<Vec<DamageAssignment>> = actions.iter()
            .filter_map(|a| match a {
                Action::AssignCombatDamage { distributions } =>
                    Some(distributions.clone()),
                _ => None,
            })
            .collect();
        // Power=5, b1 lethal=2, b2 lethal=4. Legal distributions:
        // (5, 0), (2, 3), (3, 2), (4, 1). The enumerator generates in
        // terminate-first + lethal-or-more recursion order. Canonical
        // set is the 4 distributions above.
        let flattened: std::collections::HashSet<_> = dists.iter()
            .map(|d| d[0].distribution.clone())
            .collect();
        assert!(flattened.contains(&vec![(b1, 5)]),
            "all damage to first blocker is legal");
        assert!(flattened.contains(&vec![(b1, 2), (b2, 3)]),
            "minimum-lethal-then-rest is legal");
        assert!(flattened.contains(&vec![(b1, 3), (b2, 2)]),
            "overkill-first, rest-to-second is legal");
        assert!(flattened.contains(&vec![(b1, 4), (b2, 1)]));
        // Illegal: sub-lethal to b1 with anything on b2.
        assert!(!flattened.contains(&vec![(b1, 1), (b2, 4)]));
    }

    /// The incremental-combat helpers (ordering_targets / match_ordering /
    /// damage_targets / match_damage) work against the ENGINE's real enumerated
    /// legal lists, not just synthetic ones — the path the human-play frontend
    /// drives. 5/5 attacker double-blocked by a 2/2 and a 1/4.
    #[test]
    fn combat_ui_helpers_match_real_enumeration() {
        use crate::combat::{
            damage_targets, match_damage, match_ordering, ordering_targets,
            AttackerDeclaration, BlockerDeclaration, DamageAssignment, DefendingEntity,
            PendingDamagePass,
        };
        let reg = CardRegistry::new();
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let mk = |s: &mut GameState, owner, p, t| {
            let id = s.allocate_object_id();
            let mut obj = GameObject::new(id, owner, Zone::Battlefield, 0, creature_chars(p, t));
            obj.controller = owner;
            obj.status.summoning_sick = false;
            s.objects.insert(obj);
            id
        };
        let atk = mk(&mut s, 0, 5, 5);
        let b1 = mk(&mut s, 1, 2, 2);
        let b2 = mk(&mut s, 1, 1, 4);
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        s.apply_declared_blockers(vec![
            BlockerDeclaration { blocker: b1, blocking: atk },
            BlockerDeclaration { blocker: b2, blocking: atk },
        ]);

        // --- ordering ---
        let legal = legal_actions(&s, &reg);
        let targets = ordering_targets(&legal);
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].0, atk);
        assert_eq!(targets[0].1.len(), 2, "two blockers to order");
        // Both permutations match a real legal ordering; a bogus one doesn't.
        assert!(match_ordering(&legal, &[(atk, vec![b1, b2])]).is_some());
        assert!(match_ordering(&legal, &[(atk, vec![b2, b1])]).is_some());
        assert!(match_ordering(&legal, &[(atk, vec![b1, 9999])]).is_none());

        // --- damage (after committing an order + opening the damage pass) ---
        s.apply_blocker_ordering(vec![(atk, vec![b1, b2])]);
        s.combat.as_mut().unwrap().pending_damage_assignment = Some(PendingDamagePass::Regular);
        let legal = legal_actions(&s, &reg);
        let dt = damage_targets(&legal);
        assert_eq!(dt.len(), 1);
        assert_eq!(dt[0].0, atk);
        assert_eq!(dt[0].1, vec![b1, b2], "live order from the longest distribution");
        let da = |dist| DamageAssignment { attacker: atk, distribution: dist };
        assert!(match_damage(&legal, &[da(vec![(b1, 2), (b2, 3)])]).is_some());
        assert!(match_damage(&legal, &[da(vec![(b1, 5)])]).is_some());
        assert!(match_damage(&legal, &[da(vec![(b1, 1), (b2, 4)])]).is_none(),
            "sub-lethal to first blocker is illegal");
    }

    #[test]
    fn enumerate_trample_single_blocker_covers_overflow_range() {
        // CR 702.19b — a 5/5 trample attacker vs a 2/2 blocker has four
        // legal distributions: (blk, 2), (blk, 3), (blk, 4), (blk, 5).
        // Each below 5 implicitly sends the remainder to the defender.
        use crate::combat::{
            AttackerDeclaration, BlockerDeclaration, DamageAssignment,
            DefendingEntity, PendingDamagePass,
        };
        use crate::effects::KeywordAbility;
        let reg = CardRegistry::new();
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = {
            let id = s.allocate_object_id();
            let mut chars = creature_chars(5, 5);
            chars.keywords.push(KeywordAbility::Trample);
            let mut obj = GameObject::new(id, 0, Zone::Battlefield, 0, chars);
            obj.controller = 0;
            obj.status.summoning_sick = false;
            s.objects.insert(obj);
            id
        };
        let blk = {
            let id = s.allocate_object_id();
            let chars = creature_chars(1, 2);
            let mut obj = GameObject::new(id, 1, Zone::Battlefield, 0, chars);
            obj.controller = 1;
            obj.status.summoning_sick = false;
            s.objects.insert(obj);
            id
        };
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        s.apply_declared_blockers(vec![
            BlockerDeclaration { blocker: blk, blocking: atk },
        ]);
        s.combat.as_mut().unwrap().pending_damage_assignment
            = Some(PendingDamagePass::Regular);

        let actions = legal_actions(&s, &reg);
        let dists: std::collections::HashSet<Vec<(ObjectId, u32)>> = actions.iter()
            .filter_map(|a| match a {
                Action::AssignCombatDamage { distributions } =>
                    Some(distributions.clone()),
                _ => None,
            })
            .filter(|d| d.len() == 1)
            .map(|d: Vec<DamageAssignment>| d[0].distribution.clone())
            .collect();
        assert!(dists.contains(&vec![(blk, 2)]), "lethal only, max overflow");
        assert!(dists.contains(&vec![(blk, 3)]));
        assert!(dists.contains(&vec![(blk, 4)]));
        assert!(dists.contains(&vec![(blk, 5)]), "all damage to blocker");
        // Sub-lethal isn't legal under trample (overflow requires lethal
        // to every blocker first).
        assert!(!dists.contains(&vec![(blk, 1)]));
    }

    #[test]
    fn enumerate_distributions_skips_blockers_killed_in_first_strike() {
        // CR 510.1c — after the first-strike pass kills earlier
        // blockers, the regular pass must enumerate distributions
        // over only the surviving blocker(s). Dumping the attacker's
        // damage onto a corpse while a live blocker goes untouched
        // would be illegal, and before the dead-blocker filter the
        // enumerator happily produced such distributions.
        //
        // Trample is used here so a single-live-blocker configuration
        // still requires an assignment choice (overflow vs. pile-on);
        // the same filter applies to the multi-blocker case.
        use crate::combat::{
            AttackerDeclaration, BlockerDeclaration, DamageAssignment,
            DefendingEntity, PendingDamagePass,
        };
        use crate::effects::KeywordAbility;
        let reg = CardRegistry::new();
        let mut s = GameState::new(2, 0);
        s.begin_combat();
        let atk = {
            let id = s.allocate_object_id();
            let mut chars = creature_chars(4, 4);
            chars.keywords.push(KeywordAbility::Trample);
            let mut obj = GameObject::new(id, 0, Zone::Battlefield, 0, chars);
            obj.controller = 0;
            obj.status.summoning_sick = false;
            s.objects.insert(obj);
            id
        };
        let b1 = {
            let id = s.allocate_object_id();
            let chars = creature_chars(1, 1);
            let mut obj = GameObject::new(id, 1, Zone::Battlefield, 0, chars);
            obj.controller = 1;
            obj.status.summoning_sick = false;
            s.objects.insert(obj);
            id
        };
        let b2 = {
            let id = s.allocate_object_id();
            let chars = creature_chars(2, 2);
            let mut obj = GameObject::new(id, 1, Zone::Battlefield, 0, chars);
            obj.controller = 1;
            obj.status.summoning_sick = false;
            s.objects.insert(obj);
            id
        };
        let b3 = {
            let id = s.allocate_object_id();
            let chars = creature_chars(1, 3);
            let mut obj = GameObject::new(id, 1, Zone::Battlefield, 0, chars);
            obj.controller = 1;
            obj.status.summoning_sick = false;
            s.objects.insert(obj);
            id
        };
        s.apply_declared_attackers(vec![AttackerDeclaration {
            attacker: atk, defending: DefendingEntity::Player(1),
        }]);
        s.enter_declare_blockers();
        s.apply_declared_blockers(vec![
            BlockerDeclaration { blocker: b1, blocking: atk },
            BlockerDeclaration { blocker: b2, blocking: atk },
            BlockerDeclaration { blocker: b3, blocking: atk },
        ]);
        s.apply_blocker_ordering(vec![(atk, vec![b1, b2, b3])]);
        // Simulate the first-strike pass having killed B1 and B2.
        s.objects.get_mut(b1).unwrap().damage_marked = 1;
        s.objects.get_mut(b2).unwrap().damage_marked = 2;
        s.combat.as_mut().unwrap().pending_damage_assignment
            = Some(PendingDamagePass::Regular);

        let actions = legal_actions(&s, &reg);
        let dists: std::collections::HashSet<Vec<(ObjectId, u32)>> = actions.iter()
            .filter_map(|a| match a {
                Action::AssignCombatDamage { distributions } =>
                    Some(distributions.clone()),
                _ => None,
            })
            .filter(|d| d.len() == 1)
            .map(|d: Vec<DamageAssignment>| d[0].distribution.clone())
            .collect();
        assert!(!dists.is_empty(),
            "trample + single live blocker should produce distributions");
        // No distribution should reference a dead blocker.
        for dist in &dists {
            for (blk, _) in dist {
                assert!(*blk == b3,
                    "enumeration referenced a dead blocker: {:?}", dist);
            }
        }
        // Canonical trample overflow set for power=4 vs B3 (lethal=3):
        // (B3, 3) — overflow 1, and (B3, 4) — all on blocker.
        assert!(dists.contains(&vec![(b3, 3)]),
            "lethal-to-live + overflow-1-to-defender must be legal");
        assert!(dists.contains(&vec![(b3, 4)]),
            "all four on the live blocker must be legal");
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
