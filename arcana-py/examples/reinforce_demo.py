"""Runnable REINFORCE demo for arcana self-play.

Usage::

    cd arcana-py
    maturin develop
    pip install torch
    python examples/reinforce_demo.py

This is a worked example, not a production training script. It is
deliberately small enough to run on CPU in under a minute and to be
covered by a smoke test in `tests/test_examples.py`. Don't expect
the agent to crush the baseline — at this scale you're verifying
the substrate works end-to-end, not training a real policy.

The pattern shown here:

1. Probe the legal-action distribution with a few hundred random
   episodes. Pick ``K_max`` from the empirical p99 plus headroom.
2. Build a small MLP policy that outputs ``(B, K_max)`` logits.
3. Iterate:
   a. Collect episodes with the model as `policy_a` against a
      progress-biased random `policy_b`.
   b. Sample (not argmax) from the legal subset at action selection
      time.
   c. Compute masked log-softmax in the loss step so the
      normalizer only includes legal actions.
   d. Baseline-subtract returns (mean-of-batch) to reduce variance.
"""

from __future__ import annotations

import torch
import torch.nn as nn
import torch.nn.functional as F

import arcana
from arcana.training import (
    build_batch,
    collect_episodes,
    episode_summary,
    legal_action_stats,
    to_torch,
)


N_PROBE_EPISODES = 200       # for legal_action_stats — used to pick K_max
N_EPISODES_PER_ITER = 32     # episodes per gradient update
N_ITERATIONS = 20            # short horizon for the demo
LR = 3e-4
HIDDEN_DIM = 128
SEED_BASE = 100_000


def build_model(obs_dim: int, k_max: int) -> nn.Module:
    return nn.Sequential(
        nn.Linear(obs_dim, HIDDEN_DIM),
        nn.ReLU(),
        nn.Linear(HIDDEN_DIM, HIDDEN_DIM),
        nn.ReLU(),
        nn.Linear(HIDDEN_DIM, k_max),
    )


def make_policy(model: nn.Module, k_max: int):
    """Wrap the model as a Python policy callable. Samples from the
    legal-subset Categorical distribution — required for REINFORCE
    correctness; argmax + REINFORCE is biased."""

    def policy(obs, n_legal, **kwargs):
        with torch.no_grad():
            logits = model(torch.from_numpy(obs).unsqueeze(0))[0]  # (k_max,)
            if n_legal > k_max:
                # K_max underestimate. Random over the visible window
                # rather than crashing — surface as truncated training
                # signal instead of a hard failure. If this fires
                # often, raise K_max.
                return int(torch.randint(0, k_max, (1,)).item())
            masked = logits.clone()
            masked[n_legal:] = float("-inf")
            dist = torch.distributions.Categorical(logits=masked)
            return int(dist.sample().item())

    return policy


def reinforce_loss(
    model: nn.Module,
    batch: dict,
    k_max: int,
) -> torch.Tensor:
    """Masked-log-softmax REINFORCE with mean-of-batch baseline."""
    logits = model(batch["observations"])  # (B, k_max)
    col = torch.arange(k_max, device=logits.device).unsqueeze(0)  # (1, k_max)
    legal_mask = col < batch["n_legals"].unsqueeze(1)              # (B, k_max)
    masked_logits = torch.where(
        legal_mask, logits, torch.full_like(logits, float("-inf"))
    )
    log_probs = F.log_softmax(masked_logits, dim=-1)
    chosen = log_probs.gather(1, batch["action_indices"].unsqueeze(1)).squeeze(1)

    returns = batch["returns"]
    advantages = returns - returns.mean()
    return -(chosen * advantages).mean()


def main() -> None:
    obs_dim = arcana.BASIC_E2_DIM_TWO_PLAYERS

    # 1. Probe.
    print(
        f"probing legal-action distribution over {N_PROBE_EPISODES} "
        "random-vs-random episodes..."
    )
    probe_eps = collect_episodes(
        n_episodes=N_PROBE_EPISODES,
        policy_a=arcana.policies.progress_biased(seed=1),
        policy_b=arcana.policies.progress_biased(seed=2),
        base_seed=0,
        max_steps=2000,
    )
    stats = legal_action_stats(probe_eps)
    k_max = max(stats["p99"], 8) + 8  # headroom
    print(
        f"  n_steps={stats['n_steps']}  min={stats['min']}  "
        f"p50={stats['p50']}  p99={stats['p99']}  max={stats['max']}"
    )
    print(f"  picking K_max={k_max}")

    # 2. Model + optimizer.
    model = build_model(obs_dim, k_max)
    optimizer = torch.optim.Adam(model.parameters(), lr=LR)

    # 3. Train.
    for it in range(N_ITERATIONS):
        policy = make_policy(model, k_max)
        episodes = collect_episodes(
            n_episodes=N_EPISODES_PER_ITER,
            policy_a=policy,
            policy_b=arcana.policies.progress_biased(seed=SEED_BASE + it),
            base_seed=SEED_BASE + it * 1000,
            max_steps=2000,
        )

        sm = episode_summary(episodes, perspective=0)
        print(
            f"iter {it:3d}: win_rate={sm['win_rate']:.2f}  "
            f"mean_turns={sm['mean_turns']:.1f}  "
            f"truncated={sm['n_truncated']}/{sm['n']}  "
            f"({sm['n_wins']}W/{sm['n_losses']}L/{sm['n_draws']}D)"
        )

        batch = build_batch(
            episodes, perspective=0, gamma=1.0, mask_truncated=True
        )
        if batch["observations"].shape[0] == 0:
            continue  # all truncated, skip update
        batch = to_torch(batch, device="cpu")

        loss = reinforce_loss(model, batch, k_max)
        optimizer.zero_grad()
        loss.backward()
        optimizer.step()


if __name__ == "__main__":
    main()
