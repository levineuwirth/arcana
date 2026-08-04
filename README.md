# Arcana

A *Magic: The Gathering* rules engine in Rust — a rules-accurate core, a
generated card catalog of over 20,000 cards, a playable web client, and a
frozen line of RL research that motivated the whole project.

For current project status and the active work plan, see
[`docs/audit-2026-08.md`](docs/audit-2026-08.md) — it supersedes the
status/roadmap docs below it in this README.

## What's here

- **`arcana-core`** — a pure-Rust MTG rules engine. Deterministic step
  function, full turn structure, priority, stack, combat, continuous-effect
  layers, zones. ~60,000 lines, zero `unsafe`. No card-specific logic.
- **`arcana-cards`** — a generated card registry: **20,590 cards**
  implemented against the engine (of ~20,700+ card files on disk), produced
  by an agentic generation pipeline and gated by a behavioral-probe test
  (cards are resolved in a populated game state, not just type-checked, to
  catch silent no-op scripts).
- **`arcana-web`** — a playable browser client (axum + a vanilla-JS
  frontend): play solo against a bot, or start a LAN match with a friend via
  a 4-character lobby code. See [Quickstart](#quickstart-web-client) below.
- **`arcana-cli`** — developer/player tools: an interactive terminal game
  (`play`), self-play recording, replay viewing, and bot-strength evaluation
  (`selfplay` / `replay` / `eval` / `arena`).
- **`arcana-session`** — a session layer wrapping the pure engine step
  function for human-play flows (used by both `arcana-cli` and
  `arcana-web`).
- **`arcana-ai`** — RL-facing utilities: legal-action enumeration,
  observation encoding, information-set projection/determinization, and
  several search policies (flat Monte Carlo, PIMC, ISMCTS, a learned-value
  leaf). Also home to the frozen research code below.
- **`arcana-py`** — PyO3 bindings exposing a Gymnasium-compatible `MtgEnv`
  for Python-side experimentation.
- **`arcana-gen`** — the agentic card-generation pipeline: Scryfall
  ingestion, prompt rendering, subagent codegen, cargo-check + behavioral
  verify, and the bake-off/regen tooling used to grow the catalog.

| Crate            | Role                                                                 |
| ---------------- | -------------------------------------------------------------------- |
| `arcana-core`    | Pure rules engine — state, turn structure, priority, stack, combat, layers, zones. |
| `arcana-cards`   | Generated card registry (20,590 cards), behavioral-probe gated.      |
| `arcana-ai`      | Legal-action enumeration, observation encoding, information sets, search policies; frozen RL research. |
| `arcana-py`      | PyO3 bindings exposing a Gymnasium-compatible `MtgEnv`.               |
| `arcana-gen`     | Agentic card-generation pipeline (Scryfall parsing, prompting, verify, regen). |
| `arcana-cli`     | Interactive play, self-play, replay, and bot-strength evaluation.    |
| `arcana-session` | Session layer wrapping the pure step function for human-play flows.  |
| `arcana-web`     | Browser client: solo-vs-bot and LAN 2-player over HTTP/WebSocket.    |

## Quickstart: web client

```bash
cargo run -p arcana-web
```

Opens on `http://127.0.0.1:8080` by default. Solo play works immediately;
click "Play a friend" for a 4-character lobby code the other player enters
on their machine (same LAN, or `HOST=0.0.0.0` — see below — plus port
forwarding for anything further).

Environment variables (all optional):

| Var | Effect | Default |
|---|---|---|
| `HOST` | Bind address. Set `0.0.0.0` to expose on the LAN so a friend can join. | `127.0.0.1` (loopback only) |
| `PORT` | Bind port. | `8080` |
| `ARCANA_ART_CACHE` | Directory for cached Scryfall card-art images. | `$HOME/.cache/arcana/art` (or a temp dir if `$HOME` is unset) |
| `MATCH_STATE_DIR` | Directory to persist networked-match transcripts to (opt-in; unset means matches are memory-only and don't survive a restart). | unset |
| `MATCH_TIMEOUT_SECS` | How long a networked match waits for a vanished peer before reaping it. | `60` |

Solo-game routes (`/state`, `/action`, `/new`, …) are restricted to the host
machine even on a LAN bind; the `/lobby/*` and `/m/*` networked-match routes
are the ones meant to be reachable by a guest.

## Quickstart: terminal play

```bash
cargo run -p arcana-cli -- play
```

Defaults to a human (you) vs. a bot (`snappy`, short-rollout value-MC).
Flags: `--p0 KIND --p1 KIND --seed N`, where `KIND` is one of
`human|snappy|pimc|mc|random`. Run `cargo run -p arcana-cli` with no
arguments for the full command list (`selfplay`, `replay`, `eval`, `arena`).

## Build & test

```bash
cargo build --workspace
cargo test --workspace       # ~1,624 tests across the workspace
cargo test -p arcana-core    # a specific crate
```

Tests tagged `#[ignore]` spawn external processes (cargo, network) and are
run explicitly:

```bash
cargo test -p arcana-gen --lib -- --ignored --test-threads=1
```

## Why this exists

*Magic: The Gathering* is one of the most computationally interesting
environments still missing a serious open RL benchmark: an enormous state
space, a high-branching contextual action space, large portions of hidden
state, and rules intricate enough that off-the-shelf simulators routinely
sacrifice fidelity for speed. That's the origin of this project — `arcana-core`
was built as an RL substrate first (deterministic step function,
information-set-projected observations, a pure step boundary), with
`arcana-ai` and `arcana-py` as the research-facing layers on top of it.

Performance targets were set early (>20,000 games/sec single-threaded,
<10 μs observation encoding, <20% PyO3 overhead) but were **never formally
measured** — the project's center of gravity shifted toward building out
the engine, the card catalog, and a playable client instead.

The RL research arcs that were run (self-play value-function learning,
deck-evaluation/deckbuilding) are **frozen as closed, documented negative
results** — see [`docs/rl-status.md`](docs/rl-status.md) and
[`docs/rl-benchmark.md`](docs/rl-benchmark.md) for what was tried and why
it's parked. The one live thread for a future resumption (PIMC action
distillation) is noted in [`docs/audit-2026-08.md`](docs/audit-2026-08.md).

## Status & roadmap

Current status, known issues, and the active work plan live in
[`docs/audit-2026-08.md`](docs/audit-2026-08.md) — treat it as the
up-to-date source of truth. `STATUS_2026-04-21.md`, `KEYWORDS.md`, and
`ROADMAP.md` are earlier snapshots kept for history; each now carries a
banner pointing here.

## Author

Levi Neuwirth — [ln@levineuwirth.org](mailto:ln@levineuwirth.org) · [levineuwirth.org](https://levineuwirth.org)
