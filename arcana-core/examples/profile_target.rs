//! Minimal profiling target — a single thread tight-looping the
//! workloads we want to analyze. Used for flamegraph / samply runs;
//! deliberately free of criterion so the recorded samples don't
//! include bench-framework overhead (rayon analysis, confidence
//! interval math, etc).
//!
//! Usage:
//!   cargo build --profile bench --example profile_target
//!   samply record target/bench/examples/profile_target <workload>
//!   cargo flamegraph --example profile_target -- <workload>
//!
//! Workloads:
//!   full_game      drive random-vs-random games to termination
//!   legal_actions  enumerate legal actions on a mid-game snapshot
//!   single_step    apply one step on a mid-game snapshot
//!   state_clone    clone a mid-game state

use arcana_cards::register_seed;
use arcana_core::engine::{new_game_with_format, step, EngineYield};
use arcana_core::legal_actions::legal_actions;
use arcana_core::registry::build_deck;
use arcana_core::types::CardId;
use arcana_core::{Action, CardRegistry, FormatConfig, GameState};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::hint::black_box;

fn seeded_registry() -> CardRegistry {
    let mut r = CardRegistry::new();
    register_seed(&mut r);
    r
}

fn bench_deck(r: &CardRegistry) -> Vec<CardId> {
    build_deck(
        &[
            ("Mountain", 20),
            ("Lightning Bolt", 16),
            ("Grizzly Bears", 16),
            ("Giant Spider", 8),
        ],
        r,
    )
}

fn pick_random(rng: &mut ChaCha8Rng, actions: &[Action]) -> Action {
    if actions.iter().any(|a| matches!(a, Action::MulliganKeep)) {
        return Action::MulliganKeep;
    }
    let interesting: Vec<&Action> = actions
        .iter()
        .filter(|a| !a.is_pass() && !a.is_concede())
        .collect();
    if !interesting.is_empty() {
        return interesting[rng.gen_range(0..interesting.len())].clone();
    }
    if let Some(p) = actions.iter().find(|a| a.is_pass()) {
        return p.clone();
    }
    actions[0].clone()
}

fn fresh_game(registry: &CardRegistry, seed: u64) -> (GameState, EngineYield) {
    let deck = bench_deck(registry);
    new_game_with_format(
        vec![deck.clone(), deck],
        FormatConfig::standard_2026(),
        registry,
        seed,
    )
}

fn play_one_game(registry: &CardRegistry, seed: u64) {
    let (mut state, mut yld) = fresh_game(registry, seed);
    let mut rngs = [
        ChaCha8Rng::seed_from_u64(seed.wrapping_add(101)),
        ChaCha8Rng::seed_from_u64(seed.wrapping_add(202)),
    ];
    loop {
        match yld {
            EngineYield::GameOver(_) => return,
            EngineYield::PendingDecision {
                player,
                legal_actions: la,
                ..
            } => {
                let action = pick_random(&mut rngs[player as usize], &la);
                let out = step(state, action, registry);
                state = out.0;
                yld = out.1;
            }
        }
    }
}

fn mid_game(
    registry: &CardRegistry,
    seed: u64,
    n_steps: usize,
) -> (GameState, EngineYield) {
    let (mut state, mut yld) = fresh_game(registry, seed);
    let mut rngs = [
        ChaCha8Rng::seed_from_u64(seed.wrapping_add(101)),
        ChaCha8Rng::seed_from_u64(seed.wrapping_add(202)),
    ];
    for _ in 0..n_steps {
        match yld {
            EngineYield::GameOver(_) => break,
            EngineYield::PendingDecision {
                player,
                legal_actions: la,
                ..
            } => {
                let action = pick_random(&mut rngs[player as usize], &la);
                let out = step(state, action, registry);
                state = out.0;
                yld = out.1;
            }
        }
    }
    (state, yld)
}

fn main() {
    let workload = std::env::args().nth(1).unwrap_or_else(|| "full_game".to_string());
    let registry = seeded_registry();

    match workload.as_str() {
        "full_game" => {
            // Target ~10 s of work. ~30 games/s today → 300 iterations.
            for seed in 0..300u64 {
                play_one_game(&registry, seed);
            }
        }
        "legal_actions" => {
            let (state, _) = mid_game(&registry, 17, 120);
            for _ in 0..5_000_000u64 {
                black_box(legal_actions(&state, &registry));
            }
        }
        "single_step" => {
            let (state, yld) = mid_game(&registry, 17, 120);
            let action = match yld {
                EngineYield::PendingDecision { legal_actions: la, .. } => {
                    let mut rng = ChaCha8Rng::seed_from_u64(999);
                    pick_random(&mut rng, &la)
                }
                EngineYield::GameOver(_) => Action::PassPriority,
            };
            for _ in 0..1_000_000u64 {
                let (_, _) = black_box(step(state.clone(), action.clone(), &registry));
            }
        }
        "state_clone" => {
            let (state, _) = mid_game(&registry, 17, 120);
            for _ in 0..20_000_000u64 {
                black_box(state.clone());
            }
        }
        other => {
            eprintln!("unknown workload {other:?}; try full_game | legal_actions | single_step | state_clone");
            std::process::exit(2);
        }
    }
}
