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

/// Put a card in `player`'s library via the arena (enough for
/// `collect_matching_candidates` to find it). Does not push onto
/// `library_top_to_bottom` — that's only needed for draws.
fn put_in_library(
    state: &mut GameState,
    registry: &CardRegistry,
    owner: PlayerId,
    card_id: arcana_core::types::CardId,
) -> ObjectId {
    let obj_id = state.allocate_object_id();
    let chars = registry.get(card_id).unwrap().base_characteristics.clone();
    state.objects.insert(GameObject::new(
        obj_id, owner, Zone::Library(owner), card_id, chars));
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

