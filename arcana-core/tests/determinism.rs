//! Cross-invocation determinism guard.
//!
//! These tests exist specifically to fail if `arcana-core` ever
//! regresses to a non-deterministic hasher (`std::collections::HashMap`
//! with `RandomState`). They run the same game twice from independent
//! starting states with identically-seeded random drivers and assert
//! that every decision point — and thus every final state — matches.
//!
//! The underlying invariant from spec principle P5: *given the same
//! initial state and action sequence, the engine always produces the
//! same result*. A random agent closes the loop from the other side:
//! if the hasher is deterministic, the agent's `rng.gen_range(0..n)`
//! over legal actions will pick the same index, because `legal_actions`
//! returns them in the same order on every run.
//!
//! If this file starts failing, do **not** weaken the assertions. The
//! right diagnosis is that a `HashMap` / `HashSet` somewhere on the
//! decision path is iterating in a non-deterministic order — either a
//! new `std::collections::HashMap` slipped in, or a collection's
//! ordering isn't respecting the hasher (e.g., a `Vec::extend` over a
//! `HashMap::iter()`).

use arcana_cards::register_seed;
use arcana_core::engine::{new_game_with_format, step, EngineYield};
use arcana_core::registry::{build_deck, CardRegistry};
use arcana_core::{Action, FormatConfig};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Run one full game, recording every action picked. Deterministic
/// given the seeds.
fn run_game(
    registry: &CardRegistry,
    deck: Vec<arcana_core::types::CardId>,
    format: FormatConfig,
    game_seed: u64,
    rng_seeds: [u64; 2],
) -> (Vec<Action>, arcana_core::state::GameState) {
    let mut rngs = [
        ChaCha8Rng::seed_from_u64(rng_seeds[0]),
        ChaCha8Rng::seed_from_u64(rng_seeds[1]),
    ];
    let (mut state, mut yld) = new_game_with_format(
        vec![deck.clone(), deck], format, registry, game_seed,
    );

    let mut actions = Vec::new();
    loop {
        match yld {
            EngineYield::GameOver(_) => break,
            EngineYield::PendingDecision { player, legal_actions, .. } => {
                let action = pick(&mut rngs[player as usize], &legal_actions);
                actions.push(action.clone());
                let (ns, ny) = step(state, action, registry);
                state = ns;
                yld = ny;
            }
        }
    }
    (actions, state)
}

fn pick(rng: &mut ChaCha8Rng, actions: &[Action]) -> Action {
    if actions.iter().any(|a| matches!(a, Action::MulliganKeep)) {
        return Action::MulliganKeep;
    }
    let interesting: Vec<&Action> = actions.iter()
        .filter(|a| !a.is_pass() && !a.is_concede()).collect();
    if !interesting.is_empty() {
        let idx = rng.gen_range(0..interesting.len());
        return interesting[idx].clone();
    }
    actions.iter()
        .find(|a| a.is_pass())
        .cloned()
        .unwrap_or_else(|| actions[0].clone())
}

/// Two independent runs with identical seeds must produce identical
/// action sequences and final states. This would have failed with
/// `std::collections::HashMap::new()` because each map's `RandomState`
/// differs per-invocation → `legal_actions` iteration order differs
/// → random agent picks different indices.
#[test]
fn same_seed_same_actions_across_independent_games() {
    let mut registry = CardRegistry::new();
    let _ids = register_seed(&mut registry);

    let deck = build_deck(&[
        ("Mountain", 12),
        ("Forest", 12),
        ("Grizzly Bears", 12),
        ("Lightning Bolt", 24),
    ], &registry);

    let (actions_a, final_a) = run_game(
        &registry, deck.clone(),
        FormatConfig::standard_2026(), 42, [7, 13],
    );
    let (actions_b, final_b) = run_game(
        &registry, deck.clone(),
        FormatConfig::standard_2026(), 42, [7, 13],
    );

    // Point failure: diff the first divergent action rather than
    // just comparing lengths.
    for (i, (a, b)) in actions_a.iter().zip(actions_b.iter()).enumerate() {
        assert_eq!(a, b,
            "action {i} diverges under identical seeds:\n  \
             A = {a:?}\n  B = {b:?}\n\
             Likely cause: non-deterministic HashMap hasher somewhere \
             on the legal-action / SBA / trigger path.");
    }
    assert_eq!(actions_a.len(), actions_b.len(),
        "action sequence length diverges: {} vs {}",
        actions_a.len(), actions_b.len());
    assert_eq!(final_a.result, final_b.result,
        "final result diverges");
    assert_eq!(final_a.event_log.len(), final_b.event_log.len(),
        "event-log length diverges: {} vs {}",
        final_a.event_log.len(), final_b.event_log.len());
}

/// Replay parity: re-executing the recorded action sequence against
/// a fresh starting state must reproduce the same trajectory. This
/// catches hidden state in `CardRegistry` or thread-local caches
/// that `step` might implicitly depend on.
#[test]
fn recorded_actions_replay_to_identical_state() {
    let mut registry = CardRegistry::new();
    let _ids = register_seed(&mut registry);

    let deck = build_deck(&[
        ("Mountain", 12),
        ("Forest", 12),
        ("Lightning Bolt", 36),
    ], &registry);

    let (actions, original_final) = run_game(
        &registry, deck.clone(),
        FormatConfig::standard_2026(), 99, [1, 2],
    );

    // Replay those exact actions against a fresh game state.
    let (mut state, mut yld) = new_game_with_format(
        vec![deck.clone(), deck],
        FormatConfig::standard_2026(), &registry, 99,
    );
    for (i, a) in actions.into_iter().enumerate() {
        assert!(matches!(yld, EngineYield::PendingDecision { .. }),
            "replay hit a non-pending yield at action {i}");
        let (ns, ny) = step(state, a, &registry);
        state = ns;
        yld = ny;
    }
    assert!(matches!(yld, EngineYield::GameOver(_)),
        "replay did not terminate at the same point");
    assert_eq!(original_final.result, state.result,
        "replay diverges on final result");
    assert_eq!(original_final.event_log.len(), state.event_log.len(),
        "replay diverges on event-log length");
}
