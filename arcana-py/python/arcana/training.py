"""Training-loop scaffolding for arcana self-play.

This module provides the boring plumbing — episode collection,
return computation, batch assembly, diagnostics — so you can focus
on the research bits (model shape, optimizer, algorithm) without
rewriting data flow each time.

`torch` is imported lazily and only required by `to_torch()`. The
rest of the module works with plain numpy arrays so it's usable
in environments without PyTorch.

# Example: REINFORCE-style training loop sketch

```python
import torch
import torch.nn.functional as F
import arcana
from arcana.training import (
    collect_episodes,
    build_batch,
    to_torch,
    episode_summary,
)

# Your model + optimizer (research call):
model = MyPolicyNet(arcana.BASIC_E2_DIM_TWO_PLAYERS, action_dim=...)
optimizer = torch.optim.Adam(model.parameters(), lr=3e-4)

for iteration in range(500):
    # Wrap the model as a Python policy callable. Mind the GIL cost
    # — every step is one Python callback. For real throughput,
    # collect with a Rust-side baseline policy and re-evaluate
    # actions through the model offline.
    def policy(obs, n_legal, **kwargs):
        with torch.no_grad():
            logits = model(torch.from_numpy(obs).unsqueeze(0))
            return int(torch.argmax(logits[0, :n_legal]).item())

    episodes = collect_episodes(
        n_episodes=64,
        policy_a=policy,
        policy_b=arcana.policies.progress_biased(seed=iteration),
        base_seed=iteration * 1000,
    )

    # Diagnostics — log win rate, episode length, truncation rate.
    stats = episode_summary(episodes, perspective=0)
    print(f"iter {iteration}: win_rate={stats['win_rate']:.2f}  "
          f"mean_turns={stats['mean_turns']:.1f}  "
          f"truncated={stats['n_truncated']}/{stats['n']}")

    # Build a batch of (obs, action, return) tuples from the
    # perspective-0 trajectories. Truncated episodes are skipped
    # by default so the policy isn't trained to be okay with
    # running out the clock.
    batch = build_batch(episodes, perspective=0, gamma=1.0)
    if batch['observations'].shape[0] == 0:
        continue  # all episodes truncated, skip update
    batch = to_torch(batch, device='cpu')

    # Loss computation (your call — REINFORCE shown):
    logits = model(batch['observations'])
    log_probs = F.log_softmax(logits, dim=-1)
    chosen_log_probs = log_probs.gather(
        1, batch['action_indices'].unsqueeze(1)
    ).squeeze(1)
    loss = -(chosen_log_probs * batch['returns']).mean()

    optimizer.zero_grad()
    loss.backward()
    optimizer.step()
```

This is intentionally a sketch — picking REINFORCE vs PPO vs MuZero,
designing the network, choosing the action space, deciding when to
mask vs. clip vs. baseline-subtract, are research decisions that
shouldn't live here. Fill them in.
"""

from __future__ import annotations

import warnings
from typing import List

import numpy as np

from .arcana_py import PyEpisodeResult
from . import run_episode

try:  # Torch is optional — only `to_torch` needs it.
    import torch as _torch

    _HAS_TORCH = True
except ImportError:
    _torch = None  # type: ignore[assignment]
    _HAS_TORCH = False


__all__ = [
    "collect_episodes",
    "returns_for",
    "build_batch",
    "to_torch",
    "episode_summary",
]


def collect_episodes(
    n_episodes: int,
    policy_a,
    policy_b,
    base_seed: int = 0,
    max_turns: int = 200,
    max_steps: int = 5000,
) -> List[PyEpisodeResult]:
    """Run `n_episodes` episodes serially with seeds
    `base_seed + [0, n_episodes)`.

    For parallel collection, use `multiprocessing.Pool` with this
    function as the worker. The GIL is held within `run_episode`
    so threading wouldn't help.
    """
    return [
        run_episode(
            seed=base_seed + i,
            policy_a=policy_a,
            policy_b=policy_b,
            max_turns=max_turns,
            max_steps=max_steps,
        )
        for i in range(n_episodes)
    ]


def returns_for(rewards: np.ndarray, gamma: float = 1.0) -> np.ndarray:
    """Discounted future returns: `G_t = r_t + γ G_{t+1}`.

    For the v0 sparse-terminal reward (only the final step has a
    nonzero reward) and `γ=1`, every step's return equals the
    terminal reward. For `γ < 1`, returns decay backwards from the
    terminal step.
    """
    n = rewards.shape[0]
    if n == 0:
        return rewards.copy()
    out = np.zeros_like(rewards, dtype=np.float32)
    g: float = 0.0
    for t in range(n - 1, -1, -1):
        g = float(rewards[t]) + gamma * g
        out[t] = g
    return out


