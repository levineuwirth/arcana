//! [`MtgEnv`] — Gymnasium-shaped environment wrapper around
//! [`arcana_core::state::GameState`].
//!
//! v0 stub: `reset()` returns a real observation; `step()` raises
//! `NotImplementedError` because legal-action enumeration isn't
//! plumbed through yet. The point of shipping this stub is that
//! downstream RL harness code can `import arcana` against a real
//! Python extension instead of a placeholder, and the surface
//! settles before the engine catches up.
//!
//! # Gymnasium API alignment
//!
//! Targets Gymnasium 0.26+ — `reset()` returns `(obs, info)` and
//! `step()` will eventually return `(obs, reward, terminated,
//! truncated, info)`. We deliberately do not subclass
//! `gymnasium.Env` on the Rust side: keeping gymnasium out of the
//! Rust dep tree means a build never breaks because of a Python
//! library version mismatch. The Python wrapper at
//! `python/arcana/env.py` is where gymnasium subclassing happens
//! once that integration matters.
//!
//! # Memory layout
//!
//! Each `reset()` allocates exactly one numpy array (sized to the
//! encoder's `dim()`) and fills it via the allocation-free
//! `encode_into` path on the encoder. The numpy array is returned
//! to Python; ownership transfers and the GIL handles freeing it.
//! Hot training loops should reuse the same array via the
//! eventual `Encoder.encode_into_numpy(buf)` API; we'll add that
//! when there's a consumer.

use arcana_ai::observation::{BasicE2Encoder, Encoder};
use arcana_ai::reward::{RewardFunction, TerminalReward};
use arcana_core::state::GameState;
use arcana_core::types::PlayerId;
use numpy::{PyArray1, PyArrayMethods};
use pyo3::exceptions::{PyNotImplementedError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Gymnasium-shaped environment.
///
/// Construct with `MtgEnv(num_players=2, seed=0, perspective=0)`.
/// `perspective` is the player whose viewpoint observations and
/// rewards are computed from — fix it at construction so the
/// observation distribution doesn't drift mid-episode.
#[pyclass]
pub struct MtgEnv {
    state: GameState,
    encoder: BasicE2Encoder,
    reward: TerminalReward,
    perspective: PlayerId,
    num_players: u8,
    seed: u64,
}

#[pymethods]
impl MtgEnv {
    #[new]
    #[pyo3(signature = (num_players = 2, seed = 0, perspective = 0))]
    fn new(num_players: u8, seed: u64, perspective: PlayerId) -> PyResult<Self> {
        if num_players < 1 {
            return Err(PyValueError::new_err("num_players must be >= 1"));
        }
        if perspective >= num_players {
            return Err(PyValueError::new_err(format!(
                "perspective {perspective} out of range for {num_players} players"
            )));
        }
        Ok(Self {
            state: GameState::new(num_players, seed),
            encoder: BasicE2Encoder::new(num_players),
            reward: TerminalReward,
            perspective,
            num_players,
            seed,
        })
    }

    /// Reset to the initial state. Returns `(observation, info)`
    /// per Gymnasium 0.26+. `info` is currently an empty dict;
    /// future versions may carry diagnostic fields.
    fn reset<'py>(
        &mut self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyArray1<f32>>, Bound<'py, PyDict>)> {
        self.state = GameState::new(self.num_players, self.seed);
        let obs = self.observation_array(py);
        let info = PyDict::new(py);
        Ok((obs, info))
    }

    /// Step the environment.
    ///
    /// **v0 stub**: raises `NotImplementedError`. Lands when
    /// arcana-core's legal-action enumeration is wired through. Will
    /// then return `(obs, reward, terminated, truncated, info)`.
    fn step(&mut self, _action: PyObject) -> PyResult<PyObject> {
        Err(PyNotImplementedError::new_err(
            "MtgEnv.step is a v0 stub; legal-action enumeration is \
             not yet wired through. Use reset() + observation_dim \
             for now, and follow the action_flattening.rs work for \
             the engine-step rollout.",
        ))
    }

    /// Vector dimension of the observation. Equal to
    /// `BASIC_E2_DIM_TWO_PLAYERS` for the default 2-player setup.
    #[getter]
    fn observation_dim(&self) -> usize {
        self.encoder.dim()
    }

    #[getter]
    fn num_players(&self) -> u8 {
        self.num_players
    }

    #[getter]
    fn perspective(&self) -> PlayerId {
        self.perspective
    }

    #[getter]
    fn seed(&self) -> u64 {
        self.seed
    }

    /// True after a Win/Draw/Eliminated has been recorded on the
    /// underlying state.
    fn is_terminal(&self) -> bool {
        self.state.is_game_over()
    }

    /// Sparse terminal reward for the perspective player at the
    /// current state. `0.0` while the game is ongoing.
    fn current_reward(&self) -> f32 {
        self.reward.reward(&self.state, self.perspective)
    }

    fn __repr__(&self) -> String {
        format!(
            "MtgEnv(num_players={}, seed={}, perspective={}, observation_dim={})",
            self.num_players,
            self.seed,
            self.perspective,
            self.encoder.dim()
        )
    }
}

impl MtgEnv {
    /// Allocate a numpy array sized to the encoder dim, fill it via
    /// the allocation-free `encode_into` path, and return ownership
    /// to Python.
    fn observation_array<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f32>> {
        let arr = PyArray1::<f32>::zeros(py, [self.encoder.dim()], false);
        // Safety: we just allocated this array and hold the only
        // handle to it; encode_into writes exactly dim() floats and
        // never reads.
        unsafe {
            let slice = arr.as_slice_mut().expect("freshly-allocated PyArray1 is contiguous");
            self.encoder
                .encode_into(&self.state, Some(self.perspective), slice);
        }
        arr
    }
}
