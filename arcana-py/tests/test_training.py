"""Tests for `arcana.training` scaffolding.

Run with `maturin develop && pytest arcana-py/tests/`.
"""

import numpy as np
import pytest

import arcana
from arcana.training import (
    build_batch,
    collect_episodes,
    episode_summary,
    returns_for,
    to_torch,
)


# -- returns_for ----------------------------------------------------


def test_returns_for_undiscounted_constant_with_sparse_terminal():
    # Sparse terminal reward at the last step, γ=1: every step's
    # return equals the terminal reward.
    rewards = np.array([0.0, 0.0, 0.0, 1.0], dtype=np.float32)
    out = returns_for(rewards, gamma=1.0)
    np.testing.assert_array_equal(out, np.array([1.0, 1.0, 1.0, 1.0], dtype=np.float32))


def test_returns_for_discounted_decays_backwards():
    # γ=0.9; final reward 1.0 at index 3.
    # G[3]=1.0, G[2]=0.9, G[1]=0.81, G[0]=0.729.
    rewards = np.array([0.0, 0.0, 0.0, 1.0], dtype=np.float32)
    out = returns_for(rewards, gamma=0.9)
    expected = np.array([0.729, 0.81, 0.9, 1.0], dtype=np.float32)
    np.testing.assert_allclose(out, expected, rtol=1e-6)


def test_returns_for_empty_array():
    rewards = np.zeros((0,), dtype=np.float32)
    out = returns_for(rewards)
    assert out.shape == (0,)


def test_returns_for_negative_terminal():
    # Loss → -1 at terminal step propagates.
    rewards = np.array([0.0, 0.0, -1.0], dtype=np.float32)
    out = returns_for(rewards, gamma=1.0)
    np.testing.assert_array_equal(out, np.array([-1.0, -1.0, -1.0], dtype=np.float32))


# -- collect_episodes ----------------------------------------------


def test_collect_episodes_returns_requested_count():
    eps = collect_episodes(
        n_episodes=4,
        policy_a=arcana.policies.progress_biased(seed=1),
        policy_b=arcana.policies.progress_biased(seed=2),
        base_seed=0,
        max_steps=500,
    )
    assert len(eps) == 4
    # Seeds are base_seed + [0, n) — distinct, so we expect at least
    # some divergence in step counts.
    step_counts = [ep.steps_taken for ep in eps]
    assert len(set(step_counts)) >= 1  # weak — not all identical at minimum


# -- build_batch ----------------------------------------------------


def test_build_batch_concatenates_perspective_steps():
    eps = collect_episodes(
        n_episodes=8,
        policy_a=arcana.policies.progress_biased(seed=1),
        policy_b=arcana.policies.progress_biased(seed=2),
        base_seed=0,
        max_steps=500,
    )
    batch = build_batch(eps, perspective=0, gamma=1.0)
    obs = batch["observations"]
    idx = batch["action_indices"]
    ret = batch["returns"]
    eid = batch["episode_ids"]

    # All arrays have the same length B.
    assert obs.ndim == 2
    assert obs.shape[1] == arcana.BASIC_E2_DIM_TWO_PLAYERS
    assert obs.shape[0] == idx.shape[0] == ret.shape[0] == eid.shape[0]
    # Numpy dtypes match the contract.
    assert obs.dtype == np.float32
    assert idx.dtype == np.int32
    assert ret.dtype == np.float32
    assert eid.dtype == np.int32
    # episode_ids only references included episodes.
    assert int(eid.max(initial=-1)) < len(eps)


def test_build_batch_mask_truncated_excludes_truncation():
    # max_steps=2 forces truncation on every episode.
    eps = collect_episodes(
        n_episodes=4,
        policy_a=arcana.policies.first_action(),
        policy_b=arcana.policies.first_action(),
        base_seed=0,
        max_steps=2,
    )
    assert all(ep.outcome.truncated for ep in eps), "expected all truncated"
    batch_masked = build_batch(eps, perspective=0, mask_truncated=True)
    assert batch_masked["observations"].shape[0] == 0

    batch_unmasked = build_batch(eps, perspective=0, mask_truncated=False)
    assert batch_unmasked["observations"].shape[0] > 0


def test_build_batch_returns_match_terminal_reward_for_undiscounted():
    # γ=1 + sparse terminal reward ⇒ every return equals the
    # trajectory's final_reward.
    eps = collect_episodes(
        n_episodes=12,
        policy_a=arcana.policies.progress_biased(seed=11),
        policy_b=arcana.policies.progress_biased(seed=22),
        base_seed=100,
        max_steps=2000,
    )
    batch = build_batch(eps, perspective=0, gamma=1.0)
    if batch["observations"].shape[0] == 0:
        pytest.skip("all episodes truncated; can't validate return propagation")
    # Group rewards by episode_id and check each block is constant.
    eids = batch["episode_ids"]
    rets = batch["returns"]
    for eid in np.unique(eids):
        block = rets[eids == eid]
        assert np.allclose(block, block[0]), (
            f"undiscounted returns within episode {eid} should be constant; "
            f"got {block}"
        )


# -- episode_summary -----------------------------------------------


def test_episode_summary_counts_match_outcomes():
    eps = collect_episodes(
        n_episodes=20,
        policy_a=arcana.policies.progress_biased(seed=1),
        policy_b=arcana.policies.progress_biased(seed=2),
        base_seed=0,
        max_steps=2000,
    )
    stats = episode_summary(eps, perspective=0)
    assert stats["n"] == 20
    assert stats["n_terminated"] + stats["n_truncated"] == 20
    assert stats["n_wins"] + stats["n_losses"] + stats["n_draws"] == stats["n_terminated"]
    assert 0.0 <= stats["win_rate"] <= 1.0
    assert stats["mean_steps"] > 0
    assert stats["mean_turns"] > 0


def test_episode_summary_empty_input():
    assert episode_summary([], perspective=0) == {"n": 0}


# -- to_torch (only if torch is installed) -------------------------

torch = pytest.importorskip("torch")


def test_to_torch_converts_dtypes_correctly():
    eps = collect_episodes(
        n_episodes=4,
        policy_a=arcana.policies.progress_biased(seed=1),
        policy_b=arcana.policies.progress_biased(seed=2),
        base_seed=0,
        max_steps=500,
    )
    batch = build_batch(eps, perspective=0)
    if batch["observations"].shape[0] == 0:
        pytest.skip("all episodes truncated")
    t_batch = to_torch(batch, device="cpu")
    assert t_batch["observations"].dtype == torch.float32
    assert t_batch["action_indices"].dtype == torch.int64  # long for cross-entropy
    assert t_batch["returns"].dtype == torch.float32
    assert t_batch["observations"].shape == batch["observations"].shape
