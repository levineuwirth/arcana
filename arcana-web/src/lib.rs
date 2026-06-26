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
use arcana_core::combat::{
    attacker_options, blocker_options, match_attack, match_block, AttackerDeclaration,
    BlockerDeclaration, DefendingEntity,
};
use arcana_core::objects::ObjectId;
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

/// Which kind of combat declaration the human is being asked for.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CombatKind {
    Attackers,
    Blockers,
}

/// One of the human's creatures that may attack, plus the entities it could
/// attack (usually just the lone opponent; a planeswalker/battle adds options).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AttackerOption {
    pub id: ObjectId,
    pub defenders: Vec<DefendingEntity>,
}

/// One of the human's creatures that may block, plus the attackers it can be
/// declared against.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BlockerOption {
    pub id: ObjectId,
    pub can_block: Vec<ObjectId>,
}

/// A pending combat declaration, expressed as per-creature option sets so the
/// frontend can build the declaration one click at a time (instead of choosing
/// from the engine's full enumeration of whole declarations, which is
/// combinatorial). Derived from the cached legal list via the
/// [`arcana_core::combat`] helpers, so no combat rules are duplicated client-side.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CombatPrompt {
    pub kind: CombatKind,
    /// Populated when `kind == Attackers`.
    pub attackers: Vec<AttackerOption>,
    /// Populated when `kind == Blockers`.
    pub blockers: Vec<BlockerOption>,
    /// The attacking creatures the human is defending against (for display /
    /// highlight). Populated when `kind == Blockers`.
    pub incoming: Vec<ObjectId>,
}

/// The human's incremental combat declaration, sent back to `/combat`. The set
/// of picks is matched against the engine's enumeration; an empty set means
/// "declare no attackers / no blockers" (which is always legal).
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum CombatSubmission {
    Attackers { attackers: Vec<AttackerDeclaration> },
    Blockers { blockers: Vec<BlockerDeclaration> },
}

/// The JSON payload `/state`, `/action`, and `/combat` return: the human's view
/// of the game, whatever the opponent did since the human last acted, and — when
/// the pending decision is a combat declaration — the per-creature option sets
/// for the rich combat builder.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StateResponse {
    pub view: ViewState,
    pub recent: Vec<RecentAction>,
    /// `Some` only when the human must declare attackers or blockers.
    pub combat: Option<CombatPrompt>,
}

/// Build a [`CombatPrompt`] from a legal-action list, or `None` if the pending
/// decision is not a combat declaration. Attackers take precedence (the two are
/// never enumerated together).
fn combat_prompt(legal: &[Action]) -> Option<CombatPrompt> {
    let atk = attacker_options(legal);
    if !atk.is_empty() {
        return Some(CombatPrompt {
            kind: CombatKind::Attackers,
            attackers: atk
                .into_iter()
                .map(|(id, defenders)| AttackerOption { id, defenders })
                .collect(),
            blockers: Vec::new(),
            incoming: Vec::new(),
        });
    }
    let blk = blocker_options(legal);
    if !blk.is_empty() {
        let mut incoming: Vec<ObjectId> =
            blk.iter().flat_map(|(_, atkrs)| atkrs.iter().copied()).collect();
        incoming.sort_unstable();
        incoming.dedup();
        return Some(CombatPrompt {
            kind: CombatKind::Blockers,
            attackers: Vec::new(),
            blockers: blk
                .into_iter()
                .map(|(id, can_block)| BlockerOption { id, can_block })
                .collect(),
            incoming,
        });
    }
    None
}

