//! Microbenchmarks for the observation encoder.
//!
//! Per-encode time matters: every legal-action eval in MCTS, every
//! state in a rollout buffer, and every batched policy forward pass
//! hits the encoder. Even sub-microsecond regressions multiply.
//!
//! Two variants are measured deliberately:
//!
//! * **`encode_into`** — the hot-path API. Caller-supplied buffer,
//!   zero allocation per call. This is the number that will appear
//!   in training-loop budgets.
//! * **`encode`** — the convenience wrapper that allocates a fresh
//!   `Vec<f32>` per call. Comparing the two surfaces the heap-alloc
//!   tax so we can keep the gap honest if it ever closes
//!   unexpectedly.
//!
//! Run with: `cargo bench -p arcana-ai --bench observation`.

use arcana_ai::observation::{BasicE2Encoder, Encoder, BASIC_E2_DIM_TWO_PLAYERS};
use arcana_core::state::GameState;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_encode(c: &mut Criterion) {
    let encoder = BasicE2Encoder::for_two_players();
    let state = GameState::new(2, 0);
    let mut buf = vec![0.0f32; BASIC_E2_DIM_TWO_PLAYERS];

    c.bench_function("encode_into / 2-player initial state", |b| {
        b.iter(|| {
            encoder.encode_into(black_box(&state), Some(0), black_box(&mut buf));
        });
    });

    c.bench_function("encode / 2-player initial state (allocates)", |b| {
        b.iter(|| {
            let v = encoder.encode(black_box(&state), Some(0));
            black_box(v);
        });
    });
}

criterion_group!(benches, bench_encode);
criterion_main!(benches);
