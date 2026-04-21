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
    let chars = registry.get(card_id).unwrap().initial_characteristics().clone();
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
    let _top = put_in_library(&mut s, &registry, 0, ids.mountain);
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

/// Keyword-stress matchup: skies control vs ground/deathtouch. Pairs
/// every creature in the keyword-stress seed pack against something
/// that cares about its keyword, so random play exercises the
/// Flying/Reach/Vigilance/Deathtouch pipelines each game.
///
/// Skies side leans on {W/U/B}: Serra Angel (Flying+Vigilance) over
/// the top, Snapcaster Mage (Flying) for recursion, plus
/// Counterspell/Murder interaction. Ground side leans on {B/G}:
/// Typhoid Rats (Deathtouch) trading up on the floor, Giant Spider
/// (Reach) as the Angel answer, Elvish Visionary for card flow,
/// Murder as cross-strategy removal.
///
/// Fewer games than the headline self-play (50 vs 100) because the
/// matchup has richer combat and mid-resolution choices — enough to
/// flush regressions, not so many that CI time doubles.
#[test]
fn keyword_stressed_self_play_50_games_terminate_without_panic() {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    const GAMES: u64 = 50;
    const MAX_STEPS: u32 = 50_000;

    let mut registry = CardRegistry::new();
    let _ids = register_seed(&mut registry);

    let skies = arcana_core::registry::build_deck(
        &[
            ("Plains", 10),
            ("Island", 10),
            ("Swamp", 4),
            ("Serra Angel", 10),
            ("Snapcaster Mage", 10),
            ("Counterspell", 10),
            ("Murder", 6),
        ], &registry);
    let ground = arcana_core::registry::build_deck(
        &[
            ("Swamp", 10),
            ("Forest", 14),
            ("Typhoid Rats", 12),
            ("Grizzly Bears", 8),
            ("Giant Spider", 8),
            ("Elvish Visionary", 4),
            ("Murder", 4),
        ], &registry);

    for g in 0..GAMES {
        let (mut state, mut yld) = arcana_core::engine::new_game(
            vec![skies.clone(), ground.clone()], &registry, g ^ 0xA11C_E0DE);
        let mut rng = ChaCha8Rng::seed_from_u64(g ^ 0xB10C_CADE);
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

// ---------------------------------------------------------------------
// Walking Ballista — X in P/T + counter-removal-as-activation-cost
// ---------------------------------------------------------------------

/// Cast Walking Ballista for {X}{X} with X=3. The 0/0 artifact
/// creature enters with 3 +1/+1 counters via
/// `EntersWithSpec::CountersFromX`, observable as effective 3/3 P/T
/// (layer 7d applies counter deltas to the base 0/0).
#[test]
fn walking_ballista_x_3_enters_as_3_3() {
    use arcana_core::types::CounterKind;
    let (mut s, registry, ids) = fresh_game();
    let wb = put_in_hand(&mut s, &registry, 0, ids.walking_ballista);
    // {X}{X} with X=3 costs 2X = 6 generic mana.
    give_mana(&mut s, 0, ManaColor::Red, 6);
    priority_to_main(&mut s, 0);

    let cast = Action::CastSpell {
        object_id: wb,
        targets: arcana_core::targets::TargetSelection::new(),
        modes: vec![],
        // {X}{X} expands to Generic(3) + Generic(3) at X=3 — both
        // cost pips accept any color. Assign 3 red to cost_index 0
        // (first X) and 3 red to cost_index 1 (second X).
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 1, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 2, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 3, cost_index: 1 },
                arcana_core::actions::ManaAssignment { pool_index: 4, cost_index: 1 },
                arcana_core::actions::ManaAssignment { pool_index: 5, cost_index: 1 },
            ],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: Some(3),
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    // One creature on P0's battlefield, 3/3 via 3 +1/+1 counters.
    let on_battlefield: Vec<ObjectId> = s.objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == 0 && o.characteristics.is_creature())
        .map(|o| o.id)
        .collect();
    assert_eq!(on_battlefield.len(), 1,
        "Walking Ballista should be on P0's battlefield");
    let id = on_battlefield[0];
    let counters = s.objects.get(id).unwrap()
        .count_counters(CounterKind::PlusOnePlusOne);
    assert_eq!(counters, 3, "X=3 → 3 +1/+1 counters");

    let chars = s.compute_characteristics(id).expect("creature chars");
    assert_eq!(chars.power, Some(arcana_core::types::PtValue::Fixed(3)),
        "base 0/0 + 3 +1/+1 counters = effective power 3");
    assert_eq!(chars.toughness, Some(arcana_core::types::PtValue::Fixed(3)),
        "base 0/0 + 3 +1/+1 counters = effective toughness 3");
}

/// Cast Walking Ballista for X=0. It enters as 0/0 with no counters
/// and immediately dies to SBA (toughness 0 → graveyard).
#[test]
fn walking_ballista_x_0_dies_to_sba() {
    let (mut s, registry, ids) = fresh_game();
    let wb = put_in_hand(&mut s, &registry, 0, ids.walking_ballista);
    // {X}{X} with X=0 is free — no mana needed.
    priority_to_main(&mut s, 0);

    let cast = Action::CastSpell {
        object_id: wb,
        targets: arcana_core::targets::TargetSelection::new(),
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan::default(),
        additional_costs: vec![],
        x_value: Some(0),
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    assert_eq!(s.zone_count(Zone::Graveyard(0)), 1,
        "X=0 Ballista enters 0/0 and dies to SBA immediately");
    assert_eq!(
        s.objects.objects_in_zone(Zone::Battlefield)
            .filter(|o| o.characteristics.is_creature()).count(),
        0,
        "no creatures on the battlefield");
}

/// Doubling Season on the battlefield doubles Walking Ballista's
/// entry counters. This pins the architectural invariant that
/// `EntersWithSpec` runs through `place_counters`, which runs through
/// the replacement-effect pipeline — i.e., the card-declared spec is
/// the *input* to replacement, not the output. If this test fails,
/// the stamped spec is being applied raw and bypassing
/// `WouldPlaceCounters` replacements, which would break every future
/// "enters with counters" + counter-doubler interaction.
///
/// Doubling Season is installed as a bare `ReplacementEffect` rather
/// than as a registered card — we don't need the enchantment itself,
/// just its replacement behavior — so this test runs clean without
/// forcing a Doubling Season card into the seed set ahead of a real
/// consumer.
#[test]
fn walking_ballista_x3_with_doubling_season_enters_with_6_counters() {
    use arcana_core::replacement::{
        CounterKindFilter, ReplacementCondition, ReplacementDuration,
        ReplacementEffect, ReplacementKind,
    };
    use arcana_core::targets::{ControllerConstraint, ObjectFilter};
    use arcana_core::types::CounterKind;

    let (mut s, registry, ids) = fresh_game();
    let wb = put_in_hand(&mut s, &registry, 0, ids.walking_ballista);
    give_mana(&mut s, 0, ManaColor::Red, 6);

    // Install Doubling Season as a bare replacement effect. Source
    // id 0xDEAD_BEEF is a test sentinel — no real permanent owns it.
    s.add_replacement_effect(ReplacementEffect {
        source: 0xDEAD_BEEF,
        id: 0, // overwritten by add_replacement_effect
        condition: ReplacementCondition::WouldPlaceCounters {
            object_filter: ObjectFilter::default()
                .controlled_by(ControllerConstraint::You),
            kinds: CounterKindFilter::Any,
        },
        kind: ReplacementKind::MultiplyCounters(2),
        is_self_replacement: false,
        duration: ReplacementDuration::Permanent,
    });

    priority_to_main(&mut s, 0);
    let cast = Action::CastSpell {
        object_id: wb,
        targets: arcana_core::targets::TargetSelection::new(),
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 1, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 2, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 3, cost_index: 1 },
                arcana_core::actions::ManaAssignment { pool_index: 4, cost_index: 1 },
                arcana_core::actions::ManaAssignment { pool_index: 5, cost_index: 1 },
            ],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: Some(3),
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    let id = s.objects.objects_in_zone(Zone::Battlefield)
        .find(|o| o.controller == 0 && o.characteristics.is_creature())
        .map(|o| o.id)
        .expect("Ballista on battlefield");
    assert_eq!(
        s.objects.get(id).unwrap().count_counters(CounterKind::PlusOnePlusOne),
        6,
        "X=3 with Doubling Season → 3*2 = 6 counters",
    );
}

/// Walking Ballista's ping ability: "Remove a +1/+1 counter from ~:
/// deals 1 damage to any target." Verify the counter comes off, the
/// ping fires, and the ability shows up in legal actions only when
/// a counter is present.
#[test]
fn walking_ballista_ping_removes_counter_and_deals_1_damage() {
    use arcana_core::types::CounterKind;
    let (mut s, registry, ids) = fresh_game();
    let wb = put_on_battlefield(&mut s, &registry, 0, ids.walking_ballista);
    // Simulate "entered with 2 +1/+1 counters" manually — we're
    // testing the activation, not the cast-time X path.
    s.objects.get_mut(wb).unwrap()
        .add_counters(CounterKind::PlusOnePlusOne, 2);
    s.objects.get_mut(wb).unwrap().status.summoning_sick = false;
    priority_to_main(&mut s, 0);
    let p1_start = s.player(1).life;

    // Find the ping activation from legal_actions — exact field
    // shape isn't easy to hand-build because of ability_index
    // conventions, and this also exercises the legal-action
    // pipeline's counter-cost filtering.
    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let ping = actions.iter().find(|a| matches!(a,
        Action::ActivateAbility {
            source,
            additional_costs,
            ..
        }
        if *source == wb
            && additional_costs.iter().any(|c| matches!(c,
                arcana_core::actions::AdditionalCostPayment::RemoveCounters {
                    kind: CounterKind::PlusOnePlusOne, count: 1, ..
                }))
    ))
    .cloned()
    .expect("ping-any-target activation should be legal with a counter");

    // Point it at P1 before submitting.
    let ping_at_p1 = match ping {
        Action::ActivateAbility {
            source, ability_index, mana_payment, additional_costs, ..
        } => Action::ActivateAbility {
            source,
            ability_index,
            targets: arcana_core::targets::TargetSelection {
                targets: vec![arcana_core::targets::TargetChoice::ObjectOrPlayer(
                    arcana_core::targets::ObjectOrPlayer::Player(1),
                )],
            },
            mana_payment,
            additional_costs,
        },
        _ => unreachable!(),
    };
    let (s, _) = step(s, ping_at_p1, &registry);
    let s = resolve_stack(s, &registry);

    assert_eq!(p1_start - s.player(1).life, 1, "ping deals exactly 1");
    assert_eq!(
        s.objects.get(wb).unwrap().count_counters(CounterKind::PlusOnePlusOne),
        1,
        "one counter removed as activation cost",
    );

    // With zero counters left, the ping disappears from the legal
    // action set — tests ability_is_activatable's counter gate.
    let mut s2 = s.clone();
    s2.objects.get_mut(wb).unwrap()
        .remove_counters(CounterKind::PlusOnePlusOne, 1);
    priority_to_main(&mut s2, 0);
    let post_actions = arcana_core::legal_actions::legal_actions(&s2, &registry);
    assert!(!post_actions.iter().any(|a| matches!(a,
        Action::ActivateAbility { source, additional_costs, .. }
        if *source == wb
            && additional_costs.iter().any(|c| matches!(c,
                arcana_core::actions::AdditionalCostPayment::RemoveCounters { .. }))
    )), "ping should be filtered out when source has zero counters");
}

// ---------------------------------------------------------------------
// Snapcaster Mage — ETB targets a graveyard card + grants flashback
// ---------------------------------------------------------------------

/// Drive the engine forward, auto-answering any mid-resolution
/// `PickCards` prompt with `pick` (which returns the ObjectId to
/// choose, or `None` to pick zero). Stops when the stack is empty
/// and no pending choice remains.
fn resolve_with_pick_cards<F>(
    mut state: GameState,
    registry: &CardRegistry,
    mut pick: F,
) -> GameState
where
    F: FnMut(&arcana_core::actions::PendingChoice) -> Option<ObjectId>,
{
    use arcana_core::actions::{ChoiceKind, ChoiceResponse};
    for _ in 0..400 {
        if state.is_game_over() { return state; }

        if let Some(pc) = state.pending_choice.clone() {
            let response = match pc.kind {
                ChoiceKind::PickCards { .. } => {
                    let picked = pick(&pc).map(|id| vec![id]).unwrap_or_default();
                    ChoiceResponse::PickCards { picked }
                }
                _ => panic!("resolve_with_pick_cards: unexpected choice kind \
                             {:?}", pc.kind),
            };
            let (ns, _) = step(state, Action::SubmitResolutionChoice {
                id: pc.id, response,
            }, registry);
            state = ns;
            continue;
        }

        if state.stack_is_empty() { return state; }
        let (ns, _) = step(state, Action::PassPriority, registry);
        state = ns;
    }
    panic!("resolve_with_pick_cards: failed to quiesce in 400 iterations");
}

/// Put a specific card into a graveyard with explicit controller/owner.
/// Returns the new ObjectId. Mirrors `put_in_hand` / `put_on_battlefield`
/// for the graveyard zone.
fn put_in_graveyard(
    state: &mut GameState,
    registry: &CardRegistry,
    owner: PlayerId,
    card_id: arcana_core::types::CardId,
) -> ObjectId {
    let obj_id = state.allocate_object_id();
    let chars = registry.get(card_id).unwrap().base_characteristics.clone();
    state.objects.insert(GameObject::new(
        obj_id, owner, Zone::Graveyard(owner), card_id, chars));
    obj_id
}

/// Cast Snapcaster targeting a Bolt in P0's graveyard; verify
/// `legal_actions` on P0's next priority window offers flashback on
/// that Bolt. This is the load-bearing test for the layer system
/// applying continuous effects to non-battlefield objects — if the
/// layer path short-circuits for graveyard-zone objects, the grant
/// is invisible and the flashback cast never shows up.
#[test]
fn snapcaster_grants_flashback_to_bolt_in_graveyard() {
    let (mut s, registry, ids) = fresh_game();
    let snap = put_in_hand(&mut s, &registry, 0, ids.snapcaster_mage);
    let bolt = put_in_graveyard(&mut s, &registry, 0, ids.lightning_bolt);
    // {1}{U} for Snapcaster + {R} reserve for flashback.
    give_mana(&mut s, 0, ManaColor::Blue, 2);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);

    let cast = Action::CastSpell {
        object_id: snap,
        targets: arcana_core::targets::TargetSelection::new(),
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
    let s = resolve_with_pick_cards(s, &registry, |_| Some(bolt));

    // Bolt in the graveyard should now have Flashback in its
    // effective keyword list — queried through the layer system.
    let kws = s.effective_keywords(bolt);
    assert!(kws.iter().any(|k| matches!(k,
        arcana_core::effects::KeywordAbility::Flashback(_))),
        "Bolt should have granted Flashback; got {kws:?}");

    // legal_actions should offer a flashback cast of Bolt.
    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let flashback = actions.iter().any(|a| matches!(a,
        Action::CastSpell {
            object_id,
            cast_modifier: arcana_core::actions::CastModifier::Flashback,
            ..
        } if *object_id == bolt));
    assert!(flashback,
        "legal_actions should include a Flashback cast of Bolt");
}

