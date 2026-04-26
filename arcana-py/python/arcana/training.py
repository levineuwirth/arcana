"""Training-loop scaffolding for arcana self-play.

Primitives for collecting episodes, computing returns, assembling
batches, and reading per-batch diagnostics. Deliberately not
opinionated about the algorithm — REINFORCE, PPO, MuZero, custom —
all consume the same primitives.

For a runnable end-to-end example see
``arcana-py/examples/reinforce_demo.py`` (covered by a smoke test).
The example lives outside the docstring so it stays runnable and
can't silently rot relative to the API.

`torch` is imported lazily — only `to_torch()` requires it. The
rest of the module operates on plain numpy arrays.

# Primitives

* `collect_episodes(n, policy_a, policy_b, base_seed, ...)` — serial
  episode collection.
* `returns_for(rewards, gamma=1.0)` — discounted future returns;
  vectorized cumsum, with a γ=1 fast path for sparse-terminal
  rewards.
* `build_batch(episodes, perspective, gamma, mask_truncated)` —
  flatten per-perspective trajectories into numpy SoA. Includes
  `n_legals` per step so masked-action losses are well-defined.
* `to_torch(batch, device)` — torch tensor conversion.
* `episode_summary(episodes, perspective)` — win-rate /
  termination-rate / step-and-turn statistics for logging.
* `legal_action_stats(episodes)` — empirical distribution of
  ``n_legal`` across all decision steps. Use this to pick a
  `K_max` for the policy's output dimension before training.
"""

from __future__ import annotations

import warnings
from collections import Counter
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
    "legal_action_stats",
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
    function as the worker. The GIL is held within `run_episode`,
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

    γ=1 takes a vectorized reverse-cumsum fast path — the common
    case for v0's sparse-terminal rewards. γ<1 falls back to a
    backward Python iteration; the cumsum-with-discount-divide
    trick is faster but `γ^t` underflows to zero for long
    trajectories and small γ, producing NaNs. Sub-millisecond on
    n≤1000, which is far longer than any real MTG episode.
    """
    if rewards.size == 0:
        return rewards.astype(np.float32, copy=True)
    rewards_f = rewards.astype(np.float32, copy=False)
    if gamma == 1.0:
        return np.flip(np.cumsum(np.flip(rewards_f))).copy()
    out = np.empty_like(rewards_f)
    g: float = 0.0
    for t in range(rewards_f.size - 1, -1, -1):
        g = float(rewards_f[t]) + gamma * g
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
      n_legals:       (B,)     int32  — legal-action count per step
      returns:        (B,)     float32 — discounted future reward
      episode_ids:    (B,)     int32   — index into `episodes`

    `B = sum over included episodes of len(trajectory.steps)`.

    `mask_truncated=True` excludes truncated episodes (their
    `final_reward=0` is no-signal, not a draw). Set to False to
    treat truncation as a 0-reward draw — your call.
    """
    obs_list, idx_list, nl_list, ret_list, eid_list = [], [], [], [], []

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
        nl_list.append(traj.n_legals)
        ret_list.append(returns)
        eid_list.append(np.full(n, ei, dtype=np.int32))

    if not obs_list:
        # Empty batch — caller decides whether to skip the update.
        from .arcana_py import BASIC_E2_DIM_TWO_PLAYERS as _DIM

        return {
            "observations": np.zeros((0, _DIM), dtype=np.float32),
            "action_indices": np.zeros((0,), dtype=np.int32),
            "n_legals": np.zeros((0,), dtype=np.int32),
            "returns": np.zeros((0,), dtype=np.float32),
            "episode_ids": np.zeros((0,), dtype=np.int32),
        }

    return {
        "observations": np.concatenate(obs_list, axis=0),
        "action_indices": np.concatenate(idx_list, axis=0),
        "n_legals": np.concatenate(nl_list, axis=0),
        "returns": np.concatenate(ret_list, axis=0).astype(np.float32),
        "episode_ids": np.concatenate(eid_list, axis=0),
    }


def to_torch(batch: dict, device: str = "cpu") -> dict:
    """Convert a numpy batch (from `build_batch`) into torch tensors
    on `device`. `action_indices` and `n_legals` are cast to `long`
    for use as `gather` / cross-entropy targets and as masking
    bounds respectively."""
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
        "n_legals": _torch.from_numpy(batch["n_legals"]).long().to(device),
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


def legal_action_stats(
    episodes: List[PyEpisodeResult],
    perspective_set: tuple = (0, 1),
    bucket_edges: tuple = (1, 2, 5, 10, 25, 50, 100, 250, 1000),
) -> dict:
    """Empirical distribution of `n_legal` across decision steps.
    Use this to pick a `K_max` for the policy's output dimension —
    `K_max = p99 + headroom` is the typical move.

    `perspective_set` selects which trajectories to read from each
    episode; the default `(0, 1)` aggregates both sides.
    `bucket_edges` defines histogram bucket boundaries; the
    returned `bucket_counts` is a dict mapping `"lo..hi"` → count.

    Returns a dict with:
      `n_steps`, `min`, `max`, `mean`, `p50`, `p90`, `p99`,
      `bucket_counts` (dict[str, int]).

    Bucket histogram is a rough proxy for decision context — combat
    declarations and modal spells inflate `n_legal` versus a vanilla
    priority pass. A bimodal distribution is the signal to start
    storing decision_kind explicitly in the trajectory.
    """
    counts: list[int] = []
    for ep in episodes:
        for p in perspective_set:
            traj = ep.trajectories[p]
            counts.extend(int(x) for x in traj.n_legals)

    if not counts:
        return {
            "n_steps": 0,
            "min": 0,
            "max": 0,
            "mean": 0.0,
            "p50": 0,
            "p90": 0,
            "p99": 0,
            "bucket_counts": {},
        }

    arr = np.asarray(counts, dtype=np.int64)
    p50, p90, p99 = np.percentile(arr, [50, 90, 99])

    # Bucket.
    edges = list(bucket_edges)
    bucket_counts: dict[str, int] = Counter()
    for c in counts:
        placed = False
        for lo, hi in zip(edges, edges[1:]):
            if lo <= c < hi:
                bucket_counts[f"{lo}..{hi}"] += 1
                placed = True
                break
        if not placed:
            if c < edges[0]:
                bucket_counts[f"<{edges[0]}"] += 1
            else:
                bucket_counts[f">={edges[-1]}"] += 1

    return {
        "n_steps": len(counts),
        "min": int(arr.min()),
        "max": int(arr.max()),
        "mean": float(arr.mean()),
        "p50": int(p50),
        "p90": int(p90),
        "p99": int(p99),
        "bucket_counts": dict(bucket_counts),
    }
