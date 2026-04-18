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

        let Some(cost) = obj.characteristics.mana_cost.clone() else { continue; };
        // TODO(x-enumeration): skip X-costed spells until we enumerate
        // plausible X values (Task #14 follow-up).
        if cost.x_count() > 0 { continue; }

        let ctx = SpendContext::for_spell(
            obj.characteristics.types, obj.characteristics.colors);

        // Target requirements (if the registry knows this card).
        let reqs: Vec<TargetRequirement> = registry.get(obj.card_id)
            .and_then(|def| def.spell_ability.as_ref())
            .map(|sa| sa.target_requirements.clone())
            .unwrap_or_default();
        let target_selections = enumerate_target_selections(&reqs, state, player);

        // Delve (CR 702.66). When the card has delve and the cost has
        // a generic component, enumerate exile subsets up to the
        // generic count (with characteristic-equivalence dedup). The
        // empty subset reproduces the normal cast. When the card has
        // no delve, we emit a single "no-delve" path with
        // `delve_exiles: None`.
        let delve_available = crate::engine::has_delve(state, id);
        let gen_cap = generic_total(&cost);
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
                reduce_generic_cost(&cost, subset.len() as u32)
            };
            let plans = enumerate_payment_plans(
                &reduced_cost, &state.player(player).mana_pool, None, &ctx);
            for plan in plans {
                for targets in &target_selections {
                    actions.push(Action::CastSpell {
                        object_id: id,
                        targets: targets.clone(),
                        modes: Vec::new(),
                        mana_payment: plan.clone(),
                        additional_costs: Vec::new(),
                        x_value: None,
                        cast_modifier: crate::actions::CastModifier::None,
                        delve_exiles: if delve_available {
                            Some(subset.clone())
                        } else {
                            None
                        },
                    });
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

        for cost in flashback_costs {
            // TODO(x-enumeration): mirror the hand-path skip for now.
            if cost.x_count() > 0 { continue; }
            // TODO(delve-on-flashback): no card in current Standard has
            // both flashback and delve, but composition is legal per
            // the CR. When a card arrives, enumerate delve subsets
            // here — the subtle part is excluding the cast-source
            // itself from delve candidates (its zone at cost-payment
            // time in our atomic pipeline is still Graveyard, not
            // Stack).
            let ctx = SpendContext::for_spell(
                obj.characteristics.types, obj.characteristics.colors);
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
                        x_value: None,
                        cast_modifier: crate::actions::CastModifier::Flashback,
                        delve_exiles: None,
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
    let mut index: std::collections::HashMap<K, usize> =
        std::collections::HashMap::new();
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

    for id in state.objects.ids_in_zone_sorted(Zone::Battlefield) {
        let obj = state.objects.get(id).unwrap();
        if obj.controller != player { continue; }
        let Some(def) = registry.get(obj.card_id) else { continue; };

        for (i, ability) in def.activated_abilities.iter().enumerate() {
            if !ability_is_activatable(state, obj, ability, sorcery_speed_ok) {
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
    _state: &GameState,
    obj: &crate::objects::GameObject,
    ability: &crate::registry::ActivatedAbilityDef,
    sorcery_speed_ok: bool,
) -> bool {
    if obj.zone != Zone::Battlefield { return false; }
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
    // Mana abilities can be activated at any time a player has
    // priority. Non-mana activated abilities default to sorcery
    // speed for Phase 1 (instants aren't common enough to be worth
    // a per-ability flag until Phase 2).
    if !ability.is_mana_ability && !sorcery_speed_ok {
        return false;
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
    if cost.life > 0 {
        v.push(crate::actions::AdditionalCostPayment::PayLife(cost.life));
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
    fn x_cost_spells_are_skipped_for_now() {
        let mut s = GameState::new(2, 0);
        put(&mut s, 0, Zone::Hand(0), x_instant_chars());
        add_mana(&mut s, 0, ManaColor::Red, 5);
        let actions = legal_actions(&s, &CardRegistry::new());
        assert!(!actions.iter().any(|a| matches!(a, Action::CastSpell { .. })));
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
}