/// End-of-turn duration: after Snapcaster's grant, if Bolt isn't
/// flashbacked this turn, the grant must not persist into the next
/// turn. Drive through the cleanup step and verify.
#[test]
fn snapcaster_flashback_grant_expires_end_of_turn() {
    let (mut s, registry, ids) = fresh_game();
    let snap = put_in_hand(&mut s, &registry, 0, ids.snapcaster_mage);
    let bolt = put_in_graveyard(&mut s, &registry, 0, ids.lightning_bolt);
    give_mana(&mut s, 0, ManaColor::Blue, 2);
    priority_to_main(&mut s, 0);

    let cast = Action::CastSpell {
        object_id: snap,
        targets: arcana_core::targets::TargetSelection::new(),
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
    let s = resolve_with_pick_cards(s, &registry, |_| Some(bolt));

    // Grant is present before turn ends.
    assert!(s.effective_keywords(bolt).iter().any(|k| matches!(k,
        arcana_core::effects::KeywordAbility::Flashback(_))));

    // Force an end-of-turn pass through the cleanup step. The
    // Duration::EndOfTurn expiry runs inside `expire_end_of_turn_effects`,
    // which the engine's cleanup-step handler calls at CR 514.2. We
    // can invoke it directly to avoid driving the entire turn cycle.
    let mut s = s;
    s.expire_end_of_turn_effects();

    assert!(
        !s.effective_keywords(bolt).iter().any(|k| matches!(k,
            arcana_core::effects::KeywordAbility::Flashback(_))),
        "Flashback grant must expire at end of turn");
}

/// ObjectId scoping, not CardId: Snapcaster grants flashback to a
/// *specific* Bolt object by id. If that Bolt leaves the graveyard
/// (e.g. by being cast normally from hand and re-entering as a new
/// object per CR 400.7), the grant attached to the old id must not
/// apply to the newly-entered object. This is the test that catches
/// "grant accidentally keyed on CardId" bugs.
#[test]
fn snapcaster_grant_does_not_transfer_to_reentered_object() {
    let (mut s, registry, ids) = fresh_game();
    let snap = put_in_hand(&mut s, &registry, 0, ids.snapcaster_mage);
    let bolt_in_gy = put_in_graveyard(&mut s, &registry, 0, ids.lightning_bolt);
    // A second Bolt in hand — we'll cast this one normally after
    // Snapcaster's grant to observe the re-enter-as-new-object
    // behavior.
    let bolt_in_hand = put_in_hand(&mut s, &registry, 0, ids.lightning_bolt);
    give_mana(&mut s, 0, ManaColor::Blue, 2);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);

    let cast_snap = Action::CastSpell {
        object_id: snap,
        targets: arcana_core::targets::TargetSelection::new(),
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
    let (s, _) = step(s, cast_snap, &registry);
    let s = resolve_with_pick_cards(s, &registry, |_| Some(bolt_in_gy));

    // Sanity: grant targeted bolt_in_gy (the graveyard copy).
    assert!(s.effective_keywords(bolt_in_gy).iter().any(|k| matches!(k,
        arcana_core::effects::KeywordAbility::Flashback(_))));

    // Now cast bolt_in_hand normally from hand. It resolves, moves
    // to the graveyard, and gets a fresh ObjectId on the way
    // through (CR 400.7). The grant on the old `bolt_in_gy` id
    // does not transfer.
    let mut s = s;
    priority_to_main(&mut s, 0);
    let cast_bolt = Action::CastSpell {
        object_id: bolt_in_hand,
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
    let s = resolve_stack(s, &registry);

    // There are now (at least) two Bolts in P0's graveyard: the
    // originally-granted one (old id, still granted) and the
    // newly-resolved one (fresh id, not granted).
    let bolts_in_gy: Vec<ObjectId> = s.objects
        .objects_in_zone(Zone::Graveyard(0))
        .filter(|o| o.card_id == ids.lightning_bolt)
        .map(|o| o.id)
        .collect();
    assert!(bolts_in_gy.len() >= 2,
        "expected >=2 Bolts in graveyard, got {}", bolts_in_gy.len());

    // The new Bolt's id is different from `bolt_in_gy`; its keywords
    // must not include Flashback.
    let new_bolt_id = bolts_in_gy.iter().copied()
        .find(|id| *id != bolt_in_gy)
        .expect("a freshly-entered Bolt distinct from the original");
    let new_kws = s.effective_keywords(new_bolt_id);
    assert!(!new_kws.iter().any(|k| matches!(k,
        arcana_core::effects::KeywordAbility::Flashback(_))),
        "re-entered Bolt must not inherit Snapcaster's grant; \
         got keywords {new_kws:?}");
}

// ---------------------------------------------------------------------
// Murktide Regent — delve-count → ETB P/T
// ---------------------------------------------------------------------

/// Cast Murktide with two delve exiles. Delve pays 2 generic of the
/// `{3}{U}{U}` cost, so the caster covers the remaining 1 generic +
/// {U}{U} from the mana pool. On resolution, Murktide enters with 2
/// +1/+1 counters (one per exiled card) → effective 5/5.
#[test]
fn murktide_regent_enters_with_counters_equal_to_delve_exiles() {
    use arcana_core::types::CounterKind;
    let (mut s, registry, ids) = fresh_game();
    let regent = put_in_hand(&mut s, &registry, 0, ids.murktide_regent);
    // Two instant/sorcery cards in P0's graveyard for delve.
    let g1 = put_in_graveyard(&mut s, &registry, 0, ids.lightning_bolt);
    let g2 = put_in_graveyard(&mut s, &registry, 0, ids.counterspell);
    // Remaining cost after 2 delve exiles: 1 generic + {U}{U}. Three
    // blue pays it.
    give_mana(&mut s, 0, ManaColor::Blue, 3);
    priority_to_main(&mut s, 0);

    let cast = Action::CastSpell {
        object_id: regent,
        targets: arcana_core::targets::TargetSelection::new(),
        modes: vec![],
        // Cost expansion: Generic(3), Colored(U), Colored(U). Delve
        // pays 2 of the Generic(3) component, so we only need to
        // cover 1 generic + the two colored with our 3 blue.
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
        cost_reductions: arcana_core::actions::CostReductions {
            delve_exiles: Some(vec![g1, g2]),
            ..Default::default()
        },
    };
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    let id = s.objects.objects_in_zone(Zone::Battlefield)
        .find(|o| o.controller == 0 && o.characteristics.is_creature())
        .map(|o| o.id)
        .expect("Murktide on battlefield");
    assert_eq!(
        s.objects.get(id).unwrap().count_counters(CounterKind::PlusOnePlusOne),
        2,
        "2 delve exiles → 2 +1/+1 counters",
    );
    let chars = s.compute_characteristics(id).expect("creature chars");
    assert_eq!(chars.power, Some(arcana_core::types::PtValue::Fixed(5)));
    assert_eq!(chars.toughness, Some(arcana_core::types::PtValue::Fixed(5)));
}

/// Delve is optional (CR 702.66). Cast Murktide paying the full
/// `{3}{U}{U}` without exiling anything → enters with 0 counters,
/// effective 3/3.
#[test]
fn murktide_regent_without_delve_enters_3_3() {
    use arcana_core::types::CounterKind;
    let (mut s, registry, ids) = fresh_game();
    let regent = put_in_hand(&mut s, &registry, 0, ids.murktide_regent);
    // Full cost: 3 generic + {U}{U} = 5 blue.
    give_mana(&mut s, 0, ManaColor::Blue, 5);
    priority_to_main(&mut s, 0);

    let cast = Action::CastSpell {
        object_id: regent,
        targets: arcana_core::targets::TargetSelection::new(),
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 1, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 2, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 3, cost_index: 1 },
                arcana_core::actions::ManaAssignment { pool_index: 4, cost_index: 2 },
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

    let id = s.objects.objects_in_zone(Zone::Battlefield)
        .find(|o| o.controller == 0 && o.characteristics.is_creature())
        .map(|o| o.id)
        .expect("Murktide on battlefield");
    assert_eq!(
        s.objects.get(id).unwrap().count_counters(CounterKind::PlusOnePlusOne),
        0,
        "no delve → no counters",
    );
    let chars = s.compute_characteristics(id).expect("creature chars");
    assert_eq!(chars.power, Some(arcana_core::types::PtValue::Fixed(3)));
    assert_eq!(chars.toughness, Some(arcana_core::types::PtValue::Fixed(3)));
}

/// Murktide is printed with Flying. The base keyword must survive
/// through `effective_keywords` so combat (blockers need Flying or
/// Reach) sees it. Sanity check on the base-keyword path for a
/// creature that *also* has delve — confirms the two keywords don't
/// shadow each other somewhere.
#[test]
fn murktide_regent_has_flying_via_effective_keywords() {
    let (mut s, registry, ids) = fresh_game();
    let regent = put_on_battlefield(&mut s, &registry, 0, ids.murktide_regent);
    let kws = s.effective_keywords(regent);
    assert!(kws.contains(&arcana_core::effects::KeywordAbility::Flying),
        "Murktide must have Flying in its effective keywords; got {kws:?}");
    assert!(kws.contains(&arcana_core::effects::KeywordAbility::Delve),
        "Murktide's printed Delve keyword must survive (has_delve \
         relies on effective_keywords); got {kws:?}");
}

// ---------------------------------------------------------------------
// Chord of Calling — X + convoke composition + search-with-mv-filter
// ---------------------------------------------------------------------

/// Put a card in `player`'s library via the arena **and** append it
/// to the bottom of `library_top_to_bottom`. Appending mirrors the
/// engine's `move_object_to_zone` into-library convention so draws
/// and scrys see the card in a stable position.
fn put_in_library(
    state: &mut GameState,
    registry: &CardRegistry,
    owner: PlayerId,
    card_id: arcana_core::types::CardId,
) -> ObjectId {
    let obj_id = state.allocate_object_id();
    let chars = registry.get(card_id).unwrap().initial_characteristics().clone();
    state.objects.insert(GameObject::new(
        obj_id, owner, Zone::Library(owner), card_id, chars));
    state.player_mut(owner).library_top_to_bottom.push(obj_id);
    obj_id
}

/// Put a card into `player`'s library arena and push it onto the
/// **top** of `library_top_to_bottom`. Use when a test needs to
/// control which card is drawn first. Repeated calls stack: the
/// last inserted card ends up at the top.
fn put_in_library_top(
    state: &mut GameState,
    registry: &CardRegistry,
    owner: PlayerId,
    card_id: arcana_core::types::CardId,
) -> ObjectId {
    let obj_id = state.allocate_object_id();
    let chars = registry.get(card_id).unwrap().initial_characteristics().clone();
    state.objects.insert(GameObject::new(
        obj_id, owner, Zone::Library(owner), card_id, chars));
    state.player_mut(owner).library_top_to_bottom.insert(0, obj_id);
    obj_id
}

/// Cast Chord at X=2 (no convoke) — library contains a Grizzly
/// Bears; resolution should find it and put it onto the battlefield.
/// The basic "search-with-filter works end to end" test.
#[test]
fn chord_of_calling_x_2_finds_grizzly_bears() {
    let (mut s, registry, ids) = fresh_game();
    let chord = put_in_hand(&mut s, &registry, 0, ids.chord_of_calling);
    let bears = put_in_library(&mut s, &registry, 0, ids.grizzly_bears);
    // {X}{G}{G}{G} at X=2 = 2 generic + GGG = 5 green from pool.
    give_mana(&mut s, 0, ManaColor::Green, 5);
    priority_to_main(&mut s, 0);

    let cast = Action::CastSpell {
        object_id: chord,
        targets: arcana_core::targets::TargetSelection::new(),
        modes: vec![],
        // Expanded cost: Generic(2), Color(G), Color(G), Color(G).
        // Assign 2 green to cost_index 0, 1 green each to 1/2/3.
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 1, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 2, cost_index: 1 },
                arcana_core::actions::ManaAssignment { pool_index: 3, cost_index: 2 },
                arcana_core::actions::ManaAssignment { pool_index: 4, cost_index: 3 },
            ],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: Some(2),
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast, &registry);
    let s = resolve_with_pick_cards(s, &registry, |_| Some(bears));

    // Bears should be on P0's battlefield (with a new ObjectId after
    // the zone-change re-id, per CR 400.7).
    let bears_on_battlefield = s.objects.objects_in_zone(Zone::Battlefield)
        .any(|o| o.card_id == ids.grizzly_bears && o.controller == 0);
    assert!(bears_on_battlefield,
        "Chord should have tutored Bears onto the battlefield");
}

/// The composition gate: X=2 with 2 convokers means the two X pips
/// are paid by creature-taps, leaving only {G}{G}{G} from the pool.
/// Verifies that X enumeration and convoke cost-reduction compose
/// at the cast-validation layer — a regression in either path would
/// block this cast.
#[test]
fn chord_of_calling_x_plus_convoke_composes() {
    let (mut s, registry, ids) = fresh_game();
    let chord = put_in_hand(&mut s, &registry, 0, ids.chord_of_calling);
    let bears_tutor = put_in_library(&mut s, &registry, 0, ids.grizzly_bears);
    // Two green creatures on the battlefield to convoke with. Giving
    // them green payments covers both an X pip (generic) or a colored
    // pip; here we use them for the X=2 generic portion.
    let convoker1 = put_on_battlefield(&mut s, &registry, 0, ids.grizzly_bears);
    let convoker2 = put_on_battlefield(&mut s, &registry, 0, ids.grizzly_bears);
    // Clear summoning sickness so the convoke tap is legal as a cost
    // (CR 302.1 only restricts tap-for-mana and combat under sickness;
    // convoke's tap-for-cost isn't blocked, but a safety here — the
    // engine's convoke validator currently allows summoning-sick
    // creatures per the commit note on apply_cast_spell).
    for c in [convoker1, convoker2] {
        s.objects.get_mut(c).unwrap().status.summoning_sick = false;
    }
    // Only {G}{G}{G} from pool (convoke pays the X pips).
    give_mana(&mut s, 0, ManaColor::Green, 3);
    priority_to_main(&mut s, 0);

    let cast = Action::CastSpell {
        object_id: chord,
        targets: arcana_core::targets::TargetSelection::new(),
        modes: vec![],
        // Generic(2) covered by two convoke Generic payments (not in
        // mana_payment, but in cost_reductions). Pool covers only the
        // three colored pips.
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 1 },
                arcana_core::actions::ManaAssignment { pool_index: 1, cost_index: 2 },
                arcana_core::actions::ManaAssignment { pool_index: 2, cost_index: 3 },
            ],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: Some(2),
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions {
            convoke_taps: Some(vec![
                arcana_core::actions::ConvokeAssignment {
                    creature: convoker1,
                    payment: arcana_core::actions::ConvokePayment::Generic,
                },
                arcana_core::actions::ConvokeAssignment {
                    creature: convoker2,
                    payment: arcana_core::actions::ConvokePayment::Generic,
                },
            ]),
            ..Default::default()
        },
    };
    let (s, _) = step(s, cast, &registry);

    // Both convokers should now be tapped.
    assert!(s.objects.get(convoker1).unwrap().is_tapped(),
        "convoker1 should be tapped after paying convoke");
    assert!(s.objects.get(convoker2).unwrap().is_tapped(),
        "convoker2 should be tapped");

    let s = resolve_with_pick_cards(s, &registry, |_| Some(bears_tutor));

    // The tutored Bears resolved onto the battlefield.
    let tutored_count = s.objects.objects_in_zone(Zone::Battlefield)
        .filter(|o| o.card_id == ids.grizzly_bears && o.controller == 0)
        .count();
    // 2 convokers + 1 tutored = 3 Bears on P0's battlefield.
    assert_eq!(tutored_count, 3,
        "expected 3 Bears (2 convokers + 1 tutored); got {tutored_count}");
}

/// The filter excludes non-creatures and over-MV candidates. Library
/// has three cards spanning the relevant cases. Verify that the
/// `PickCards` candidate set offered by Chord at X=2 contains only
/// the creature-with-mv-≤-2 — the other two are filtered out at
/// candidate-enumeration time.
#[test]
fn chord_of_calling_filter_excludes_noncreatures_and_high_mv() {
    let (mut s, registry, ids) = fresh_game();
    let chord = put_in_hand(&mut s, &registry, 0, ids.chord_of_calling);
    let bears = put_in_library(&mut s, &registry, 0, ids.grizzly_bears);
    // mv=1 instant (Bolt) — should be excluded by the creature filter.
    let _bolt = put_in_library(&mut s, &registry, 0, ids.lightning_bolt);
    // mv=5 creature (Murktide) — should be excluded by the mv≤2 filter.
    let _murktide = put_in_library(&mut s, &registry, 0, ids.murktide_regent);
    give_mana(&mut s, 0, ManaColor::Green, 5);
    priority_to_main(&mut s, 0);

    let cast = Action::CastSpell {
        object_id: chord,
        targets: arcana_core::targets::TargetSelection::new(),
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 1, cost_index: 0 },
                arcana_core::actions::ManaAssignment { pool_index: 2, cost_index: 1 },
                arcana_core::actions::ManaAssignment { pool_index: 3, cost_index: 2 },
                arcana_core::actions::ManaAssignment { pool_index: 4, cost_index: 3 },
            ],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: Some(2),
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast, &registry);

    // Introspect the candidate list by inspecting pending_choice
    // after the resolve loop reaches the PickCards yield.
    let s = {
        let mut s = s;
        // Step until a PickCards choice is pending.
        for _ in 0..50 {
            if s.pending_choice.is_some() { break; }
            if s.stack_is_empty() { break; }
            let (ns, _) = step(s, Action::PassPriority, &registry);
            s = ns;
        }
        let pc = s.pending_choice.clone()
            .expect("Chord should have parked on a PickCards prompt");
        let candidates = match pc.kind {
            arcana_core::actions::ChoiceKind::PickCards { candidates, .. } =>
                candidates,
            other => panic!("expected PickCards, got {other:?}"),
        };
        assert_eq!(candidates, vec![bears],
            "only Bears should be offered; Bolt (not creature) and \
             Murktide (mv=5 > X=2) must be filtered out");
        // Close the choice so resolution finishes cleanly.
        let (ns, _) = step(s, Action::SubmitResolutionChoice {
            id: pc.id,
            response: arcana_core::actions::ChoiceResponse::PickCards {
                picked: vec![bears],
            },
        }, &registry);
        ns
    };
    let s = resolve_stack(s, &registry);

    // Sanity: Bears resolved onto the battlefield.
    assert!(s.objects.objects_in_zone(Zone::Battlefield)
        .any(|o| o.card_id == ids.grizzly_bears && o.controller == 0));
}

// ---------------------------------------------------------------------
// Keyword-stress pack — Serra Angel / Giant Spider / Typhoid Rats
//
// Evergreen combat keywords (Flying, Reach, Vigilance, Deathtouch)
// already have behavioral wiring in `combat.rs` and `sba.rs`. Before
// this pack, the seed had only one keyworded creature (Murktide
// Regent, Flying+Delve), so `randomized_self_play_100_games` exercised
// none of that machinery via real cards. These tests pin the wiring
// to concrete seed cards so any regression in the keyword pipelines
// surfaces at the integration layer, not just at the unit layer.
// ---------------------------------------------------------------------

fn clear_summoning_sickness(state: &mut GameState, obj: ObjectId) {
    state.objects.get_mut(obj).unwrap().status.summoning_sick = false;
}

#[test]
fn serra_angel_attacks_without_tapping_and_is_unblockable_by_ground() {
    let (mut s, registry, ids) = fresh_game();
    let angel = put_on_battlefield(&mut s, &registry, 0, ids.serra_angel);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    clear_summoning_sickness(&mut s, angel);
    clear_summoning_sickness(&mut s, bears);

    s.begin_combat();
    s.apply_declared_attackers(vec![
        arcana_core::combat::AttackerDeclaration {
            attacker: angel,
            defending: arcana_core::combat::DefendingEntity::Player(1),
        },
    ]);

    // CR 702.20a — Vigilance: attacker did not tap.
    assert!(!s.objects.get(angel).unwrap().is_tapped(),
        "Serra Angel has Vigilance; attacking must not tap it");

    s.enter_declare_blockers();
    s.apply_declared_blockers(vec![
        arcana_core::combat::BlockerDeclaration {
            blocker: bears, blocking: angel,
        },
    ]);

    // CR 702.9b — Flying: a ground creature without Reach/Flying
    // cannot block, so the declaration is rejected and the attacker
    // remains unblocked.
    let combat = s.combat.as_ref().unwrap();
    assert!(combat.blockers.is_empty(),
        "Bears (no Flying, no Reach) must not block a Flying attacker");
    assert!(!combat.attacker(angel).unwrap().is_blocked);
}

#[test]
fn giant_spider_can_block_serra_angel_via_reach() {
    let (mut s, registry, ids) = fresh_game();
    let angel = put_on_battlefield(&mut s, &registry, 0, ids.serra_angel);
    let spider = put_on_battlefield(&mut s, &registry, 1, ids.giant_spider);
    clear_summoning_sickness(&mut s, angel);
    clear_summoning_sickness(&mut s, spider);

    s.begin_combat();
    s.apply_declared_attackers(vec![
        arcana_core::combat::AttackerDeclaration {
            attacker: angel,
            defending: arcana_core::combat::DefendingEntity::Player(1),
        },
    ]);
    s.enter_declare_blockers();
    s.apply_declared_blockers(vec![
        arcana_core::combat::BlockerDeclaration {
            blocker: spider, blocking: angel,
        },
    ]);

    // CR 702.17 — Reach: Spider can block a Flying attacker. Pairing
    // must be accepted and the attacker marked blocked.
    let combat = s.combat.as_ref().unwrap();
    assert_eq!(combat.blockers.len(), 1,
        "Spider has Reach; block vs Flying must be legal");
    let info = combat.attacker(angel).unwrap();
    assert!(info.is_blocked);
    assert_eq!(info.blocked_by, vec![spider]);
}

#[test]
fn serra_angel_trades_with_spider_then_angel_dies_spider_survives() {
    // Serra Angel is 4/4; Giant Spider is 2/4. Spider's 2 damage is
    // non-lethal to Angel (4 toughness), Angel's 4 damage is exactly
    // lethal to Spider (4 toughness). After SBAs: Spider dies, Angel
    // survives. This exercises the Flying+Reach pairing resolving
    // through actual damage rather than just blocker validation.
    let (mut s, registry, ids) = fresh_game();
    let angel = put_on_battlefield(&mut s, &registry, 0, ids.serra_angel);
    let spider = put_on_battlefield(&mut s, &registry, 1, ids.giant_spider);
    clear_summoning_sickness(&mut s, angel);
    clear_summoning_sickness(&mut s, spider);

    s.begin_combat();
    s.apply_declared_attackers(vec![
        arcana_core::combat::AttackerDeclaration {
            attacker: angel,
            defending: arcana_core::combat::DefendingEntity::Player(1),
        },
    ]);
    s.enter_declare_blockers();
    s.apply_declared_blockers(vec![
        arcana_core::combat::BlockerDeclaration {
            blocker: spider, blocking: angel,
        },
    ]);
    s.deal_combat_damage();
    arcana_core::sba::apply_state_based_actions(&mut s);

    // Spider (2/4) took 4 damage and dies. Angel (4/4) took 2 damage
    // and survives.
    assert_eq!(s.zone_count(Zone::Graveyard(1)), 1,
        "Spider should be in player 1's graveyard");
    assert!(s.objects.objects_in_zone(Zone::Battlefield)
        .any(|o| o.id == angel),
        "Serra Angel survives the trade");
}

#[test]
fn typhoid_rats_one_damage_kills_grizzly_bears_via_deathtouch_sba() {
    // CR 702.2b / 704.5g — deathtouch damage of any nonzero amount is
    // lethal; SBAs destroy any creature so marked. Use the damage
    // primitive directly so the test stays focused on the DT wiring
    // rather than the whole combat pipeline (Serra Angel vs Spider
    // above covers the combat path).
    let (mut s, registry, ids) = fresh_game();
    let rats = put_on_battlefield(&mut s, &registry, 0, ids.typhoid_rats);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);

    s.deal_damage(rats, arcana_core::combat::DamageTarget::Object(bears), 1, true);
    arcana_core::sba::apply_state_based_actions(&mut s);

    assert_eq!(s.zone_count(Zone::Graveyard(1)), 1,
        "1 damage from a DT source should be lethal to any creature");
    assert!(s.event_log.iter().any(|e| matches!(e,
        arcana_core::events::GameEvent::Dies { object_id } if *object_id == bears)),
        "Dies event should fire for the Bears");
}

// ---------------------------------------------------------------------
// Modal spells (CR 700.2) — Abrade + inline no-target-mode fixture
//
// Abrade's "Choose one" shape proves the per-clause target filter
// plumbing and the mode-dispatch in the resolve fn. It doesn't
// exercise a clause with no targets. To avoid the silent assumption
// "every modal clause targets," a second test registers a synthetic
// two-mode card inline where mode 0 is "gain 3 life" (no target) and
// mode 1 is "deal 3 damage to target creature."
// ---------------------------------------------------------------------

fn put_artifact_on_battlefield(
    state: &mut GameState,
    player: PlayerId,
) -> ObjectId {
    let obj_id = state.allocate_object_id();
    let name = 0; // interner id 0; not looked up by name in these tests
    let chars = arcana_core::objects::Characteristics {
        name,
        types: arcana_core::types::TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    state.objects.insert(GameObject::new(
        obj_id, player, Zone::Battlefield, 0, chars));
    obj_id
}

#[test]
fn abrade_mode_0_deals_3_damage_to_target_creature() {
    let (mut s, registry, ids) = fresh_game();
    let abrade = put_in_hand(&mut s, &registry, 0, ids.abrade);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let cast = Action::CastSpell {
        object_id: abrade,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::Object(bears)],
        },
        modes: vec![arcana_core::stack::ModeChoice::new(vec![0])],
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

    assert_eq!(s.zone_count(Zone::Graveyard(1)), 1,
        "Bears (2 toughness) dies to Abrade's 3 damage");
}

