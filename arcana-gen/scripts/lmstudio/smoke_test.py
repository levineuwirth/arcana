"""Pre-flight checks for the LMStudio-driven bake-off.

Run this before `run_bakeoff.py`. It validates connectivity, that
each configured model is loaded, basic latency, and whether the
LMStudio backend honors the OpenAI `seed` parameter (which varies
by underlying engine — llama.cpp does, mlx-lm doesn't, etc.).

Exit code 0 = ready to run; non-zero = something needs attention.

Usage:
    uv run python smoke_test.py
"""

from __future__ import annotations

import sys
import time
from pathlib import Path

import yaml
from openai import OpenAI


CONFIG_PATH = Path(__file__).parent / "config.yaml"


def load_config() -> dict:
    with open(CONFIG_PATH) as f:
        return yaml.safe_load(f)


def make_client(cfg: dict) -> OpenAI:
    return OpenAI(
        base_url=cfg["lmstudio"]["base_url"],
        api_key=cfg["lmstudio"]["api_key"],
    )


def check_endpoint(client: OpenAI) -> list[str] | None:
    """Return the list of model names visible to LMStudio, or None
    if connectivity fails."""
    try:
        models = client.models.list()
    except Exception as e:
        print(f"FAIL  connectivity: {e}")
        return None
    names = [m.id for m in models.data]
    print(f"PASS  connectivity: {len(names)} models visible")
    return names


def check_models_loaded(visible: list[str], wanted: list[str]) -> bool:
    missing = [m for m in wanted if m not in visible]
    if missing:
        print(f"FAIL  models loaded: missing {missing}")
        print(f"      visible: {visible}")
        return False
    print(f"PASS  models loaded: {wanted}")
    return True


def _gen_kwargs(cfg: dict, model: str, *, seed: int | None = None) -> dict:
    smoke = cfg["smoke"]
    kwargs = {
        "model": model,
        "messages": [{"role": "user", "content": smoke["prompt"]}],
        "max_tokens": smoke["max_tokens"],
    }
    if seed is not None:
        kwargs["seed"] = seed
    if "extra_body" in cfg["lmstudio"]:
        kwargs["extra_body"] = cfg["lmstudio"]["extra_body"]
    return kwargs


def check_latency(client: OpenAI, cfg: dict, model: str) -> bool:
    """One short generation; print wall-clock + reply preview."""
    start = time.time()
    try:
        resp = client.chat.completions.create(**_gen_kwargs(cfg, model))
    except Exception as e:
        print(f"FAIL  latency [{model}]: {e}")
        return False
    elapsed = time.time() - start
    text = (resp.choices[0].message.content or "").strip().replace("\n", " ")
    print(f"PASS  latency [{model}]: {elapsed:.1f}s — '{text[:60]}'")
    return True


def check_seed_honored(client: OpenAI, cfg: dict, model: str) -> None:
    """Run the same prompt twice with a fixed seed; report whether
    outputs match. Informational only — not a hard fail. Some
    LMStudio backends silently ignore seed."""
    seed = cfg["smoke"]["seed"]
    try:
        a = client.chat.completions.create(**_gen_kwargs(cfg, model, seed=seed))
        b = client.chat.completions.create(**_gen_kwargs(cfg, model, seed=seed))
    except Exception as e:
        print(f"WARN  seed honored [{model}]: error: {e}")
        return
    text_a = (a.choices[0].message.content or "").strip()
    text_b = (b.choices[0].message.content or "").strip()
    if text_a == text_b:
        print(f"PASS  seed honored [{model}]: identical outputs under seed={seed}")
    else:
        print(
            f"WARN  seed honored [{model}]: outputs differ under seed={seed}\n"
            f"      a: {text_a[:60]!r}\n"
            f"      b: {text_b[:60]!r}\n"
            f"      The bake-off will still run, but reproducibility is lost."
        )


def main() -> int:
    cfg = load_config()
    client = make_client(cfg)

    visible = check_endpoint(client)
    if visible is None:
        return 1
    if not check_models_loaded(visible, cfg["lmstudio"]["models"]):
        return 1

    all_ok = True
    for model in cfg["lmstudio"]["models"]:
        all_ok &= check_latency(client, cfg, model)
        check_seed_honored(client, cfg, model)

    if not all_ok:
        return 1
    print("\nSmoke test complete. Ready to run `uv run python run_bakeoff.py`.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
