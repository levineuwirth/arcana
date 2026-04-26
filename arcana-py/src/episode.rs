//! `arcana.run_episode` — stateless self-play binding.
//!
//! Wraps [`arcana_ai::selfplay::run_episode`] in a PyO3 surface
//! that:
//!
//! * Constructs the Lightning Bolt seed-deck setup internally so
//!   Python doesn't need to bind `CardRegistry` / `Deck` types yet.
//! * Accepts policies as either built-in [`PyPolicy`] instances
//!   (constructed via the `arcana.policies.*` factories) or as
//!   plain Python callables `(obs: np.ndarray, n_legal: int) -> int`.
//! * Returns a [`PyEpisodeResult`] whose interior is **numpy
//!   struct-of-arrays** — one big contiguous array per field
//!   (observations, action_indices, rewards) rather than a list of
//!   per-step Python objects. Required for fast batched training-
//!   loop access patterns.
//!
//! # The Python-callback performance trade-off
//!
//! Python policies cross the FFI boundary for every decision. With
//! ~600-1000 decisions per game, that's a few ms of GIL acquisition
//! overhead per game on top of whatever the Python policy itself
//! costs (typically a PyTorch single-input forward pass — ~1-10× a
//! batched forward pass). Acceptable for development; **not the
//! high-throughput training path**.
//!
//! For training-data collection, prefer Rust-side policies:
//! [`arcana_ai::selfplay::RandomPolicy`] /
//! [`arcana_ai::selfplay::ProgressBiasedRandomPolicy`] /
//! [`arcana_ai::selfplay::FirstActionPolicy`] — exposed in Python
//! as `arcana.policies.random()` / `arcana.policies.progress_biased()` /
//! `arcana.policies.first_action()`. These run inside the Rust
//! harness without crossing back into Python at every decision.