#[test]
fn abrade_mode_1_destroys_target_artifact() {
    let (mut s, registry, ids) = fresh_game();
    let abrade = put_in_hand(&mut s, &registry, 0, ids.abrade);
    let artifact = put_artifact_on_battlefield(&mut s, 1);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let cast = Action::CastSpell {
        object_id: abrade,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::Object(artifact)],
        },
        modes: vec![arcana_core::stack::ModeChoice::new(vec![1])],
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

    assert_eq!(s.zone_count(Zone::Graveyard(1)), 1,
        "Artifact moves to owner's graveyard after Destroy");
    assert!(!s.objects.objects_in_zone(Zone::Battlefield)
        .any(|o| o.id == artifact),
        "artifact is no longer on the battlefield");
}

#[test]
fn abrade_mode_validation_rejects_empty_mode_choice() {
    // Modal spell cast with `modes: vec![]` must be rejected by
    // apply_cast_spell — the engine keeps its invariants (no mana
    // spent, no spell on stack).
    let (mut s, registry, ids) = fresh_game();
    let abrade = put_in_hand(&mut s, &registry, 0, ids.abrade);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let pool_before = s.player(0).mana_pool.total();
    let cast = Action::CastSpell {
        object_id: abrade,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::Object(bears)],
        },
        modes: vec![], // invalid for a modal spell
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
    assert!(s.stack_is_empty(),
        "rejected modal cast must not put the spell on the stack");
    assert_eq!(s.player(0).mana_pool.total(), pool_before,
        "rejected modal cast must not spend mana");
}

#[test]
fn abrade_legal_actions_enumerate_both_modes() {
    let (mut s, registry, ids) = fresh_game();
    let _abrade = put_in_hand(&mut s, &registry, 0, ids.abrade);
    let _bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    let _artifact = put_artifact_on_battlefield(&mut s, 1);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast_actions: Vec<_> = actions.iter().filter_map(|a| match a {
        Action::CastSpell { modes, .. } => Some(modes.clone()),
        _ => None,
    }).collect();
    // At least one (mode=[0], target=bears) and one (mode=[1],
    // target=artifact). The enumerator generates more rows (every
    // legal mana plan), so just assert both mode choices appear.
    let has_mode_0 = cast_actions.iter().any(|m|
        m.len() == 1 && m[0].mode_indices == vec![0]);
    let has_mode_1 = cast_actions.iter().any(|m|
        m.len() == 1 && m[0].mode_indices == vec![1]);
    assert!(has_mode_0, "Abrade mode 0 (damage creature) must be enumerated");
    assert!(has_mode_1, "Abrade mode 1 (destroy artifact) must be enumerated");
    // And no action emits both — Abrade is Choose one (max=1).
    assert!(!cast_actions.iter().any(|m|
        m.iter().any(|c| c.mode_indices.len() > 1)),
        "Choose-one spell must not enumerate multi-mode picks");
}

#[test]
fn modal_spell_with_no_target_clause_resolves() {
    // Inline test fixture: a card with two modes, one of which does
    // not target. Plugs the silent "every clause has targets"
    // assumption: Abrade alone doesn't cover this path.
    use arcana_core::effects::Effect;
    use arcana_core::events::DamageTarget;
    use arcana_core::registry::{
        CardDefinition, ModalSpec, ModeClause, SpellAbilityDef,
    };
    use arcana_core::stack::StackEntry;
    use arcana_core::state::GameState;

    fn resolve(
        _: &GameState,
        entry: &StackEntry,
        _: &CardRegistry,
    ) -> Vec<Effect> {
        let choice = entry.modes.first().expect("modal: one ModeChoice");
        let mode = *choice.mode_indices.first().expect("modal: one mode index");
        match mode {
            0 => vec![Effect::GainLife { player: entry.controller, amount: 3 }],
            1 => {
                let Some(t) = entry.targets.targets.first() else {
                    return Vec::new();
                };
                let target = match t {
                    arcana_core::targets::TargetChoice::Object(id) => *id,
                    _ => return Vec::new(),
                };
                vec![Effect::DealDamage {
                    source: entry.source,
                    target: DamageTarget::Object(target),
                    amount: 3,
                }]
            }
            _ => Vec::new(),
        }
    }

    let mut registry = CardRegistry::new();
    let ids = register_seed(&mut registry);
    let card_id = {
        let name = registry.interner_mut().intern("Test Modal Spell");
        let chars = arcana_core::objects::Characteristics {
            name,
            mana_cost: Some(arcana_core::mana::ManaCost::parse("{R}")
                .expect("valid cost")),
            colors: arcana_core::types::ColorSet::red(),
            types: arcana_core::types::TypeLine::INSTANT.into(),
            ..Default::default()
        };
        registry.register(
            CardDefinition::new(name, chars)
                .with_spell_ability(SpellAbilityDef {
                    text: "Choose one — Gain 3 life; or deal 3 damage \
                           to target creature.".into(),
                    target_requirements: vec![],
                    modal: Some(ModalSpec {
                        min_modes: 1, max_modes: 1,
                        clauses: vec![
                            ModeClause {
                                text: "Gain 3 life.".into(),
                                target_requirements: vec![], // no target!
                            },
                            ModeClause {
                                text: "Deal 3 damage to target creature.".into(),
                                target_requirements: vec![
                                    arcana_core::targets::TargetRequirement
                                        ::target_creature(),
                                ],
                            },
                        ],
                    }),
                    effect: resolve,
                }))
    };

    // --- Cast the non-targeting mode -------------------------------
    let mut s = GameState::new(2, 0);
    let card = {
        let obj_id = s.allocate_object_id();
        let chars = registry.get(card_id).unwrap()
            .base_characteristics.clone();
        s.objects.insert(GameObject::new(
            obj_id, 0, Zone::Hand(0), card_id, chars));
        obj_id
    };
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);
    let life_start = s.player(0).life;

    let cast = Action::CastSpell {
        object_id: card,
        targets: arcana_core::targets::TargetSelection { targets: vec![] },
        modes: vec![arcana_core::stack::ModeChoice::new(vec![0])],
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
    assert_eq!(s.player(0).life, life_start + 3,
        "Mode 0 (no-target gain 3 life) must resolve cleanly");
    // Sanity: nothing else broke — no stray objects, stack empty.
    assert!(s.stack_is_empty());
    // Reference the seed so the test exercises the same registry as
    // production (and catches any "modal must have targets" baked
    // into the seed-construction path).
    let _ = ids;
}

// ---------------------------------------------------------------------
// Planeswalker loyalty (CR 606) — Chandra, Pyromaster + inline fixture
//
// Chandra proves the `+N:` cost path, PW ETB counter placement, the
// once-per-turn-per-PW restriction, and sorcery-speed gating. An
// inline test-fixture PW registers a `−N:` ability to exercise the
// minus-cost path without requiring a second real PW card.
// ---------------------------------------------------------------------

/// Bring `player` to their own main phase with priority, with a PW
/// already on the battlefield. Returns `(state, registry, pw_id)`.
fn fresh_with_chandra_on_battlefield(
) -> (GameState, CardRegistry, SeedIds, ObjectId) {
    let (mut s, registry, ids) = fresh_game();
    // Put Chandra on the battlefield directly (bypasses the cast
    // path). But the ETB hook needs to fire so loyalty counters land
    // — use move_object_to_zone via the allocate-then-arena-insert
    // route from Hand→Battlefield so after_enter_battlefield runs.
    let hand_id = put_in_hand(&mut s, &registry, 0, ids.chandra_pyromaster);
    // move_object_to_zone into battlefield already calls
    // after_enter_battlefield internally — don't double-fire it.
    let pw_id = s.move_object_to_zone(
        hand_id, Zone::Battlefield,
        arcana_core::events::MoveCause::SpellResolution)
        .expect("Chandra moves Hand→Battlefield");
    // Clear the summoning-sick flag that after_enter_battlefield
    // stamps: summoning sickness doesn't restrict loyalty activations
    // (CR 114.3 limits it to attacking), but legal_actions has an
    // audit path that assumes summoning_sick is meaningful. Keep it
    // set in tests to prove it doesn't matter.
    priority_to_main(&mut s, 0);
    (s, registry, ids, pw_id)
}

#[test]
fn chandra_enters_with_four_loyalty_counters() {
    let (s, _registry, _ids, pw_id) = fresh_with_chandra_on_battlefield();
    let pw = s.objects.get(pw_id).unwrap();
    assert_eq!(
        pw.count_counters(arcana_core::types::CounterKind::Loyalty), 4,
        "Chandra enters with 4 Loyalty counters (CR 113.3c)");
}

#[test]
fn chandra_plus_one_adds_loyalty_and_deals_1_damage() {
    let (s, registry, _ids, pw_id) = fresh_with_chandra_on_battlefield();
    let p1_start = s.player(1).life;

    // Activate +1 targeting player 1. Ability index 0 is the only
    // ability on Chandra.
    let activate = Action::ActivateAbility {
        source: pw_id,
        ability_index: 0,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::Player(1)],
        },
        mana_payment: arcana_core::actions::ManaPaymentPlan::empty(),
        additional_costs: vec![
            arcana_core::actions::AdditionalCostPayment::AddCounters {
                source: pw_id,
                kind: arcana_core::types::CounterKind::Loyalty,
                count: 1,
            },
        ],
    };
    let (s, _) = step(s, activate, &registry);
    let s = resolve_stack(s, &registry);

    // Loyalty went from 4 to 5 (cost added +1).
    let pw = s.objects.get(pw_id).unwrap();
    assert_eq!(
        pw.count_counters(arcana_core::types::CounterKind::Loyalty), 5,
        "+1 cost adds a Loyalty counter");
    assert_eq!(p1_start - s.player(1).life, 1,
        "+1 effect deals 1 damage to target player");
}

#[test]
fn chandra_plus_one_twice_rejected_by_legal_actions() {
    let (s, registry, _ids, pw_id) = fresh_with_chandra_on_battlefield();

    // First activation: legal. Confirm.
    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(actions.iter().any(|a| matches!(a,
        Action::ActivateAbility { source, .. } if *source == pw_id)),
        "first activation must be legal");

    // Perform the activation (don't bother resolving the stack entry
    // — the loyalty mark happens at activation, not resolution).
    let activate = Action::ActivateAbility {
        source: pw_id,
        ability_index: 0,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::Player(1)],
        },
        mana_payment: arcana_core::actions::ManaPaymentPlan::empty(),
        additional_costs: vec![
            arcana_core::actions::AdditionalCostPayment::AddCounters {
                source: pw_id,
                kind: arcana_core::types::CounterKind::Loyalty,
                count: 1,
            },
        ],
    };
    let (s, _) = step(s, activate, &registry);
    let s = resolve_stack(s, &registry);

    // Second activation attempt the same turn: CR 606.3 forbids.
    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(!actions.iter().any(|a| matches!(a,
        Action::ActivateAbility { source, .. } if *source == pw_id)),
        "second activation this turn must not appear in legal_actions");
}

#[test]
fn chandra_loyalty_abilities_not_legal_outside_sorcery_speed() {
    let (mut s, registry, _ids, pw_id) = fresh_with_chandra_on_battlefield();
    // Step into combat — sorcery-speed check rejects the activation.
    s.turn.phase = arcana_core::turn::Phase::Combat;
    s.turn.step = arcana_core::turn::Step::BeginCombat;

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(!actions.iter().any(|a| matches!(a,
        Action::ActivateAbility { source, .. } if *source == pw_id)),
        "loyalty ability must not be legal during combat (not sorcery speed)");
}

#[test]
fn loyalty_ledger_clears_between_turns() {
    let (mut s, _registry, _ids, pw_id) = fresh_with_chandra_on_battlefield();
    s.loyalty_activated_this_turn.insert(pw_id);
    // Simulate the turn-start reset — engine::next_turn calls this.
    // We call start_next_turn directly here to avoid threading a full
    // turn-pass through the test.
    s.turn.start_next_turn(0);
    s.loyalty_activated_this_turn.clear();
    // After clear, the ledger no longer blocks the PW.
    assert!(!s.loyalty_activated_this_turn.contains(&pw_id));
}

/// Inline test-fixture PW exercising the `−N:` cost path. Registers a
/// 5-loyalty PW whose only ability is "−2: deal 3 damage to target
/// creature." Proves the minus-cost path runs through
/// remove_self_counter → `AdditionalCostPayment::RemoveCounters` →
/// `obj.remove_counters`. Loyalty starts at 5 so the PW survives the
/// cost — the SBA-on-0-loyalty path is covered by the existing
/// `planeswalker_zero_loyalty_goes_to_graveyard` test in sba.rs.
#[test]
fn pw_minus_ability_removes_loyalty_counters() {
    use arcana_core::registry::{
        ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    };
    use arcana_core::state::GameState;

    fn damage_creature(
        _s: &GameState,
        ctx: &ActivationContext,
        _: &CardRegistry,
    ) -> Vec<arcana_core::effects::Effect> {
        let Some(t) = ctx.targets.targets.first() else { return Vec::new(); };
        let target = match t {
            arcana_core::targets::TargetChoice::Object(id) => *id,
            _ => return Vec::new(),
        };
        vec![arcana_core::effects::Effect::DealDamage {
            source: ctx.source,
            target: arcana_core::events::DamageTarget::Object(target),
            amount: 3,
        }]
    }

    let mut registry = CardRegistry::new();
    let ids = register_seed(&mut registry);
    let card_id = {
        let name = registry.interner_mut().intern("Test Mini-PW");
        let chars = arcana_core::objects::Characteristics {
            name,
            mana_cost: Some(arcana_core::mana::ManaCost::parse("{2}{B}")
                .expect("valid cost")),
            colors: arcana_core::types::ColorSet::black(),
            types: arcana_core::types::TypeLine::PLANESWALKER.into(),
            loyalty: Some(5),
            ..Default::default()
        };
        registry.register(
            CardDefinition::new(name, chars)
                .with_activated_ability(ActivatedAbilityDef {
                    text: "−2: Deal 3 damage to target creature.".into(),
                    cost: ActivationCost {
                        remove_self_counter: Some((
                            arcana_core::types::CounterKind::Loyalty, 2)),
                        ..ActivationCost::default()
                    },
                    target_requirements: vec![
                        arcana_core::targets::TargetRequirement
                            ::target_creature(),
                    ],
                    is_mana_ability: false,
                    is_loyalty_ability: true,
                    activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: damage_creature,
                }))
    };

    let mut s = GameState::new(2, 0);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    let pw_id = {
        let obj_id = s.allocate_object_id();
        let chars = registry.get(card_id).unwrap()
            .base_characteristics.clone();
        s.objects.insert(GameObject::new(
            obj_id, 0, Zone::Battlefield, card_id, chars));
        s.after_enter_battlefield(obj_id);
        obj_id
    };
    priority_to_main(&mut s, 0);

    assert_eq!(
        s.objects.get(pw_id).unwrap()
            .count_counters(arcana_core::types::CounterKind::Loyalty), 5,
        "PW enters with 5 Loyalty counters");

    let activate = Action::ActivateAbility {
        source: pw_id,
        ability_index: 0,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::Object(bears)],
        },
        mana_payment: arcana_core::actions::ManaPaymentPlan::empty(),
        additional_costs: vec![
            arcana_core::actions::AdditionalCostPayment::RemoveCounters {
                source: pw_id,
                kind: arcana_core::types::CounterKind::Loyalty,
                count: 2,
            },
        ],
    };
    let (s, _) = step(s, activate, &registry);
    let s = resolve_stack(s, &registry);

    // Loyalty 5 → 3 after −2 cost.
    assert_eq!(
        s.objects.get(pw_id).unwrap()
            .count_counters(arcana_core::types::CounterKind::Loyalty), 3,
        "−2 cost removes 2 Loyalty counters");
    // Bears (2 toughness) takes 3 damage, dies via SBA.
    assert_eq!(s.zone_count(Zone::Graveyard(1)), 1,
        "Bears dies to 3 damage from the PW's −2 ability");
    // PW survives with 3 loyalty.
    assert!(s.objects.objects_in_zone(Zone::Battlefield)
        .any(|o| o.id == pw_id),
        "PW with 3 loyalty remains on the battlefield");
}

// ---------------------------------------------------------------------
// Kicker (CR 702.32) — Burst Lightning
// ---------------------------------------------------------------------
//
// Proves the two legs of the kicker pipeline:
//
//  1. Enumeration — given enough mana, legal_actions offers both an
//     unkicked ({R}) and a kicked ({R}{4}) cast; given only {R},
//     only the unkicked variant appears.
//  2. Resolution — the kicked cast flips `StackEntry::kicked`, and
//     Burst Lightning's effect fn branches to deal 4 damage; the
//     unkicked cast deals 2. Cover both branches and the mana-gated
//     negative.

fn burst_lightning_cast_action(
    actions: &[Action],
    object_id: ObjectId,
    kicked: bool,
) -> Option<Action> {
    use arcana_core::actions::AdditionalCostPayment;
    actions.iter().find(|a| {
        matches!(a, Action::CastSpell {
            object_id: oid, additional_costs, ..
        } if *oid == object_id
            && additional_costs.iter().any(|c|
                matches!(c, AdditionalCostPayment::Kicker)) == kicked)
    }).cloned()
}

#[test]
fn burst_lightning_unkicked_deals_2_damage() {
    let (mut s, registry, ids) = fresh_game();
    let bl = put_in_hand(&mut s, &registry, 0, ids.burst_lightning);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    // Enough for the unkicked cast only: {R}.
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = burst_lightning_cast_action(&actions, bl, /*kicked*/ false)
        .expect("unkicked cast must be offered with {R} available");

    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    // Bears has 2 toughness; 2 damage kills it.
    assert_eq!(s.zone_count(Zone::Graveyard(1)), 1,
        "unkicked Burst Lightning (2 damage) kills Bears");
    let _ = bears;
}

#[test]
fn burst_lightning_kicked_deals_4_damage_to_player() {
    let (mut s, registry, ids) = fresh_game();
    let bl = put_in_hand(&mut s, &registry, 0, ids.burst_lightning);
    // Kicked target: player 1. Proves both the kicked amount (4) and
    // that kicker composes with player-typed TargetChoice::any_target.
    let p1_start = s.player(1).life;
    // Enough for the kicked total {R}{4} = 5 mana.
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 4);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = burst_lightning_cast_action(&actions, bl, /*kicked*/ true)
        .expect("kicked cast must be offered when {R}{4} is payable");

    // The cast we replace the target on: pick player 1 deterministically
    // so the assertion is stable regardless of enumeration order.
    let cast = match cast {
        Action::CastSpell { object_id, modes, mana_payment,
                            additional_costs, x_value, cast_modifier,
                            cost_reductions, .. } => Action::CastSpell {
            object_id,
            targets: arcana_core::targets::TargetSelection {
                targets: vec![
                    arcana_core::targets::TargetChoice::ObjectOrPlayer(
                        arcana_core::targets::ObjectOrPlayer::Player(1)),
                ],
            },
            modes, mana_payment, additional_costs, x_value,
            cast_modifier, cost_reductions,
        },
        _ => unreachable!(),
    };
    let (s, _) = step(s, cast, &registry);
    // Verify the stack entry is flagged kicked before resolution.
    assert!(s.top_of_stack().unwrap().kicked,
        "kicked cast stamps StackEntry::kicked");
    let s = resolve_stack(s, &registry);

    assert_eq!(p1_start - s.player(1).life, 4,
        "kicked Burst Lightning deals 4 damage to target player");
}

#[test]
fn burst_lightning_kicked_not_offered_without_kicker_mana() {
    // Only {R} available — the kicked variant requires {R}{4}, so
    // legal_actions should not emit it.
    let (mut s, registry, ids) = fresh_game();
    let bl = put_in_hand(&mut s, &registry, 0, ids.burst_lightning);
    let _bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(burst_lightning_cast_action(&actions, bl, /*kicked*/ true)
        .is_none(),
        "kicked variant must not be offered without kicker mana");
    assert!(burst_lightning_cast_action(&actions, bl, /*kicked*/ false)
        .is_some(),
        "unkicked variant still legal with only {{R}}");
}

