//! Integration tests that combine multiple seed cards and drive
//! them through the real engine pipeline. These are the first real
//! consumers of arcana-cards and will surface any gaps between the
//! synthetic per-card unit tests and end-to-end gameplay.
//!
//! Strategy: construct `GameState` directly, put cards in hand /
//! battlefield, give mana, and step through the actions. Avoids the
//! ~60-card-deck new_game setup so each test is compact and the
//! state under test is explicit.

use arcana_cards::{register_seed, SeedIds};
use arcana_core::engine::step;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::types::{ManaColor, PlayerId};
use arcana_core::zones::Zone;
use arcana_core::{Action, EngineYield, ObjectId};

// ---------------------------------------------------------------------
// Shared fixtures
// ---------------------------------------------------------------------

fn fresh_game() -> (GameState, CardRegistry, SeedIds) {
    let mut registry = CardRegistry::new();
    let ids = register_seed(&mut registry);
    let state = GameState::new(2, 0);
    (state, registry, ids)
}

fn put_in_hand(
    state: &mut GameState,
    registry: &CardRegistry,
    player: PlayerId,
    card_id: arcana_core::types::CardId,
) -> ObjectId {
    let obj_id = state.allocate_object_id();
    let chars = registry.get(card_id).unwrap().base_characteristics.clone();
    state.objects.insert(GameObject::new(
        obj_id, player, Zone::Hand(player), card_id, chars));
    obj_id
}

fn put_on_battlefield(
    state: &mut GameState,
    registry: &CardRegistry,
    player: PlayerId,
    card_id: arcana_core::types::CardId,
) -> ObjectId {
    let obj_id = state.allocate_object_id();
    let chars = registry.get(card_id).unwrap().base_characteristics.clone();
    state.objects.insert(GameObject::new(
        obj_id, player, Zone::Battlefield, card_id, chars));
    obj_id
}

/// Give `player` `count` mana of `color` in their pool.
fn give_mana(state: &mut GameState, player: PlayerId, color: ManaColor, count: u32) {
    state.player_mut(player).mana_pool.add_mana(color, count, 0);
}

fn priority_to_main(state: &mut GameState, player: PlayerId) {
    state.priority.give_to(player);
    state.turn.phase = arcana_core::turn::Phase::PreCombatMain;
    state.turn.step = arcana_core::turn::Step::Main;
}

/// Repeatedly pass priority until the stack is empty, returning the
/// resolved state. Since `resolve_top_of_stack` is private, tests
/// drive resolution through the public `step()` API by passing
/// priority — the engine auto-resolves the top when both players
/// pass. Capped at 200 steps to catch infinite loops.
fn resolve_stack(state: GameState, registry: &CardRegistry) -> GameState {
    let mut s = state;
    for _ in 0..200 {
        if s.stack_is_empty() { return s; }
        let (ns, yld) = step(s, Action::PassPriority, registry);
        s = ns;
        if matches!(yld, EngineYield::GameOver(_)) { return s; }
    }
    s
}

// ---------------------------------------------------------------------
// Lightning Bolt + Grizzly Bears
// ---------------------------------------------------------------------

#[test]
fn bolt_kills_grizzly_bears() {
    let (mut s, registry, ids) = fresh_game();
    let bolt = put_in_hand(&mut s, &registry, 0, ids.lightning_bolt);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);

    let cast = Action::CastSpell {
        object_id: bolt,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::ObjectOrPlayer(
                arcana_core::targets::ObjectOrPlayer::Object(bears),
            )],
        },
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![arcana_core::actions::ManaAssignment {
                pool_index: 0, cost_index: 0,
            }],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: None,
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);
    // Bears moved to graveyard (zone change re-ids, so the count is
    // the load-bearing assertion).
    assert_eq!(s.zone_count(Zone::Graveyard(1)), 1,
        "Bears should be dead in p1's graveyard");
    assert_eq!(s.objects.objects_in_zone(Zone::Battlefield)
        .filter(|o| o.characteristics.is_creature()).count(), 0,
        "no creatures remain on the battlefield");
}

#[test]
fn bolt_deals_3_to_player() {
    let (mut s, registry, ids) = fresh_game();
    let bolt = put_in_hand(&mut s, &registry, 0, ids.lightning_bolt);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);
    let p1_start = s.player(1).life;

    let cast = Action::CastSpell {
        object_id: bolt,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::ObjectOrPlayer(
                arcana_core::targets::ObjectOrPlayer::Player(1),
            )],
        },
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![arcana_core::actions::ManaAssignment {
                pool_index: 0, cost_index: 0,
            }],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: None,
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);
    assert_eq!(p1_start - s.player(1).life, 3, "Bolt deals exactly 3");
}

// ---------------------------------------------------------------------
// Murder
// ---------------------------------------------------------------------