use arcana_ai::observation::{BasicE2Encoder, BASIC_E2_DIM_TWO_PLAYERS};
use arcana_ai::reward::TerminalReward;
use arcana_ai::selfplay::{
    self, EpisodeConfig, FirstActionPolicy, Policy, PolicyChoice, ProgressBiasedRandomPolicy,
    RandomPolicy, Trajectory,
};
use arcana_cards::register_seed;
use arcana_core::actions::Action;
use arcana_core::engine::{new_game, EngineYield};
use arcana_core::registry::{build_deck, CardRegistry};
use arcana_core::state::GameResult;
use numpy::{PyArray1, PyArray2, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

// =============================================================================
// PyPolicy — wraps the four supported policy variants
// =============================================================================

/// Opaque policy handle constructed by `arcana.policies.random()`,
/// `arcana.policies.progress_biased()`, or
/// `arcana.policies.first_action()`. Pass to
/// [`run_episode`] as `policy_a` / `policy_b`.
///
/// Python callables can also be passed directly without going
/// through `PyPolicy` — `run_episode` accepts either a `PyPolicy`
/// or any callable matching the `(obs, n_legal) -> int` shape.
#[pyclass]
pub struct PyPolicy {
    inner: PolicyImpl,
}

enum PolicyImpl {
    Random(RandomPolicy),
    ProgressBiased(ProgressBiasedRandomPolicy),
    FirstAction(FirstActionPolicy),
    /// Python callable holding a `PyObject`. Calling crosses the
    /// FFI boundary; see module docs on perf characteristics.
    PythonCallable(PyObject),
}

impl Clone for PolicyImpl {
    fn clone(&self) -> Self {
        match self {
            PolicyImpl::Random(p) => PolicyImpl::Random(p.clone()),
            PolicyImpl::ProgressBiased(p) => PolicyImpl::ProgressBiased(p.clone()),
            PolicyImpl::FirstAction(p) => PolicyImpl::FirstAction(p.clone()),
            // PyObject isn't Clone (clone_ref needs the GIL); acquire
            // it here. clone_ref is cheap (refcount bump).
            PolicyImpl::PythonCallable(callable) => Python::with_gil(|py| {
                PolicyImpl::PythonCallable(callable.clone_ref(py))
            }),
        }
    }
}

impl Policy for PolicyImpl {
    fn select_action(&mut self, obs: &[f32], legal: &[Action]) -> PolicyChoice {
        match self {
            PolicyImpl::Random(p) => p.select_action(obs, legal),
            PolicyImpl::ProgressBiased(p) => p.select_action(obs, legal),
            PolicyImpl::FirstAction(p) => p.select_action(obs, legal),
            PolicyImpl::PythonCallable(callable) => {
                Python::with_gil(|py| call_python_policy(py, callable, obs, legal.len()))
            }
        }
    }
}

/// Bridge a Python callable to a [`PolicyChoice::Index`]. Copies
/// the observation slice into a fresh numpy array (396 bytes for
/// the v0 99-dim obs — negligible) so the Python callback owns its
/// data and doesn't alias the Rust buffer.
fn call_python_policy(
    py: Python<'_>,
    callable: &PyObject,
    obs: &[f32],
    n_legal: usize,
) -> PolicyChoice {
    let obs_array = PyArray1::<f32>::from_slice(py, obs);
    let result = callable
        .bind(py)
        .call1((obs_array, n_legal))
        .unwrap_or_else(|e| {
            panic!(
                "python policy raised an exception: {}",
                e.to_string()
            )
        });
    let idx: usize = result.extract().unwrap_or_else(|e| {
        panic!(
            "python policy must return an int (got error: {})",
            e.to_string()
        )
    });
    if idx >= n_legal {
        panic!(
            "python policy returned index {idx} but only {n_legal} legal actions available"
        );
    }
    PolicyChoice::Index(idx)
}

// =============================================================================
// Built-in policy factories (private — exposed via Python wrappers)
// =============================================================================

#[pyfunction]
pub fn _make_random_policy(seed: u64) -> PyPolicy {
    PyPolicy { inner: PolicyImpl::Random(RandomPolicy::new(seed)) }
}

#[pyfunction]
pub fn _make_progress_biased_policy(seed: u64) -> PyPolicy {
    PyPolicy {
        inner: PolicyImpl::ProgressBiased(ProgressBiasedRandomPolicy::new(seed)),
    }
}

#[pyfunction]
pub fn _make_first_action_policy() -> PyPolicy {
    PyPolicy { inner: PolicyImpl::FirstAction(FirstActionPolicy) }
}

// =============================================================================
// Result types
// =============================================================================

/// Outcome of an episode. `terminated` and `truncated` are
/// **distinct** flags — truncation isn't a kind of termination, it's
/// the harness running out of budget before the engine produced a
/// terminal yield.
#[pyclass]
#[derive(Debug, Clone)]
pub struct PyEpisodeOutcome {
    /// Player id of the winner, or `None` for a draw / truncation.
    /// Terminal-and-winner-None ⇔ `Draw`. Truncated ⇒ `winner=None`
    /// always (no truncation-implies-winner heuristic in v0).
    #[pyo3(get)]
    pub winner: Option<u8>,
    /// True iff the engine produced an `EngineYield::GameOver`.
    #[pyo3(get)]
    pub terminated: bool,
    /// True iff the harness hit `max_turns` or `max_steps` before
    /// reaching `GameOver`.
    #[pyo3(get)]
    pub truncated: bool,
}

#[pymethods]
impl PyEpisodeOutcome {
    fn __repr__(&self) -> String {
        format!(
            "EpisodeOutcome(winner={:?}, terminated={}, truncated={})",
            self.winner, self.terminated, self.truncated
        )
    }
}

/// One player's trajectory through an episode. Numpy arrays are
/// **struct-of-arrays**: one contiguous array per field. Test
/// downstream callers with `arr.flags['C_CONTIGUOUS'] is True` to
/// pin the contract.
#[pyclass]
pub struct PyTrajectory {
    #[pyo3(get)]
    pub perspective: u8,
    /// `(N, 99)` float32, row-major. Each row is the encoded
    /// observation at decision time, projected from this
    /// perspective.
    #[pyo3(get)]
    pub observations: Py<PyArray2<f32>>,
    /// `(N,)` int32. Index of the chosen action in the legal-action
    /// list at that decision step.
    #[pyo3(get)]
    pub action_indices: Py<PyArray1<i32>>,
    /// `(N,)` float32. Per-step reward (0 except possibly at
    /// terminal).
    #[pyo3(get)]
    pub rewards: Py<PyArray1<f32>>,
    /// Sum of `rewards` (= terminal reward if terminated, else 0).
    #[pyo3(get)]
    pub final_reward: f32,
}

#[pymethods]
impl PyTrajectory {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let n = self.observations.bind(py).readonly().shape()[0];
        Ok(format!(
            "Trajectory(perspective={}, n_steps={}, final_reward={})",
            self.perspective, n, self.final_reward
        ))
    }
}

