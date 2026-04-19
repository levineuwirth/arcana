//! Session layer for `arcana-core`.
//!
//! The core engine (`arcana-core`) is a pure function from
//! `(GameState, Action)` to `(GameState, EngineYield)`. That is the
//! right shape for the AI/RL path, which drives it directly in a tight
//! loop. For human play, interactive debugging, replay, and any other
//! consumer that needs orchestration around the engine, this crate
//! provides a thin wrapper.
//!
//! # What lives here
//!
//! * [`GameSession`] owns a [`GameState`] plus a shared
//!   [`CardRegistry`], drives it to completion, and dispatches
//!   decisions to per-player [`PlayerAgent`]s.
//! * [`GameObserver`] is a sink for [`GameEvent`]s and lifecycle
//!   hooks. [`LogObserver`] writes human-readable lines to a
//!   `std::io::Write`.
//! * [`StopSettings`] and [`StopCondition`] control auto-pass: the
//!   session passes priority on a player's behalf unless a stop
//!   fires or the legal-action set contains more than just
//!   `PassPriority` / `Concede`.
//! * [`PlayerAgent`] is the common decision surface for `Random`
//!   bots (for tests, benchmarking, demos) and `Ai` policies (any
//!   object that implements [`AiPolicy`]).
//!
//! # What *doesn't* live here
//!
//! The AI/RL training loop never touches this crate. It calls
//! `arcana_core::step` directly for maximum throughput. This split is
//! deliberate: the training path pays nothing for session-layer
//! features it does not use.
//!
//! # Sync for now
//!
//! The v0.2 spec describes this layer with `async` methods (to
//! accommodate a future [`Human`]/network connection that awaits on
//! player input). Until a real remote-player connection exists, every
//! public method is synchronous — `Random` and `Ai` policies both
//! return decisions without yielding. When the Human path lands, the
//! signatures can grow `async` without reshaping the rest.
//!
//! [`GameState`]: arcana_core::GameState
//! [`CardRegistry`]: arcana_core::CardRegistry
//! [`GameEvent`]: arcana_core::events::GameEvent
//! [`Human`]: PlayerAgent::Human

pub mod agent;
pub mod observer;
pub mod session;
pub mod stop;

pub use agent::{AiPolicy, PlayerAgent};
pub use observer::{GameObserver, LogObserver};
pub use session::{GameSession, GameSessionBuilder, SessionBuildError, UndoError};
pub use stop::{StopCondition, StopSettings};
