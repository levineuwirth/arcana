//! [`GameSession`] drives the core engine to completion.
//!
//! A session owns the current [`GameState`], a shared
//! [`CardRegistry`] (immutable after startup), and per-player
//! [`PlayerAgent`]s. Calling [`GameSession::run`] loops the engine's
//! yield-driven state machine until it produces a [`GameResult`].
//!
//! # Undo buffer
//!
//! [`GameSession::apply`] snapshots the pre-action state into a
//! bounded ring buffer before mutating, and [`GameSession::undo`]
//! pops the most recent snapshot. The buffer size is set by
//! [`GameSessionBuilder::history_depth`] (default 0 — undo disabled).
//! Snapshots are full `GameState::clone`s, so enabling undo roughly
//! doubles per-step memory traffic; the AI/RL path, which doesn't
//! need it, avoids the cost by using `arcana_core::step` directly.
//!
//! [`CardRegistry`]: arcana_core::CardRegistry
//! [`GameResult`]: arcana_core::state::GameResult

use std::collections::VecDeque;
use std::sync::Arc;

use arcana_core::{Action, CardRegistry, FormatConfig, GameState};
use arcana_core::actions::DecisionContext;
use arcana_core::engine::{self, EngineYield};
use arcana_core::format::DeckValidationError;
use arcana_core::state::GameResult;
use arcana_core::types::{CardId, PlayerId};

use crate::agent::PlayerAgent;
use crate::observer::GameObserver;
use crate::stop::{should_auto_pass, StopSettings};

/// Orchestration wrapper around `arcana_core`. See the crate-level
/// docs and the module docs above for the full contract.
pub struct GameSession {
    state: GameState,
    pending: EngineYield,
    registry: Arc<CardRegistry>,
    #[allow(dead_code)]  // surfaced for inspection; consulted by future session logic.
    format: FormatConfig,
    agents: Vec<PlayerAgent>,
    stop_settings: Vec<StopSettings>,
    observers: Vec<Box<dyn GameObserver>>,
    history: VecDeque<GameState>,
    history_depth: usize,
    events_logged: usize,
}

impl GameSession {
    pub fn builder() -> GameSessionBuilder {
        GameSessionBuilder::default()
    }

    /// Drive the game to completion, dispatching each pending
    /// decision to the relevant [`PlayerAgent`] and notifying every
    /// registered observer along the way. Returns the [`GameResult`]
    /// the engine settled on.
    ///
    /// Panics if the session has already ended (the `GameOver` yield
    /// is preserved, so calling `run` twice is a programmer bug).
    pub fn run(&mut self) -> GameResult {
        loop {
            // Take ownership of the current yield so we can match on
            // it while still mutating `self` inside the arms. The
            // `GameOver` arm restores it so the session stays in the
            // terminal state for any post-run introspection.
            let yld = std::mem::replace(
                &mut self.pending,
                EngineYield::GameOver(GameResult::Draw),
            );
            match yld {
                EngineYield::GameOver(result) => {
                    for obs in &mut self.observers {
                        obs.on_game_over(&result);
                    }
                    self.pending = EngineYield::GameOver(result.clone());
                    return result;
                }
                EngineYield::PendingDecision { player, legal_actions, context } => {
                    let settings = self.stop_settings
                        .get(player as usize)
                        .cloned()
                        .unwrap_or_default();
                    let action = if should_auto_pass(&settings, &legal_actions) {
                        Action::PassPriority
                    } else {
                        for obs in &mut self.observers {
                            obs.on_pending_decision(player, &legal_actions, &context);
                        }
                        self.agents[player as usize]
                            .request_decision(&self.state, &legal_actions, &context)
                    };
                    self.apply(action);
                }
            }
        }
    }

    /// Apply a single action outside the [`run`] loop. Useful for
    /// interactive drivers (an `arcana-cli` debugger) and for tests
    /// that want to step the session one move at a time. Snapshots
    /// into the undo buffer if history is enabled; notifies observers
    /// of any events emitted by the step.
    ///
    /// [`run`]: Self::run
    pub fn apply(&mut self, action: Action) {
        if self.history_depth > 0 {
            if self.history.len() == self.history_depth {
                self.history.pop_front();
            }
            self.history.push_back(self.state.clone());
        }

        let before = self.state.event_log.len();
        let (new_state, yld) = engine::step(
            self.state.clone(),
            action,
            &self.registry,
        );
        self.state = new_state;

        let tail = self.state.event_log[before..].to_vec();
        self.events_logged += tail.len();
        for obs in &mut self.observers {
            obs.on_events(&tail, &self.state);
        }
        self.pending = yld;
    }

