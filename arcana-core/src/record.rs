//! Deterministic game record + replay.
//!
//! The engine is deterministic: a full game is captured by `(decks, seed,
//! actions)`. A [`GameRecord`] is therefore a tiny, fully-serializable game
//! transcript ([`Action`] and [`GameResult`] both derive serde) — and
//! [`replay_to`] reconstructs any intermediate [`GameState`] by re-running it,
//! WITHOUT serializing the live state (which can't yet, due to fn-pointer-
//! bearing continuous/replacement/triggered effects — see the TODO in
//! `state.rs`). This is the observability/repro backbone for self-play
//! analysis, human play, and multiplayer.

use serde::{Deserialize, Serialize};

use crate::actions::Action;
use crate::engine::{new_game, step, EngineYield};
use crate::registry::CardRegistry;
use crate::state::{GameResult, GameState};
use crate::types::CardId;

/// A complete, replayable game transcript.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameRecord {
    /// Per-player starting decks (the `new_game` argument).
    pub decks: Vec<Vec<CardId>>,
    /// RNG seed passed to `new_game` (drives shuffles).
    pub seed: u64,
    /// The action taken at each successive decision, in order.
    pub actions: Vec<Action>,
    /// The terminal result, once the recorded game ended (`None` if the record
    /// is partial / still in progress).
    pub result: Option<GameResult>,
}

impl GameRecord {
    /// An empty record for a game about to be played/recorded.
    pub fn new(decks: Vec<Vec<CardId>>, seed: u64) -> Self {
        Self { decks, seed, actions: Vec::new(), result: None }
    }

    /// Number of recorded decisions.
    pub fn len(&self) -> usize { self.actions.len() }
    pub fn is_empty(&self) -> bool { self.actions.is_empty() }
}

/// Re-run a record, applying its first `n` actions (clamped to the record
/// length), and return the resulting `(GameState, EngineYield)`. `n >=
/// actions.len()` replays the entire transcript. Stops early if the game ends
/// before `n` actions (extra actions in a well-formed record never do).
pub fn replay_to(
    record: &GameRecord,
    registry: &CardRegistry,
    n: usize,
) -> (GameState, EngineYield) {
    let (mut state, mut yld) = new_game(record.decks.clone(), registry, record.seed);
    for action in record.actions.iter().take(n) {
        if matches!(yld, EngineYield::GameOver(_)) { break; }
        let (s, y) = step(state, action.clone(), registry);
        state = s;
        yld = y;
    }
    (state, yld)
}

/// Replay the whole record to its final `(GameState, EngineYield)`.
pub fn replay(record: &GameRecord, registry: &CardRegistry) -> (GameState, EngineYield) {
    replay_to(record, registry, record.actions.len())
}

// Integration tests (which need the card catalog via arcana-cards) live in
// arcana-ai, where arcana-core is a normal dependency and the types unify —
// arcana-core's own dev-dep on arcana-cards would otherwise compile a second,
// non-unifying copy of arcana-core.