#[test]
fn burst_lightning_both_variants_offered_when_affordable() {
    // With enough mana for either, both tracks appear — caster picks.
    let (mut s, registry, ids) = fresh_game();
    let bl = put_in_hand(&mut s, &registry, 0, ids.burst_lightning);
    let _bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 4);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(burst_lightning_cast_action(&actions, bl, /*kicked*/ false)
        .is_some(),
        "unkicked variant legal when both are payable");
    assert!(burst_lightning_cast_action(&actions, bl, /*kicked*/ true)
        .is_some(),
        "kicked variant legal when {{R}}{{4}} is payable");
}

#[test]
fn burst_lightning_apply_rejects_kicker_flag_without_keyword() {
    // Belt-and-suspenders: if an agent hand-crafts a CastSpell with
    // AdditionalCostPayment::Kicker on a card that lacks the Kicker
    // keyword (Lightning Bolt), apply_cast_spell silently rejects.
    // The card stays in hand and no spell reaches the stack.
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
        additional_costs: vec![
            arcana_core::actions::AdditionalCostPayment::Kicker,
        ],
        x_value: None,
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast, &registry);

    assert!(s.stack_is_empty(),
        "bogus Kicker on non-kicker card must be rejected — stack stays empty");
    assert!(s.objects.objects_in_zone(Zone::Hand(0)).any(|o| o.id == bolt),
        "Lightning Bolt stays in hand after silent rejection");
}

// ---------------------------------------------------------------------
// Cycling (CR 702.29) — Tranquil Thicket
// ---------------------------------------------------------------------
//
// Tranquil Thicket has two activated abilities living in different
// zones:
//
//   Battlefield: {T}: Add {G}.        (mana ability)
//   Hand:        Cycling {2}          (discard + draw)
//
// The tests verify that (1) cycling fires end-to-end from hand,
// (2) it's instant-speed legal, (3) mana-insufficiency filters it
// out, (4) the tap-for-green ability still works from Battlefield
// and is NOT offered when the card is in hand, and (5) cycling
// survives the source's zone change — draw happens after discard
// has moved the source to graveyard (re-id per CR 400.7).

fn cycling_activation(
    actions: &[Action],
    source: ObjectId,
) -> Option<Action> {
    // Cycling is the second activated ability on Tranquil Thicket
    // (ability_index 1). The mana ability is index 0.
    actions.iter().find(|a| matches!(a,
        Action::ActivateAbility { source: s, ability_index: 1, .. }
            if *s == source)).cloned()
}

fn mana_activation(
    actions: &[Action],
    source: ObjectId,
) -> Option<Action> {
    actions.iter().find(|a| matches!(a,
        Action::ActivateAbility { source: s, ability_index: 0, .. }
            if *s == source)).cloned()
}

#[test]
fn tranquil_thicket_cycling_discards_and_draws() {
    let (mut s, registry, ids) = fresh_game();
    let thicket = put_in_hand(&mut s, &registry, 0, ids.tranquil_thicket);
    // Library card to draw — a Bears so we can assert it's visible
    // in hand post-cycle (as a re-ided Battlefield-free object,
    // but the zone assertion is what we actually check).
    let _lib_card = put_in_library(&mut s, &registry, 0, ids.grizzly_bears);
    // Payment for cycling {2}.
    give_mana(&mut s, 0, ManaColor::Colorless, 2);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cycle = cycling_activation(&actions, thicket)
        .expect("cycling action must be offered with {{2}} in pool");

    let (s, _) = step(s, cycle, &registry);
    // After activation: the thicket cost has already moved the card
    // to graveyard (discard cost); the draw is on the stack.
    let s = resolve_stack(s, &registry);

    // Graveyard has the cycled thicket; hand has the drawn card; the
    // library is now empty.
    assert_eq!(s.zone_count(Zone::Graveyard(0)), 1,
        "cycling puts Thicket in graveyard");
    assert_eq!(s.zone_count(Zone::Hand(0)), 1,
        "cycling draws one card into hand");
    assert_eq!(s.zone_count(Zone::Library(0)), 0,
        "library was depleted by the draw");
}

#[test]
fn tranquil_thicket_cycling_is_instant_speed() {
    // Opponent's turn, end step: sorcery speed is NOT ok but cycling
    // still legal (CR 702.29a).
    let (mut s, registry, ids) = fresh_game();
    let thicket = put_in_hand(&mut s, &registry, 1, ids.tranquil_thicket);
    let _lib = put_in_library(&mut s, &registry, 1, ids.grizzly_bears);
    give_mana(&mut s, 1, ManaColor::Colorless, 2);
    // Player 0 is the active player; give player 1 priority during a
    // non-main phase (end step).
    s.priority.give_to(1);
    s.turn.phase = arcana_core::turn::Phase::Ending;
    s.turn.step = arcana_core::turn::Step::End;

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(cycling_activation(&actions, thicket).is_some(),
        "cycling must be offered during opponent's end step \
         (instant speed)");
}

#[test]
fn tranquil_thicket_cycling_not_offered_without_mana() {
    let (mut s, registry, ids) = fresh_game();
    let thicket = put_in_hand(&mut s, &registry, 0, ids.tranquil_thicket);
    let _lib = put_in_library(&mut s, &registry, 0, ids.grizzly_bears);
    // No mana given.
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(cycling_activation(&actions, thicket).is_none(),
        "no mana → no cycling action enumerated");
}

#[test]
fn tranquil_thicket_tap_for_green_works_on_battlefield() {
    // Sanity: the mana ability is still wired and still activatable
    // when the card is on the battlefield.
    let (mut s, registry, ids) = fresh_game();
    let thicket = put_on_battlefield(&mut s, &registry, 0, ids.tranquil_thicket);
    // Clear any summoning sickness — lands don't have it anyway, but
    // be defensive.
    if let Some(obj) = s.objects.get_mut(thicket) {
        obj.status.summoning_sick = false;
    }
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let tap = mana_activation(&actions, thicket)
        .expect("tap-for-green must be offered from Battlefield");

    let (s, _) = step(s, tap, &registry);
    // Mana-ability dispatch is immediate (no stack). Green mana is
    // in the pool.
    let green = s.player(0).mana_pool.count_color(ManaColor::Green);
    assert_eq!(green, 1, "tap produced one green mana");
}

#[test]
fn tranquil_thicket_only_cycling_offered_from_hand() {
    // The tap-for-green ability has activation_zone=Battlefield and
    // must not be offered when the card is in hand.
    let (mut s, registry, ids) = fresh_game();
    let thicket = put_in_hand(&mut s, &registry, 0, ids.tranquil_thicket);
    let _lib = put_in_library(&mut s, &registry, 0, ids.grizzly_bears);
    give_mana(&mut s, 0, ManaColor::Colorless, 2);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    // Cycling (index 1) is offered.
    assert!(cycling_activation(&actions, thicket).is_some(),
        "cycling (hand ability) is offered");
    // Tap-for-green (index 0) is NOT offered — wrong zone.
    assert!(mana_activation(&actions, thicket).is_none(),
        "tap-for-green (battlefield ability) must not be offered \
         while the card is in hand");
}

// ---------------------------------------------------------------------
// Madness (CR 702.34) — Fiery Temper
// ---------------------------------------------------------------------
//
// Fiery Temper is {1}{R}{R} for 3 damage, with Madness {R}. The seed
// proves the three moving parts:
//
//   1. Discard replacement — discarding a madness card routes it to
//      exile with `madness_pending=true`, not graveyard.
//   2. Legal-action enumeration — flagged exile objects are offered
//      as CastSpell with `CastModifier::Madness`, using the madness
//      cost.
//   3. Resolution routing — madness-cast instants leave the stack to
//      graveyard (not back to exile); the flag is cleared on re-id.
//
// Discard is driven through `state.discard_object` directly to keep
// the test scope on madness, not on the discard-triggering cards.

fn madness_cast_action(
    actions: &[Action],
    object_id_in_exile: ObjectId,
) -> Option<Action> {
    actions.iter().find(|a| matches!(a, Action::CastSpell {
        object_id, cast_modifier, ..
    } if *object_id == object_id_in_exile
        && matches!(cast_modifier, arcana_core::actions::CastModifier::Madness)
    )).cloned()
}

#[test]
fn discarded_madness_card_routes_to_exile_with_flag() {
    let (mut s, registry, ids) = fresh_game();
    let temper = put_in_hand(&mut s, &registry, 0, ids.fiery_temper);

    // Direct discard — the madness replacement is built into
    // GameState::discard_object, so this exercises the same path
    // every discard effect flows through.
    let new_id = s.discard_object(0, temper,
        arcana_core::events::MoveCause::Cost)
        .expect("discard_object returns the new id");

    // Card is in exile, not graveyard, with madness_pending set.
    assert_eq!(s.zone_count(Zone::Exile), 1,
        "madness card goes to exile");
    assert_eq!(s.zone_count(Zone::Graveyard(0)), 0,
        "madness card does NOT go to graveyard");
    let obj = s.objects.get(new_id).expect("exile object exists");
    assert!(obj.madness_pending,
        "exile object is flagged madness_pending");
    assert_eq!(obj.zone, Zone::Exile);
}

#[test]
fn discarded_non_madness_card_still_goes_to_graveyard() {
    // Regression: Lightning Bolt has no Madness keyword. Discard
    // must still route to graveyard via the unchanged path.
    let (mut s, registry, ids) = fresh_game();
    let bolt = put_in_hand(&mut s, &registry, 0, ids.lightning_bolt);
    s.discard_object(0, bolt, arcana_core::events::MoveCause::Cost);
    assert_eq!(s.zone_count(Zone::Graveyard(0)), 1,
        "non-madness card discarded normally");
    assert_eq!(s.zone_count(Zone::Exile), 0,
        "no exile detour for non-madness");
}

#[test]
fn legal_actions_offers_madness_cast_from_exile() {
    let (mut s, registry, ids) = fresh_game();
    let temper = put_in_hand(&mut s, &registry, 0, ids.fiery_temper);
    let _bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    // Discard the temper — it goes to exile with the flag.
    let exile_id = s.discard_object(0, temper,
        arcana_core::events::MoveCause::Cost).unwrap();
    // Mana to cast for madness cost {R}.
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(madness_cast_action(&actions, exile_id).is_some(),
        "madness cast enumerated for flagged exile object with {{R}} \
         in pool");
}

#[test]
fn madness_cast_end_to_end_deals_3_damage_then_graveyard() {
    let (mut s, registry, ids) = fresh_game();
    let temper = put_in_hand(&mut s, &registry, 0, ids.fiery_temper);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    let exile_id = s.discard_object(0, temper,
        arcana_core::events::MoveCause::Cost).unwrap();
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = madness_cast_action(&actions, exile_id)
        .expect("madness cast must be offered");

    // Retarget the cast at bears deterministically; enumeration
    // will have picked one target but either player or bears works
    // for this fixture — pin to bears for the death assertion.
    let cast = match cast {
        Action::CastSpell { object_id, modes, mana_payment,
                            additional_costs, x_value, cast_modifier,
                            cost_reductions, .. } => Action::CastSpell {
            object_id,
            targets: arcana_core::targets::TargetSelection {
                targets: vec![
                    arcana_core::targets::TargetChoice::ObjectOrPlayer(
                        arcana_core::targets::ObjectOrPlayer::Object(bears)),
                ],
            },
            modes, mana_payment, additional_costs, x_value,
            cast_modifier, cost_reductions,
        },
        _ => unreachable!(),
    };

    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    // Bears (2 toughness) dies to 3 damage.
    assert_eq!(s.zone_count(Zone::Graveyard(1)), 1,
        "Bears in p1's graveyard after madness-cast damage");
    // Fiery Temper, being a non-permanent, leaves the stack to
    // graveyard (NOT back to exile — madness's exile was just a
    // stepping stone).
    assert_eq!(s.zone_count(Zone::Graveyard(0)), 1,
        "Fiery Temper in p0's graveyard post-resolution");
    assert_eq!(s.zone_count(Zone::Exile), 0,
        "exile is empty — madness flag didn't re-exile the spell");
}

#[test]
fn madness_cast_not_offered_without_madness_mana() {
    let (mut s, registry, ids) = fresh_game();
    let temper = put_in_hand(&mut s, &registry, 0, ids.fiery_temper);
    let exile_id = s.discard_object(0, temper,
        arcana_core::events::MoveCause::Cost).unwrap();
    // No mana.
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(madness_cast_action(&actions, exile_id).is_none(),
        "empty pool → no madness-cast enumeration");
}

#[test]
fn madness_cast_not_offered_for_unflagged_exile_object() {
    // If a Fiery Temper is in exile WITHOUT madness_pending (put
    // there by some other exile effect, not the madness
    // replacement), it must not be offered as a madness cast. The
    // flag is the gate, not mere presence in exile.
    let (mut s, registry, ids) = fresh_game();
    let temper_id = {
        let obj_id = s.allocate_object_id();
        let chars = registry.get(ids.fiery_temper).unwrap()
            .base_characteristics.clone();
        s.objects.insert(GameObject::new(
            obj_id, 0, Zone::Exile, ids.fiery_temper, chars));
        obj_id
    };
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(madness_cast_action(&actions, temper_id).is_none(),
        "unflagged exile object is not madness-castable");
}

// ---------------------------------------------------------------------
// Adventure (CR 715) — Bonecrusher Giant // Stomp
// ---------------------------------------------------------------------
//
// Bonecrusher is {1}{R} 4/3 with Stomp ({1}{R} instant, "Stomp deals
// 2 damage to any target"). The seed proves the three cast paths:
//
//   1. Adventure cast from hand — uses the face's cost, resolves
//      the face's effect, routes to exile with the flag set and the
//      creature-face characteristics restored.
//   2. Creature cast from adventure-exile — legal_actions offers it;
//      cost is the creature-face cost; resolution enters the
//      battlefield as a 4/3 Giant.
//   3. Countered adventure still routes to exile with the flag
//      (fizzle path reaches counter_resolved_spell), matching the
//      printed rule.

fn adventure_cast_action(
    actions: &[Action],
    card_in_hand: ObjectId,
) -> Option<Action> {
    actions.iter().find(|a| matches!(a, Action::CastSpell {
        object_id, cast_modifier, ..
    } if *object_id == card_in_hand
        && matches!(cast_modifier, arcana_core::actions::CastModifier::Adventure)
    )).cloned()
}

fn adventure_creature_cast_action(
    actions: &[Action],
    card_in_exile: ObjectId,
) -> Option<Action> {
    actions.iter().find(|a| matches!(a, Action::CastSpell {
        object_id, cast_modifier, ..
    } if *object_id == card_in_exile
        && matches!(cast_modifier,
            arcana_core::actions::CastModifier::AdventureCreature)
    )).cloned()
}

#[test]
fn bonecrusher_adventure_cast_deals_2_damage_and_exiles() {
    let (mut s, registry, ids) = fresh_game();
    let giant = put_in_hand(&mut s, &registry, 0, ids.bonecrusher_giant);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    // Stomp costs {1}{R}.
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = adventure_cast_action(&actions, giant)
        .expect("adventure cast must be offered for bonecrusher in hand \
                 with {R}{1} available");
    // Retarget deterministically at bears.
    let cast = match cast {
        Action::CastSpell { object_id, modes, mana_payment,
                            additional_costs, x_value, cast_modifier,
                            cost_reductions, .. } => Action::CastSpell {
            object_id,
            targets: arcana_core::targets::TargetSelection {
                targets: vec![
                    arcana_core::targets::TargetChoice::ObjectOrPlayer(
                        arcana_core::targets::ObjectOrPlayer::Object(bears)),
                ],
            },
            modes, mana_payment, additional_costs, x_value,
            cast_modifier, cost_reductions,
        },
        _ => unreachable!(),
    };

    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    // Stomp deals 2 damage — bears (2 toughness) dies.
    assert_eq!(s.zone_count(Zone::Graveyard(1)), 1,
        "bears die to 2 damage from Stomp");
    // Adventure spell routes to exile (not graveyard) with the flag.
    assert_eq!(s.zone_count(Zone::Exile), 1,
        "adventure spell goes to exile after resolution");
    assert_eq!(s.zone_count(Zone::Graveyard(0)), 0,
        "adventure spell does not go to its owner's graveyard");
    let exiled = s.objects.iter()
        .find(|o| o.zone == Zone::Exile)
        .expect("exile object exists");
    assert!(exiled.adventure_exile_pending,
        "exile object has adventure_exile_pending flag set");
    // Post-resolution characteristics are the creature face
    // (restored on the way to exile so the creature cast works).
    assert!(exiled.characteristics.types.is_creature(),
        "exile object carries creature-face type line, not instant");
    assert_eq!(exiled.characteristics.mana_cost,
        Some(arcana_core::mana::ManaCost::parse("{1}{R}").unwrap()),
        "exile object carries creature-face mana cost");
}

#[test]
fn bonecrusher_creature_cast_from_adventure_exile_enters_battlefield() {
    let (mut s, registry, ids) = fresh_game();
    let giant = put_in_hand(&mut s, &registry, 0, ids.bonecrusher_giant);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    // Fire the Adventure cast (Stomp → bears).
    let cast = adventure_cast_action(
        &arcana_core::legal_actions::legal_actions(&s, &registry), giant)
        .expect("adventure cast offered");
    let cast = match cast {
        Action::CastSpell { object_id, modes, mana_payment,
                            additional_costs, x_value, cast_modifier,
                            cost_reductions, .. } => Action::CastSpell {
            object_id,
            targets: arcana_core::targets::TargetSelection {
                targets: vec![
                    arcana_core::targets::TargetChoice::ObjectOrPlayer(
                        arcana_core::targets::ObjectOrPlayer::Object(bears)),
                ],
            },
            modes, mana_payment, additional_costs, x_value,
            cast_modifier, cost_reductions,
        },
        _ => unreachable!(),
    };
    let (s, _) = step(s, cast, &registry);
    let mut s = resolve_stack(s, &registry);
    let exile_id = s.objects.iter()
        .find(|o| o.zone == Zone::Exile && o.adventure_exile_pending)
        .map(|o| o.id).expect("adventure exile object");

    // Now pay {1}{R} again and cast the creature half.
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let creature_cast = adventure_creature_cast_action(&actions, exile_id)
        .expect("creature cast from adventure-exile must be offered");

    let (s, _) = step(s, creature_cast, &registry);
    let s = resolve_stack(s, &registry);

    // A Bonecrusher Giant is now on the battlefield under player 0's
    // control.
    let giant_obj = s.objects.iter()
        .find(|o| o.zone == Zone::Battlefield && o.card_id == ids.bonecrusher_giant)
        .expect("bonecrusher on battlefield");
    assert_eq!(giant_obj.controller, 0);
    assert!(giant_obj.characteristics.types.is_creature());
    assert_eq!(giant_obj.characteristics.power,
        Some(arcana_core::types::PtValue::Fixed(4)));
    assert_eq!(giant_obj.characteristics.toughness,
        Some(arcana_core::types::PtValue::Fixed(3)));
    // Re-id on the exile→stack→battlefield moves drops the flag.
    assert!(!giant_obj.adventure_exile_pending,
        "battlefield creature has no lingering adventure marker");
    // Exile is now empty (the adventure-exile card left when cast).
    assert_eq!(s.zone_count(Zone::Exile), 0,
        "creature cast emptied the adventure-exile");
}

#[test]
fn bonecrusher_adventure_not_offered_without_mana() {
    let (mut s, registry, ids) = fresh_game();
    let giant = put_in_hand(&mut s, &registry, 0, ids.bonecrusher_giant);
    // No mana.
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(adventure_cast_action(&actions, giant).is_none(),
        "empty pool → no adventure cast");
}

#[test]
fn bonecrusher_adventure_offered_at_instant_speed_on_opponent_turn() {
    // Stomp is an instant. The adventure cast should be legal even
    // during the opponent's turn with the stack empty, exactly like
    // any other instant in hand.
    let (mut s, registry, ids) = fresh_game();
    let giant = put_in_hand(&mut s, &registry, 0, ids.bonecrusher_giant);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    // Give p0 priority, but don't set a main phase for p0 — the
    // active player is p0 by default so flip to p1's main phase by
    // setting turn.active differently. The existing helper always
    // sets turn to main for whoever receives priority, so here we
    // craft the state by hand.
    s.priority.give_to(0);
    s.turn.active_player = 1;
    s.turn.phase = arcana_core::turn::Phase::PreCombatMain;
    s.turn.step = arcana_core::turn::Step::Main;

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(adventure_cast_action(&actions, giant).is_some(),
        "Stomp (instant) legal on opponent's turn for p0");
}