    /// Restore the most recent pre-action state from the undo buffer.
    /// Errors if the buffer is empty or undo was never enabled.
    pub fn undo(&mut self) -> Result<(), UndoError> {
        let prior = self.history.pop_back().ok_or(UndoError::NoHistory)?;
        self.state = prior;
        // We don't store pre-action EngineYields, so recompute the
        // yield by asking the engine for the decision that would be
        // pending now. `step` on `PassPriority` would mutate, so
        // instead we use `legal_actions` + a synthesized Priority
        // context — adequate for the scaffolding. Richer restoration
        // is a Phase 3 polish task.
        let legal = arcana_core::legal_actions::legal_actions(
            &self.state, &self.registry,
        );
        let player = self.state.priority.player;
        self.pending = EngineYield::PendingDecision {
            player,
            legal_actions: legal,
            context: DecisionContext::Priority,
        };
        for obs in &mut self.observers {
            obs.on_undo(&self.state);
        }
        Ok(())
    }

    // --- read-only accessors for tests / UIs ---------------------------------

    pub fn state(&self) -> &GameState { &self.state }

    pub fn registry(&self) -> &CardRegistry { &self.registry }

    pub fn pending(&self) -> &EngineYield { &self.pending }

    /// Total number of [`GameEvent`]s emitted across every applied
    /// action in this session's lifetime.
    ///
    /// [`GameEvent`]: arcana_core::events::GameEvent
    pub fn events_logged(&self) -> usize { self.events_logged }

    /// Current depth of the undo ring buffer (number of restorable
    /// prior states).
    pub fn history_depth(&self) -> usize { self.history.len() }
}

// =============================================================================
// Builder
// =============================================================================

/// Fluent constructor for [`GameSession`]. Every field has a
/// defaulted value except for decks, the registry, and at least one
/// agent — those three are required and [`build`] reports the
/// missing ones via [`SessionBuildError`].
///
/// [`build`]: Self::build
pub struct GameSessionBuilder {
    registry: Option<Arc<CardRegistry>>,
    format: Option<FormatConfig>,
    decks: Vec<Option<Vec<CardId>>>,
    agents: Vec<Option<PlayerAgent>>,
    stop_settings: Vec<StopSettings>,
    observers: Vec<Box<dyn GameObserver>>,
    seed: u64,
    history_depth: usize,
    skip_validation: bool,
}

impl Default for GameSessionBuilder {
    fn default() -> Self {
        Self {
            registry: None,
            format: None,
            decks: Vec::new(),
            agents: Vec::new(),
            stop_settings: Vec::new(),
            observers: Vec::new(),
            seed: 0,
            history_depth: 0,
            skip_validation: false,
        }
    }
}

impl GameSessionBuilder {
    pub fn registry(mut self, r: Arc<CardRegistry>) -> Self {
        self.registry = Some(r); self
    }

    pub fn format(mut self, f: FormatConfig) -> Self {
        self.format = Some(f); self
    }

    /// Register the deck for `player`. Overrides any prior deck for
    /// the same index.
    pub fn deck(mut self, player: usize, deck: Vec<CardId>) -> Self {
        ensure_len(&mut self.decks, player + 1);
        self.decks[player] = Some(deck);
        self
    }

    /// Register the decision source for `player`. Overrides any
    /// prior agent for the same index.
    pub fn agent(mut self, player: usize, agent: PlayerAgent) -> Self {
        ensure_len(&mut self.agents, player + 1);
        self.agents[player] = Some(agent);
        self
    }

    /// Install a custom [`StopSettings`] for `player`. Defaults to
    /// `StopSettings::default()` (auto-pass on trivial priority).
    pub fn stop_settings(mut self, player: usize, s: StopSettings) -> Self {
        ensure_len_default(&mut self.stop_settings, player + 1);
        self.stop_settings[player] = s;
        self
    }

