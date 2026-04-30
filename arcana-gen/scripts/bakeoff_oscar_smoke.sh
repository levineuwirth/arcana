#!/bin/bash
# 5-minute shakedown for the Oscar pipeline.
#
# Pre-flight check before committing array jobs. Runs one tiny shard
# (5 cards/tier, 1 model, 2 attempts, no T4 control) to validate:
#     - module load works
#     - OLLAMA_MODELS shared dir resolves
#     - Ollama starts and answers /api/tags
#     - the bakeoff binary exists and runs
#     - output lands in /oscar/data/$USER/bakeoff-runs/smoke/
#
# Usage:
#     sbatch arcana-gen/scripts/bakeoff_oscar_smoke.sh

#SBATCH --job-name=arcana-bakeoff-smoke
#SBATCH --partition=gpu-debug
#SBATCH --gres=gpu:1
#SBATCH --constraint=l40s
#SBATCH --cpus-per-task=4
#SBATCH --mem=32G
#SBATCH --time=00:30:00
#SBATCH --output=logs/%x-%j.out
#SBATCH --error=logs/%x-%j.err

set -euo pipefail

module load ollama
module load rust

export OLLAMA_MODELS="${OLLAMA_MODELS:-/oscar/data/shared/ollama_models}"
export OLLAMA_HOST=127.0.0.1:11434
export OLLAMA_KEEP_ALIVE="${OLLAMA_KEEP_ALIVE:-15m}"

REPO="${REPO:-$HOME/arcana-engine}"
DATA_ROOT="${DATA_ROOT:-/oscar/data/$USER/bakeoff-runs}"
SMOKE_MODEL="${SMOKE_MODEL:-glm-4.5:latest}"
mkdir -p "$DATA_ROOT/smoke" logs

echo "=================================================="
echo "arcana smoke test on $(hostname)"
echo "  GPU: $(nvidia-smi --query-gpu=name,memory.total --format=csv,noheader,nounits | head -1)"
echo "  model: $SMOKE_MODEL"
echo "  output: $DATA_ROOT/smoke/smoke-${SLURM_JOB_ID}.jsonl"
echo "=================================================="

ollama serve > "logs/ollama-smoke-${SLURM_JOB_ID}.log" 2>&1 &
OLLAMA_PID=$!
trap 'kill $OLLAMA_PID 2>/dev/null || true' EXIT

for _ in $(seq 1 60); do
    curl -sf http://127.0.0.1:11434/api/tags > /dev/null && break
    sleep 1
done
curl -sf http://127.0.0.1:11434/api/tags > /dev/null || { echo "ollama did not come up" >&2; exit 1; }

# Quick sanity: confirm the model is resolvable.
if ! ollama list | grep -q "$(echo "$SMOKE_MODEL" | sed 's/:.*//')"; then
    echo "warn: $SMOKE_MODEL not in 'ollama list'; the bakeoff will pull it on first call (may add latency)" >&2
fi

cd "$REPO"
"$REPO/target/release/bakeoff" \
    --model "$SMOKE_MODEL" \
    --sample-size-per-tier 5 \
    --max-attempts 2 \
    --tiers 1,2 \
    --t4-control 0 \
    --no-preflight \
    --card-seed 0 \
    --model-seed 0 \
    --ollama-endpoint http://127.0.0.1:11434 \
    --output "$DATA_ROOT/smoke/smoke-${SLURM_JOB_ID}.jsonl"

echo "smoke test complete"
