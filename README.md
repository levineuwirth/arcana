# Arcana Engine

A high-performance Magic: The Gathering rules engine in Rust, designed as a
substrate for AI research.

## Workspace layout

Cargo workspace, seven crates:

| Crate            | Role                                                                 |
| ---------------- | -------------------------------------------------------------------- |
| `arcana-core`    | Rules engine. Game state, turn structure, priority, stack, combat, layers, zones. No card-specific logic. |
| `arcana-cards`   | Card registry. Generated card code lives in `src/generated/`.        |
| `arcana-session` | Session layer wrapping the pure `arcana-core` step function for human-play flows. |
| `arcana-ai`      | AI-facing utilities: legal-action enumeration, observation encoding, reward computation, information-set projection. |
| `arcana-py`      | PyO3 bindings exposing a Gymnasium-compatible `MtgEnv`.              |
| `arcana-gen`     | Agentic card-generation pipeline: Scryfall parsing, classifier, prompt rendering, cargo-check verify, bake-off driver + analyzer. |
| `arcana-cli`     | Developer tools: interactive debugger, state inspector, replay viewer, benchmarks. |

## Key documents

- `arcana_engine_spec.pdf` — engine specification (v0.1).
- `arcana_engine_addendum.pdf` — addendum filling spec gaps with concrete types and the Phase-1 task breakdown (v0.1.1).
- `CLAUDE.md` — codebase conventions and design principles.
- `KEYWORDS.md` — keyword-ability implementation tracking.

## Build & test

```bash
cargo check                    # Type-check workspace
cargo test                     # Run all non-ignored tests
cargo test -p arcana-core      # Test a specific crate
cargo bench -p arcana-core     # Benchmarks
```

Tests tagged `#[ignore]` spawn external processes (cargo, network) and are run
explicitly, e.g.:

```bash
cargo test -p arcana-gen --lib -- --ignored --test-threads=1
```

## Status

Phase 1 (core engine) — in progress. See the addendum's Section 13 for the
21-task breakdown. Phase 3 card generation is being prepared in parallel via
`arcana-gen`.