    /// Attach an observer. Observers run in registration order on
    /// every relevant callback.
    pub fn observer(mut self, obs: Box<dyn GameObserver>) -> Self {
        self.observers.push(obs); self
    }

    pub fn seed(mut self, seed: u64) -> Self { self.seed = seed; self }

    /// How many prior states to retain for `undo`. Default 0 — undo
    /// disabled. Must be set to `>= 1` for [`GameSession::undo`] to
    /// succeed on the very first action.
    pub fn history_depth(mut self, n: usize) -> Self {
        self.history_depth = n; self
    }

    /// Skip [`FormatConfig::validate_deck`] in [`Self::build`]. Use
    /// for tests that intentionally construct non-conforming decks to
    /// exercise engine behavior in isolation (e.g. a 48-card
    /// playtest deck for a combat corner case). Production drivers
    /// that take untrusted decks — the eventual CLI runner, or
    /// generated decks from `arcana-gen` — should leave validation
    /// on so a malformed deck fails loudly at session construction
    /// instead of as a mid-game empty-library loss.
    pub fn skip_validation(mut self) -> Self {
        self.skip_validation = true; self
    }

    pub fn build(self) -> Result<GameSession, SessionBuildError> {
        let registry = self.registry.ok_or(SessionBuildError::MissingRegistry)?;
        let format = self.format.unwrap_or_else(FormatConfig::standard_2026);

        let Self {
            decks, agents, stop_settings, observers, seed, history_depth,
            skip_validation,
            ..
        } = self;

        let n_players = decks.len().max(agents.len());
        if n_players < 2 {
            return Err(SessionBuildError::TooFewPlayers(n_players));
        }

        let mut deck_vec = Vec::with_capacity(n_players);
        for (i, d) in decks.into_iter().enumerate() {
            deck_vec.push(d.ok_or(SessionBuildError::MissingDeck(i))?);
        }
        while deck_vec.len() < n_players {
            return Err(SessionBuildError::MissingDeck(deck_vec.len()));
        }

        if !skip_validation {
            for (i, deck) in deck_vec.iter().enumerate() {
                if let Err(errors) = format.validate_deck(deck) {
                    return Err(SessionBuildError::InvalidDeck {
                        player: i as PlayerId,
                        errors,
                    });
                }
            }
        }

        let mut agent_vec = Vec::with_capacity(n_players);
        for (i, a) in agents.into_iter().enumerate() {
            agent_vec.push(a.ok_or(SessionBuildError::MissingAgent(i))?);
        }
        while agent_vec.len() < n_players {
            return Err(SessionBuildError::MissingAgent(agent_vec.len()));
        }

        let mut stops = stop_settings;
        ensure_len_default(&mut stops, n_players);

        let (state, yld) = engine::new_game_with_format(
            deck_vec, format.clone(), &registry, seed,
        );

        Ok(GameSession {
            state,
            pending: yld,
            registry,
            format,
            agents: agent_vec,
            stop_settings: stops,
            observers,
            history: VecDeque::with_capacity(history_depth),
            history_depth,
            events_logged: 0,
        })
    }
}

fn ensure_len<T>(v: &mut Vec<Option<T>>, n: usize) {
    while v.len() < n { v.push(None); }
}

fn ensure_len_default<T: Default>(v: &mut Vec<T>, n: usize) {
    while v.len() < n { v.push(T::default()); }
}

// =============================================================================
// Errors
// =============================================================================

#[derive(Debug, thiserror::Error)]
pub enum SessionBuildError {
    #[error("missing registry — call GameSessionBuilder::registry")]
    MissingRegistry,
    #[error("missing deck for player {0}")]
    MissingDeck(usize),
    #[error("missing agent for player {0}")]
    MissingAgent(usize),
    #[error("need at least 2 players, got {0}")]
    TooFewPlayers(usize),
    #[error("deck for player {player} fails format validation: {errors:?}")]
    InvalidDeck {
        player: PlayerId,
        errors: Vec<DeckValidationError>,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum UndoError {
    #[error("undo buffer is empty")]
    NoHistory,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_errors_are_informative() {
        let result = GameSession::builder().build();
        let err = match result {
            Ok(_) => panic!("expected MissingRegistry, got Ok"),
            Err(e) => e,
        };
        assert!(matches!(err, SessionBuildError::MissingRegistry));
    }
}