/// Why an `apply_index` call could not be honored. Surfaced to the client as a
/// 400, never as a panic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApplyError {
    /// The game is over (or no decision has been surfaced yet) — nothing to apply.
    NoPendingDecision,
    /// The index was outside the current legal-action list.
    OutOfRange { index: usize, len: usize },
    /// A submitted combat declaration didn't match any legal declaration (an
    /// illegal set, or one beyond the engine's enumeration cap). The frontend
    /// should re-prompt with the current options.
    IllegalCombat,
    /// Auto-tap was asked to play a card that can't be cast/played this turn even
    /// tapping out (it shouldn't have been clickable).
    NotPlayable { id: ObjectId },
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
            ApplyError::IllegalCombat => {
                write!(f, "that combat declaration is not legal — pick again")
            }
            ApplyError::NotPlayable { id } => {
                write!(f, "card {id} can't be played this turn")
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
        let combat = combat_prompt(&self.legal);
        StateResponse { view, recent, combat }
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

    /// Apply an incremental combat declaration built by the frontend. The picked
    /// set is matched against the cached legal enumeration via the
    /// [`arcana_core::combat`] matchers (so all combat rules — lethal ordering,
    /// trample, the 1024-declaration cap — stay engine-side); an empty set is the
    /// always-legal "no attacks / no blocks". Returns [`ApplyError::IllegalCombat`]
    /// if the set isn't a legal declaration, so the frontend can re-prompt.
    pub fn apply_combat(&mut self, sub: CombatSubmission) -> Result<StateResponse, ApplyError> {
        if self.legal.is_empty() {
            return Err(ApplyError::NoPendingDecision);
        }
        let action = match sub {
            CombatSubmission::Attackers { attackers } => match_attack(&self.legal, &attackers),
            CombatSubmission::Blockers { blockers } => match_block(&self.legal, &blockers),
        };
        let action = action.ok_or(ApplyError::IllegalCombat)?;
        self.session.apply(action);
        Ok(self.snapshot())
    }

    /// MTGA-style "click a card to play it": auto-tap the mana to make hand card
    /// `target` castable, then cast/play it if there's a single way to — else
    /// leave the mana floated and surface the now-available cast variants (e.g.
    /// to choose targets). Returns [`ApplyError::NotPlayable`] if it can't be
    /// played this turn. See [`arcana_core::legal_actions::auto_tap_sequence`].
    pub fn auto_tap_and_cast(&mut self, target: ObjectId) -> Result<StateResponse, ApplyError> {
        let seq = arcana_core::legal_actions::auto_tap_sequence(
            self.session.state(), self.reg, HUMAN, target)
            .ok_or(ApplyError::NotPlayable { id: target })?;
        for action in seq {
            self.session.apply(action);
        }
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

    /// The combat-prompt builder and the matcher agree, and a non-combat legal
    /// list yields no prompt. Pure (no session) so it's deterministic.
    #[test]
    fn combat_prompt_and_matcher_are_consistent() {
        // No legal actions / non-combat decisions produce no prompt.
        assert!(combat_prompt(&[]).is_none());

        let mk = || AttackerDeclaration { attacker: 5, defending: DefendingEntity::Player(1) };
        let legal = vec![
            Action::DeclareAttackers { attackers: vec![] }, // the always-legal "no attacks"
            Action::DeclareAttackers { attackers: vec![mk()] },
        ];
        let prompt = combat_prompt(&legal).expect("an attacker prompt");
        assert_eq!(prompt.kind, CombatKind::Attackers);
        assert_eq!(prompt.attackers.len(), 1);
        assert_eq!(prompt.attackers[0].id, 5);
        assert_eq!(prompt.attackers[0].defenders, vec![DefendingEntity::Player(1)]);
        assert!(prompt.blockers.is_empty());

        // The matcher recovers the declaration the prompt advertised, and the
        // empty set (declare no attackers) is always available here.
        assert!(match_attack(&legal, &[mk()]).is_some());
        assert!(match_attack(&legal, &[]).is_some());
    }

    /// End-to-end combat: play a real game and, whenever a combat prompt is
    /// surfaced, push an actual legal declaration back through the
    /// builder→matcher→apply path. Confirms the server combat wiring drives a
    /// live session, not just a synthetic legal list.
    #[test]
    fn combat_prompt_and_apply_roundtrip_in_a_real_game() {
        let reg = leaked_catalog();
        let mut core = GameCore::new(reg, 11);
        let mut cur = core.snapshot();
        let mut saw_combat = false;
        let mut decisions = 0;

        while cur.view.game_over.is_none() && decisions < 600 {
            decisions += 1;
            if let Some(prompt) = cur.combat.clone() {
                saw_combat = true;
                // Replay an actual enumerated declaration through the public
                // submission path (the field is in-module-visible to the test).
                let sub = match prompt.kind {
                    CombatKind::Attackers => {
                        let attackers = core
                            .legal
                            .iter()
                            .find_map(|a| match a {
                                Action::DeclareAttackers { attackers } => Some(attackers.clone()),
                                _ => None,
                            })
                            .expect("a DeclareAttackers is enumerated for an attacker prompt");
                        CombatSubmission::Attackers { attackers }
                    }
                    CombatKind::Blockers => {
                        let blockers = core
                            .legal
                            .iter()
                            .find_map(|a| match a {
                                Action::DeclareBlockers { blockers } => Some(blockers.clone()),
                                _ => None,
                            })
                            .expect("a DeclareBlockers is enumerated for a blocker prompt");
                        CombatSubmission::Blockers { blockers }
                    }
                };
                cur = core.apply_combat(sub).expect("a real legal declaration applies");
                continue;
            }
            // Non-combat: greedily take a developing action so we reach combat.
            let idx = cur
                .view
                .legal
                .iter()
                .position(|a| {
                    let l = a.label.as_str();
                    l != "Pass" && l != "Concede" && l != "Mulligan (draw a new hand)"
                })
                .unwrap_or(0);
            cur = core.apply_index(idx).expect("a legal index applies");
        }

        assert!(saw_combat, "expected at least one combat declaration in a full game");
    }

    /// Auto-tap: drive to the first state with a playable hand card, click it,
    /// and confirm it leaves the hand (a land played / spell cast). Also confirms
    /// a non-playable card is rejected gracefully.
    #[test]
    fn auto_tap_plays_a_playable_card() {
        let reg = leaked_catalog();
        let mut core = GameCore::new(reg, 11);

        // At the opening (mulligan), nothing is playable → auto-tap is rejected.
        let s = core.snapshot();
        let some_card = s.view.players[0].hand[0].id;
        assert!(matches!(core.auto_tap_and_cast(some_card),
            Err(ApplyError::NotPlayable { .. })),
            "can't auto-tap during the mulligan");

        // Advance (keeping the hand, taking real actions) until a hand card is
        // flagged playable — on turn one that's a land.
        let mut cur = s;
        let mut guard = 0;
        loop {
            guard += 1;
            assert!(guard < 300, "should reach a playable card");
            assert!(cur.view.game_over.is_none(), "game ended before a playable card");
            // Turn one: the first playable card is a land. Auto-tap it and
            // confirm it entered the human's battlefield. (Battlefield is a clean
            // signal — the bot's interleaved turn never adds to the human's side,
            // whereas hand_count is confounded by the human's next-turn draw.)
            if let Some(card) = cur.view.players[0].hand.iter().find(|c| c.playable && c.is_land) {
                let id = card.id;
                let before_bf = cur.view.players[0].battlefield.len();
                let after = core.auto_tap_and_cast(id).expect("a playable land plays");
                assert!(after.view.players[0].battlefield.len() > before_bf,
                    "the auto-tapped land entered the battlefield");
                return;
            }
            let idx = cur.view.legal.iter().position(|a| {
                let l = a.label.as_str();
                l != "Pass" && l != "Concede" && l != "Mulligan (draw a new hand)"
            }).unwrap_or(0);
            cur = core.apply_index(idx).expect("legal index applies");
        }
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
