# arcana-py

Python bindings for the Arcana Engine. The self-play path is
functional today: `run_episode` plays full N-policy games and returns
trajectories + outcomes, with built-in policy factories (random /
progress-biased / first-action) and the 123-float `BasicE2Encoder`
observation exposed to Python. What is *not* yet wired is the
gym-style per-action loop: `MtgEnv.step()` remains a stub pending
arcana-core's legal-action enumeration being surfaced through PyO3.

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

# env.step() raises NotImplementedError in v0 — legal-action
# enumeration is not yet wired through. For self-play today, drive
# whole episodes with run_episode (below) instead of stepping.
```

### Self-play episodes (the working path)

```python
import arcana

# Defaults to random policies seeded from `seed`; deterministic.
result = arcana.run_episode(seed=0)

print(result.outcome.winner)      # 0, 1, or None (draw/truncated)
print(result.steps_taken, result.turns_taken)

# Per-player trajectories carry the encoded observations + reward.
for traj in result.trajectories:
    print(traj)                   # Trajectory(perspective=…, n_steps=…, final_reward=…)

# A policy slot also accepts a plain callable (obs, n_legal) -> action_index:
def first_action(obs, n_legal):
    return 0
result = arcana.run_episode(seed=1, policy_a=first_action)
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
