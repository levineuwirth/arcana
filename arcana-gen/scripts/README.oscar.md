# Running the bake-off on Brown CCV Oscar

Two scripts:
- `bakeoff_oscar_smoke.sh` — single 30-minute shakedown on `gpu-debug`. Run this first.
- `bakeoff_oscar.sh` — array-job production template, parameterized by env vars.

Both scripts assume the bake-off binary is **pre-built** at `$REPO/target/release/bakeoff`. Submit-time builds invite race conditions across array tasks; build once interactively.

## One-time setup

On a login node:

```bash
module load rust
cd $HOME/arcana-engine                 # adjust to your clone path
cargo build --release -p arcana-gen --bin bakeoff
mkdir -p logs                          # SLURM stderr/stdout land here
```

If you need a model that isn't pre-hosted in `/oscar/data/shared/ollama_models`, pull it once into a personal cache:

```bash
mkdir -p /oscar/data/$USER/ollama_models
export OLLAMA_MODELS=/oscar/data/$USER/ollama_models
ollama serve &     # on a GPU node, in an interactive session
ollama pull <tag>
```

Then prepend that path to `OLLAMA_MODELS` when submitting:

```bash
sbatch --export=ALL,OLLAMA_MODELS=/oscar/data/$USER/ollama_models:/oscar/data/shared/ollama_models \
       arcana-gen/scripts/bakeoff_oscar.sh
```

## Step 1: smoke test

```bash
sbatch arcana-gen/scripts/bakeoff_oscar_smoke.sh
```

Watch `logs/arcana-bakeoff-smoke-<jobid>.out`. Expect ~5-10 minutes of compute on a `gpu-debug` L40s. Output lands at `/oscar/data/$USER/bakeoff-runs/smoke/`. If this works, the pipeline is wired.

## Step 2: pick GPU + model fit

The big-model picks for the cluster:

| Model size                          | Resource ask                                    | Notes                                                                  |
| ----------------------------------- | ----------------------------------------------- | ---------------------------------------------------------------------- |
| Qwen3 235B q4_K_M (~180GB resident) | `--gres=gpu:1 --constraint=b200` on `gpu-he`    | Cleanest. B200 has 192GB; node-rare, expect queue.                     |
| Qwen3 235B q4_K_M alternative       | `--gres=gpu:4 --constraint=l40s` on `gpu`       | 4×48=192GB, Ollama splits across GPUs. Default in `bakeoff_oscar.sh`.  |
| 70B q4 (~42GB)                      | `--gres=gpu:1 --constraint=l40s` on `gpu`       | Single L40s comfortably fits.                                          |
| 32B q4 (~20GB)                      | `--gres=gpu:1` on `gpu` (any partition default) | A5000 / A5500 / 3090 all work.                                         |

Edit the `#SBATCH --gres=` and `#SBATCH --constraint=` lines in `bakeoff_oscar.sh` to match the smallest fit for your model lineup. Smaller asks queue faster.

**About multi-model jobs.** The default script runs `qwen3:235b-instruct-q4_K_M` and `glm-4.5:latest` in the same shard. If both don't fit in VRAM simultaneously, Ollama swaps them in/out per call — correct but slower. To avoid the swap cost, run one model per array (set `MODELS` to a single tag) and submit two separate array jobs, then concatenate the JSONL.

## Step 3: production array

Default: 50 shards × 30 cards/tier × 3 tiers × 2 models. Each shard gets a distinct `--model-seed` (its `SLURM_ARRAY_TASK_ID`); all shards share `--card-seed=0`.

```bash
sbatch --array=0-49 arcana-gen/scripts/bakeoff_oscar.sh
```

Knobs (set before `sbatch` or via `--export`):

| Variable           | Default                                           |
| ------------------ | ------------------------------------------------- |
| `MODELS`           | `qwen3:235b-instruct-q4_K_M glm-4.5:latest`       |
| `CARD_SEED`        | `0`                                               |
| `SAMPLE_PER_TIER`  | `30`                                              |
| `MAX_ATTEMPTS`     | `3`                                               |
| `TIERS`            | `1,2,3`                                           |
| `T4_CONTROL`       | `10`                                              |
| `REPO`             | `$HOME/arcana-engine`                             |
| `DATA_ROOT`        | `/oscar/data/$USER/bakeoff-runs`                  |
| `OLLAMA_KEEP_ALIVE`| `1h`                                              |

Examples:

```bash
# Quick 4-shard validation across 16 cards/tier:
SAMPLE_PER_TIER=16 sbatch --array=0-3 arcana-gen/scripts/bakeoff_oscar.sh

# Big run, 100 shards, single-model:
MODELS="qwen3:235b-instruct-q4_K_M" sbatch --array=0-99 arcana-gen/scripts/bakeoff_oscar.sh

# Different card slice (different sample); same shard count:
CARD_SEED=42 sbatch --array=0-49 arcana-gen/scripts/bakeoff_oscar.sh
```

Each array job creates `/oscar/data/$USER/bakeoff-runs/<run_tag>/shard_<task_id>.jsonl`. The `run_tag` defaults to `SLURM_ARRAY_JOB_ID` so multiple array submissions don't collide.

## Step 4: monitor

```bash
myq                                                # your queued + running jobs
sacct -j <array_job_id> --format=JobID,State,ExitCode,Elapsed
tail -f logs/arcana-bakeoff-<array_job_id>_<task_id>.out
```

Failed shards: re-submit with `sbatch --array=<failed_ids> arcana-gen/scripts/bakeoff_oscar.sh` against the same `RUN_TAG`. Each shard's output is independent; replacing a single shard's file is safe.

## Step 5: pull results back + analyze

```bash
# From your local dev box:
rsync -avz oscar:/oscar/data/$USER/bakeoff-runs/<run_tag>/ ./local-runs/<run_tag>/
cat ./local-runs/<run_tag>/shard_*.jsonl > ./local-runs/<run_tag>/combined.jsonl
cargo run -p arcana-gen --bin bakeoff_analyze --release -- \
    ./local-runs/<run_tag>/combined.jsonl \
    --top-errors 15 \
    --top-hard 30 \
    | tee ./local-runs/<run_tag>/report.txt
```

**Analyzer caveat for sharded runs.** The pass-rate matrix and error histograms aggregate correctly across shards (each row is one trial; counts and stats accumulate). The `corpus_hard` / `corpus_brittle` / `model_split` buckets currently treat each (card, model) row as a single trial — with N shards, these categories will surface every row as a separate datapoint. Read those sections per-shard, or wait for the analyzer to gain a shard-aware aggregation pass (filed as a deferred follow-up).

## Recovery hints

- **"ollama did not come up"** in shard logs → check `logs/ollama-<jobid>.log`. Most common cause is a stale `OLLAMA_HOST` or a port collision on the node.
- **Out-of-memory model load** → check the `--constraint` matched the model's VRAM need. Multi-GPU L40s requires the 4-GPU `--gres=gpu:4` ask; a single L40s won't load Qwen3 235B.
- **Long queue waits** on `--constraint=b200` → fall back to `--constraint=l40s` with `--gres=gpu:4`. Less rare hardware, faster start.
- **Re-running a single shard** for replay or debugging:
  ```bash
  sbatch --export=ALL,RUN_TAG=<existing_tag> --array=<task_id> arcana-gen/scripts/bakeoff_oscar.sh
  ```
