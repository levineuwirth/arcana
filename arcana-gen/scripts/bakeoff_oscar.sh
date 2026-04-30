#!/bin/bash
# Bake-off launcher for Brown CCV Oscar.
#
# Submit as a job array; each task runs one shard with a distinct
# `--model-seed` and a constant `--card-seed`. Aggregate the
# resulting JSONL files locally with bakeoff-analyze.
#
# Usage:
#     sbatch --array=0-49 arcana-gen/scripts/bakeoff_oscar.sh
#
# Environment overrides (set before sbatch or via `--export`):
#     MODELS              Space-separated Ollama tags. Default: a small mix.
#     CARD_SEED           Card-sampler seed, held constant across the array.
#                         Default 0.
#     SAMPLE_PER_TIER     Cards per tier, per shard. Default 30.
#     MAX_ATTEMPTS        Retry budget per (card, model). Default 3.
#     TIERS               Comma-sep tier list. Default 1,2,3.
#     T4_CONTROL          T4 sanity-anchor sample size. Default 10.
#     REPO                Path to your arcana-engine clone. Default $HOME/arcana-engine.
#     DATA_ROOT           Where shard JSONL goes. Default /oscar/data/$USER/bakeoff-runs.
#     OLLAMA_KEEP_ALIVE   How long Ollama keeps weights resident. Default 1h.
#
# Resource requests are chosen for the multi-model "big" case
# (Qwen3 235B + a peer): 4× L40s = 192GB VRAM, 64 cores, 256GB RAM,
# 12 hours. For smaller-model runs, copy this script and shrink.

#SBATCH --job-name=arcana-bakeoff
#SBATCH --partition=gpu
#SBATCH --gres=gpu:4
#SBATCH --constraint=l40s
#SBATCH --cpus-per-task=8
#SBATCH --mem=64G
#SBATCH --time=12:00:00
#SBATCH --output=logs/%x-%A_%a.out
#SBATCH --error=logs/%x-%A_%a.err

set -euo pipefail

# --- modules + env -------------------------------------------------
module load ollama
module load rust

# Pre-hosted shared model cache. Override with a personal dir if you
# need to `ollama pull` a non-pre-hosted tag.
export OLLAMA_MODELS="${OLLAMA_MODELS:-/oscar/data/shared/ollama_models}"
export OLLAMA_HOST=127.0.0.1:11434
export OLLAMA_KEEP_ALIVE="${OLLAMA_KEEP_ALIVE:-1h}"

REPO="${REPO:-$HOME/arcana-engine}"
DATA_ROOT="${DATA_ROOT:-/oscar/data/$USER/bakeoff-runs}"
RUN_TAG="${RUN_TAG:-${SLURM_ARRAY_JOB_ID:-$SLURM_JOB_ID}}"
TASK_ID="${SLURM_ARRAY_TASK_ID:-0}"

mkdir -p "$DATA_ROOT/$RUN_TAG" logs

# --- params --------------------------------------------------------
MODELS="${MODELS:-qwen3:235b-instruct-q4_K_M glm-4.5:latest}"
CARD_SEED="${CARD_SEED:-0}"
SAMPLE_PER_TIER="${SAMPLE_PER_TIER:-30}"
MAX_ATTEMPTS="${MAX_ATTEMPTS:-3}"
TIERS="${TIERS:-1,2,3}"
T4_CONTROL="${T4_CONTROL:-10}"

# Build the --model arg list from the space-separated MODELS env var.
MODEL_ARGS=()
for m in $MODELS; do
    MODEL_ARGS+=(--model "$m")
done

OUTPUT="$DATA_ROOT/$RUN_TAG/shard_${TASK_ID}.jsonl"

echo "=================================================="
echo "arcana bakeoff Oscar shard"
echo "  job_id:        ${SLURM_JOB_ID}"
echo "  array:         ${SLURM_ARRAY_JOB_ID:-N/A} task ${TASK_ID}"
echo "  node:          $(hostname)"
echo "  GPU info:      $(nvidia-smi --query-gpu=name,memory.total --format=csv,noheader,nounits | head -1)"
echo "  models:        $MODELS"
echo "  card_seed:     $CARD_SEED  (constant across array)"
echo "  model_seed:    $TASK_ID    (= SLURM_ARRAY_TASK_ID)"
echo "  sample/tier:   $SAMPLE_PER_TIER"
echo "  max attempts:  $MAX_ATTEMPTS"
echo "  output:        $OUTPUT"
echo "=================================================="

# --- start Ollama in the background --------------------------------
ollama serve > "logs/ollama-${SLURM_JOB_ID}.log" 2>&1 &
OLLAMA_PID=$!
trap 'kill $OLLAMA_PID 2>/dev/null || true' EXIT

# Wait for the API to come up. Polling is cheaper than a fixed
# sleep, and surfaces "ollama failed to launch" within seconds
# instead of after a minute of dead air.
for _ in $(seq 1 60); do
    if curl -sf http://127.0.0.1:11434/api/tags > /dev/null; then
        echo "ollama up"
        break
    fi
    sleep 1
done
if ! curl -sf http://127.0.0.1:11434/api/tags > /dev/null; then
    echo "ERROR: ollama did not come up within 60s; aborting" >&2
    exit 1
fi

# --- run -----------------------------------------------------------
cd "$REPO"
BIN="$REPO/target/release/bakeoff"
if [ ! -x "$BIN" ]; then
    echo "ERROR: $BIN not found. Run \`cargo build --release -p arcana-gen --bin bakeoff\` once before submitting." >&2
    exit 1
fi

"$BIN" \
    "${MODEL_ARGS[@]}" \
    --sample-size-per-tier "$SAMPLE_PER_TIER" \
    --max-attempts "$MAX_ATTEMPTS" \
    --tiers "$TIERS" \
    --t4-control "$T4_CONTROL" \
    --card-seed "$CARD_SEED" \
    --model-seed "$TASK_ID" \
    --ollama-endpoint http://127.0.0.1:11434 \
    --output "$OUTPUT"

echo "shard ${TASK_ID} done; output at ${OUTPUT}"