def build_batch(
    episodes: List[PyEpisodeResult],
    perspective: int = 0,
    gamma: float = 1.0,
    mask_truncated: bool = True,
) -> dict:
    """Assemble a flat training batch from a list of episodes.

    Returns a dict with keys:
      observations:   (B, 99)  float32
      action_indices: (B,)     int32
      returns:        (B,)     float32 — discounted future reward
      episode_ids:    (B,)     int32   — index into `episodes`

    `B = sum over included episodes of len(trajectory.steps)`.

    `mask_truncated=True` excludes truncated episodes (their
    `final_reward=0` is no-signal, not a draw). Set to False to
    treat truncation as a 0-reward draw — your call.
    """
    obs_list, idx_list, ret_list, eid_list = [], [], [], []

    for ei, ep in enumerate(episodes):
        if mask_truncated and ep.outcome.truncated:
            continue
        traj = ep.trajectories[perspective]
        n = traj.observations.shape[0]
        if n == 0:
            continue
        returns = returns_for(traj.rewards, gamma=gamma)
        obs_list.append(traj.observations)
        idx_list.append(traj.action_indices)
        ret_list.append(returns)
        eid_list.append(np.full(n, ei, dtype=np.int32))

    if not obs_list:
        # Empty batch — caller decides whether to skip the update.
        from .arcana_py import BASIC_E2_DIM_TWO_PLAYERS as _DIM

        return {
            "observations": np.zeros((0, _DIM), dtype=np.float32),
            "action_indices": np.zeros((0,), dtype=np.int32),
            "returns": np.zeros((0,), dtype=np.float32),
            "episode_ids": np.zeros((0,), dtype=np.int32),
        }

    return {
        "observations": np.concatenate(obs_list, axis=0),
        "action_indices": np.concatenate(idx_list, axis=0),
        "returns": np.concatenate(ret_list, axis=0).astype(np.float32),
        "episode_ids": np.concatenate(eid_list, axis=0),
    }


def to_torch(batch: dict, device: str = "cpu") -> dict:
    """Convert a numpy batch (from `build_batch`) into torch tensors
    on `device`. `action_indices` is cast to `long` for use as
    cross-entropy targets / `gather` indices."""
    if not _HAS_TORCH:
        warnings.warn(
            "torch is not installed; arcana.training.to_torch returned "
            "the numpy batch unchanged. Install torch to use this helper.",
            RuntimeWarning,
            stacklevel=2,
        )
        return batch
    return {
        "observations": _torch.from_numpy(batch["observations"]).to(device),
        "action_indices": _torch.from_numpy(batch["action_indices"]).long().to(device),
        "returns": _torch.from_numpy(batch["returns"]).to(device),
        "episode_ids": _torch.from_numpy(batch["episode_ids"]).to(device),
    }


def episode_summary(
    episodes: List[PyEpisodeResult], perspective: int = 0
) -> dict:
    """Per-iteration diagnostic stats. Suitable for logging.

    Returns a dict with `n`, `n_terminated`, `n_truncated`,
    `n_wins` / `n_losses` / `n_draws` from `perspective`'s side,
    `win_rate` (over terminated games only), `mean_steps`,
    `mean_turns`.
    """
    n = len(episodes)
    if n == 0:
        return {"n": 0}

    n_terminated = sum(1 for ep in episodes if ep.outcome.terminated)
    n_truncated = sum(1 for ep in episodes if ep.outcome.truncated)

    n_wins = sum(
        1
        for ep in episodes
        if ep.outcome.terminated and ep.outcome.winner == perspective
    )
    n_losses = sum(
        1
        for ep in episodes
        if ep.outcome.terminated
        and ep.outcome.winner is not None
        and ep.outcome.winner != perspective
    )
    n_draws = sum(
        1
        for ep in episodes
        if ep.outcome.terminated and ep.outcome.winner is None
    )

    return {
        "n": n,
        "n_terminated": n_terminated,
        "n_truncated": n_truncated,
        "n_wins": n_wins,
        "n_losses": n_losses,
        "n_draws": n_draws,
        "win_rate": n_wins / max(n_terminated, 1),
        "mean_steps": float(np.mean([ep.steps_taken for ep in episodes])),
        "mean_turns": float(np.mean([ep.turns_taken for ep in episodes])),
    }