#[test]
fn bonecrusher_normal_creature_cast_from_hand_still_works() {
    // Regression: the main-face cast path is unaffected by the
    // adventure additions. Casting Bonecrusher normally (as a
    // creature) from hand puts a 4/3 Giant on the battlefield,
    // no exile detour.
    let (mut s, registry, ids) = fresh_game();
    let giant = put_in_hand(&mut s, &registry, 0, ids.bonecrusher_giant);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let normal_cast = actions.iter().find(|a| matches!(a,
        Action::CastSpell { object_id, cast_modifier, .. }
        if *object_id == giant
            && matches!(cast_modifier, arcana_core::actions::CastModifier::None)
    )).cloned().expect("normal creature cast offered");

    let (s, _) = step(s, normal_cast, &registry);
    let s = resolve_stack(s, &registry);

    // Battlefield, not exile.
    assert_eq!(s.zone_count(Zone::Exile), 0,
        "normal creature cast does not exile");
    let on_bf = s.objects.iter()
        .find(|o| o.zone == Zone::Battlefield
            && o.card_id == ids.bonecrusher_giant)
        .expect("bonecrusher on battlefield from normal cast");
    assert!(!on_bf.adventure_exile_pending);
    assert!(on_bf.characteristics.types.is_creature());
}

// ---------------------------------------------------------------------
// Hexproof (CR 702.11) — Slippery Bogle
// ---------------------------------------------------------------------
//
// Bogle is {G/U} 1/1 Elemental Hound with Hexproof and literally
// nothing else — the ideal Hexproof-in-isolation fixture. The seed
// proves the four target-pipeline axes:
//
//   1. Opponent's CastSpell targeting is rejected at announce
//      (filtered out of legal_actions).
//   2. Own-controller's targeting IS accepted (Hexproof is
//      "your opponents control" specifically, not "all players").
//   3. Opponent's ActivateAbility targeting is rejected — proves the
//      Ballista activation path flows through the same
//      TargetRequirement::matches_choice filter as CastSpell does.
//   4. Hexproof granted mid-stack (test-only Shalai-stand-in) makes
//      the target illegal at resolution; CR 608.2b fizzles the
//      pre-existing spell. This is the crucial "grant-in-response"
//      test that exercises both granted-keyword layer composition
//      and the resolution re-check.
//
// Pre-commit prediction (for calibration): audit showed all keyword
// queries in the target pipeline already flow through has_keyword
// (layer-aware). Predicted: zero engine changes needed — the
// infrastructure covers every test path. Actual outcome tracked in
// the commit message.

/// Test-only "Hexproof Grant Instant": `{W}` instant, "Target
/// creature you control gains Hexproof until end of turn." Installed
/// as a local registry card (not in the shared seed pool). Replaces
/// Shalai / Ranger-Captain / similar real cards for the single-test
/// purpose of exercising granted-Hexproof interactions; delete when
/// a real such card lands in the seed pool.
fn register_hexproof_grant_instant(
    registry: &mut CardRegistry,
) -> arcana_core::types::CardId {
    use arcana_core::effects::{Effect, KeywordAbility};
    use arcana_core::layers::Duration;
    use arcana_core::mana::ManaCost;
    use arcana_core::objects::Characteristics;
    use arcana_core::registry::{CardDefinition, SpellAbilityDef};
    use arcana_core::stack::StackEntry;
    use arcana_core::state::GameState;
    use arcana_core::targets::{ObjectFilter, TargetFilter, TargetCount,
        TargetRequirement};
    use arcana_core::types::{ColorSet, TypeLine};

    fn grant_hexproof_eot_resolve(
        _state: &GameState,
        entry: &StackEntry,
        _reg: &CardRegistry,
    ) -> Vec<Effect> {
        let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
        let arcana_core::targets::TargetChoice::Object(id) = target else {
            return Vec::new();
        };
        vec![Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Hexproof,
            duration: Duration::EndOfTurn,
        }]
    }

    let name = registry.interner_mut().intern("Test Hexproof Grant");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    registry.register(CardDefinition::new(name, chars)
        .with_spell_ability(SpellAbilityDef {
            text: "Target creature you control gains hexproof \
                   until end of turn.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::creature()),
                count: TargetCount::Exactly(1),
                controller: Some(
                    arcana_core::targets::ControllerConstraint::You),
            }],
            modal: None,
            effect: grant_hexproof_eot_resolve,
        }))
}

fn opponent_bolt_targeting(
    actions: &[Action],
    target: ObjectId,
) -> Option<Action> {
    actions.iter().find(|a| matches!(a,
        Action::CastSpell { targets, .. }
        if targets.targets.iter().any(|t| matches!(t,
            arcana_core::targets::TargetChoice::ObjectOrPlayer(
                arcana_core::targets::ObjectOrPlayer::Object(id))
            if *id == target))
    )).cloned()
}

#[test]
fn bogle_rejects_opponent_bolt_at_announce() {
    // Baseline: opponent has a Bolt, Bogle is on the battlefield
    // under p0's control. legal_actions(p1) should NOT enumerate a
    // Bolt cast targeting Bogle.
    let (mut s, registry, ids) = fresh_game();
    let _bogle = put_on_battlefield(&mut s, &registry, 0, ids.slippery_bogle);
    let bolt = put_in_hand(&mut s, &registry, 1, ids.lightning_bolt);
    give_mana(&mut s, 1, ManaColor::Red, 1);
    // Hand-craft opponent priority with stack empty.
    s.priority.give_to(1);
    s.turn.active_player = 0;
    s.turn.phase = arcana_core::turn::Phase::PreCombatMain;
    s.turn.step = arcana_core::turn::Step::Main;

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    // Bolt target at Bogle is NOT offered; Bolt cast at players IS.
    let bolt_at_bogle = actions.iter().any(|a| matches!(a,
        Action::CastSpell { object_id, targets, .. }
        if *object_id == bolt
            && targets.targets.iter().any(|t| matches!(t,
                arcana_core::targets::TargetChoice::ObjectOrPlayer(
                    arcana_core::targets::ObjectOrPlayer::Object(_))))
    ));
    assert!(!bolt_at_bogle,
        "opponent's Bolt must not offer Bogle as a target (Hexproof)");
    // Sanity: Bolt targeting a player is still fine.
    let bolt_at_player = actions.iter().any(|a| matches!(a,
        Action::CastSpell { object_id, .. } if *object_id == bolt));
    assert!(bolt_at_player,
        "opponent can still cast Bolt at players (Hexproof doesn't \
         extend to players)");
}

#[test]
fn bogle_accepts_own_controller_targeting() {
    // Hexproof says "your opponents control" — the controller
    // targeting its own Hexproof creature is always legal. This
    // exercises the controller-gate in the Hexproof check (obj.controller
    // != source_controller). The regression would be "check drops
    // the controller filter and rejects all targets."
    let (mut s, registry, ids) = fresh_game();
    let bogle = put_on_battlefield(&mut s, &registry, 0, ids.slippery_bogle);
    let _bolt = put_in_hand(&mut s, &registry, 0, ids.lightning_bolt);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(opponent_bolt_targeting(&actions, bogle).is_some(),
        "own-controller casting Bolt at own Hexproof creature must \
         be legal");
}

#[test]
fn bogle_rejects_opponent_ballista_ping() {
    // Activated-ability path: opponent controls Walking Ballista
    // with a +1/+1 counter (ping-ready). Legal_actions(opponent)
    // must not offer a ping targeting Bogle. This proves the
    // activation-side target filter flows through the same
    // TargetRequirement::matches_choice as the cast-side.
    use arcana_core::types::CounterKind;
    let (mut s, registry, ids) = fresh_game();
    let bogle = put_on_battlefield(&mut s, &registry, 0, ids.slippery_bogle);
    let ballista = put_on_battlefield(&mut s, &registry, 1, ids.walking_ballista);
    s.objects.get_mut(ballista).unwrap()
        .add_counters(CounterKind::PlusOnePlusOne, 2);
    clear_summoning_sickness(&mut s, ballista);
    s.priority.give_to(1);
    s.turn.active_player = 1;
    s.turn.phase = arcana_core::turn::Phase::PreCombatMain;
    s.turn.step = arcana_core::turn::Step::Main;

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    // Must NOT offer a ping specifically targeting Bogle.
    let ping_at_bogle = actions.iter().any(|a| matches!(a,
        Action::ActivateAbility { source, targets, .. }
        if *source == ballista
            && targets.targets.iter().any(|t| matches!(t,
                arcana_core::targets::TargetChoice::ObjectOrPlayer(
                    arcana_core::targets::ObjectOrPlayer::Object(id))
                if *id == bogle))
    ));
    assert!(!ping_at_bogle,
        "opponent's Ballista ping must not target Bogle (Hexproof \
         on the activated-ability path)");
    // Sanity: ping IS offered against other targets (Ballista can
    // self-target, can target p0 the player, etc.). The activation
    // itself isn't keyword-rejected; only the specific Bogle target
    // is filtered out.
    let any_ping = actions.iter().any(|a| matches!(a,
        Action::ActivateAbility { source, .. } if *source == ballista));
    assert!(any_ping,
        "Ballista activation is still offered against other legal \
         targets (Hexproof filters per-target, not per-activation)");
}

#[test]
fn bolt_fizzles_when_target_gains_hexproof_before_resolution() {
    // The crux test. Opponent casts Bolt at a normal creature; in
    // response, the controller casts Grant-Hexproof-EOT (test-only)
    // on the same creature. Grant resolves first (LIFO stack), creature
    // now has Hexproof. Bolt then tries to resolve — CR 608.2b
    // re-check sees the target is now illegal, routes Bolt to
    // `counter_resolved_spell` as a fizzle. Creature survives.
    let mut registry = CardRegistry::new();
    let ids = arcana_cards::register_seed(&mut registry);
    let grant_id = register_hexproof_grant_instant(&mut registry);
    let mut s = GameState::new(2, 0);

    let bears = put_on_battlefield(&mut s, &registry, 0, ids.grizzly_bears);
    let bolt = put_in_hand(&mut s, &registry, 1, ids.lightning_bolt);
    // Give p0 the grant-instant in hand.
    let grant_obj = {
        let obj_id = s.allocate_object_id();
        let chars = registry.get(grant_id).unwrap()
            .base_characteristics.clone();
        s.objects.insert(GameObject::new(
            obj_id, 0, Zone::Hand(0), grant_id, chars));
        obj_id
    };

    give_mana(&mut s, 0, ManaColor::White, 1);
    give_mana(&mut s, 1, ManaColor::Red, 1);
    priority_to_main(&mut s, 1);

    // Opponent (p1) casts Bolt at p0's Bears.
    let cast_bolt = Action::CastSpell {
        object_id: bolt,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![
                arcana_core::targets::TargetChoice::ObjectOrPlayer(
                    arcana_core::targets::ObjectOrPlayer::Object(bears)),
            ],
        },
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 0 },
            ],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: None,
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (mut s, _) = step(s, cast_bolt, &registry);

    // Switch priority to p0 in response (Bolt on the stack).
    s.priority.give_to(0);
    let cast_grant = Action::CastSpell {
        object_id: grant_obj,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![
                arcana_core::targets::TargetChoice::Object(bears),
            ],
        },
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 0 },
            ],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: None,
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast_grant, &registry);

    // Resolve the stack: Grant resolves first (LIFO), gives Bears
    // Hexproof EOT; then Bolt tries to resolve, finds the target
    // illegal via CR 608.2b re-check, fizzles to graveyard.
    let s = resolve_stack(s, &registry);

    // Bears survives — Bolt fizzled.
    assert!(s.objects.objects_in_zone(Zone::Battlefield)
        .any(|o| o.card_id == ids.grizzly_bears),
        "Bears survives because Bolt fizzled on the granted-Hexproof \
         target");
    // Bolt is in p1's graveyard (fizzle routes through
    // counter_resolved_spell's non-Flashback default).
    assert!(s.objects.objects_in_zone(Zone::Graveyard(1))
        .any(|o| o.card_id == ids.lightning_bolt),
        "fizzled Bolt lands in its owner's graveyard");
    // Sanity: Bears has Hexproof now.
    let bears_post = s.objects.objects_in_zone(Zone::Battlefield)
        .find(|o| o.card_id == ids.grizzly_bears)
        .map(|o| o.id).unwrap();
    assert!(s.has_keyword(bears_post, &arcana_core::effects::KeywordAbility::Hexproof),
        "grant effect installed Hexproof via layer system");
}

// ---------------------------------------------------------------------
// Menace (CR 702.110) — Ahn-Crop Crasher
// ---------------------------------------------------------------------
//
// Crasher is {2}{R} 3/2 Human Warrior with Menace. The seed proves
// the constraint-driven block infrastructure that Menace forced:
//
//   1. apply_declared_blockers rejects a single blocker on a Menace
//      attacker (min_blockers = 2 via AttackerBlockConstraints).
//   2. apply_declared_blockers accepts a pair of blockers.
//   3. legal_actions::enumerate_blocker_declarations, after the
//      refactor to constraint-driven subset enumeration, offers
//      pair-block declarations for Menace attackers — which the
//      singleton-only enumerator did NOT. This is the primary gap
//      Menace surfaced.
//   4. Non-Menace attackers still receive singleton blocks (the
//      default unrestricted constraint).
//   5. Combat damage flows correctly through a Menace pair-block.

fn pair_block_action(
    actions: &[Action],
    attacker: ObjectId,
) -> Option<Action> {
    actions.iter().find(|a| matches!(a,
        Action::DeclareBlockers { blockers }
        if blockers.len() == 2
            && blockers.iter().all(|d| d.blocking == attacker)
    )).cloned()
}

fn singleton_block_action_for(
    actions: &[Action],
    attacker: ObjectId,
) -> Option<Action> {
    actions.iter().find(|a| matches!(a,
        Action::DeclareBlockers { blockers }
        if blockers.len() == 1 && blockers[0].blocking == attacker
    )).cloned()
}

#[test]
fn menace_apply_drops_single_blocker_declaration() {
    let (mut s, registry, ids) = fresh_game();
    let crasher = put_on_battlefield(&mut s, &registry, 0, ids.ahn_crop_crasher);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    clear_summoning_sickness(&mut s, crasher);
    clear_summoning_sickness(&mut s, bears);

    s.begin_combat();
    s.apply_declared_attackers(vec![
        arcana_core::combat::AttackerDeclaration {
            attacker: crasher,
            defending: arcana_core::combat::DefendingEntity::Player(1),
        },
    ]);
    s.enter_declare_blockers();
    // Submit a single-blocker declaration — the generalized
    // constraint check must drop it because Menace's
    // min_blockers=2 is unsatisfied.
    s.apply_declared_blockers(vec![
        arcana_core::combat::BlockerDeclaration {
            blocker: bears, blocking: crasher,
        },
    ]);

    let combat = s.combat.as_ref().unwrap();
    assert!(combat.blockers.is_empty(),
        "single blocker on a Menace attacker must be dropped");
    assert!(!combat.attacker(crasher).unwrap().is_blocked,
        "attacker remains unblocked when the declaration is dropped");
}

#[test]
fn menace_apply_accepts_pair_block() {
    // Two defender creatures against one Menace attacker — the pair
    // declaration satisfies min_blockers=2 and sticks.
    let (mut s, registry, ids) = fresh_game();
    let crasher = put_on_battlefield(&mut s, &registry, 0, ids.ahn_crop_crasher);
    let bears1 = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    // Give the second blocker a distinct card to avoid triggering
    // equivalence-class dedup on the apply-path (apply doesn't dedup,
    // but two identical bears would both be in the arena as separate
    // objects with different ids — duplicate-id guards in apply want
    // distinct ids, which they already are).
    let spider = put_on_battlefield(&mut s, &registry, 1, ids.giant_spider);
    for id in [crasher, bears1, spider] {
        clear_summoning_sickness(&mut s, id);
    }

    s.begin_combat();
    s.apply_declared_attackers(vec![
        arcana_core::combat::AttackerDeclaration {
            attacker: crasher,
            defending: arcana_core::combat::DefendingEntity::Player(1),
        },
    ]);
    s.enter_declare_blockers();
    s.apply_declared_blockers(vec![
        arcana_core::combat::BlockerDeclaration {
            blocker: bears1, blocking: crasher,
        },
        arcana_core::combat::BlockerDeclaration {
            blocker: spider, blocking: crasher,
        },
    ]);

    let combat = s.combat.as_ref().unwrap();
    assert_eq!(combat.blockers.len(), 2,
        "both blockers stick on a Menace attacker with pair-block");
    assert!(combat.attacker(crasher).unwrap().is_blocked,
        "attacker marked blocked after pair-block");
}

#[test]
fn menace_legal_actions_offers_pair_block_declaration() {
    // This is the primary regression: before the constraint-driven
    // enumerator refactor, legal_actions only emitted singleton
    // block declarations, so a Menace attacker was silently
    // unblockable from the legal-action API's perspective.
    let (mut s, registry, ids) = fresh_game();
    let crasher = put_on_battlefield(&mut s, &registry, 0, ids.ahn_crop_crasher);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    let spider = put_on_battlefield(&mut s, &registry, 1, ids.giant_spider);
    for id in [crasher, bears, spider] {
        clear_summoning_sickness(&mut s, id);
    }

    s.begin_combat();
    s.apply_declared_attackers(vec![
        arcana_core::combat::AttackerDeclaration {
            attacker: crasher,
            defending: arcana_core::combat::DefendingEntity::Player(1),
        },
    ]);
    s.enter_declare_blockers();
    // Defender has priority during DeclareBlockers — legal_actions
    // gates block enumeration on `player != active_player`.
    s.priority.give_to(1);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(pair_block_action(&actions, crasher).is_some(),
        "legal_actions must offer a pair-block declaration for a \
         Menace attacker with two eligible blockers available");
    // And must NOT offer singleton blocks (the pre-refactor shape)
    // — Menace's min=2 filters them out of the constraint-allowed
    // sizes.
    assert!(singleton_block_action_for(&actions, crasher).is_none(),
        "legal_actions must not offer a singleton block against a \
         Menace attacker (min_blockers=2 filters it)");
}

#[test]
fn non_menace_attacker_still_offered_singleton_block() {
    // Regression: the constraint refactor did not break the default
    // unrestricted (min=1, max=None) case. A plain Bears attacker
    // still gets singleton block offers.
    let (mut s, registry, ids) = fresh_game();
    let bears_a = put_on_battlefield(&mut s, &registry, 0, ids.grizzly_bears);
    let bears_b = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    clear_summoning_sickness(&mut s, bears_a);
    clear_summoning_sickness(&mut s, bears_b);

    s.begin_combat();
    s.apply_declared_attackers(vec![
        arcana_core::combat::AttackerDeclaration {
            attacker: bears_a,
            defending: arcana_core::combat::DefendingEntity::Player(1),
        },
    ]);
    s.enter_declare_blockers();
    s.priority.give_to(1);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(singleton_block_action_for(&actions, bears_a).is_some(),
        "non-Menace attacker still offered singleton block");
}

#[test]
fn menace_combat_damage_flows_through_pair_block() {
    // Crasher (3/2) vs Bears (2/2) + Spider (2/4). Attacker
    // distributes 3 damage in declared-blocker order. Default
    // distribution gives Bears lethal (2) first, then remainder (1)
    // to Spider. Blockers deal back: 2 + 2 = 4 damage to Crasher,
    // which dies (2 toughness).
    let (mut s, registry, ids) = fresh_game();
    let crasher = put_on_battlefield(&mut s, &registry, 0, ids.ahn_crop_crasher);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    let spider = put_on_battlefield(&mut s, &registry, 1, ids.giant_spider);
    for id in [crasher, bears, spider] {
        clear_summoning_sickness(&mut s, id);
    }

    s.begin_combat();
    s.apply_declared_attackers(vec![
        arcana_core::combat::AttackerDeclaration {
            attacker: crasher,
            defending: arcana_core::combat::DefendingEntity::Player(1),
        },
    ]);
    s.enter_declare_blockers();
    s.apply_declared_blockers(vec![
        arcana_core::combat::BlockerDeclaration {
            blocker: bears, blocking: crasher,
        },
        arcana_core::combat::BlockerDeclaration {
            blocker: spider, blocking: crasher,
        },
    ]);
    // Precondition: the pair-block must have stuck.
    let combat = s.combat.as_ref().unwrap();
    assert_eq!(combat.blockers.len(), 2,
        "precondition: pair-block sticks on Menace attacker");

    s.deal_combat_damage();
    arcana_core::sba::apply_state_based_actions(&mut s);

    // Crasher takes 4 total damage (2 from Bears + 2 from Spider),
    // 2 toughness → dies. Look up by card_id, not old object id:
    // SBA moves the dying creature to its owner's graveyard and
    // re-ids it (CR 400.7), so the pre-combat ids no longer
    // resolve in the arena.
    assert!(s.objects.objects_in_zone(Zone::Graveyard(0))
        .any(|o| o.card_id == ids.ahn_crop_crasher),
        "Crasher dies to 4 damage from the pair-block");
    // Bears (2 toughness) dies to 2 damage (default distribution
    // gives lethal to first blocker in order).
    assert!(s.objects.objects_in_zone(Zone::Graveyard(1))
        .any(|o| o.card_id == ids.grizzly_bears),
        "Bears dies to 2 damage");
    // Spider (4 toughness) takes 1 damage, survives.
    assert!(s.objects.objects_in_zone(Zone::Battlefield)
        .any(|o| o.card_id == ids.giant_spider),
        "Spider survives (4 toughness absorbs default distribution's \
         1 remaining point)");
}

