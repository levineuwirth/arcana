//! Core-engine throughput benchmarks.
//!
//! Spec §19 single-thread targets:
//!   - full game avg       < 50 ms
//!   - games per second    > 20,000
//!   - state clone         < 5 µs
//!   - legal action enum   < 50 µs
//!   - single step         < 100 µs
//!
//! The benches drive a random-vs-random game over the seed-card
//! registry using a Lightning Bolt / Grizzly Bears fixture (spec §10
//! "First Milestone Test"), padded to Standard's 60-card minimum so
//! deck-validation-capable future refactors don't need the fixture
//! re-plumbed.
//!
//! The random agent here mirrors `arcana_session::PlayerAgent::Random`
//! but is inlined so the bench has no session-layer dep and measures
//! pure engine throughput.

use arcana_cards::register_seed;
use arcana_core::engine::{new_game_with_format, step, EngineYield};
use arcana_core::legal_actions::legal_actions as enumerate_legal_actions;
use arcana_core::registry::build_deck;
use arcana_core::types::CardId;
use arcana_core::{Action, CardRegistry, FormatConfig, GameState};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

// ----- fixtures ------------------------------------------------------------

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

fn play_one_game(registry: &CardRegistry, seed: u64) -> (GameState, usize) {
    let (mut state, mut yld) = fresh_game(registry, seed);
    let mut rngs = [
        ChaCha8Rng::seed_from_u64(seed.wrapping_add(101)),
        ChaCha8Rng::seed_from_u64(seed.wrapping_add(202)),
    ];
    let mut steps = 0usize;
    loop {
        match yld {
            EngineYield::GameOver(_) => return (state, steps),
            EngineYield::PendingDecision {
                player,
                legal_actions: la,
                ..
            } => {
                let action = pick_random(&mut rngs[player as usize], &la);
                let out = step(state, action, registry);
                state = out.0;
                yld = out.1;
                steps += 1;
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

// ----- benches -------------------------------------------------------------

fn bench_full_game(c: &mut Criterion) {
    let registry = seeded_registry();
    let mut group = c.benchmark_group("full_game");
    group.throughput(Throughput::Elements(1));
    group.sample_size(30);
    let mut seed: u64 = 0;
    group.bench_function("random_vs_random", |b| {
        b.iter(|| {
            seed = seed.wrapping_add(1);
            play_one_game(&registry, seed)
        });
    });
    group.finish();
}

fn bench_state_clone(c: &mut Criterion) {
    let registry = seeded_registry();
    let (state, _) = mid_game(&registry, 17, 120);
    c.bench_function("state_clone_midgame", |b| {
        b.iter(|| std::hint::black_box(state.clone()));
    });
}

fn bench_legal_actions(c: &mut Criterion) {
    let registry = seeded_registry();
    let (state, _yld) = mid_game(&registry, 17, 120);
    c.bench_function("legal_actions_midgame", |b| {
        b.iter(|| std::hint::black_box(enumerate_legal_actions(&state, &registry)));
    });
}

fn bench_single_step(c: &mut Criterion) {
    let registry = seeded_registry();
    let (state, yld) = mid_game(&registry, 17, 120);
    let action = match yld {
        EngineYield::PendingDecision { legal_actions: la, .. } => {
            let mut rng = ChaCha8Rng::seed_from_u64(999);
            pick_random(&mut rng, &la)
        }
        EngineYield::GameOver(_) => Action::PassPriority,
    };
    c.bench_function("single_step_midgame", |b| {
        b.iter_batched(
            || (state.clone(), action.clone()),
            |(st, ac)| std::hint::black_box(step(st, ac, &registry)),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    bench_full_game,
    bench_state_clone,
    bench_legal_actions,
    bench_single_step,
);
criterion_main!(benches);
