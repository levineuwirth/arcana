"""Read config.yaml and invoke the arcana bake-off binary against LMStudio.

Thin orchestrator over the Rust binary. Replication is the user's
responsibility for now — this script does one invocation. To
replicate, change config.yaml's `model_seed` and re-run; output
files are seed-tagged so they don't collide.

Usage:
    uv run python run_bakeoff.py
"""

from __future__ import annotations

import json
import shlex
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

import yaml


CONFIG_PATH = Path(__file__).parent / "config.yaml"
SCRIPT_DIR = Path(__file__).parent.resolve()


def load_config() -> dict:
    with open(CONFIG_PATH) as f:
        return yaml.safe_load(f)


def resolve_path(path_str: str) -> Path:
    """Resolve config-relative paths against the script directory.
    Absolute paths pass through unchanged."""
    p = Path(path_str)
    if p.is_absolute():
        return p
    return (SCRIPT_DIR / p).resolve()


def build_invocation(cfg: dict) -> tuple[list[str], Path]:
    bk = cfg["bakeoff"]
    lm = cfg["lmstudio"]

    binary = resolve_path(bk["binary"])
    if not binary.exists():
        sys.exit(
            f"ERROR: bakeoff binary not found at {binary}\n"
            f"  Build with: cargo build --release -p arcana-gen --bin bakeoff"
        )

    output_dir = resolve_path(bk["output_dir"])
    output_dir.mkdir(parents=True, exist_ok=True)
    timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H%M%SZ")
    output_path = (
        output_dir
        / f"lmstudio_{timestamp}_card{bk['card_seed']}_model{bk['model_seed']}.jsonl"
    )

    cmd = [str(binary), "--openai-endpoint", lm["base_url"]]
    for model in lm["models"]:
        cmd += ["--openai-model", model]

    if "extra_body" in lm and lm["extra_body"]:
        cmd += ["--openai-extra-body", json.dumps(lm["extra_body"])]

    cmd += [
        "--sample-size-per-tier", str(bk["sample_size_per_tier"]),
        "--max-attempts", str(bk["max_attempts"]),
        "--tiers", ",".join(str(t) for t in bk["tiers"]),
        "--t4-control", str(bk["t4_control"]),
        "--card-seed", str(bk["card_seed"]),
        "--model-seed", str(bk["model_seed"]),
        "--output", str(output_path),
    ]
    if bk.get("no_preflight"):
        cmd.append("--no-preflight")

    return cmd, output_path


def main() -> int:
    cfg = load_config()
    cmd, output_path = build_invocation(cfg)

    print("=" * 70)
    print("arcana bakeoff via LMStudio")
    print("  output: " + str(output_path))
    print("  invocation:")
    print("    " + " \\\n    ".join(shlex.quote(c) for c in cmd))
    print("=" * 70)

    result = subprocess.run(cmd)
    if result.returncode == 0:
        print(f"\nrun complete; JSONL at {output_path}")
        print(
            "Analyze with:\n"
            f"  cargo run -p arcana-gen --bin bakeoff_analyze --release -- {output_path}"
        )
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())
