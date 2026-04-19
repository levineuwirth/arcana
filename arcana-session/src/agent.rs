//! Per-player decision surface.
//!
//! Every [`PlayerAgent`] variant answers the same question: given the
//! current state and the set of legal actions, produce an `Action`.
//! [`GameSession`] calls [`PlayerAgent::request_decision`] whenever
//! the engine yields a `PendingDecision` that isn't auto-passed.
//!
//! The `Human` variant is reserved for a future remote-player
//! connection and is intentionally empty for now — a placeholder in
//! the enum so callers can match exhaustively.
//!
//! [`GameSession`]: crate::GameSession

use arcana_core::{Action, GameState};
use arcana_core::actions::DecisionContext;
use rand_chacha::ChaCha8Rng;

/// A player in a [`GameSession`].
///
/// [`GameSession`]: crate::GameSession
pub enum PlayerAgent {
    /// Uniform-random legal-action picker. Seeded per player so
    /// sessions are replayable. The random agent's heuristic is the
    /// same one `arcana-core` integration tests use:
    ///
    /// * On a mulligan prompt, always `MulliganKeep` (otherwise a
    ///   random agent stalls the game by mulliganning forever).
    /// * Otherwise filter out `PassPriority` / `Concede`; pick
    ///   uniformly from what's left.
    /// * Fall back to `PassPriority` if only trivial actions remain.
    Random { rng: ChaCha8Rng },

    /// A policy-driven agent. The concrete policy lives behind
    /// [`AiPolicy`] and receives full state access (not an
    /// information-set projection); projecting for hidden information
    /// is the AI crate's job, not the session's.
    Ai { policy: Box<dyn AiPolicy> },

    /// Reserved. A future human-player variant will carry a
    /// connection handle (WebSocket, IPC, etc.) that streams
    /// decisions back from a client. Empty today to keep the enum
    /// shape stable.
    Human,
}

/// Plug-in point for reinforcement-learning or heuristic policies.
pub trait AiPolicy: Send {
    fn select_action(
        &mut self,
        state: &GameState,
        legal_actions: &[Action],
        context: &DecisionContext,
    ) -> Action;
}

impl PlayerAgent {
    /// Ask this agent for its next action. Panics if the legal-action
    /// set is empty — the engine is required to offer at least
    /// `Action::Concede` at every priority point, so an empty set
    /// indicates a core-engine bug.
    pub fn request_decision(
        &mut self,
        state: &GameState,
        legal_actions: &[Action],
        context: &DecisionContext,
    ) -> Action {
        assert!(!legal_actions.is_empty(),
            "PlayerAgent::request_decision called with no legal actions");
        match self {
            PlayerAgent::Random { rng } => pick_random(rng, legal_actions),
            PlayerAgent::Ai { policy } => {
                policy.select_action(state, legal_actions, context)
            }
            PlayerAgent::Human => panic!(
                "PlayerAgent::Human has no decision source yet — \
                 the remote-connection variant is reserved for a \
                 future phase. Use Random or Ai."
            ),
        }
    }
}

fn pick_random(rng: &mut ChaCha8Rng, actions: &[Action]) -> Action {
    use rand::Rng;

    if actions.iter().any(|a| matches!(a, Action::MulliganKeep)) {
        return Action::MulliganKeep;
    }
    let interesting: Vec<&Action> = actions.iter()
        .filter(|a| !a.is_pass() && !a.is_concede())
        .collect();
    if !interesting.is_empty() {
        let idx = rng.gen_range(0..interesting.len());
        return interesting[idx].clone();
    }
    if let Some(p) = actions.iter().find(|a| a.is_pass()) {
        return p.clone();
    }
    actions[0].clone()
}