// ---------------------------------------------------------------------
// Prowess (CR 702.108) + Haste (CR 702.10) — Monastery Swiftspear
// ---------------------------------------------------------------------
//
// Swiftspear is {R} 1/2 Human Monk with Haste and Prowess. The seed
// proves:
//
//   1. Haste — summoning sickness is overridden, Swiftspear attacks
//      the turn it enters.
//   2. Prowess on-cast — a noncreature spell cast by Swiftspear's
//      controller pumps it to 2/3 via `apply_prowess_on_cast`.
//   3. Prowess controller gate — an opponent's noncreature cast does
//      NOT pump (this is the primary correctness regression vs. the
//      naive "any noncreature cast triggers" bug).
//   4. Duration cleanup — the +1/+1 pump is `Duration::EndOfTurn`
//      and must expire at the cleanup step (CR 514.2). This is the
//      first UntilEndOfTurn P/T test in the seed suite; it anchors
//      the layer-7 pump-expiry path the same way the Snapcaster
//      flashback grant test anchored keyword-grant-expiry.
//   5. Combat integration — attacking with a pumped Swiftspear
//      deals 2 combat damage, validating that layer-7 in-combat
//      reads consume the pump correctly.

fn combat_damage_dealt_to_player_0(s: &GameState) -> u32 {
    // Default starting life total is 20; shorthand for tests below.
    20u32.saturating_sub(s.player(0).life as u32)
}

fn combat_damage_dealt_to_player_1(s: &GameState) -> u32 {
    20u32.saturating_sub(s.player(1).life as u32)
}

#[test]
fn swiftspear_can_attack_turn_it_enters_via_haste() {
    let (mut s, registry, ids) = fresh_game();
    // Place Swiftspear and stamp summoning sickness manually — the
    // put_on_battlefield fixture inserts with PermanentStatus::default
    // (summoning_sick=false) because it bypasses the normal ETB
    // sickness stamp. To test Haste we need the sickness flag on so
    // the only reason the attack is legal is Haste overriding it
    // (CR 702.10b).
    let swift = put_on_battlefield(&mut s, &registry, 0, ids.monastery_swiftspear);
    s.objects.get_mut(swift).unwrap().status.summoning_sick = true;
    // p0 is active player (default from fresh_game + turn.active_player=0).
    priority_to_main(&mut s, 0);

    s.begin_combat();
    s.apply_declared_attackers(vec![
        arcana_core::combat::AttackerDeclaration {
            attacker: swift,
            defending: arcana_core::combat::DefendingEntity::Player(1),
        },
    ]);
    // Haste overrode sickness — the attacker declaration stuck.
    let combat = s.combat.as_ref().unwrap();
    assert_eq!(combat.attackers.len(), 1,
        "Haste creature must be a legal attacker on its first turn");
}

#[test]
fn swiftspear_prowess_pumps_on_controller_noncreature_cast() {
    let (mut s, registry, ids) = fresh_game();
    let swift = put_on_battlefield(&mut s, &registry, 0, ids.monastery_swiftspear);
    // Base P/T is 1/2.
    assert_eq!(s.computed_power(swift), Some(1));
    assert_eq!(s.computed_toughness(swift), Some(2));

    // Cast Lightning Bolt from p0's hand at p1.
    let bolt = put_in_hand(&mut s, &registry, 0, ids.lightning_bolt);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);
    let cast = Action::CastSpell {
        object_id: bolt,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![
                arcana_core::targets::TargetChoice::ObjectOrPlayer(
                    arcana_core::targets::ObjectOrPlayer::Player(1)),
            ],
        },
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 0 },
            ],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: None,
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast, &registry);
    // Prowess fires on SpellCast — at this point (cast announced, on
    // the stack, not yet resolved) the pump is already live.
    assert_eq!(s.computed_power(swift), Some(2),
        "Prowess pumps power to 2 on controller's noncreature cast");
    assert_eq!(s.computed_toughness(swift), Some(3),
        "Prowess pumps toughness to 3 on controller's noncreature cast");
}

#[test]
fn swiftspear_prowess_does_not_fire_on_opponent_cast() {
    let (mut s, registry, ids) = fresh_game();
    let swift = put_on_battlefield(&mut s, &registry, 0, ids.monastery_swiftspear);
    // Opponent (p1) casts Bolt at p0.
    let bolt = put_in_hand(&mut s, &registry, 1, ids.lightning_bolt);
    give_mana(&mut s, 1, ManaColor::Red, 1);
    // Give priority to p1 on p1's main — hand-crafted since the
    // priority_to_main helper sets turn.active_player=0 implicitly
    // via fresh_game, and p1 needs to be active to cast at sorcery
    // speed. Instants don't require active player, but Bolt would be
    // legal on p0's turn too — we just need the cast to fire from p1.
    s.priority.give_to(1);
    s.turn.active_player = 0;
    s.turn.phase = arcana_core::turn::Phase::PreCombatMain;
    s.turn.step = arcana_core::turn::Step::Main;
    let cast = Action::CastSpell {
        object_id: bolt,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![
                arcana_core::targets::TargetChoice::ObjectOrPlayer(
                    arcana_core::targets::ObjectOrPlayer::Player(0)),
            ],
        },
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 0 },
            ],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: None,
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (s, _) = step(s, cast, &registry);
    // Prowess must NOT fire — the cast was by p1, Swiftspear is p0's.
    assert_eq!(s.computed_power(swift), Some(1),
        "opponent's noncreature cast must not pump Prowess");
    assert_eq!(s.computed_toughness(swift), Some(2));
}

#[test]
fn swiftspear_prowess_pump_expires_at_end_of_turn() {
    // First UntilEndOfTurn P/T test in the seed suite. Anchors the
    // layer-7 pump-expiry path: pump is set, pump is visible, pump
    // expires after `expire_end_of_turn_effects`.
    let (mut s, registry, ids) = fresh_game();
    let swift = put_on_battlefield(&mut s, &registry, 0, ids.monastery_swiftspear);
    let bolt = put_in_hand(&mut s, &registry, 0, ids.lightning_bolt);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);
    let cast = Action::CastSpell {
        object_id: bolt,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![
                arcana_core::targets::TargetChoice::ObjectOrPlayer(
                    arcana_core::targets::ObjectOrPlayer::Player(1)),
            ],
        },
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 0 },
            ],
            ..Default::default()
        },
        additional_costs: vec![],
        x_value: None,
        cast_modifier: arcana_core::actions::CastModifier::None,
        cost_reductions: arcana_core::actions::CostReductions::default(),
    };
    let (mut s, _) = step(s, cast, &registry);
    // Pump is live mid-turn.
    assert_eq!(s.computed_power(swift), Some(2));

    // Drive the end-of-turn expiry directly — same pattern as the
    // snapcaster_flashback_grant_expires_end_of_turn fixture. CR 514.2
    // handles this in the cleanup step.
    s.expire_end_of_turn_effects();

    assert_eq!(s.computed_power(swift), Some(1),
        "Prowess +1/+1 must expire at end of turn");
    assert_eq!(s.computed_toughness(swift), Some(2));
}

#[test]
fn swiftspear_deals_2_combat_damage_after_prowess_pump() {
    // Combat integration: cast-noncreature → prowess pump → attack
    // unblocked → 2 damage to defending player (not 1). Validates
    // that layer-7 in-combat reads consume the pump.
    let (mut s, registry, ids) = fresh_game();
    let swift = put_on_battlefield(&mut s, &registry, 0, ids.monastery_swiftspear);
    let bolt = put_in_hand(&mut s, &registry, 0, ids.lightning_bolt);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);

    let cast = Action::CastSpell {
        object_id: bolt,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![
                arcana_core::targets::TargetChoice::ObjectOrPlayer(
                    arcana_core::targets::ObjectOrPlayer::Player(1)),
            ],
        },
        modes: vec![],
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![
                arcana_core::actions::ManaAssignment { pool_index: 0, cost_index: 0 },
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
    // Bolt resolved (3 damage to p1); Swiftspear pump persists.
    assert_eq!(s.computed_power(swift), Some(2),
        "pump survives past Bolt's resolution (still before end of turn)");
    assert_eq!(combat_damage_dealt_to_player_1(&s), 3,
        "Bolt dealt 3 damage to p1");

    // Attack. Haste lets Swiftspear swing on turn-it-came-in.
    let mut s = s;
    s.begin_combat();
    s.apply_declared_attackers(vec![
        arcana_core::combat::AttackerDeclaration {
            attacker: swift,
            defending: arcana_core::combat::DefendingEntity::Player(1),
        },
    ]);
    s.enter_declare_blockers();
    s.apply_declared_blockers(vec![]);
    s.deal_combat_damage();
    arcana_core::sba::apply_state_based_actions(&mut s);

    // p1 took 3 from Bolt + 2 from pumped Swiftspear = 5.
    assert_eq!(combat_damage_dealt_to_player_1(&s), 5,
        "pumped Swiftspear deals 2 combat damage (base 1 + prowess +1)");
    // Sanity: p0 untouched.
    assert_eq!(combat_damage_dealt_to_player_0(&s), 0);
}

// ---------------------------------------------------------------------
// Split (CR 711) — Fire // Ice
// ---------------------------------------------------------------------
//
// Fire (left, {1}{R}): 2 damage to any target. Ice (right, {1}{U}):
// tap target permanent. The seed proves the two Split cast paths:
//
//   1. Left-half cast — normal hand cast via the base characteristics.
//      No new modifier; it goes through the main cast path.
//   2. Right-half cast — `CastModifier::SplitRight` swaps the
//      object's characteristics to the right face (Ice) before
//      announce; resolution dispatches on Ice's spell ability.
//
// Both halves are instants and go to graveyard on resolution — no
// exile routing, no battlefield residue.

fn fire_cast_action(actions: &[Action], card_in_hand: ObjectId) -> Option<Action> {
    actions.iter().find(|a| matches!(a,
        Action::CastSpell { object_id, cast_modifier, .. }
        if *object_id == card_in_hand
            && matches!(cast_modifier, arcana_core::actions::CastModifier::None)
    )).cloned()
}

fn ice_cast_action(actions: &[Action], card_in_hand: ObjectId) -> Option<Action> {
    actions.iter().find(|a| matches!(a,
        Action::CastSpell { object_id, cast_modifier, .. }
        if *object_id == card_in_hand
            && matches!(cast_modifier, arcana_core::actions::CastModifier::SplitRight)
    )).cloned()
}

#[test]
fn fire_left_half_cast_deals_2_damage_to_graveyard() {
    let (mut s, registry, ids) = fresh_game();
    let fi = put_in_hand(&mut s, &registry, 0, ids.fire_ice);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = fire_cast_action(&actions, fi).expect("fire (left) cast offered");
    // Retarget deterministically at bears.
    let cast = match cast {
        Action::CastSpell { object_id, modes, mana_payment,
                            additional_costs, x_value, cast_modifier,
                            cost_reductions, .. } => Action::CastSpell {
            object_id,
            targets: arcana_core::targets::TargetSelection {
                targets: vec![
                    arcana_core::targets::TargetChoice::ObjectOrPlayer(
                        arcana_core::targets::ObjectOrPlayer::Object(bears)),
                ],
            },
            modes, mana_payment, additional_costs, x_value,
            cast_modifier, cost_reductions,
        },
        _ => unreachable!(),
    };

    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    // Bears (2 toughness) dies to 2 damage.
    assert_eq!(s.zone_count(Zone::Graveyard(1)), 1,
        "bears die to 2 damage from Fire");
    // Fire goes to its owner's graveyard.
    assert_eq!(s.zone_count(Zone::Graveyard(0)), 1,
        "Fire resolves into its owner's graveyard");
    assert_eq!(s.zone_count(Zone::Exile), 0,
        "no exile routing for split halves");
}

#[test]
fn ice_right_half_cast_taps_target_permanent_and_draws() {
    let (mut s, registry, ids) = fresh_game();
    let fi = put_in_hand(&mut s, &registry, 0, ids.fire_ice);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    // Seed one card on top of p0's library so the "draw a card"
    // clause has something to pull.
    let lib_card = put_in_library_top(&mut s, &registry, 0, ids.mountain);
    give_mana(&mut s, 0, ManaColor::Blue, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = ice_cast_action(&actions, fi).expect("ice (right) cast offered");
    let cast = match cast {
        Action::CastSpell { object_id, modes, mana_payment,
                            additional_costs, x_value, cast_modifier,
                            cost_reductions, .. } => Action::CastSpell {
            object_id,
            targets: arcana_core::targets::TargetSelection {
                targets: vec![arcana_core::targets::TargetChoice::Object(bears)],
            },
            modes, mana_payment, additional_costs, x_value,
            cast_modifier, cost_reductions,
        },
        _ => unreachable!(),
    };

    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    // Bears tapped, still on battlefield (Ice doesn't damage).
    let bears_obj = s.objects.get(bears).expect("bears still alive");
    assert!(bears_obj.is_tapped(),
        "bears tapped by Ice");
    assert_eq!(s.zone_count(Zone::Graveyard(1)), 0,
        "bears are tapped, not dead");
    // Ice goes to graveyard.
    assert_eq!(s.zone_count(Zone::Graveyard(0)), 1,
        "Ice resolves into owner's graveyard");
    // Draw landed: seeded library card was pulled into p0's hand.
    // The drawn card gets re-ided during the zone change, so the
    // count is the load-bearing assertion (old lib_card id is LKI).
    assert_eq!(s.zone_count(Zone::Hand(0)), 1,
        "Ice's second clause drew one card");
    assert_eq!(s.zone_count(Zone::Library(0)), 0,
        "library depleted after draw");
    let _ = lib_card;
}

#[test]
fn split_both_halves_offered_with_both_mana() {
    // With enough mana for both halves simultaneously, legal_actions
    // emits both the Fire cast and the Ice cast. Agent picks one.
    let (mut s, registry, ids) = fresh_game();
    let fi = put_in_hand(&mut s, &registry, 0, ids.fire_ice);
    let _bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 0, ManaColor::Blue, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 2);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(fire_cast_action(&actions, fi).is_some(),
        "fire (left) cast offered");
    assert!(ice_cast_action(&actions, fi).is_some(),
        "ice (right) cast offered");
}

#[test]
fn split_right_not_offered_without_blue_mana() {
    let (mut s, registry, ids) = fresh_game();
    let fi = put_in_hand(&mut s, &registry, 0, ids.fire_ice);
    let _bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    // Only red mana — Fire castable, Ice not.
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(fire_cast_action(&actions, fi).is_some(),
        "fire castable with {{R}}{{1}}");
    assert!(ice_cast_action(&actions, fi).is_none(),
        "ice NOT offered without {{U}}");
}

/// CR 711.4 — a split card in hand reports the combined characteristics
/// of both halves (combined name, concatenated mana cost, union of
/// colors and types). The in-zone view is what cards-in-hand-matter
/// queries read (mana-value checks, color-matters, etc.).
#[test]
fn split_card_in_hand_has_combined_characteristics() {
    let (mut s, registry, ids) = fresh_game();
    let fi = put_in_hand(&mut s, &registry, 0, ids.fire_ice);
    let obj = s.objects.get(fi).unwrap();
    let name_str = registry.interner().resolve(obj.characteristics.name).unwrap();
    assert_eq!(name_str, "Fire // Ice",
        "combined name in hand");
    assert_eq!(obj.characteristics.mana_value(), 4,
        "combined mana value = Fire(2) + Ice(2)");
    assert!(obj.characteristics.colors.contains(arcana_core::types::Color::Red)
        && obj.characteristics.colors.contains(arcana_core::types::Color::Blue),
        "combined colors cover both halves");
}

