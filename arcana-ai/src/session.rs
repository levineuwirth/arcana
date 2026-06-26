//! UI-agnostic interactive game session — the shared core for human play,
//! reused as-is by the CLI now and a GUI later.
//!
//! The naive approach (a `HumanPolicy: StatePolicy` that blocks on stdin inside
//! `choose`) can't back a GUI, which is event-driven and can't sit inside a
//! blocking call. So this is a RESUMABLE driver instead of a loop:
//!
//! * [`Session::advance`] runs the engine through every BOT decision and every
//!   trivial decision (a single legal action, or "pass / concede") on its own,
//!   then returns control with a [`Turn`] — either `AwaitingHuman` (whose move
//!   it is, that player's projected view, the legal actions, and the decision
//!   context) or `GameOver`.
//! * [`Session::apply`] applies the human's chosen action; the caller then calls
//!   `advance` again.
//!
//! Both `Turn` payloads are STRUCTURED data ([`ObservableState`] + `Vec<Action>`),
//! never rendered strings — a GUI reads the fields directly; the CLI renders
//! them. Hidden information is handled here: each human decision exposes
//! [`project`]ed state (own hand visible, opponents' hidden). Every action is
//! recorded into a [`GameRecord`] for replay.

use arcana_core::actions::{Action, DecisionContext};
use arcana_core::engine::{new_game, step, EngineYield};
use arcana_core::record::GameRecord;
use arcana_core::registry::CardRegistry;
use arcana_core::state::{GameResult, GameState};
use arcana_core::types::{CardId, PlayerId};

use crate::information_set::{project, ObservableState};
use crate::search::StatePolicy;

/// Who controls a seat: a human (the frontend prompts) or a bot policy.
pub enum Seat {
    Human,
    Bot(Box<dyn StatePolicy>),
}

/// What [`Session::advance`] surfaces: a human decision (with everything the
/// frontend needs to present it) or the finished game.
pub enum Turn {
    AwaitingHuman {
        player: PlayerId,
        /// The deciding player's view (own hand visible, opponents anonymized).
        view: ObservableState,
        /// The legal actions to choose among (length ≥ 2; trivial decisions are
        /// auto-resolved and never surfaced).
        legal: Vec<Action>,
        context: DecisionContext,
    },
    GameOver(GameResult),
}

/// An interactive session over one game. Borrows the registry for its lifetime.
pub struct Session<'a> {
    state: GameState,
    yld: EngineYield,
    registry: &'a CardRegistry,
    seats: Vec<Seat>,
    record: GameRecord,
}

/// A decision the engine can resolve without prompting: a single legal action,
/// or "nothing to do but pass" (only pass/concede offered → pass). Applied to
/// humans AND bots so neither is pestered with forced passes.
fn auto_action(legal: &[Action]) -> Option<Action> {
    if legal.len() <= 1 {
        return legal.first().cloned();
    }
    if legal.iter().all(|a| matches!(a, Action::PassPriority | Action::Concede)) {
        return legal.iter().find(|a| matches!(a, Action::PassPriority)).cloned();
    }
    None
}

impl<'a> Session<'a> {
    /// Start a game. `seats[p]` controls player `p`; `decks.len()` must equal
    /// `seats.len()`.
    pub fn new(
        decks: Vec<Vec<CardId>>,
        registry: &'a CardRegistry,
        seats: Vec<Seat>,
        seed: u64,
    ) -> Self {
        assert_eq!(decks.len(), seats.len(), "one seat per deck");
        let record = GameRecord::new(decks.clone(), seed);
        let (state, yld) = new_game(decks, registry, seed);
        Self { state, yld, registry, seats, record }
    }

    /// The current (full, un-projected) game state — for spectator rendering.
    pub fn state(&self) -> &GameState { &self.state }
    /// The action transcript so far (replayable via `arcana_core::record`).
    pub fn record(&self) -> &GameRecord { &self.record }

    /// Drive the game through bot + trivial decisions until a human must choose
    /// or the game ends.
    pub fn advance(&mut self) -> Turn {
        loop {
            let (player, legal, context) = match &self.yld {
                EngineYield::GameOver(r) => {
                    let r = r.clone();
                    self.record.result = Some(r.clone());
                    return Turn::GameOver(r);
                }
                EngineYield::PendingDecision { player, legal_actions, context } => {
                    (*player, legal_actions.clone(), context.clone())
                }
            };

            if let Some(action) = auto_action(&legal) {
                self.apply_internal(action);
                continue;
            }

            // Either surface the human decision or let the bot choose. The match
            // evaluates to the bot's action; the human arm returns early.
            let action = match &mut self.seats[player as usize] {
                Seat::Human => {
                    return Turn::AwaitingHuman {
                        player,
                        view: project(&self.state, player),
                        legal,
                        context,
                    };
                }
                Seat::Bot(policy) => policy.choose(&self.state, self.registry, player, &legal),
            };
            self.apply_internal(action);
        }
    }

    /// Apply the human's chosen `action` (from the most recent `AwaitingHuman`).
    /// Call `advance` afterward to continue. Panics if no decision is pending.
    pub fn apply(&mut self, action: Action) {
        assert!(matches!(self.yld, EngineYield::PendingDecision { .. }),
            "Session::apply called with no pending decision");
        self.apply_internal(action);
    }

    fn apply_internal(&mut self, action: Action) {
        self.record.actions.push(action.clone());
        let n = self.state.num_players();
        let state = std::mem::replace(&mut self.state, GameState::new(n, 0));
        let (s, y) = step(state, action, self.registry);
        self.state = s;
        self.yld = y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::RandomStatePolicy;

    /// A scripted "human" (progress-biased: keep mulligans, never concede, first
    /// real action) drives a Session against a bot to completion — validates the
    /// advance/apply core end-to-end without real I/O.
    #[test]
    fn scripted_human_vs_bot_completes() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        let seats = vec![Seat::Human, Seat::Bot(Box::new(RandomStatePolicy::new(2)))];
        let mut session = Session::new(vec![deck.clone(), deck], &reg, seats, 42);

        let mut human_decisions = 0;
        let mut guard = 0;
        let result = loop {
            guard += 1;
            assert!(guard < 100_000, "session failed to terminate");
            match session.advance() {
                Turn::GameOver(r) => break r,
                Turn::AwaitingHuman { legal, player, .. } => {
                    human_decisions += 1;
                    assert_eq!(player, 0, "only P0 is human");
                    assert!(legal.len() >= 2, "trivial decisions should be auto-resolved");
                    // Pick: keep a mulligan, else first non-concede.
                    let pick = legal.iter()
                        .position(|a| matches!(a, Action::MulliganKeep))
                        .or_else(|| legal.iter().position(|a| !a.is_concede()))
                        .unwrap_or(0);
                    session.apply(legal[pick].clone());
                }
            }
        };
        let _ = result;
        assert!(human_decisions > 0, "the human should have faced real decisions");
        // The transcript replays (record was populated + result set).
        assert!(session.record().result.is_some());
        assert!(!session.record().actions.is_empty());
    }

    /// All-bot seats: advance() runs the whole game with no human stop.
    #[test]
    fn all_bots_runs_to_completion() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 3);
        let seats = vec![
            Seat::Bot(Box::new(RandomStatePolicy::new(1))),
            Seat::Bot(Box::new(RandomStatePolicy::new(2))),
        ];
        let mut session = Session::new(vec![deck.clone(), deck], &reg, seats, 7);
        match session.advance() {
            Turn::GameOver(_) => {}
            Turn::AwaitingHuman { .. } => panic!("no human seats — should not await"),
        }
    }
}
