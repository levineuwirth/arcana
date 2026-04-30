# Running the bake-off on a single Mac (M-series)

Single-machine workflow. Everything (Ollama server, bake-off
driver, JSONL output, analyzer) runs on one box. For cluster runs
see `README.oscar.md`; for the bake-off-on-Linux-against-Ollama-on-Mac
hybrid see the bottom of this file.

## One-time setup

```bash
brew install ollama                       # or download from ollama.com
brew install rustup-init && rustup-init   # if rust isn't already installed

# Start ollama as a background service. Survives reboot until you
# `brew services stop ollama`.
brew services start ollama                # or: ollama serve & in a spare terminal
```

Verify the server is up:

```bash
curl http://localhost:11434/api/tags
```

## Pull the models

Sizes worth knowing before you run `ollama pull`:

| Tag                                | Disk size | Resident VRAM | Reasonable on        |
| ---------------------------------- | --------- | ------------- | -------------------- |
| `qwen3:235b-instruct-q4_K_M`       | ~140GB    | ~180GB        | M3 Ultra 192GB+      |
| `glm-4.5:latest` (varies by tag)   | varies    | varies        | check `ollama show`  |
| `qwen3:32b-instruct-q4_K_M`        | ~20GB     | ~24GB         | any M-series 32GB+   |
| `qwen2.5-coder:32b-instruct-q4_K_M`| ~20GB     | ~24GB         | any M-series 32GB+   |

`qwen3:235b` paged on a 128GB M3 Max swaps heavily — first-token
latency goes from seconds to minutes. If you don't have unified
memory ≥ 192GB, drop to a 32B-class model for development; reserve
the 235B run for cluster compute (`README.oscar.md`).

```bash
ollama pull qwen3:235b-instruct-q4_K_M
ollama pull glm-4.5:latest
ollama list                               # confirm both landed
```

## Warm the models (avoids first-call timeout)

Cold weight load on a 235B model can exceed `OllamaClient`'s 180s
HTTP timeout. Force a load before invoking the bake-off:

```bash
ollama run qwen3:235b-instruct-q4_K_M "ok"
ollama run glm-4.5:latest "ok"
```

Each exits to shell after the first reply. Models stay resident
for `OLLAMA_KEEP_ALIVE` (default 5 minutes after last request).
Bump it for long bake-off runs:

```bash
launchctl setenv OLLAMA_KEEP_ALIVE 1h
brew services restart ollama
```

## Build

```bash
cd /path/to/arcana-engine
cargo build --release -p arcana-gen --bin bakeoff
cargo build --release -p arcana-gen --bin bakeoff_analyze
```

## Smoke test

Catches any pipeline issue in ~5 minutes instead of in the middle
of an overnight run:

```bash
./target/release/bakeoff \
    --model glm-4.5:latest \
    --sample-size-per-tier 2 \
    --tiers 1,2 \
    --t4-control 0 \
    --max-attempts 2 \
    --no-preflight \
    --seed 0
```

Watch stderr — `precheck passed` then a few "T1 ..." lines, then
"sampled ... cards", then per-card progress. If the precheck fails
the verify pipeline is broken (run `cargo test -p arcana-gen --lib
verify::tests::precheck_passes_on_clean_workspace -- --ignored`
to diagnose). If preflight hangs, the model endpoint is the
problem, not the bake-off.

## Production run

Default invocation (matches what was running locally before the
cluster path):

```bash
./target/release/bakeoff \
    --model qwen3:235b-instruct-q4_K_M \
    --model glm-4.5:latest \
    --sample-size-per-tier 30 \
    --max-attempts 3 \
    --seed 0
```

Wall-clock is dominated by the 235B model's per-call latency on
single-Mac hardware — figure 60-120s per attempt × ~90 supported
cards × 2 models × up to 3 attempts ≈ 9-18 hours. Run it before
you step away; output streams per-row so a mid-run crash loses at
most one in-flight attempt.

To replicate on the same card sample (Mac analog of the cluster
shard pattern):

```bash
# Same cards, two independent model trials:
./target/release/bakeoff --card-seed 0 --model-seed 0 ...   # output A
./target/release/bakeoff --card-seed 0 --model-seed 1 ...   # output B
```

`--seed N` (legacy) sets both `--card-seed N` and `--model-seed N`.
For a single-shot run with no replication, just use `--seed`.

## Analyze

```bash
./target/release/bakeoff_analyze \
    target/bakeoff-runs/<timestamp>.jsonl \
    --top-errors 15 \
    --top-hard 30 \
    | tee target/bakeoff-runs/<timestamp>.report.txt
```

`--format json` if you need machine-readable output. The terminal
output puts `INFRA WARNINGS` (T4 leaks, classifier misses) at the
top so you see whether to trust the rest of the data.

## Variant: drive the bake-off from a Linux box, Ollama on the Mac

Useful if your dev environment is on Linux and the M-series box is
where the unified memory lives.

On the Mac, expose Ollama on the LAN (default binds localhost only):

```bash
launchctl setenv OLLAMA_HOST 0.0.0.0:11434
brew services restart ollama
```

From the Linux side, confirm reachability and submit:

```bash
curl http://<m3-hostname>:11434/api/tags

./target/release/bakeoff \
    --model qwen3:235b-instruct-q4_K_M \
    --model glm-4.5:latest \
    --sample-size-per-tier 30 \
    --max-attempts 3 \
    --seed 0 \
    --ollama-endpoint http://<m3-hostname>:11434
```

Ollama has no auth — only do this on a trusted network. JSONL lands
on whichever box runs the bake-off binary; analyzer runs there too.