#[pyclass]
pub struct PyEpisodeResult {
    /// One trajectory per player, indexed by player id (0, 1).
    #[pyo3(get)]
    pub trajectories: Py<PyTuple>,
    #[pyo3(get)]
    pub outcome: Py<PyEpisodeOutcome>,
    #[pyo3(get)]
    pub steps_taken: u32,
    #[pyo3(get)]
    pub turns_taken: u32,
}

#[pymethods]
impl PyEpisodeResult {
    fn __repr__(&self, py: Python<'_>) -> String {
        let outcome = self.outcome.bind(py).borrow();
        format!(
            "EpisodeResult(outcome=EpisodeOutcome(winner={:?}, terminated={}, truncated={}), \
             steps_taken={}, turns_taken={})",
            outcome.winner, outcome.terminated, outcome.truncated, self.steps_taken, self.turns_taken
        )
    }
}

// =============================================================================
// run_episode entry point
// =============================================================================

/// Drive one self-play episode end-to-end and return the per-
/// perspective trajectories.
///
/// Both policy slots accept either a [`PyPolicy`] (built-in
/// constructed via `arcana.policies.*`) or a plain Python callable
/// `(obs: np.ndarray, n_legal: int) -> int`. Passing `None`
/// defaults to `RandomPolicy` seeded from the episode seed.
///
/// `seed` deterministically determines the entire episode trace —
/// engine RNG, default-policy RNGs, and any tie-breaking inside the
/// harness. Re-running with the same seed produces byte-identical
/// trajectories.
#[pyfunction]
#[pyo3(signature = (seed = 0, policy_a = None, policy_b = None, max_turns = 200, max_steps = 5000))]
pub fn run_episode(
    py: Python<'_>,
    seed: u64,
    policy_a: Option<Bound<'_, PyAny>>,
    policy_b: Option<Bound<'_, PyAny>>,
    max_turns: u32,
    max_steps: u32,
) -> PyResult<PyEpisodeResult> {
    // --- Build the engine state from the Lightning Bolt seed setup.
    let mut registry = CardRegistry::new();
    register_seed(&mut registry);
    let deck_a = build_deck(
        &[
            ("Mountain", 10),
            ("Lightning Bolt", 4),
            ("Grizzly Bears", 4),
        ],
        &registry,
    );
    let deck_b = build_deck(&[("Forest", 10), ("Grizzly Bears", 8)], &registry);
    let (state, yld) = new_game(vec![deck_a, deck_b], &registry, seed);

    // --- Resolve policy slots.
    let policy_a_impl = extract_or_default(policy_a.as_ref(), seed ^ 0xA5A5)?;
    let policy_b_impl = extract_or_default(policy_b.as_ref(), seed ^ 0x5A5A)?;

    let mut policies: Vec<Box<dyn Policy>> =
        vec![Box::new(policy_a_impl), Box::new(policy_b_impl)];
    let encoder = BasicE2Encoder::for_two_players();
    let reward = TerminalReward;
    let config = EpisodeConfig { max_turns, max_steps };

    // --- Drive the episode.
    let outcome = selfplay::run_episode(
        state, yld, &registry, &mut policies, &encoder, &reward, &config,
    );

    // --- Convert to Py types.
    convert_to_py_result(py, outcome)
}

