"""Smoke tests for runnable examples under arcana-py/examples/.

Each example is imported, has its hyperparameters shrunk to a tiny
horizon, and is run end-to-end. This catches API rot — if a future
change to arcana.training breaks the docstring example, this test
fails immediately rather than the example silently going stale.
"""

import importlib.util
import sys
from pathlib import Path

import pytest


torch = pytest.importorskip("torch")


def _load_example(name: str):
    path = Path(__file__).parent.parent / "examples" / f"{name}.py"
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def test_reinforce_demo_runs_with_tiny_horizon():
    """Run the REINFORCE demo at minimum scale.

    The demo's `main()` reads its hyperparameters as module-level
    globals lazily, so reassigning after import shrinks the run
    without editing the example. This is what keeps the example
    runnable as documentation while the smoke test stays fast.
    """
    demo = _load_example("reinforce_demo")
    demo.N_PROBE_EPISODES = 4
    demo.N_EPISODES_PER_ITER = 2
    demo.N_ITERATIONS = 2
    demo.main()
