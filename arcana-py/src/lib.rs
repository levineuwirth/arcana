//! PyO3 bindings exposing the engine + AI utilities to Python.
//!
//! Build with `maturin develop` from the `arcana-py/` directory; see
//! `README.md` for the full setup. After install:
//!
//! ```python
//! import arcana
//! env = arcana.MtgEnv(num_players=2, seed=0)
//! obs, info = env.reset()
//! assert obs.shape == (arcana.BASIC_E2_DIM_TWO_PLAYERS,)
//! ```
//!
//! v0 is intentionally a stub: `MtgEnv.step()` raises
//! `NotImplementedError` because legal-action enumeration is not
//! plumbed through yet. The point of shipping the binding now is
//! that downstream RL harness work can target a real importable
//! module instead of a hypothetical one.

use pyo3::prelude::*;

pub mod conversions;
pub mod env;
pub mod episode;

/// Module entry point. Name must match the cdylib name
/// (`arcana_py`) for PyO3 to register it correctly. Maturin's
/// `module-name = "arcana.arcana_py"` in `pyproject.toml` lands the
/// compiled `.so` inside the `arcana` Python package, so consumers
/// import as `arcana.arcana_py` (or via `from .arcana_py import *`
/// in `python/arcana/__init__.py`).
#[pymodule]
fn arcana_py(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<env::MtgEnv>()?;
    episode::register(m)?;
    m.add(
        "BASIC_E2_DIM_TWO_PLAYERS",
        arcana_ai::observation::BASIC_E2_DIM_TWO_PLAYERS,
    )?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
