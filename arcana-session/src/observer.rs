//! Event sinks attached to a [`GameSession`].
//!
//! Observers receive the [`GameEvent`]s emitted by each engine step,
//! plus lifecycle callbacks for pending decisions, game end, and
//! undo. Each call site runs every observer in registration order —
//! if one observer panics, later observers and the session itself go
//! down with it, so observers must treat their work as infallible or
//! catch their own panics.
//!
//! The built-in [`LogObserver`] writes events to any `std::io::Write`
//! using `Debug`. Richer observers (metrics, replay recorders,
//! UI broadcasters) belong in downstream crates once they have a
//! consumer to justify their shape.
//!
//! [`GameSession`]: crate::GameSession

use std::io::Write;

use arcana_core::{Action, GameState};
use arcana_core::actions::DecisionContext;
use arcana_core::events::GameEvent;
use arcana_core::state::GameResult;
use arcana_core::types::PlayerId;

/// An event sink attached to a [`GameSession`]. All methods have
/// default no-op impls so observers only override what they care
/// about.
///
/// [`GameSession`]: crate::GameSession
pub trait GameObserver: Send {
    /// Called once per engine step, with the slice of events emitted
    /// by that step and the post-step state.
    fn on_events(&mut self, _events: &[GameEvent], _state: &GameState) {}

    /// Called when the engine yields a pending decision and the
    /// session is about to dispatch it to an agent (i.e. after any
    /// auto-pass filter). Useful for UI rendering.
    fn on_pending_decision(
        &mut self,
        _player: PlayerId,
        _legal_actions: &[Action],
        _context: &DecisionContext,
    ) {}

    /// Called exactly once, when `GameSession::run` is about to
    /// return.
    fn on_game_over(&mut self, _result: &GameResult) {}

    /// Called when `GameSession::undo` successfully restores a prior
    /// state. The argument is the new current state.
    fn on_undo(&mut self, _state: &GameState) {}
}

/// Writes each [`GameEvent`] to a `Write` sink on its own line using
/// `Debug`. Good enough for test logs, `println!`-style debugging,
/// and the `arcana-cli` replay viewer.
pub struct LogObserver<W: Write + Send> {
    writer: W,
}

impl<W: Write + Send> LogObserver<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W: Write + Send> GameObserver for LogObserver<W> {
    fn on_events(&mut self, events: &[GameEvent], _state: &GameState) {
        for e in events {
            let _ = writeln!(self.writer, "{e:?}");
        }
    }

    fn on_game_over(&mut self, result: &GameResult) {
        let _ = writeln!(self.writer, "GAME OVER: {result:?}");
    }
}
