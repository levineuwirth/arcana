"""End-to-end smoke tests for the arcana Python bindings.

Run with `maturin develop && pytest arcana-py/tests/`. Requires the
native module to be built and installed in the active venv.
"""

import numpy as np
import pytest

import arcana


def test_module_exports_expected_names():
    assert hasattr(arcana, "MtgEnv")
    assert hasattr(arcana, "BASIC_E2_DIM_TWO_PLAYERS")
    assert hasattr(arcana, "__version__")
    assert isinstance(arcana.BASIC_E2_DIM_TWO_PLAYERS, int)
    assert arcana.BASIC_E2_DIM_TWO_PLAYERS > 0


def test_reset_returns_observation_of_expected_shape_and_dtype():
    env = arcana.MtgEnv(num_players=2, seed=0)
    obs, info = env.reset()
    assert isinstance(obs, np.ndarray)
    assert obs.dtype == np.float32
    assert obs.shape == (arcana.BASIC_E2_DIM_TWO_PLAYERS,)
    assert isinstance(info, dict)
    assert info == {}


def test_observation_is_finite():
    # Non-finite values would silently corrupt training. Catch them
    # at the binding boundary.
    env = arcana.MtgEnv()
    obs, _ = env.reset()
    assert np.isfinite(obs).all()


def test_observation_dim_matches_constant():
    env = arcana.MtgEnv(num_players=2)
    assert env.observation_dim == arcana.BASIC_E2_DIM_TWO_PLAYERS


def test_reset_is_deterministic_under_seed():
    a, _ = arcana.MtgEnv(num_players=2, seed=42).reset()
    b, _ = arcana.MtgEnv(num_players=2, seed=42).reset()
    np.testing.assert_array_equal(a, b)


def test_step_raises_until_engine_step_lands():
    env = arcana.MtgEnv()
    env.reset()
    with pytest.raises(NotImplementedError):
        env.step(0)


def test_invalid_perspective_rejected():
    with pytest.raises(ValueError):
        arcana.MtgEnv(num_players=2, perspective=5)


def test_zero_players_rejected():
    with pytest.raises(ValueError):
        arcana.MtgEnv(num_players=0)


def test_initial_state_not_terminal():
    env = arcana.MtgEnv()
    env.reset()
    assert not env.is_terminal()
    assert env.current_reward() == 0.0


def test_repr_includes_construction_params():
    env = arcana.MtgEnv(num_players=2, seed=7, perspective=1)
    s = repr(env)
    assert "num_players=2" in s
    assert "seed=7" in s
    assert "perspective=1" in s
