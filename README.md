# Arcana

A high-performance *Magic: The Gathering* rules engine in Rust, designed as a substrate for reinforcement-learning research.

> Phase 1 (core engine) nearly complete. As of 2026-04-21: **919 tests passing** across the workspace (783 core lib, 108 seed integration, 28 peripheral). Phase 2 (effects, keywords, seed cards) actively developed. Phase 3 (agentic card generation) scaffolded.

## Why this exists

*Magic: The Gathering* is one of the most computationally interesting environments still missing a serious open RL benchmark. The state space is enormous, the action space is high-branching and contextual, large portions of state are hidden, and the rules are sufficiently intricate that off-the-shelf simulators routinely sacrifice fidelity for speed. Existing engines are built for human play, not as RL substrates: they lack deterministic replay, they don't expose information-set-projected observations, and their step functions are not pure.

Arcana is built from the ground up to fix those things. The rules engine is a pure Cargo workspace — `arcana-core` exposes a deterministic step function, `arcana-ai` provides legal-action enumeration and observation encoding, and `arcana-py` exposes a Gymnasium-compatible MTG environment via PyO3. The aim is an engine fast enough for large-scale self-play and faithful enough to support meaningful claims about MTG-specific RL strategies.

## Status

- **919 tests** passing across the workspace.
- **42 keyword abilities** wired (14/15 evergreen; flashback, kicker, madness, convoke, delve, equip, enchant, prowess, storm and cascade as full triggers, …).
- **63 effect primitives** of an 80-target set, spanning damage/life, card flow, zone moves, library manipulation, tokens, copies, cascade, counters, P/T modifications, stack manipulation, prompts, mana, phase steps, decision-requiring effects, and composites.
- **37 seed cards across 21 sets** — vanilla, activated, triggered (incl. targeted), modal, split, adventure, MDFC, continuous-effect, planeswalker, alt/additional-cost (kicker/madness/convoke/delve), hybrid mana. Zero `todo!()` / `unimplemented!()` in the seed roster.
- **Deterministic replay** via ChaCha8-seeded RNG + full event log. All three mulligan variants (London, Paris, Vancouver) and scry implemented.
- **CR coverage:** §100, §103, §117, §400.7, §500–510 (incl. 510.1c combat-step edge), §601–608 (incl. 608.2b recheck), §613.1–7, §702 (42 keywords), §704.5, §711.4 (split combine), §712.2b/4 (MDFC), §715 (adventure).

## Workspace

Cargo workspace, seven crates:

| Crate            | Role                                                                 |
| ---------------- | -------------------------------------------------------------------- |
| `arcana-core`    | Pure rules engine. Game state, turn structure, priority, stack, combat, layers, zones. No card-specific logic. |
| `arcana-cards`   | Card registry. Generated card code lives in `src/generated/`.        |
| `arcana-session` | Session layer wrapping the pure step function for human-play flows. |
| `arcana-ai`      | RL-facing utilities: legal-action enumeration, observation encoding, reward computation, information-set projection. |
| `arcana-py`      | PyO3 bindings exposing a Gymnasium-compatible `MtgEnv`.              |
| `arcana-gen`     | Agentic card-generation pipeline: Scryfall parsing, classifier, prompt rendering, cargo-check verify, bake-off driver. |
| `arcana-cli`     | Developer tools: interactive debugger, state inspector, replay viewer, benchmarks. |

## Build & test

```bash
cargo check                    # Type-check workspace
cargo test                     # All non-ignored tests
cargo test -p arcana-core      # Specific crate
cargo bench -p arcana-core     # Benchmarks
```

Tests tagged `#[ignore]` spawn external processes (cargo, network) and are run explicitly:

```bash
cargo test -p arcana-gen --lib -- --ignored --test-threads=1
```

## Roadmap

| Phase | Scope                                                     | Status              |
|-------|-----------------------------------------------------------|---------------------|
| 1     | Core engine (state, turn, priority, stack, combat, layers, mulligan) | nearly complete |
| 2     | Effects, keywords, seed cards, session layer              | active              |
| 3     | Agentic card-generation pipeline                          | scaffolded          |
| 4     | Python / AI / Gym bindings                                | scaffolded          |
| 5     | Polish, research, human play                              | not yet started     |

Performance targets (not yet measured): >20,000 games/sec single-threaded, <10 μs observation encoding, <20% PyO3 overhead. Bench harness is the next deliverable on the Phase 1 task list.

## Author

Levi Neuwirth — [ln@levineuwirth.org](mailto:ln@levineuwirth.org) · [levineuwirth.org](https://levineuwirth.org)