#[test]
fn murder_destroys_target_creature() {
    let (mut s, registry, ids) = fresh_game();
    let murder = put_in_hand(&mut s, &registry, 0, ids.murder);
    let victim = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    // {1}{B}{B}: need 1 generic + 2 black
    give_mana(&mut s, 0, ManaColor::Black, 3);
    priority_to_main(&mut s, 0);

    let cast = Action::CastSpell {
        object_id: murder,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::Object(victim)],
        },
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 1, cost_index: 1 },
                arcana_core::actions::ManaAssignment { pool_index: 2, cost_index: 2 },
            ],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: None,
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);
    assert_eq!(s.zone_count(Zone::Graveyard(1)), 1,
        "victim should be in p1's graveyard");
    assert_eq!(s.objects.objects_in_zone(Zone::Battlefield).count(), 0,
        "battlefield empty");
}

// ---------------------------------------------------------------------
// Elvish Visionary — ETB trigger draws a card
// ---------------------------------------------------------------------

#[test]
fn elvish_visionary_draws_a_card_on_etb() {
    let (mut s, registry, ids) = fresh_game();
    let vis = put_in_hand(&mut s, &registry, 0, ids.elvish_visionary);
    // Stock P0's library with something to draw.
    let top = {
        let obj_id = s.allocate_object_id();
        let chars = registry.get(ids.mountain).unwrap()
            .base_characteristics.clone();
        s.objects.insert(GameObject::new(
            obj_id, 0, Zone::Library(0), ids.mountain, chars));
        obj_id
    };
    give_mana(&mut s, 0, ManaColor::Green, 2);
    priority_to_main(&mut s, 0);
    let hand_before = s.objects.count_in_zone(Zone::Hand(0));

    let cast = Action::CastSpell {
        object_id: vis,
        targets: arcana_core::targets::TargetSelection { targets: vec![] },
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 1, cost_index: 1 },
            ],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: None,
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);
    // Visionary is on the battlefield.
    let creatures = s.objects.objects_in_zone(Zone::Battlefield)
        .filter(|o| o.characteristics.is_creature())
        .count();
    assert_eq!(creatures, 1, "Visionary entered battlefield");
    // Hand should have grown net-1 (lost Visionary to cast, gained
    // Mountain from trigger = same count as before, after the draw
    // resolves). Pre-cast: 1 (Visionary). Post-cast: 0. Post-ETB
    // trigger resolution: 1 (Mountain).
    assert_eq!(s.objects.count_in_zone(Zone::Hand(0)), hand_before,
        "net-0 hand size: cast Visionary, drew a card from ETB");
    let _ = top;
}

// ---------------------------------------------------------------------
// Glorious Anthem — layer 7c anthem
// ---------------------------------------------------------------------

#[test]
fn anthem_buffs_controllers_creatures() {
    let (mut s, registry, ids) = fresh_game();
    // Anthem already on battlefield (bypass casting for this test).
    let anthem = put_on_battlefield(&mut s, &registry, 0, ids.glorious_anthem);
    // Manually install the anthem's effect (what the ETB trigger
    // would do — the test isolates the layer-7c computation, not
    // the ETB wiring).
    s.add_continuous_effect(arcana_core::layers::ContinuousEffect::anthem(
        anthem, 0, 1, 1,
        arcana_core::layers::Duration::WhileSourceOnBattlefield,
    ));
    let bears = put_on_battlefield(&mut s, &registry, 0, ids.grizzly_bears);
    let opp_bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);

    let bears_chars = s.compute_characteristics(bears).unwrap();
    let opp_chars = s.compute_characteristics(opp_bears).unwrap();
    assert_eq!(bears_chars.power,
        Some(arcana_core::types::PtValue::Fixed(3)),
        "p0's Bears buffed to 3 power");
    assert_eq!(opp_chars.power,
        Some(arcana_core::types::PtValue::Fixed(2)),
        "p1's Bears unaffected");
}

#[test]
fn anthem_effect_expires_when_anthem_leaves() {
    let (mut s, registry, ids) = fresh_game();
    let anthem = put_on_battlefield(&mut s, &registry, 0, ids.glorious_anthem);
    s.add_continuous_effect(arcana_core::layers::ContinuousEffect::anthem(
        anthem, 0, 1, 1,
        arcana_core::layers::Duration::WhileSourceOnBattlefield,
    ));
    let bears = put_on_battlefield(&mut s, &registry, 0, ids.grizzly_bears);

    // Pre-destroy: Bears are 3/3.
    assert_eq!(s.compute_characteristics(bears).unwrap().power,
        Some(arcana_core::types::PtValue::Fixed(3)));

    // Move anthem off the battlefield.
    s.move_object_to_zone(
        anthem, Zone::Graveyard(0),
        arcana_core::events::MoveCause::StateBasedAction);
    // Post-destroy: the continuous-effect cleanup fires and Bears
    // revert to 2/2. Bears was not itself moved, so `bears` id is
    // still valid — anthem moved and re-idded.
    assert_eq!(s.compute_characteristics(bears).unwrap().power,
        Some(arcana_core::types::PtValue::Fixed(2)),
        "anthem expiration reverts Bears");
}

