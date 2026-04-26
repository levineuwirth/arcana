"""Tests for `arcana.run_episode` — stateless self-play binding.

Run with `maturin develop && pytest arcana-py/tests/`.
"""

import numpy as np
import pytest

import arcana


# -- Smoke ------------------------------------------------------------


def test_module_exports_run_episode_and_policies():
    assert callable(arcana.run_episode)
    assert hasattr(arcana, "policies")
    assert callable(arcana.policies.random)
    assert callable(arcana.policies.progress_biased)
    assert callable(arcana.policies.first_action)


def test_seed_deck_setup_is_introspectable():
    setup = arcana.SEED_DECK_SETUP
    assert ("Lightning Bolt", 4) in setup.player_0_deck
    assert ("Grizzly Bears", 8) in setup.player_1_deck
    assert "Mountain" in setup.card_pool
    assert isinstance(setup.description, str)


# -- Default-policy episode ------------------------------------------


def test_run_episode_with_defaults_terminates():
    result = arcana.run_episode(seed=42)
    # Either terminated or truncated, never both.
    assert result.outcome.terminated ^ result.outcome.truncated
    assert result.steps_taken > 0
    assert len(result.trajectories) == 2


def test_trajectory_arrays_are_struct_of_arrays_and_contiguous():
    result = arcana.run_episode(
        seed=0,
        policy_a=arcana.policies.progress_biased(seed=1),
        policy_b=arcana.policies.progress_biased(seed=2),
    )
    for traj in result.trajectories:
        # observations: (N, 99) float32, contiguous.
        obs = traj.observations
        assert isinstance(obs, np.ndarray)
        assert obs.dtype == np.float32
        assert obs.ndim == 2
        assert obs.shape[1] == arcana.BASIC_E2_DIM_TWO_PLAYERS
        assert obs.flags["C_CONTIGUOUS"]
        assert np.isfinite(obs).all()

        # action_indices: (N,) int32.
        idx = traj.action_indices
        assert idx.dtype == np.int32
        assert idx.ndim == 1
        assert idx.shape[0] == obs.shape[0]
        assert idx.flags["C_CONTIGUOUS"]
        assert (idx >= 0).all()  # no -1 sentinels in normal play

        # rewards: (N,) float32.
        rew = traj.rewards
        assert rew.dtype == np.float32
        assert rew.ndim == 1
        assert rew.shape[0] == obs.shape[0]
        assert rew.flags["C_CONTIGUOUS"]


def test_terminal_reward_stamping():
    """Across 16 seeds at least one game produces a winner; verify
    the winner gets +1 final_reward and the loser gets -1."""
    for seed in range(16):
        result = arcana.run_episode(
            seed=seed,
            policy_a=arcana.policies.progress_biased(seed=seed * 2 + 1),
            policy_b=arcana.policies.progress_biased(seed=seed * 2 + 2),
        )
        if result.outcome.winner is not None and result.outcome.terminated:
            w = result.outcome.winner
            l = 1 - w
            assert result.trajectories[w].final_reward == 1.0
            assert result.trajectories[l].final_reward == -1.0
            return
    pytest.fail("no game produced a winner across 16 seeds")


def test_truncation_is_distinct_from_termination():
    """Aggressive max_steps cap forces truncation; verify the
    outcome carries truncated=True, terminated=False, winner=None,
    and final_reward=0 for both perspectives."""
    result = arcana.run_episode(
        seed=0,
        max_steps=2,  # well below any real game's needs
    )
    assert result.outcome.truncated
    assert not result.outcome.terminated
    assert result.outcome.winner is None
    for traj in result.trajectories:
        assert traj.final_reward == 0.0


# -- Determinism -----------------------------------------------------


def test_same_seed_produces_byte_identical_trajectories():
    a = arcana.run_episode(seed=42)
    b = arcana.run_episode(seed=42)
    # Outcome match.
    assert a.outcome.winner == b.outcome.winner
    assert a.outcome.terminated == b.outcome.terminated
    assert a.outcome.truncated == b.outcome.truncated
    assert a.steps_taken == b.steps_taken
    assert a.turns_taken == b.turns_taken
    # Trajectories match byte-for-byte.
    for ta, tb in zip(a.trajectories, b.trajectories):
        np.testing.assert_array_equal(ta.observations, tb.observations)
        np.testing.assert_array_equal(ta.action_indices, tb.action_indices)
        np.testing.assert_array_equal(ta.rewards, tb.rewards)


def test_different_seeds_produce_different_trajectories():
    a = arcana.run_episode(seed=0)
    b = arcana.run_episode(seed=1)
    # At least the action_indices should diverge somewhere.
    if (
        a.trajectories[0].action_indices.shape
        == b.trajectories[0].action_indices.shape
    ):
        assert not np.array_equal(
            a.trajectories[0].action_indices, b.trajectories[0].action_indices
        )
    # else: different lengths, trivially different.


# -- Python callable policies ----------------------------------------


def test_python_callable_policy_works():
    """A trivial Python policy that always picks index 0 should run
    end-to-end. Equivalent to `policies.first_action()` but
    exercises the FFI callback path."""
    calls = {"count": 0}

    def always_first(obs, n_legal):
        calls["count"] += 1
        assert isinstance(obs, np.ndarray)
        assert obs.dtype == np.float32
        assert obs.shape == (arcana.BASIC_E2_DIM_TWO_PLAYERS,)
        assert n_legal > 0
        return 0

    result = arcana.run_episode(
        seed=0, policy_a=always_first, policy_b=always_first, max_steps=200
    )
    # Policy was actually called.
    assert calls["count"] > 0


def test_python_policy_invalid_index_raises():
    def bad(obs, n_legal):
        return n_legal + 100  # always out of range

    with pytest.raises(BaseException):
        arcana.run_episode(seed=0, policy_a=bad, max_steps=10)


def test_invalid_policy_arg_rejected():
    with pytest.raises(TypeError):
        arcana.run_episode(seed=0, policy_a=42)  # int isn't a policy


# -- Mixed Rust + Python policies ------------------------------------


def test_can_mix_rust_and_python_policies():
    def py_pol(obs, n_legal):
        return 0

    result = arcana.run_episode(
        seed=0,
        policy_a=arcana.policies.progress_biased(seed=1),
        policy_b=py_pol,
        max_steps=300,
    )
    # Both trajectories populated.
    assert len(result.trajectories) == 2
    assert result.steps_taken > 0


# -- repr ------------------------------------------------------------


def test_repr_strings_are_informative():
    result = arcana.run_episode(seed=0, max_steps=50)
    s = repr(result)
    assert "EpisodeResult" in s
    assert "winner=" in s
    assert "terminated=" in s
    assert "truncated=" in s

    s2 = repr(result.trajectories[0])
    assert "Trajectory" in s2
    assert "perspective=0" in s2
