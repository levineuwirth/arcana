# arcana-py

Python bindings for the Arcana Engine. v0 is a deliberate stub: the
build pipeline + module surface land now so downstream RL harness
work has a real Python extension to target. The full Phase 4 API
fills in once arcana-core's legal-action enumeration is wired
through.

## Build

```bash
# From the workspace root, in an active Python ≥ 3.10 venv:
pip install maturin
cd arcana-py
maturin develop --release
```

`maturin develop` compiles the cdylib and installs it into the
active venv as `arcana.arcana_py`, importable as `arcana`. Use
`maturin build --release` to produce a wheel in `target/wheels/`.

The build uses PyO3's stable-ABI mode (`abi3-py310`), so the
compiled binary is forward-compatible with future Python releases
(currently tested against 3.14).

## Usage

```python
import numpy as np
import arcana

env = arcana.MtgEnv(num_players=2, seed=0, perspective=0)
obs, info = env.reset()

assert obs.shape == (arcana.BASIC_E2_DIM_TWO_PLAYERS,)
assert obs.dtype == np.float32

# step() raises NotImplementedError in v0 — legal-action enumeration
# is not yet wired through.
```

## Tests

```bash
pip install pytest
pytest arcana-py/tests/
```

## What's stubbed

* `MtgEnv.step()` raises `NotImplementedError`. Reset + observation
  + reward inspection all work today.
* No direct `GameState` / `Action` bindings. v0 keeps the engine
  state opaque on the Rust side.
* No `gymnasium.Env` subclass. The Rust side is duck-typed; a
  Python-level wrapper at `python/arcana/env.py` will handle the
  gymnasium integration when that becomes the consumer's concern.
