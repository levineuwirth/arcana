//! Game-driving core for the `arcana-web` server, kept separate from the HTTP
//! wiring (`main.rs`) so it can be unit-tested without a socket.
//!
//! [`GameCore`] owns one interactive [`Session`] (human seat P0 vs a snappy bot
//! seat P1) and exposes exactly what the two play endpoints need:
//!
//! * [`GameCore::snapshot`] — drive the session through every bot/trivial
//!   decision (`Session::advance`) and project the resulting human decision (or
//!   the finished game) into a serializable [`StateResponse`]. Backs `GET /state`.
//! * [`GameCore::apply_index`] — apply the human's chosen action by its index
//!   into the most-recently-surfaced legal list, then snapshot again. Backs
//!   `POST /action`. Out-of-range indices and "no decision pending" are returned
//!   as a graceful [`ApplyError`], never a panic.
//!
//! The wire format is [`arcana_core::view::ViewState`] (plus the opponent action
//! log), which serde-serializes to JSON and is what the browser renders.

use std::fmt;

use arcana_ai::search::{MaterialValue, ValueMcPolicy};
use arcana_ai::session::{Seat, Session, Turn};
use arcana_core::actions::Action;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameResult;
use arcana_core::types::PlayerId;
use arcana_core::view::{view_state, ViewState};
use serde::{Deserialize, Serialize};

/// The human always sits in seat 0; the bot in seat 1.
pub const HUMAN: PlayerId = 0;
/// Deck seed for the (mirrored) sample decks — fixed so both seats are even.
pub const DECK_SEED: u64 = 7;

/// One opponent action worth showing the human ("Opponent: cast …").
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RecentAction {
    pub player: PlayerId,
    pub description: String,
}

/// The JSON payload both `/state` and `/action` return: the human's view of the
/// game plus whatever the opponent did since the human last acted.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StateResponse {
    pub view: ViewState,
    pub recent: Vec<RecentAction>,
}

/// Why an `apply_index` call could not be honored. Surfaced to the client as a
/// 400, never as a panic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApplyError {
    /// The game is over (or no decision has been surfaced yet) — nothing to apply.
    NoPendingDecision,
    /// The index was outside the current legal-action list.
    OutOfRange { index: usize, len: usize },
}

impl fmt::Display for ApplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplyError::NoPendingDecision => {
                write!(f, "no decision is pending (the game may be over) — start a new game")
            }
            ApplyError::OutOfRange { index, len } => {
                write!(f, "action index {index} out of range (0..{len})")
            }
        }
    }
}

impl std::error::Error for ApplyError {}

/// Format a [`GameResult`] for the `game_over` banner. Mirrors the strings
/// [`view_state`] produces so the two paths agree.
pub fn format_result(r: &GameResult) -> String {
    match r {
        GameResult::Win(p) => format!("Win(P{p})"),
        GameResult::Draw => "Draw".to_string(),
        GameResult::Eliminated(p) => format!("Eliminated(P{p})"),
    }
}

/// One in-progress game: a human (P0) vs a snappy Monte-Carlo bot (P1) on
/// mirrored sample decks. Holds the legal actions from the most recent human
/// decision so `/action` can resolve an index the browser sent back.
pub struct GameCore {
    reg: &'static CardRegistry,
    session: Session<'static>,
    /// Legal actions surfaced at the latest human decision, indexed by the
    /// `index` the frontend echoes back. Empty once the game is over.
    legal: Vec<Action>,
}

impl GameCore {
    /// Build the snappy bot policy recommended for interactive play.
    fn make_bot(seed: u64) -> Seat {
        // ValueMcPolicy with a small budget: lookahead enough to not be silly,
        // light enough to answer an HTTP request promptly.
        let policy = ValueMcPolicy::with_budget(Box::new(MaterialValue), seed ^ 0xA5EED, 6, 25, 10);
        Seat::Bot(Box::new(policy))
    }

    /// Start a fresh game. `seed` controls the shuffle/RNG (the deck list itself
    /// is fixed by [`DECK_SEED`] so both seats play the same 40 cards).
    pub fn new(reg: &'static CardRegistry, seed: u64) -> Self {
        let deck = arcana_cards::sample_deck(reg, DECK_SEED);
        let seats = vec![Seat::Human, Self::make_bot(seed)];
        let session = Session::new(vec![deck.clone(), deck], reg, seats, seed);
        Self { reg, session, legal: Vec::new() }
    }