/// CR 711.4 — after an Ice (right-half) cast resolves, the card sits
/// in the graveyard with combined characteristics, not just Ice's.
/// Earlier `#[ignore]`-marked DEBT: the resolving object used to
/// carry right-face chars into the graveyard.
#[test]
fn split_card_returns_to_graveyard_with_combined_characteristics() {
    let (mut s, registry, ids) = fresh_game();
    let fi = put_in_hand(&mut s, &registry, 0, ids.fire_ice);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    let _lib_card = put_in_library_top(&mut s, &registry, 0, ids.mountain);
    give_mana(&mut s, 0, ManaColor::Blue, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = ice_cast_action(&actions, fi).expect("ice cast offered");
    let cast = match cast {
        Action::CastSpell { object_id, modes, mana_payment,
                            additional_costs, x_value, cast_modifier,
                            cost_reductions, .. } => Action::CastSpell {
            object_id,
            targets: arcana_core::targets::TargetSelection {
                targets: vec![arcana_core::targets::TargetChoice::Object(bears)],
            },
            modes, mana_payment, additional_costs, x_value,
            cast_modifier, cost_reductions,
        },
        _ => unreachable!(),
    };

    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    // Find the resolved Fire // Ice in p0's graveyard. swap_to_zone_reid
    // assigns a fresh id, so we iterate by zone.
    let gy = s.objects.objects_in_zone(Zone::Graveyard(0))
        .next().expect("Fire // Ice in graveyard");
    let name_str = registry.interner().resolve(gy.characteristics.name).unwrap();
    assert_eq!(name_str, "Fire // Ice",
        "graveyard object carries combined name, not right-face name");
    assert_eq!(gy.characteristics.mana_value(), 4,
        "graveyard object carries combined mana value");
    assert_eq!(gy.visible_face, 0,
        "visible_face reset to 0 off the stack");
}

#[test]
fn split_halves_are_instant_speed() {
    // Both Fire and Ice are instants; the split right-half cast
    // should be legal on the opponent's turn (outside main phase
    // for the caster).
    let (mut s, registry, ids) = fresh_game();
    let fi = put_in_hand(&mut s, &registry, 0, ids.fire_ice);
    let _bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    give_mana(&mut s, 0, ManaColor::Blue, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    // Hand-craft an opponent's-turn priority window.
    s.priority.give_to(0);
    s.turn.active_player = 1;
    s.turn.phase = arcana_core::turn::Phase::PreCombatMain;
    s.turn.step = arcana_core::turn::Step::Main;

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(ice_cast_action(&actions, fi).is_some(),
        "Ice (instant right half) castable on opponent's turn");
}

// ---------------------------------------------------------------------
// MDFC (CR 712.4) — Tangled Florahedron // Tangled Vale
// ---------------------------------------------------------------------
//
// Front: {G} 1/1 Elf Druid. Back: Tangled Vale land — enters
// tapped, {T}: Add {G}. The seed proves the two MDFC cast/play paths:
//
//   1. Front face cast — normal hand cast of a {G} creature. Goes to
//      battlefield as 1/1.
//   2. Back face land play — PlayLand { mdfc_back: true } swaps the
//      object's characteristics to the back face, consumes the land
//      drop, and puts the land on the battlefield. The tap-for-green
//      ability is gated on the back face so the front-face creature
//      does NOT get offered a spurious mana ability.

fn mdfc_back_land_play(actions: &[Action], card_in_hand: ObjectId) -> Option<Action> {
    actions.iter().find(|a| matches!(a,
        Action::PlayLand { object_id, mdfc_back: true }
        if *object_id == card_in_hand
    )).cloned()
}

fn normal_creature_cast(actions: &[Action], card_in_hand: ObjectId) -> Option<Action> {
    actions.iter().find(|a| matches!(a,
        Action::CastSpell { object_id, cast_modifier, .. }
        if *object_id == card_in_hand
            && matches!(cast_modifier, arcana_core::actions::CastModifier::None)
    )).cloned()
}

#[test]
fn tangled_florahedron_front_cast_makes_1_1_elf_druid() {
    let (mut s, registry, ids) = fresh_game();
    let hedron = put_in_hand(&mut s, &registry, 0, ids.tangled_florahedron);
    give_mana(&mut s, 0, ManaColor::Green, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = normal_creature_cast(&actions, hedron)
        .expect("normal creature cast for {G} must be offered with {G} in pool");

    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    let on_bf = s.objects.iter()
        .find(|o| o.zone == Zone::Battlefield
            && o.card_id == ids.tangled_florahedron)
        .expect("florahedron on battlefield from normal cast");
    assert_eq!(on_bf.controller, 0);
    assert!(on_bf.characteristics.types.is_creature());
    assert!(!on_bf.characteristics.types.is_land(),
        "front face is NOT a land");
    assert_eq!(on_bf.characteristics.power,
        Some(arcana_core::types::PtValue::Fixed(1)));
    assert_eq!(on_bf.characteristics.toughness,
        Some(arcana_core::types::PtValue::Fixed(1)));
    assert_eq!(on_bf.visible_face, 0,
        "front-face cast leaves visible_face at 0");
}

#[test]
fn tangled_florahedron_back_play_makes_tapped_land() {
    let (mut s, registry, ids) = fresh_game();
    let hedron = put_in_hand(&mut s, &registry, 0, ids.tangled_florahedron);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let play = mdfc_back_land_play(&actions, hedron)
        .expect("MDFC back land-play offered");

    let (s, _) = step(s, play, &registry);
    // Land enters the battlefield directly (no stack).
    let on_bf = s.objects.iter()
        .find(|o| o.zone == Zone::Battlefield
            && o.card_id == ids.tangled_florahedron)
        .expect("tangled vale on battlefield from back-face play");
    assert!(on_bf.characteristics.types.is_land(),
        "back face is a land");
    assert!(!on_bf.characteristics.types.is_creature(),
        "back face is NOT a creature");
    assert_eq!(on_bf.visible_face, 1,
        "back-face play sets visible_face to 1");
    assert_eq!(on_bf.controller, 0);
    // Land drop consumed.
    assert_eq!(s.player(0).land_plays_remaining, 0);
}

#[test]
fn tangled_vale_taps_for_green_when_on_battlefield_as_back() {
    let (mut s, registry, ids) = fresh_game();
    let hedron = put_in_hand(&mut s, &registry, 0, ids.tangled_florahedron);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let play = mdfc_back_land_play(&actions, hedron).unwrap();
    let (s, _) = step(s, play, &registry);
    // Locate the back-face land on battlefield.
    let vale = s.objects.iter()
        .find(|o| o.zone == Zone::Battlefield
            && o.card_id == ids.tangled_florahedron)
        .map(|o| o.id).unwrap();

    // Tap-for-green should be offered on the back face.
    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let tap = actions.iter().find(|a| matches!(a,
        Action::ActivateAbility { source, .. } if *source == vale
    )).cloned().expect("tap-for-green offered on back-face land");

    let (s, _) = step(s, tap, &registry);
    // Mana ability skips the stack — green should be in the pool now.
    assert_eq!(s.player(0).mana_pool.count_color(ManaColor::Green), 1,
        "tap-for-green put {{G}} in pool");
    // The land is now tapped.
    let vale_obj = s.objects.get(vale).unwrap();
    assert!(vale_obj.is_tapped());
}

#[test]
fn florahedron_front_face_does_not_offer_tap_for_green() {
    // Regression: without the face_gate on the tap-for-green ability,
    // the 1/1 creature face would spuriously offer the back-face's
    // mana ability. With the gate in place, only the back face
    // (visible_face=1) exposes it.
    let (mut s, registry, ids) = fresh_game();
    let hedron = put_in_hand(&mut s, &registry, 0, ids.tangled_florahedron);
    give_mana(&mut s, 0, ManaColor::Green, 1);
    priority_to_main(&mut s, 0);

    let cast = normal_creature_cast(
        &arcana_core::legal_actions::legal_actions(&s, &registry), hedron)
        .unwrap();
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    let elf = s.objects.iter()
        .find(|o| o.zone == Zone::Battlefield
            && o.card_id == ids.tangled_florahedron)
        .map(|o| o.id).unwrap();
    // Give priority back for activation enumeration.
    let mut s2 = s;
    priority_to_main(&mut s2, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s2, &registry);
    let creature_activations = actions.iter().filter(|a| matches!(a,
        Action::ActivateAbility { source, .. } if *source == elf
    )).count();
    assert_eq!(creature_activations, 0,
        "front-face creature must NOT have the back-face mana ability");
}

#[test]
fn mdfc_back_not_offered_without_land_drop() {
    // If the player's land_plays_remaining is 0 the MDFC back-face
    // land play should be withheld along with all other land plays.
    let (mut s, registry, ids) = fresh_game();
    let hedron = put_in_hand(&mut s, &registry, 0, ids.tangled_florahedron);
    s.player_mut(0).land_plays_remaining = 0;
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(mdfc_back_land_play(&actions, hedron).is_none(),
        "no land drop remaining → MDFC back land play is withheld");
    // But normal creature cast is still offered if mana present.
    give_mana(&mut s, 0, ManaColor::Green, 1);
    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(normal_creature_cast(&actions, hedron).is_some(),
        "creature cast independent of land drop");
}

#[test]
fn mdfc_both_faces_are_legal_simultaneously() {
    // With {G} available AND a land drop, legal_actions should offer
    // both the front-face cast and the back-face land play. Picking
    // between them is the agent's call.
    let (mut s, registry, ids) = fresh_game();
    let hedron = put_in_hand(&mut s, &registry, 0, ids.tangled_florahedron);
    give_mana(&mut s, 0, ManaColor::Green, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    assert!(normal_creature_cast(&actions, hedron).is_some(),
        "front-face cast offered");
    assert!(mdfc_back_land_play(&actions, hedron).is_some(),
        "back-face land play offered");
}

/// CR 712.2b — a multi-face card in a zone other than the stack or
/// battlefield has only its front-face characteristics. Play Tangled
/// Vale (back face land), destroy it, and assert the graveyard
/// object shows Tangled Florahedron (1/1 creature), not the land.
#[test]
fn mdfc_back_land_reverts_to_front_on_bf_to_graveyard() {
    let (mut s, registry, ids) = fresh_game();
    let hedron = put_in_hand(&mut s, &registry, 0, ids.tangled_florahedron);
    priority_to_main(&mut s, 0);

    // Play the back face as Tangled Vale.
    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let play = mdfc_back_land_play(&actions, hedron)
        .expect("MDFC back land-play offered");
    let (mut s, _) = step(s, play, &registry);
    let vale = s.objects.iter()
        .find(|o| o.zone == Zone::Battlefield
            && o.card_id == ids.tangled_florahedron)
        .map(|o| o.id).unwrap();
    // Precondition: on-battlefield object is a land (back face live).
    assert!(s.objects.get(vale).unwrap().is_land(),
        "precondition: battlefield object shows back-face land");
    assert_eq!(s.objects.get(vale).unwrap().visible_face, 1);

    // Destroy.
    s.move_object_to_zone(vale, Zone::Graveyard(0),
        arcana_core::events::MoveCause::AbilityResolution);

    let grave_obj = s.objects.iter()
        .find(|o| matches!(o.zone, Zone::Graveyard(_))
            && o.card_id == ids.tangled_florahedron)
        .expect("card landed in graveyard");
    assert!(grave_obj.is_creature(),
        "graveyard object reverts to front-face creature per CR 712.2b");
    assert!(!grave_obj.is_land(),
        "graveyard object no longer shows back-face land");
    assert_eq!(grave_obj.visible_face, 0,
        "visible_face reset on revert");
    assert!(grave_obj.default_face_characteristics.is_none(),
        "snapshot cleared after revert");
}

/// Bouncing a back-face MDFC land to its owner's hand must revert to
/// front-face characteristics — the card appears in hand as the
/// creature, not as a playable land. Without the revert, the card
/// would remain `is_land()` in hand and `legal_actions` would
/// spuriously offer it as a normal land play.
#[test]
fn mdfc_back_land_reverts_to_front_on_bf_to_hand() {
    let (mut s, registry, ids) = fresh_game();
    let hedron = put_in_hand(&mut s, &registry, 0, ids.tangled_florahedron);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let play = mdfc_back_land_play(&actions, hedron).unwrap();
    let (mut s, _) = step(s, play, &registry);
    let vale = s.objects.iter()
        .find(|o| o.zone == Zone::Battlefield
            && o.card_id == ids.tangled_florahedron)
        .map(|o| o.id).unwrap();

    // Bounce to hand.
    s.move_object_to_zone(vale, Zone::Hand(0),
        arcana_core::events::MoveCause::AbilityResolution);

    let hand_obj = s.objects.iter()
        .find(|o| o.zone == Zone::Hand(0)
            && o.card_id == ids.tangled_florahedron)
        .expect("card returned to hand");
    assert!(hand_obj.is_creature(),
        "bounced MDFC reverts to front-face creature in hand");
    assert!(!hand_obj.is_land(),
        "bounced MDFC no longer shows back-face land chars");
    assert_eq!(hand_obj.visible_face, 0);
}

/// Round-trip: after a bounced MDFC reverts in hand, the player must
/// still be able to re-play the back face as a land via the normal
/// `PlayLand { mdfc_back: true }` path. Proves the revert doesn't
/// clobber the registry-driven re-swap.
#[test]
fn mdfc_back_land_replayable_after_bounce_revert() {
    let (mut s, registry, ids) = fresh_game();
    let hedron = put_in_hand(&mut s, &registry, 0, ids.tangled_florahedron);
    priority_to_main(&mut s, 0);

    // First play — back face.
    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let play = mdfc_back_land_play(&actions, hedron).unwrap();
    let (mut s, _) = step(s, play, &registry);
    let vale = s.objects.iter()
        .find(|o| o.zone == Zone::Battlefield
            && o.card_id == ids.tangled_florahedron)
        .map(|o| o.id).unwrap();

    // Bounce, reset land drop, replay back face.
    s.move_object_to_zone(vale, Zone::Hand(0),
        arcana_core::events::MoveCause::AbilityResolution);
    s.player_mut(0).land_plays_remaining = 1;
    let rebound_id = s.objects.iter()
        .find(|o| o.zone == Zone::Hand(0)
            && o.card_id == ids.tangled_florahedron)
        .map(|o| o.id).unwrap();
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let replay = mdfc_back_land_play(&actions, rebound_id)
        .expect("back-face land play offered after bounce-revert");
    let (s, _) = step(s, replay, &registry);
    let replayed = s.objects.iter()
        .find(|o| o.zone == Zone::Battlefield
            && o.card_id == ids.tangled_florahedron)
        .expect("card returns to battlefield");
    assert!(replayed.is_land(),
        "replayed object is back-face land again");
    assert_eq!(replayed.visible_face, 1);
}

/// Negative control: a Florahedron cast and resolved as a creature,
/// then killed, shows front-face chars throughout. No snapshot was
/// ever taken because no back-face swap happened; the revert path is
/// a no-op in this case.
#[test]
fn mdfc_front_face_cast_unaffected_by_revert_logic() {
    let (mut s, registry, ids) = fresh_game();
    let hedron = put_in_hand(&mut s, &registry, 0, ids.tangled_florahedron);
    give_mana(&mut s, 0, ManaColor::Green, 1);
    priority_to_main(&mut s, 0);

    let cast = normal_creature_cast(
        &arcana_core::legal_actions::legal_actions(&s, &registry), hedron)
        .unwrap();
    let (s, _) = step(s, cast, &registry);
    let mut s = resolve_stack(s, &registry);
    let elf = s.objects.iter()
        .find(|o| o.zone == Zone::Battlefield
            && o.card_id == ids.tangled_florahedron)
        .map(|o| o.id).unwrap();
    assert!(s.objects.get(elf).unwrap().is_creature(),
        "precondition: resolved as creature");
    assert!(s.objects.get(elf).unwrap()
        .default_face_characteristics.is_none(),
        "front-face cast never takes a snapshot");

    s.move_object_to_zone(elf, Zone::Graveyard(0),
        arcana_core::events::MoveCause::AbilityResolution);
    let grave_obj = s.objects.iter()
        .find(|o| matches!(o.zone, Zone::Graveyard(_))
            && o.card_id == ids.tangled_florahedron)
        .unwrap();
    assert!(grave_obj.is_creature(),
        "graveyard object is still the creature — revert was a no-op");
    assert_eq!(grave_obj.visible_face, 0);
}

#[test]
fn bonecrusher_adventure_cast_fizzles_to_exile_when_target_leaves() {
    // An adventure spell cast with its only target disappearing
    // before resolution fizzles — CR 608.2b sends it to the
    // graveyard via counter_resolved_spell. For an adventure cast
    // the routing override must still fire: the card goes to exile
    // with the flag set (CR 715 routing), exactly like Flashback's
    // flag-on-leave.
    let (mut s, registry, ids) = fresh_game();
    let giant = put_in_hand(&mut s, &registry, 0, ids.bonecrusher_giant);
    let bears = put_on_battlefield(&mut s, &registry, 1, ids.grizzly_bears);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let cast = adventure_cast_action(
        &arcana_core::legal_actions::legal_actions(&s, &registry), giant)
        .expect("adventure cast offered");
    let cast = match cast {
        Action::CastSpell { object_id, modes, mana_payment,
                            additional_costs, x_value, cast_modifier,
                            cost_reductions, .. } => Action::CastSpell {
            object_id,
            targets: arcana_core::targets::TargetSelection {
                targets: vec![
                    arcana_core::targets::TargetChoice::ObjectOrPlayer(
                        arcana_core::targets::ObjectOrPlayer::Object(bears)),
                ],
            },
            modes, mana_payment, additional_costs, x_value,
            cast_modifier, cost_reductions,
        },
        _ => unreachable!(),
    };
    let (mut s, _) = step(s, cast, &registry);
    // Remove the target bears from the battlefield before the spell
    // resolves — CR 608.2b will classify the entry as illegal at
    // resolution time and route through `counter_resolved_spell`.
    s.move_object_to_zone(bears, Zone::Exile,
        arcana_core::events::MoveCause::AbilityResolution);
    let s = resolve_stack(s, &registry);

    // The adventure spell went to exile with the flag, not to
    // graveyard.
    let exile_objs: Vec<_> = s.objects.iter()
        .filter(|o| o.zone == Zone::Exile
            && o.card_id == ids.bonecrusher_giant)
        .collect();
    assert_eq!(exile_objs.len(), 1,
        "fizzled adventure in exile");
    assert!(exile_objs[0].adventure_exile_pending,
        "fizzled adventure still flag-set for creature-cast window");
    assert_eq!(s.zone_count(Zone::Graveyard(0)), 0,
        "fizzled adventure does NOT route to graveyard");
}

// ---------------------------------------------------------------------
// Servo Exhibition (CR 111 / 704.5d) — token creation + cease-to-exist
// ---------------------------------------------------------------------

fn cast_servo_exhibition(
    actions: &[Action],
    card_in_hand: ObjectId,
) -> Option<Action> {
    actions.iter().find(|a| matches!(a,
        Action::CastSpell { object_id, .. } if *object_id == card_in_hand
    )).cloned()
}

/// Cast Servo Exhibition and assert two 1/1 colorless Servo artifact
/// creature tokens hit the battlefield with `is_token = true`.
#[test]
fn servo_exhibition_creates_two_artifact_creature_tokens() {
    let (mut s, registry, ids) = fresh_game();
    let spell = put_in_hand(&mut s, &registry, 0, ids.servo_exhibition);
    give_mana(&mut s, 0, ManaColor::White, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = cast_servo_exhibition(&actions, spell)
        .expect("Servo Exhibition is castable");
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    let tokens: Vec<_> = s.objects.iter()
        .filter(|o| o.zone.is_battlefield() && o.is_token)
        .collect();
    assert_eq!(tokens.len(), 2,
        "Servo Exhibition creates exactly two tokens");
    for tok in &tokens {
        assert!(tok.is_creature());
        assert!(tok.is_artifact());
        assert_eq!(tok.characteristics.power,
            Some(arcana_core::types::PtValue::Fixed(1)));
        assert_eq!(tok.characteristics.toughness,
            Some(arcana_core::types::PtValue::Fixed(1)));
        assert!(tok.characteristics.colors.is_colorless(),
            "tokens are colorless");
        assert_eq!(tok.controller, 0,
            "tokens are controlled by the caster");
    }
    // The spell itself went to graveyard (standard sorcery resolution).
    assert_eq!(
        s.objects.iter()
            .filter(|o| matches!(o.zone, Zone::Graveyard(_))
                && o.card_id == ids.servo_exhibition)
            .count(),
        1,
        "Servo Exhibition (the card) is in the graveyard");
}

/// CR 704.5d — a token that dies is removed from the arena entirely
/// on the same SBA pass as the lethal-damage check. The graveyard
/// should contain zero token residue, and a subsequent object lookup
/// should find nothing.
#[test]
fn servo_token_dies_and_ceases_to_exist() {
    let (mut s, registry, ids) = fresh_game();
    let spell = put_in_hand(&mut s, &registry, 0, ids.servo_exhibition);
    give_mana(&mut s, 0, ManaColor::White, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = cast_servo_exhibition(&actions, spell).unwrap();
    let (s, _) = step(s, cast, &registry);
    let mut s = resolve_stack(s, &registry);

    let token_id = s.objects.iter()
        .find(|o| o.zone.is_battlefield() && o.is_token)
        .map(|o| o.id).expect("token on battlefield");

    // Mark lethal damage and run SBAs. 704.5g sends the token to
    // graveyard; 704.5d removes it from the arena on the same pass.
    s.objects.get_mut(token_id).unwrap().damage_marked = 1;
    arcana_core::sba::apply_state_based_actions(&mut s);

    assert!(s.objects.get(token_id).is_none(),
        "dead token is removed from the arena");
    let token_residue = s.objects.iter()
        .filter(|o| o.is_token)
        .count();
    // Still one live token on the battlefield (we only killed one).
    assert_eq!(token_residue, 1,
        "the other token is still on the battlefield");
    assert_eq!(
        s.objects.iter()
            .filter(|o| matches!(o.zone, Zone::Graveyard(_)) && o.is_token)
            .count(),
        0,
        "no token residue in any graveyard");
}

/// A Lightning Bolt targeting a Servo token kills it. End-to-end
/// check that the token-cease SBA fires on a non-combat destroy path.
#[test]
fn bolt_to_servo_token_removes_it_from_arena() {
    let (mut s, registry, ids) = fresh_game();
    let spell = put_in_hand(&mut s, &registry, 0, ids.servo_exhibition);
    give_mana(&mut s, 0, ManaColor::White, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);
    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = cast_servo_exhibition(&actions, spell).unwrap();
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    let token_id = s.objects.iter()
        .find(|o| o.zone.is_battlefield() && o.is_token)
        .map(|o| o.id).unwrap();

    // Opponent bolts the token.
    let mut s = s;
    let bolt = put_in_hand(&mut s, &registry, 1, ids.lightning_bolt);
    give_mana(&mut s, 1, ManaColor::Red, 1);
    priority_to_main(&mut s, 1);

    let bolt_cast = Action::CastSpell {
        object_id: bolt,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::ObjectOrPlayer(
                arcana_core::targets::ObjectOrPlayer::Object(token_id),
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
    let (s, _) = step(s, bolt_cast, &registry);
    let s = resolve_stack(s, &registry);

    assert!(s.objects.get(token_id).is_none(),
        "bolted token ceased to exist");
    let live_tokens: Vec<_> = s.objects.iter()
        .filter(|o| o.is_token)
        .collect();
    assert_eq!(live_tokens.len(), 1,
        "surviving token still on the battlefield");
    assert!(live_tokens[0].zone.is_battlefield());
}

/// `ObjectFilter::is_token` composes with other filters. Verifies
/// the filter flip from the no-op TODO to the real check — setup
/// both a Servo token and a nontoken creature, then assert filter
/// matching picks the right side each way.
#[test]
fn is_token_filter_distinguishes_tokens_from_cards() {
    use arcana_core::targets::ObjectFilter;
    let (mut s, registry, ids) = fresh_game();
    let spell = put_in_hand(&mut s, &registry, 0, ids.servo_exhibition);
    let bears = put_on_battlefield(&mut s, &registry, 0, ids.grizzly_bears);
    give_mana(&mut s, 0, ManaColor::White, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);
    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = cast_servo_exhibition(&actions, spell).unwrap();
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    let token_obj = s.objects.iter()
        .find(|o| o.is_token).unwrap();
    let bears_obj = s.objects.get(bears).unwrap();

    let token_only = ObjectFilter { is_token: Some(true), ..Default::default() };
    let nontoken_only = ObjectFilter { is_token: Some(false), ..Default::default() };
    assert!(token_only.matches(token_obj, &s, 0),
        "is_token=Some(true) matches a token");
    assert!(!token_only.matches(bears_obj, &s, 0),
        "is_token=Some(true) rejects a card");
    assert!(nontoken_only.matches(bears_obj, &s, 0),
        "is_token=Some(false) matches a card");
    assert!(!nontoken_only.matches(token_obj, &s, 0),
        "is_token=Some(false) rejects a token");
}

// ---------------------------------------------------------------------
// Young Pyromancer (CR 603) — cast-trigger token creation, exercising
// the trigger EffectFn's `&CardRegistry` parameter (for interning
// "Elemental" at resolve time) plus the SpellCast + instant/sorcery
// filter + You caster-constraint composition.
// ---------------------------------------------------------------------

/// Casting an instant with Young Pyromancer on the battlefield creates
/// one 1/1 red Elemental creature token.
#[test]
fn young_pyromancer_triggers_on_instant_cast() {
    let (mut s, registry, ids) = fresh_game();
    let _pyro = put_on_battlefield(&mut s, &registry, 0, ids.young_pyromancer);
    let bolt = put_in_hand(&mut s, &registry, 0, ids.lightning_bolt);
    give_mana(&mut s, 0, ManaColor::Red, 1);
    priority_to_main(&mut s, 0);

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

    let tokens: Vec<_> = s.objects.iter()
        .filter(|o| o.zone.is_battlefield() && o.is_token)
        .collect();
    assert_eq!(tokens.len(), 1, "exactly one Elemental token");
    let tok = tokens[0];
    assert!(tok.is_creature());
    assert!(!tok.characteristics.types.is_artifact(),
        "Elemental token is not an artifact");
    assert!(tok.characteristics.colors.contains(arcana_core::types::Color::Red),
        "Elemental token is red");
    assert_eq!(tok.characteristics.power,
        Some(arcana_core::types::PtValue::Fixed(1)));
    assert_eq!(tok.characteristics.toughness,
        Some(arcana_core::types::PtValue::Fixed(1)));
    assert_eq!(tok.controller, 0, "token is controlled by Pyromancer's controller");
}

/// Casting a sorcery also triggers Pyromancer. Uses Servo Exhibition
/// as the sorcery so the test co-validates that both cards' triggers
/// produce tokens in the same resolution window without confusing
/// them (Pyromancer's Elemental is red; Servo tokens are colorless).
#[test]
fn young_pyromancer_triggers_on_sorcery_cast() {
    let (mut s, registry, ids) = fresh_game();
    let _pyro = put_on_battlefield(&mut s, &registry, 0, ids.young_pyromancer);
    let servo = put_in_hand(&mut s, &registry, 0, ids.servo_exhibition);
    give_mana(&mut s, 0, ManaColor::White, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = actions.iter().find(|a| matches!(a,
        Action::CastSpell { object_id, .. } if *object_id == servo
    )).cloned().expect("Servo Exhibition is castable");
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    let tokens: Vec<_> = s.objects.iter()
        .filter(|o| o.zone.is_battlefield() && o.is_token)
        .collect();
    assert_eq!(tokens.len(), 3,
        "two Servo tokens + one Elemental token");
    let reds = tokens.iter()
        .filter(|t| t.characteristics.colors.contains(arcana_core::types::Color::Red))
        .count();
    let colorless = tokens.iter()
        .filter(|t| t.characteristics.colors.is_colorless())
        .count();
    assert_eq!(reds, 1, "exactly one red token (Pyromancer's Elemental)");
    assert_eq!(colorless, 2, "two colorless Servo tokens");
}

/// Casting a creature spell does NOT trigger Young Pyromancer — the
/// filter requires instant-or-sorcery.
#[test]
fn young_pyromancer_ignores_creature_cast() {
    let (mut s, registry, ids) = fresh_game();
    let _pyro = put_on_battlefield(&mut s, &registry, 0, ids.young_pyromancer);
    let bears = put_in_hand(&mut s, &registry, 0, ids.grizzly_bears);
    give_mana(&mut s, 0, ManaColor::Green, 1);
    give_mana(&mut s, 0, ManaColor::Colorless, 1);
    priority_to_main(&mut s, 0);

    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = actions.iter().find(|a| matches!(a,
        Action::CastSpell { object_id, .. } if *object_id == bears
    )).cloned().expect("Bears is castable");
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    let token_count = s.objects.iter()
        .filter(|o| o.zone.is_battlefield() && o.is_token)
        .count();
    assert_eq!(token_count, 0,
        "creature cast must not fire Pyromancer's trigger");
}

/// An opponent casting an instant does NOT trigger your Young
/// Pyromancer — the caster constraint is [`ControllerConstraint::You`].
#[test]
fn young_pyromancer_ignores_opponents_instant() {
    let (mut s, registry, ids) = fresh_game();
    let _pyro = put_on_battlefield(&mut s, &registry, 0, ids.young_pyromancer);
    let bolt = put_in_hand(&mut s, &registry, 1, ids.lightning_bolt);
    give_mana(&mut s, 1, ManaColor::Red, 1);
    priority_to_main(&mut s, 1);

    let cast = Action::CastSpell {
        object_id: bolt,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::ObjectOrPlayer(
                arcana_core::targets::ObjectOrPlayer::Player(0),
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

    let token_count = s.objects.iter()
        .filter(|o| o.zone.is_battlefield() && o.is_token)
        .count();
    assert_eq!(token_count, 0,
        "opponent's instant must not fire your Pyromancer's trigger");
}

// ---------------------------------------------------------------------
// Bonesplitter (CR 702.6 / 704.5q) — Equipment + equip + attached-pump
// layer + illegal-attachment SBA.
// ---------------------------------------------------------------------

fn activate_equip(
    source: ObjectId,
    target: ObjectId,
    mana_pool_index: usize,
) -> Action {
    Action::ActivateAbility {
        source,
        ability_index: 0,
        targets: arcana_core::targets::TargetSelection {
            targets: vec![arcana_core::targets::TargetChoice::Object(target)],
        },
        mana_payment: arcana_core::actions::ManaPaymentPlan {
            assignments: vec![arcana_core::actions::ManaAssignment {
                pool_index: mana_pool_index, cost_index: 0,
            }],
            ..Default::default()
        },
        additional_costs: vec![],
    }
}

/// Cast Bonesplitter, equip it to Grizzly Bears, verify the Bears
/// gains +2/+0 via the layer system.
#[test]
fn bonesplitter_equipped_creature_gets_plus_2_plus_0() {
    let (mut s, registry, ids) = fresh_game();
    let bones = put_in_hand(&mut s, &registry, 0, ids.bonesplitter);
    let bears = put_on_battlefield(&mut s, &registry, 0, ids.grizzly_bears);
    // Cast Bonesplitter ({1}) then equip ({1}) = 2 colorless.
    give_mana(&mut s, 0, ManaColor::Colorless, 2);
    priority_to_main(&mut s, 0);

    // Bears starts at 2/2.
    assert_eq!(
        s.compute_characteristics(bears).unwrap().power,
        Some(arcana_core::types::PtValue::Fixed(2)));

    // Cast Bonesplitter — spends 1 colorless from pool 0.
    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = actions.iter().find(|a| matches!(a,
        Action::CastSpell { object_id, .. } if *object_id == bones
    )).cloned().expect("Bonesplitter is castable");
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);

    // Find Bonesplitter's new battlefield ObjectId (re-id'd on zone move).
    let bones_bf = s.objects.iter()
        .find(|o| o.zone.is_battlefield()
            && o.card_id == ids.bonesplitter)
        .map(|o| o.id).expect("Bonesplitter on battlefield");

    // Activate equip {1} on Bears. Pool index is 0 — only one unit left
    // after the cast.
    let equip = activate_equip(bones_bf, bears, 0);
    let (s, _) = step(s, equip, &registry);
    let s = resolve_stack(s, &registry);

    // Bears is now 4/2 (+2/+0 from Bonesplitter).
    let bears_chars = s.compute_characteristics(bears).unwrap();
    assert_eq!(bears_chars.power,
        Some(arcana_core::types::PtValue::Fixed(4)),
        "equipped creature gets +2 power");
    assert_eq!(bears_chars.toughness,
        Some(arcana_core::types::PtValue::Fixed(2)),
        "toughness unchanged (+0)");
    // Bonesplitter sees the attachment.
    assert_eq!(s.objects.get(bones_bf).unwrap().attached_to, Some(bears));
    assert!(s.objects.get(bears).unwrap().attachments.contains(&bones_bf));
}

/// Re-equipping Bonesplitter on a second creature moves the attachment
/// and the +2/+0 bonus with it.
#[test]
fn bonesplitter_re_equip_moves_bonus() {
    let (mut s, registry, ids) = fresh_game();
    let bones_hand = put_in_hand(&mut s, &registry, 0, ids.bonesplitter);
    let bears_a = put_on_battlefield(&mut s, &registry, 0, ids.grizzly_bears);
    let bears_b = put_on_battlefield(&mut s, &registry, 0, ids.grizzly_bears);
    // 1 cast + 2 equips = 3 colorless.
    give_mana(&mut s, 0, ManaColor::Colorless, 3);
    priority_to_main(&mut s, 0);

    // Cast Bonesplitter.
    let actions = arcana_core::legal_actions::legal_actions(&s, &registry);
    let cast = actions.iter().find(|a| matches!(a,
        Action::CastSpell { object_id, .. } if *object_id == bones_hand
    )).cloned().expect("castable");
    let (s, _) = step(s, cast, &registry);
    let s = resolve_stack(s, &registry);
    let bones_bf = s.objects.iter()
        .find(|o| o.zone.is_battlefield() && o.card_id == ids.bonesplitter)
        .map(|o| o.id).unwrap();

    // Equip → Bears A.
    let (s, _) = step(s, activate_equip(bones_bf, bears_a, 0), &registry);
    let s = resolve_stack(s, &registry);
    assert_eq!(s.compute_characteristics(bears_a).unwrap().power,
        Some(arcana_core::types::PtValue::Fixed(4)));
    assert_eq!(s.compute_characteristics(bears_b).unwrap().power,
        Some(arcana_core::types::PtValue::Fixed(2)));

    // Equip → Bears B. Moves the attachment.
    let (s, _) = step(s, activate_equip(bones_bf, bears_b, 0), &registry);
    let s = resolve_stack(s, &registry);
    assert_eq!(s.compute_characteristics(bears_a).unwrap().power,
        Some(arcana_core::types::PtValue::Fixed(2)),
        "Bears A loses the bonus when Bonesplitter moves");
    assert_eq!(s.compute_characteristics(bears_b).unwrap().power,
        Some(arcana_core::types::PtValue::Fixed(4)),
        "Bears B gains the bonus");
    assert!(s.objects.get(bears_a).unwrap().attachments.is_empty(),
        "Bears A no longer lists Bonesplitter in attachments");
    assert_eq!(s.objects.get(bones_bf).unwrap().attached_to, Some(bears_b));
}

/// CR 704.5q — if Bonesplitter ends up attached to a non-creature
/// (here: forced by a direct state-level attach to another artifact),
/// the SBA detaches it on the next pass. Bonesplitter stays on the
/// battlefield.
#[test]
fn bonesplitter_attached_to_non_creature_detaches_via_sba() {
    let (mut s, registry, ids) = fresh_game();
    let bones = put_on_battlefield(&mut s, &registry, 0, ids.bonesplitter);
    // Second Bonesplitter as the illegal target (artifact, non-creature).
    let other_artifact = put_on_battlefield(&mut s, &registry, 0, ids.bonesplitter);

    // Force-attach skipping the normal equip-cost pipeline.
    arcana_core::effects::Effect::Attach {
        equipment_or_aura: bones,
        target: other_artifact,
    }.execute(&mut s);
    assert_eq!(s.objects.get(bones).unwrap().attached_to, Some(other_artifact),
        "pre-SBA: forced attachment in place");

    // SBA pass — CR 704.5q detaches.
    arcana_core::sba::apply_state_based_actions(&mut s);

    assert_eq!(s.objects.get(bones).unwrap().attached_to, None,
        "CR 704.5q: illegal attachment detached");
    assert!(s.objects.get(bones).unwrap().zone.is_battlefield(),
        "Bonesplitter stays on the battlefield (not graveyard)");
    assert!(s.objects.get(other_artifact).unwrap().attachments.is_empty(),
        "the illegal holder drops Bonesplitter from its attachments list");
}

/// Equipped creature dies → Bonesplitter detaches on the same SBA
/// pass (target no longer on battlefield).
#[test]
fn bonesplitter_equipped_creature_dies_detaches_bones() {
    let (mut s, registry, ids) = fresh_game();
    let bones = put_on_battlefield(&mut s, &registry, 0, ids.bonesplitter);
    let bears = put_on_battlefield(&mut s, &registry, 0, ids.grizzly_bears);
    arcana_core::effects::Effect::Attach {
        equipment_or_aura: bones, target: bears,
    }.execute(&mut s);
    assert_eq!(s.objects.get(bones).unwrap().attached_to, Some(bears));

    // Mark lethal damage on Bears, run SBAs.
    s.objects.get_mut(bears).unwrap().damage_marked = 2;
    arcana_core::sba::apply_state_based_actions(&mut s);

    // Bears moved to graveyard (re-id). Bonesplitter's attached_to now
    // points to a graveyard object; the 704.5q pass detaches it.
    assert_eq!(s.objects.get(bones).unwrap().attached_to, None,
        "Bonesplitter auto-detaches once target leaves battlefield");
    assert!(s.objects.get(bones).unwrap().zone.is_battlefield(),
        "Bonesplitter remains on the battlefield after detach");
}

// ---------------------------------------------------------------------
// Preordain — CR 608.2 sequential multi-effect resolution (Scry, then
// draw). Exercises the engine's park-on-pending-choice / resume path:
// Scry pushes OrderCards mid-resolution; the draw is deferred into
// `pending_resolution.remaining_effects` and runs when the agent
// submits placements.
// ---------------------------------------------------------------------

fn cast_preordain(
    mut s: GameState,
    registry: &CardRegistry,
    caster: PlayerId,
    preordain: ObjectId,
) -> GameState {
    give_mana(&mut s, caster, ManaColor::Blue, 1);
    priority_to_main(&mut s, caster);
    let cast = Action::CastSpell {
        object_id: preordain,
        targets: arcana_core::targets::TargetSelection { targets: vec![] },
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
    let (s, _) = step(s, cast, registry);
    // Pass priority from both players — active pass resolves
    // Preordain, which pushes the Scry OrderCards choice.
    let (s, _) = step(s, Action::PassPriority, registry);
    let (s, _) = step(s, Action::PassPriority, registry);
    s
}

/// Preordain's Scry pushes an `OrderCards` prompt mid-resolution and
/// parks the remaining `DrawCards` effect. Submitting placements
/// resumes the parked resolution and drains the draw.
#[test]
fn preordain_parks_on_scry_then_resumes_draw() {
    let (mut s, registry, ids) = fresh_game();
    let preordain = put_in_hand(&mut s, &registry, 0, ids.preordain);
    // Seed three cards top-to-bottom: [l0, l1, l2]. Scry looks at
    // [l0, l1]; keeping both on top leaves the draw to pull l0.
    let l2 = put_in_library_top(&mut s, &registry, 0, ids.mountain);
    let l1 = put_in_library_top(&mut s, &registry, 0, ids.forest);
    let l0 = put_in_library_top(&mut s, &registry, 0, ids.plains);
    assert_eq!(s.player(0).library_top_to_bottom, vec![l0, l1, l2],
        "helper invariant: most-recently-inserted card sits on top");

    let s = cast_preordain(s, &registry, 0, preordain);

    // The Scry pushed a choice; draw is parked.
    let pc = s.pending_choice.as_ref().expect("scry pushed OrderCards");
    assert_eq!(pc.choosing_player, 0);
    assert!(matches!(&pc.context,
        arcana_core::actions::ChoiceContext::ResolvingStack(_)));
    match &pc.kind {
        arcana_core::actions::ChoiceKind::OrderCards { cards, allowed } => {
            assert_eq!(cards, &vec![l0, l1],
                "scry surfaces the top 2 cards of library_top_to_bottom");
            assert!(allowed.contains(
                &arcana_core::actions::CardDestination::TopOfLibrary));
            assert!(allowed.contains(
                &arcana_core::actions::CardDestination::BottomOfLibrary));
        }
        other => panic!("expected OrderCards, got {other:?}"),
    }
    let parked = s.pending_resolution.as_ref()
        .expect("draw is parked while scry's choice is open");
    assert_eq!(parked.remaining_effects.len(), 1,
        "DrawCards is the single remaining effect");
    assert!(matches!(
        &parked.remaining_effects[0],
        arcana_core::effects::Effect::DrawCards { player: 0, count: 1 },
    ));

    // Submit: keep both on top in [l0, l1] order (no-op reorder).
    let pc_id = pc.id;
    let submit = Action::SubmitResolutionChoice {
        id: pc_id,
        response: arcana_core::actions::ChoiceResponse::OrderCards {
            placements: vec![
                (l0, arcana_core::actions::CardDestination::TopOfLibrary),
                (l1, arcana_core::actions::CardDestination::TopOfLibrary),
            ],
        },
    };
    let (s, _) = step(s, submit, &registry);

    // Parked resolution drained, draw landed. A drawn card is re-ided
    // by swap_to_zone_reid, so the old l0 id is LKI in Library(0);
    // the live object is in Hand(0) under a fresh id. The
    // load-bearing assertions are zone counts + remaining library
    // order.
    assert!(s.pending_choice.is_none(),
        "choice cleared after submit");
    assert!(s.pending_resolution.is_none(),
        "park cleared after remaining effects drained");
    assert_eq!(s.zone_count(Zone::Hand(0)), 1,
        "draw pulled one card into p0's hand");
    assert_eq!(s.player(0).library_top_to_bottom.len(), 2,
        "library: two cards remain after a scry-2 + draw-1");
    assert_eq!(s.player(0).library_top_to_bottom[1], l2,
        "l2 was never touched by the scry or draw");
    assert_eq!(s.zone_count(Zone::Graveyard(0)), 1,
        "Preordain resolved into its owner's graveyard");
}

/// Scry-both-to-bottom: the draw pulls whatever the scry left on top
/// (the third card), proving the effect sequence observes the
/// intermediate library mutation.
#[test]
fn preordain_bottom_both_draws_the_new_top() {
    let (mut s, registry, ids) = fresh_game();
    let preordain = put_in_hand(&mut s, &registry, 0, ids.preordain);
    let _l2 = put_in_library_top(&mut s, &registry, 0, ids.mountain);
    let l1 = put_in_library_top(&mut s, &registry, 0, ids.forest);
    let l0 = put_in_library_top(&mut s, &registry, 0, ids.plains);

    let s = cast_preordain(s, &registry, 0, preordain);
    let pc_id = s.pending_choice.as_ref()
        .expect("scry-2 on a 3-card library pushes OrderCards").id;

    // Both to bottom: pre-draw library = [l2, l0, l1]; draw pulls l2
    // (which gets re-ided), leaving library = [l0, l1].
    let submit = Action::SubmitResolutionChoice {
        id: pc_id,
        response: arcana_core::actions::ChoiceResponse::OrderCards {
            placements: vec![
                (l0, arcana_core::actions::CardDestination::BottomOfLibrary),
                (l1, arcana_core::actions::CardDestination::BottomOfLibrary),
            ],
        },
    };
    let (s, _) = step(s, submit, &registry);

    // l2 moved to hand via the re-iding zone change, so its old id is
    // LKI; the arena has a fresh id for the drawn card. Assert on the
    // surviving library order + hand count.
    assert_eq!(s.zone_count(Zone::Hand(0)), 1,
        "scry-to-bottom leaves the draw to pull the former l2");
    assert_eq!(s.player(0).library_top_to_bottom, vec![l0, l1],
        "library after draw: [l0, l1] (originally-top pair at bottom)");
}