// ---------------------------------------------------------------------
// Counterspell
// ---------------------------------------------------------------------

#[test]
fn counterspell_counters_target_spell() {
    let (mut s, registry, ids) = fresh_game();
    // P0 casts Bolt at P1's face; P1 responds with Counterspell.
    let bolt = put_in_hand(&mut s, &registry, 0, ids.lightning_bolt);
    let cs = put_in_hand(&mut s, &registry, 1, ids.counterspell);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 1, ManaColor::Blue, 2);
    priority_to_main(&mut s, 0);
    let p1_start = s.player(1).life;

    // P0 casts Bolt targeting P1.
    let cast_bolt = Action::CastSpell {
        object_id: bolt,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::ObjectOrPlayer(
                arcana_core::targets::ObjectOrPlayer::Player(1),
            )],
        },
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![arcana_core::actions::ManaAssignment {
                pool_index: 0, cost_index: 0,
            }],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: None,
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast_bolt, &registry);
    let mut s = s;

    // P1 casts Counterspell at the Bolt on the stack.
    let bolt_stack_id = s.top_of_stack().expect("bolt on stack").id;
    s.priority.give_to(1);
    let cast_cs = Action::CastSpell {
        object_id: cs,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::Object(bolt_stack_id)],
        },
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 1, cost_index: 1 },
            ],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: None,
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast_cs, &registry);
    // Resolve the stack top-down: Counterspell first, then Bolt
    // (which should be countered already).
    let s = resolve_stack(s, &registry);

    // P1 took no damage — Bolt was countered.
    assert_eq!(s.player(1).life, p1_start, "Bolt countered, no damage");
    // Both spells in graveyards.
    assert_eq!(s.zone_count(Zone::Graveyard(0)), 1, "Bolt in P0's yard");
    assert_eq!(s.zone_count(Zone::Graveyard(1)), 1, "Counterspell in P1's yard");
}

// ---------------------------------------------------------------------
// Randomized self-play survival — 100 games, no panic
// ---------------------------------------------------------------------

/// Run N random-policy games with the seed set, asserting only that
/// no game panics and every game terminates. This is a *survival*
/// test, not a correctness test — it catches the class of bug where
/// an unreachable-in-unit-tests sequence of legal actions produces
/// an engine state that crashes, loops, or throws on an invariant.
/// The finding rate should decay fast as the engine matures.
#[test]
fn randomized_self_play_100_games_terminate_without_panic() {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    const GAMES: u64 = 100;
    const MAX_STEPS: u32 = 50_000;

    let mut registry = CardRegistry::new();
    let _ids = register_seed(&mut registry);

    // Two diverse, asymmetric decks. Red burn aggro vs green
    // creature-plus-removal. Different strategies on each side
    // surface interaction bugs that mirror decks can mask.
    let red_aggro = arcana_core::registry::build_deck(
        &[
            ("Mountain", 24),
            ("Lightning Bolt", 12),
            ("Grizzly Bears", 12),  // splash green creatures
            ("Forest", 12),
        ], &registry);
    let blue_control = arcana_core::registry::build_deck(
        &[
            ("Island", 20),
            ("Counterspell", 8),
            ("Murder", 4),
            ("Swamp", 8),
            ("Elvish Visionary", 8),
            ("Forest", 12),
        ], &registry);

    for g in 0..GAMES {
        let (mut state, mut yld) = arcana_core::engine::new_game(
            vec![red_aggro.clone(), blue_control.clone()], &registry, g);
        let mut rng = ChaCha8Rng::seed_from_u64(g ^ 0xDEAD_BEEF);
        let mut step_count = 0u32;
        loop {
            match yld {
                EngineYield::GameOver(_) => break,
                EngineYield::PendingDecision { ref legal_actions, .. } => {
                    assert!(!legal_actions.is_empty(),
                        "game {g} step {step_count}: empty legal_actions");
                    let action = pick_random_action(&mut rng, legal_actions);
                    let (ns, ny) = step(state, action, &registry);
                    state = ns;
                    yld = ny;
                }
            }
            step_count += 1;
            assert!(step_count < MAX_STEPS,
                "game {g}: {MAX_STEPS} steps without termination");
        }
    }
}