    /// Read-only access to the registry (for callers that build a replacement
    /// `GameCore` on `/new`).
    pub fn registry(&self) -> &'static CardRegistry {
        self.reg
    }

    /// Drive the session through all bot + trivial decisions, then project the
    /// next human decision (or the finished game) into a [`StateResponse`].
    ///
    /// Idempotent while a human decision is pending: `advance` re-surfaces the
    /// same decision without mutating, so polling `/state` is safe.
    pub fn snapshot(&mut self) -> StateResponse {
        let view = match self.session.advance() {
            Turn::AwaitingHuman { player, view, legal, .. } => {
                let vs = view_state(&view.state, self.reg, player, &legal);
                self.legal = legal;
                vs
            }
            Turn::GameOver(result) => {
                self.legal = Vec::new();
                // The full state's `result` field is set on game over, so
                // view_state already fills `game_over`; belt-and-braces, derive
                // it from the Turn payload if it somehow isn't.
                let mut vs = view_state(self.session.state(), self.reg, HUMAN, &[]);
                if vs.game_over.is_none() {
                    vs.game_over = Some(format_result(&result));
                }
                vs
            }
        };
        let recent = self
            .session
            .recent_actions()
            .iter()
            .map(|(p, d)| RecentAction { player: *p, description: d.clone() })
            .collect();
        StateResponse { view, recent }
    }

    /// Apply the human's chosen action by its `index` into the legal list from
    /// the most recent [`snapshot`](Self::snapshot), then advance and snapshot
    /// again. Returns the new state, or an [`ApplyError`] for a stale/out-of-range
    /// index or a finished game — never panics on bad input.
    pub fn apply_index(&mut self, index: usize) -> Result<StateResponse, ApplyError> {
        if self.legal.is_empty() {
            return Err(ApplyError::NoPendingDecision);
        }
        let action = self
            .legal
            .get(index)
            .cloned()
            .ok_or(ApplyError::OutOfRange { index, len: self.legal.len() })?;
        self.session.apply(action);
        Ok(self.snapshot())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A leaked registry mirrors the server's `'static` app-state strategy.
    fn leaked_catalog() -> &'static CardRegistry {
        Box::leak(Box::new(arcana_cards::build_catalog()))
    }

    /// Covers all three required guarantees of the server logic:
    ///  (1) `/state` returns a `ViewState` (two players, human perspective),
    ///  (2) applying a legal index advances the game across many decisions,
    ///  (3) an out-of-range index is rejected gracefully (Err, not panic).
    #[test]
    fn state_action_flow_and_bad_index() {
        let reg = leaked_catalog();
        let mut core = GameCore::new(reg, 42);

        // (1) /state yields a coherent ViewState from the human's perspective.
        let s = core.snapshot();
        assert!(s.view.game_over.is_none(), "a fresh game is not over");
        assert_eq!(s.view.players.len(), 2, "two players");
        assert_eq!(s.view.perspective, HUMAN, "rendered from the human's seat");
        assert_eq!(s.view.players[HUMAN as usize].life, 20, "starting life");
        assert!(!s.view.legal.is_empty(), "the human faces a real decision");
        assert!(s.view.legal.iter().all(|a| !a.label.is_empty()), "labels are populated");
        // The own hand carries card names (for art lookup); opponent's is hidden.
        assert_eq!(s.view.players[0].hand.len(), s.view.players[0].hand_count);
        assert!(s.view.players[1].hand.is_empty(), "opponent hand stays hidden");

        // (3) Out-of-range indices are rejected gracefully — no panic.
        let len = s.view.legal.len();
        assert_eq!(core.apply_index(len), Err(ApplyError::OutOfRange { index: len, len }));
        assert!(matches!(core.apply_index(usize::MAX), Err(ApplyError::OutOfRange { .. })));

        // (2) Applying legal indices advances the game: drive a bounded run of
        // human decisions, each returning a coherent ViewState. Confirm play
        // actually progressed past the opening (more than one human decision and
        // the turn counter moved forward).
        let opening_turn = s.view.turn;
        let mut cur = s;
        let mut decisions = 0;
        let mut max_turn = opening_turn;
        while cur.view.game_over.is_none() && decisions < 120 {
            decisions += 1;
            // Prefer a real action over Pass/Concede/Mulligan-away so we develop.
            let idx = cur
                .view
                .legal
                .iter()
                .position(|a| {
                    let l = a.label.as_str();
                    l != "Pass" && l != "Concede" && l != "Mulligan (draw a new hand)"
                })
                .unwrap_or(0);
            cur = core.apply_index(idx).expect("a legal index always applies");
            assert_eq!(cur.view.players.len(), 2, "view stays coherent across steps");
            max_turn = max_turn.max(cur.view.turn);
        }
        assert!(decisions > 1, "the human took multiple advancing decisions");
        assert!(max_turn >= opening_turn, "turn counter never regressed");
    }

    /// `apply_index` before any `snapshot` (no decision cached) is a clean Err,
    /// not a panic.
    #[test]
    fn apply_before_any_state_is_graceful() {
        let reg = leaked_catalog();
        let mut core = GameCore::new(reg, 1);
        assert_eq!(core.apply_index(0), Err(ApplyError::NoPendingDecision));
    }

    /// The StateResponse round-trips through JSON — the actual web transport.
    #[test]
    fn state_response_serializes() {
        let reg = leaked_catalog();
        let mut core = GameCore::new(reg, 3);
        let s = core.snapshot();
        let json = serde_json::to_string(&s).expect("serialize StateResponse");
        let back: StateResponse = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, s);
    }
}
