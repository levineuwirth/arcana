//! Performance benchmarks for the simulation loop.
//!
//! Targets (per spec Section 19):
//! - Game state clone: < 5 µs
//! - Legal action enum: < 50 µs
//! - Single action step: < 100 µs
//! - Full game (avg): < 50 ms
//! - Games per second (single thread): > 20,000

use criterion::{criterion_group, criterion_main, Criterion};

fn placeholder_bench(c: &mut Criterion) {
    c.bench_function("placeholder", |b| {
        b.iter(|| {
            // TODO: replace with real simulation benchmark once engine is implemented
            std::hint::black_box(1 + 1)
        })
    });
}

criterion_group!(benches, placeholder_bench);
criterion_main!(benches);