fn pick_random_action(
    rng: &mut rand_chacha::ChaCha8Rng,
    actions: &[Action],
) -> Action {
    use rand::Rng;
    if actions.iter().any(|a| matches!(a, Action::MulliganKeep)) {
        return Action::MulliganKeep;
    }
    let interesting: Vec<&Action> = actions.iter()
        .filter(|a| !a.is_pass() && !a.is_concede()).collect();
    if !interesting.is_empty() {
        let idx = rng.gen_range(0..interesting.len());
        return interesting[idx].clone();
    }
    if let Some(p) = actions.iter().find(|a| a.is_pass()) {
        return p.clone();
    }
    actions[0].clone()
}

// ---------------------------------------------------------------------
// X-cost (CR 107.3 / 601.2b) — Disintegrate
// ---------------------------------------------------------------------

#[test]
fn disintegrate_x_equals_5_deals_5_damage() {
    let (mut s, registry, ids) = fresh_game();
    let dis = put_in_hand(&mut s, &registry, 0, ids.disintegrate);
    // Need 6 red mana: X=5 + {R}.
    give_mana(&mut s, 0, ManaColor::Red, 6);
    priority_to_main(&mut s, 0);
    let p1_start = s.player(1).life;

    // Build the cast manually with X=5. Mana plan assigns 5 generic
    // pips to the Generic(5) expansion and 1 red to the {R}.
    let cast = Action::CastSpell {
        object_id: dis,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::ObjectOrPlayer(
                arcana_core::targets::ObjectOrPlayer::Player(1),
            )],
        },
        modes: vec![],
        // Expanded cost is Generic(5) + Colored(R). Mana plan fills
        // the 5 Generic pip (cost_index=0) with 5 red units (they're
        // all indexes 0..4 in the pool, but spending is by-count not
        // by-order); the last red pays the Colored pip.
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 1, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 2, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 3, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 4, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 5, cost_index: 1 },
            ],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: Some(5),
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);
    assert_eq!(p1_start - s.player(1).life, 5,
        "X=5 deals exactly 5 damage");
}

#[test]
fn legal_actions_enumerates_disintegrate_x_values() {
    use arcana_core::engine::new_game;
    use arcana_core::registry::build_deck;

    // Set up a realistic game: Disintegrate in hand, some Mountains
    // on the battlefield (tapped → mana pool filled after
    // enumeration would run). Actually a pre-game cast requires
    // mana in pool, not lands untapped. Simpler: directly fill pool.
    let (mut s, registry, ids) = fresh_game();
    put_in_hand(&mut s, &registry, 0, ids.disintegrate);
    give_mana(&mut s, 0, ManaColor::Red, 5);
    priority_to_main(&mut s, 0);
    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);

    // Expected: X = 0..=4 feasible (X=5 needs 6 mana total,
    // 5{X} + 1{R} = 6 but pool has only 5). For each X, at least
    // one action (may be 2 if any_target enumerates both players
    // P0 and P1).
    let x_values: std::collections::HashSet<u32> = actions.iter()
        .filter_map(|a| match a {
            Action::CastSpell { x_value: Some(x), .. } => Some(*x),
            _ => None,
        }).collect();
    for x in 0..=4 {
        assert!(x_values.contains(&x), "X={x} must be enumerable");
    }
    assert!(!x_values.contains(&5),
        "X=5 needs 6 mana, only 5 available");

    // Also sanity-check: the build_deck / new_game path works with
    // Disintegrate registered. This is a smoke test for the seed-
    // set integration.
    let deck = build_deck(&[("Disintegrate", 4), ("Mountain", 20)], &registry);
    let _ = new_game(vec![deck.clone(), deck], &registry, 42);
}

#[test]
fn disintegrate_x_zero_is_legal_and_deals_no_damage() {
    let (mut s, registry, ids) = fresh_game();
    let dis = put_in_hand(&mut s, &registry, 0, ids.disintegrate);
    // Only 1 red: enough for {R} with X=0.
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);
    let p1_start = s.player(1).life;

    let cast = Action::CastSpell {
        object_id: dis,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::ObjectOrPlayer(
                arcana_core::targets::ObjectOrPlayer::Player(1),
            )],
        },
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![arcana_core::actions::ManaAssignment {
                pool_index: 0, cost_index: 1,
            }],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: Some(0),
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);
    assert_eq!(p1_start - s.player(1).life, 0,
        "X=0 deals 0 damage");
}

#[test]
fn disintegrate_x_cost_bound_by_pool_not_offered_when_underfunded() {
    // Pool is empty except for 1 red. X=0 is the only feasible X.
    let (mut s, registry, ids) = fresh_game();
    put_in_hand(&mut s, &registry, 0, ids.disintegrate);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);
    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let x_values: std::collections::HashSet<u32> = actions.iter()
        .filter_map(|a| match a {
            Action::CastSpell { x_value: Some(x), .. } => Some(*x),
            _ => None,
        }).collect();
    assert!(x_values.contains(&0),
        "X=0 must be enumerable with only the colored pip payable");
    assert!(!x_values.contains(&1),
        "X=1 would need 2 mana total, only 1 available");
}

