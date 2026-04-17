//! Integration test: the Lightning Bolt milestone game.
//!
//! Addendum Section 14 / Listing 20 — the "Phase 1 complete" gate.
//!
//! Two players, four registered cards, random legal actions. Asserts
//! the engine:
//!   - terminates (no infinite loop)
//!   - reaches a `GameOver` yield (Win or Draw)
//!   - bounds turn count below a sanity cap
//!   - doesn't panic under a deep random policy

use arcana_core::engine::new_game;
use arcana_core::registry::build_deck;
use arcana_core::sample_cards::register_all_phase1_samples;
use arcana_core::state::GameResult;
use arcana_core::{step, Action, CardRegistry, EngineYield, GameState};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Cap on total engine steps. A typical 2-player random-policy game
/// with Bolts + 2/2s settles in a few hundred steps; 100_000 is a
/// generous "clearly a bug" threshold.
const MAX_STEPS: u32 = 100_000;
/// Cap on turn count. The game must end long before this — but if
/// random play exhausts both libraries to decking, that's still
/// well under 50 turns.
const MAX_TURNS: u32 = 500;

#[test]
fn lightning_bolt_game_runs_to_completion() {
    let state = run_game(42);
    // Sanity: the game should have played more than a couple of turns
    // before someone died. If every game ends on turn 1 via concede or
    // immediate panic, the integration test isn't really exercising
    // the engine.
    assert!(
        state.turn.turn_number >= 2 || state.result.is_some(),
        "game ended in turn 1 — policy is probably conceding immediately",
    );
}

#[test]
fn lightning_bolt_game_seeds_are_independent() {
    // Spot-check determinism: the same seed runs the same way.
    let a = run_game(123);
    let b = run_game(123);
    assert_eq!(a.result, b.result);
}

#[test]
fn lightning_bolt_game_across_many_seeds_never_panics() {
    // Fuzz a range of seeds to catch stray panics in the engine.
    for seed in 0..16u64 {
        let _ = run_game(seed);
    }
}

#[test]
fn lightning_bolt_game_exercises_real_gameplay() {
    // Across a batch of seeds, assert games actually play out:
    // multiple turns, damage dealt, winners determined by life
    // reduction rather than trivial turn-1 effects.
    let mut any_deep = false;
    let mut any_life_lost = false;
    for seed in 0..16u64 {
        let state = run_game(seed);
        if state.turn.turn_number >= 3 {
            any_deep = true;
        }
        if state.players.iter().any(|p| p.life < 20) {
            any_life_lost = true;
        }
    }
    assert!(
        any_deep,
        "no game reached turn 3 — engine isn't really playing"
    );
    assert!(
        any_life_lost,
        "no game saw life-total changes — damage pipeline may be broken"
    );
}

/// Build the registry + the two decks, run the game under a random
/// legal-action policy keyed off `seed`, and return the final state.
fn run_game(seed: u64) -> GameState {
    let mut registry = CardRegistry::new();
    let ids = register_all_phase1_samples(&mut registry);
    let _ = ids;

    // P0: 10 Mountain, 4 Lightning Bolt, 4 Grizzly Bears
    // P1: 10 Forest, 8 Grizzly Bears
    let deck_a = build_deck(
        &[
            ("Mountain", 10),
            ("Lightning Bolt", 4),
            ("Grizzly Bears", 4),
        ],
        &registry,
    );
    let deck_b = build_deck(&[("Forest", 10), ("Grizzly Bears", 8)], &registry);

    let (mut state, mut yld) = new_game(vec![deck_a, deck_b], &registry, seed);

    // Separate RNG for the action-picking policy so determinism
    // is clean across engine changes.
    let mut rng = ChaCha8Rng::seed_from_u64(seed ^ 0xA5A5);

    for step_count in 0..MAX_STEPS {
        match yld {
            EngineYield::GameOver(ref result) => {
                assert!(
                    matches!(
                        result,
                        GameResult::Win(_) | GameResult::Draw | GameResult::Eliminated(_)
                    ),
                    "step {step_count}: GameOver carries unexpected result {result:?}",
                );
                assert!(
                    state.turn.turn_number < MAX_TURNS,
                    "step {step_count}: game took {} turns — likely a state-machine bug",
                    state.turn.turn_number,
                );
                return state;
            }
            EngineYield::PendingDecision {
                ref legal_actions, ..
            } => {
                assert!(
                    !legal_actions.is_empty(),
                    "step {step_count}: PendingDecision with no legal actions"
                );
                let action = pick_action(&mut rng, legal_actions);
                let (new_state, new_yld) = step(state, action, &registry);
                state = new_state;
                yld = new_yld;
            }
        }
    }
    panic!(
        "game did not terminate in {MAX_STEPS} steps (turn {})",
        state.turn.turn_number
    );
}

/// Choose a legal action under a mild "make progress" bias:
///
/// - If `MulliganKeep` is on the menu, always take it. The engine
///   doesn't yet implement the London-mulligan bottom-cards flow
///   ([`apply_mulligan_keep`] skips owed-bottoms), so taking
///   mulligans means opening with fewer and fewer cards — and once
///   the hand is empty, only pass/concede remain and games end
///   degenerately on turn 1.
/// - Otherwise prefer non-pass, non-concede actions when any exist,
///   so combat and casts actually happen.
/// - Fall back to uniform over all legal actions.
fn pick_action<R: Rng>(rng: &mut R, actions: &[Action]) -> Action {
    if actions.iter().any(|a| matches!(a, Action::MulliganKeep)) {
        return Action::MulliganKeep;
    }
    // Never voluntarily concede — that would terminate the game
    // before the engine is actually exercised.
    let interesting: Vec<&Action> = actions
        .iter()
        .filter(|a| !a.is_pass() && !a.is_concede())
        .collect();
    if !interesting.is_empty() {
        let idx = rng.gen_range(0..interesting.len());
        return interesting[idx].clone();
    }
    // Fall back to a plain pass when nothing else is on offer.
    if let Some(pass) = actions.iter().find(|a| a.is_pass()) {
        return pass.clone();
    }
    // Degenerate: only concede left. Take it.
    actions[0].clone()
}
