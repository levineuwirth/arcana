//! Rust ↔ Python type conversion helpers.
//!
//! v0: empty placeholder. Lands here when the engine surface needs
//! to expose `GameState`, `Action`, or `Observation` directly to
//! Python — e.g. when MCTS rollouts are driven from Python and need
//! to read structured state mid-game. v0's `MtgEnv` keeps the
//! `GameState` opaque on the Rust side and exposes only encoded
//! observations, so no conversions are needed yet.
