# Runbook — bulk card generation via Claude Code subagents

This is a **self-contained, machine-portable procedure**. It does not
depend on any prior chat session. Everything needed is in this branch
(`cardgen-subagent-pipeline`) plus this file. A fresh Claude Code
session on any machine can execute it end to end.

## What this does

`dump prompts → parallel subagent generation → 3-layer batched verify
→ review → land`. Proven on a 126-card T1 run at **99% pass**; this
runbook targets the **safe declarative class** (vanilla +
french-vanilla creatures: pure stats + keywords, no resolver bodies,
so Layer 2 fully certifies and there is no rules-text semantic risk).

Pool sizes (all sets, `card_seed 0`): **343 vanilla + 682
french-vanilla = 1,025** cards. That is the entire safe pool — it is
deterministic and complete, not a sample.

## Prerequisites

- Repo checked out on branch `cardgen-subagent-pipeline`.
- Rust toolchain (workspace builds with stable).
- Network access on the **first** run only (downloads the Scryfall
  oracle bulk into the gitignored `target/scryfall-cache/`; cached
  thereafter).
- Run from the workspace root (`.../mtg`).

## Step 1 — Dump prompts (deterministic, no LLM, no GPU)

```bash
cargo run -q -p arcana-gen --bin bakeoff -- \
  --dump-prompts target/cardgen/run01 \
  --tiers 1,2 --shapes vanilla,french-vanilla \
  --all-sets --sample-size-per-tier 100000 \
  --t4-control 0 --no-preflight --card-seed 0
```

Expect: `dumped 1025 prompt(s), … → target/cardgen/run01`. Produces
`target/cardgen/run01/manifest.jsonl` and
`target/cardgen/run01/prompts/NNN_slug.txt` (one per card).
`card_seed 0` makes the set reproducible.

## Step 2 — Generate (parallel Claude Code subagents)

The generation step IS a fleet of Claude Code subagents. Spawn them
in parallel (one Agent call each, all in one message). Size the
fleet to the machine — on a strong desktop, **~20 agents ×
~50 cards** is reasonable. Each agent owns a contiguous 3-digit
index range and globs its slice.

Prompt files are numbered `000…1024`. Assign agent _k_ the range
`[k·S, k·S+S-1]`. Use `subagent_type: general-purpose`,
`model: sonnet` (proven sufficient for this class; faster/cheaper
than opus at ~equal yield here).

**Verbatim per-agent prompt template** (substitute LO, HI, and the
dir):

> Code-generation task for the Arcana MTG engine. Directory:
> `<ABS>/target/cardgen/run01`
>
> List the files in `prompts/`. Process EVERY file whose 3-digit
> numeric prefix NNN is in the range `LO`–`HI` inclusive (some
> numbers may be absent — process whatever exists).
>
> For each: the file has `#` header lines, then `===== SYSTEM =====`
> (authoritative system instructions), then `===== USER =====` (the
> task). Produce EXACTLY the Rust source the prompt asks for — no
> markdown fences, no prose, just the `.rs` body starting with
> `//!`. Obey the prompt's API-DISCIPLINE rule strictly: use ONLY
> types/constructors/variants/methods shown in that prompt's
> reference example; never invent APIs; do NOT read other
> repository files. Write each result with the Write tool to the
> sibling path = same directory (NOT `prompts/`), filename = prompt
> filename with `.txt`→`.rs`. Do not run cargo. Report the files
> you wrote.

These are vanilla/french-vanilla creatures: stats, types, colors,
subtypes, and (french-vanilla) evergreen keywords only. No
resolver/effect functions.

## Step 3 — Verify (batched: one cargo check + one cargo test per chunk)

```bash
cargo build -q -p arcana-gen --bin verify_dir
./target/debug/verify_dir --dir target/cardgen/run01 --batch
```

`--batch` collapses 2·N cargo invocations to 2·⌈N/128⌉ — the
throughput lever; results are byte-identical to the serial path
(verified). Writes `target/cardgen/run01/verify-report.jsonl` and
prints a summary. Exit code is non-zero if any card failed (that is
expected when there are stragglers; it is not an error).

Expected, by analogy to the 126-card pilot (99%): the vast majority
PASS; a small % L1-fail on one-off slips (e.g. a model using `|=`
where `ColorSet` only impls `BitOr`). L3 stubs should be ~0 (this
class has no resolver). Failures are **quarantined, not landed** —
the 1% straggler is handled by a human or a re-gen of just those
indices, not by blocking the batch.

## Step 4 — Review & land

```bash
# Dry-run: shows every set module / lib.rs edit, writes nothing.
python3 arcana-gen/scripts/land_cards.py --dir target/cardgen/run01

# Spot-check a sample of the generated .rs by hand. For this class
# Layer 2 already certifies name/cost/colors/types/P-T/keywords vs
# Scryfall, so review is light — confirm a handful look sane.

python3 arcana-gen/scripts/land_cards.py --dir target/cardgen/run01 --apply

cargo check -p arcana-cards
cargo test  -p arcana-cards --lib     # seed regression
```

`land_cards.py` writes one file per card into its canonical set
module, creates set `mod.rs` + the `pub mod` line in `lib.rs`,
sanitizes digit-leading Scryfall codes (`10e`→`s10e`), is
idempotent (skips existing unless `--force`), and **does not** touch
`register_seed` (landed cards are catalog, not the curated seed).

## Invariants / gotchas

- Determinism: same `--card-seed` + same Scryfall cache = same set.
- Scratch slots (`candidate*`) are gitignored and RAII-restored on
  verify exit; an aborted run never leaves the workspace broken.
- `arcana-cards/build.rs` owns `_scratch/mod.rs` + slot bootstrap;
  `N_SCRATCH_SLOTS` (128) is the batch chunk cap.
- Do **not** run multiple `verify_dir`/cargo against this workspace
  concurrently — cargo holds a target lock; batching is the
  parallelism, not concurrent cargo.
- To extend to spells (T2 SingleEffectSpell): NOT safe yet — needs
  the few-shot Effect pack expanded; those stub-prone cards are
  excluded here by `--shapes`.