fn extract_or_default(
    maybe: Option<&Bound<'_, PyAny>>,
    default_seed: u64,
) -> PyResult<PolicyImpl> {
    let Some(obj) = maybe else {
        return Ok(PolicyImpl::Random(RandomPolicy::new(default_seed)));
    };
    if let Ok(py_pol) = obj.extract::<PyRef<PyPolicy>>() {
        return Ok(py_pol.inner.clone());
    }
    if obj.is_callable() {
        return Ok(PolicyImpl::PythonCallable(obj.clone().unbind()));
    }
    Err(PyTypeError::new_err(
        "policy must be an arcana.policies.* handle or a callable matching \
         (obs: np.ndarray, n_legal: int) -> int",
    ))
}

fn convert_to_py_result(
    py: Python<'_>,
    outcome: selfplay::EpisodeOutcome,
) -> PyResult<PyEpisodeResult> {
    // --- Outcome.
    let (winner, terminated, truncated) = match &outcome.final_yield {
        EngineYield::GameOver(GameResult::Win(p)) => (Some(*p as u8), true, false),
        EngineYield::GameOver(GameResult::Draw) => (None, true, false),
        EngineYield::GameOver(GameResult::Eliminated(p)) => (Some(*p as u8), true, false),
        EngineYield::PendingDecision { .. } => (None, false, true),
    };
    let py_outcome = Py::new(
        py,
        PyEpisodeOutcome { winner, terminated, truncated },
    )?;

    // --- Trajectories.
    let trajs: Vec<Py<PyTrajectory>> = outcome
        .trajectories
        .iter()
        .map(|t| trajectory_to_py(py, t))
        .collect::<PyResult<Vec<_>>>()?;

    let traj_tuple = PyTuple::new(py, trajs.into_iter().map(|t| t.into_any()))?.unbind();

    Ok(PyEpisodeResult {
        trajectories: traj_tuple,
        outcome: py_outcome,
        steps_taken: outcome.steps_taken,
        turns_taken: outcome.turns_taken,
    })
}

fn trajectory_to_py(py: Python<'_>, traj: &Trajectory) -> PyResult<Py<PyTrajectory>> {
    let n = traj.steps.len();
    let dim = BASIC_E2_DIM_TWO_PLAYERS;

    // --- Observations (N, dim) float32.
    let obs_arr = PyArray2::<f32>::zeros(py, [n, dim], false);
    {
        // Safety: just allocated, single owner.
        let slice = unsafe { obs_arr.as_slice_mut()? };
        for (i, step) in traj.steps.iter().enumerate() {
            debug_assert_eq!(step.observation.len(), dim);
            slice[i * dim..(i + 1) * dim].copy_from_slice(&step.observation);
        }
    }

    // --- Action indices (N,) int32.
    let idx_arr = PyArray1::<i32>::zeros(py, [n], false);
    {
        let slice = unsafe { idx_arr.as_slice_mut()? };
        for (i, step) in traj.steps.iter().enumerate() {
            slice[i] = step.action_index.map(|x| x as i32).unwrap_or(-1);
        }
    }

    // --- Rewards (N,) float32.
    let rew_arr = PyArray1::<f32>::zeros(py, [n], false);
    {
        let slice = unsafe { rew_arr.as_slice_mut()? };
        for (i, step) in traj.steps.iter().enumerate() {
            slice[i] = step.reward;
        }
    }

    Py::new(
        py,
        PyTrajectory {
            perspective: traj.perspective as u8,
            observations: obs_arr.unbind(),
            action_indices: idx_arr.unbind(),
            rewards: rew_arr.unbind(),
            final_reward: traj.final_reward,
        },
    )
}

// =============================================================================
// Module helper for lib.rs
// =============================================================================

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPolicy>()?;
    m.add_class::<PyEpisodeOutcome>()?;
    m.add_class::<PyTrajectory>()?;
    m.add_class::<PyEpisodeResult>()?;
    m.add_function(wrap_pyfunction!(run_episode, m)?)?;
    m.add_function(wrap_pyfunction!(_make_random_policy, m)?)?;
    m.add_function(wrap_pyfunction!(_make_progress_biased_policy, m)?)?;
    m.add_function(wrap_pyfunction!(_make_first_action_policy, m)?)?;
    Ok(())
}
