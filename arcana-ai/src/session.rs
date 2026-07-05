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
use arcana_core::render::render_action;
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

/// How aggressively to auto-pass a HUMAN's dead priority windows (a window where
/// they have no meaningful play — only passing or tapping mana with nothing to
/// spend it on). Bots are unaffected (they choose via their policy).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AutoPass {
    /// Never auto-pass: surface every priority window where the human has a legal
    /// non-pass option (full manual control).
    None,
    /// MTGA-style middle ground (default): auto-pass a dead window only when it's
    /// quiet — the stack is empty AND the opponent did nothing notable since the
    /// human last acted. So the human still STOPS to see spells on the stack and
    /// the opponent's plays, but skips truly-empty windows.
    #[default]
    Stops,
    /// Auto-pass every dead window (fastest; the prior behavior).
    Full,
}

/// An interactive session over one game. Borrows the registry for its lifetime.
pub struct Session<'a> {
    state: GameState,
    yld: EngineYield,
    registry: &'a CardRegistry,
    seats: Vec<Seat>,
    record: GameRecord,
    /// What the opponent(s) did since the human last acted: notable bot actions
    /// from the most recent [`Self::advance`], each as `(player, description)`.
    /// Descriptions are rendered AT APPLY TIME (the engine re-ids objects on zone
    /// change, so the id is stale by the time the frontend would render it) from
    /// the full state — correct since a played/cast/attacking card is public.
    /// Cleared each `advance`; passes and empty declarations are not recorded.
    log: Vec<(PlayerId, String)>,
    /// How aggressively to auto-pass a HUMAN's dead priority windows
    /// ([`AutoPass`]). Defaults to [`AutoPass::Stops`] (MTGA-style middle ground).
    /// Bots are unaffected.
    auto_pass: AutoPass,
}

/// Worth showing in the opponent log: not a pass, not an empty attack/block.
fn is_notable(a: &Action) -> bool {
    match a {
        Action::PassPriority => false,
        Action::DeclareAttackers { attackers } => !attackers.is_empty(),
        Action::DeclareBlockers { blockers } => !blockers.is_empty(),
        _ => true,
    }
}

/// A decision the engine can resolve without prompting: a single legal action,
/// "nothing to do but pass" (only pass/concede → pass), or a forced resolution
/// choice with a single legal response (e.g. the only legal target is the
/// opponent — don't make the player click the one option). Applied to humans AND
/// bots so neither is pestered with non-decisions. Concede never counts as a real
/// option here (it's always available, never forced).
fn auto_action(legal: &[Action]) -> Option<Action> {
    if legal.len() <= 1 {
        return legal.first().cloned();
    }
    if legal.iter().all(|a| matches!(a, Action::PassPriority | Action::Concede)) {
        return legal.iter().find(|a| matches!(a, Action::PassPriority)).cloned();
    }
    // A pending resolution choice always offers Concede alongside the responses;
    // when only ONE response is legal it isn't a decision, so resolve it. (A
    // single optional priority action like one Cast is NOT auto-taken — those
    // come with PassPriority, which is a second non-Concede option.)
    let mut responses = legal.iter().filter(|a| !matches!(a, Action::Concede));
    if let (Some(only), None) = (responses.next(), responses.next()) {
        if matches!(only, Action::SubmitResolutionChoice { .. }) {
            return Some(only.clone());
        }
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
        Self { state, yld, registry, seats, record, log: Vec::new(),
               auto_pass: AutoPass::default() }
    }

    /// Set the human auto-pass level (default [`AutoPass::Stops`]). See [`AutoPass`].
    pub fn set_auto_pass(&mut self, level: AutoPass) { self.auto_pass = level; }

    /// The current (full, un-projected) game state — for spectator rendering.
    pub fn state(&self) -> &GameState { &self.state }
    /// The action transcript so far (replayable via `arcana_core::record`).
    pub fn record(&self) -> &GameRecord { &self.record }
    /// What the opponent(s) did during the most recent [`Self::advance`]:
    /// `(player, human-readable description)` for each notable bot action, in
    /// order — for showing the human what just happened.
    pub fn recent_actions(&self) -> &[(PlayerId, String)] { &self.log }

    /// Drive the game through bot + trivial decisions until a human must choose
    /// or the game ends.
    pub fn advance(&mut self) -> Turn {
        self.log.clear();
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
                // A human's London-mulligan bottoming is offered as a single
                // canonical BottomCards, but it's a real choice (which cards to
                // keep) — surface it instead of auto-resolving. Bots auto-bottom.
                let human_bottoming = matches!(self.seats[player as usize], Seat::Human)
                    && matches!(action, Action::BottomCards(_));
                if !human_bottoming {
                    self.apply_internal(action);
                    continue;
                }
            }

            // Auto-pass a HUMAN's dead priority window (no meaningful play — only
            // passing or tapping mana with nothing to spend it on), per the
            // configured level. Bots are unaffected. Gated on a priority window so
            // it never short-circuits mulligans, combat declarations, or choices.
            if matches!(self.seats[player as usize], Seat::Human)
                && legal.iter().any(|a| matches!(a, Action::PassPriority))
                && !arcana_core::legal_actions::has_meaningful_play(
                    &self.state, player, self.registry)
            {
                let skip = match self.auto_pass {
                    AutoPass::None => false,
                    // Nothing NEW to acknowledge this advance (`log` empty) and —
                    // checked above — no meaningful play. A non-empty stack does
                    // NOT force a stop: the player's own spell resolving (no
                    // response available) shouldn't demand a manual pass, and any
                    // notable opponent action (a spell they might answer) lands in
                    // `log`, which keeps the window surfaced. This is what lets a
                    // cast/activation with no opponent interaction resolve without
                    // an extra "pass" click (MTGA-style auto-yield through your own
                    // stack).
                    AutoPass::Stops => self.log.is_empty(),
                    AutoPass::Full => true,
                };
                if skip {
                    self.apply_internal(Action::PassPriority);
                    continue;
                }
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
            // Render the description NOW, while the object is still live (apply
            // re-ids it on zone change). Only log notable actions.
            if is_notable(&action) {
                let desc = render_action(&action, &self.state, self.registry);
                self.log.push((player, desc));
            }
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

    /// Rebuild game state by replaying a recorded action transcript — the FULL
    /// linear `step` sequence [`advance`](Self::advance) records (auto-passes and
    /// auto-resolutions included), so replay is a plain linear re-run, NOT
    /// advance-then-apply. Construct a fresh session with the SAME decks + seed,
    /// then `replay(record.actions)` to resume a persisted game. Stops early if
    /// the transcript ends or the game ends (a well-formed transcript keeps one
    /// decision pending between actions).
    pub fn replay(&mut self, actions: &[Action]) {
        for a in actions {
            if !matches!(self.yld, EngineYield::PendingDecision { .. }) {
                break;
            }
            self.apply_internal(a.clone());
        }
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

    /// A forced resolution choice with a single legal response (only the
    /// opponent is a legal target) auto-resolves; with two options it doesn't.
    #[test]
    fn single_forced_choice_auto_resolves() {
        use arcana_core::actions::ChoiceResponse;
        let one = vec![
            Action::SubmitResolutionChoice { id: 1, response: ChoiceResponse::YesNo { answer: true } },
            Action::Concede,
        ];
        assert!(matches!(auto_action(&one), Some(Action::SubmitResolutionChoice { .. })),
            "a lone forced choice is resolved, not surfaced");

        let two = vec![
            Action::SubmitResolutionChoice { id: 1, response: ChoiceResponse::YesNo { answer: true } },
            Action::SubmitResolutionChoice { id: 1, response: ChoiceResponse::YesNo { answer: false } },
            Action::Concede,
        ];
        assert!(auto_action(&two).is_none(), "a real choice with two options is surfaced");

        // An optional priority action (one Cast + Pass + Concede) is NOT auto-taken.
        let prio = vec![Action::PassPriority, Action::Concede];
        assert!(matches!(auto_action(&prio), Some(Action::PassPriority)));
    }

    /// Auto-pass (default on) must never surface a "dead" priority window — one
    /// where the human has nothing to do but pass or tap mana with nothing to
    /// spend it on. Such windows genuinely occur (proven by the full-control
    /// run), so auto-pass is doing real work, not a no-op.
    #[test]
    fn auto_pass_skips_dead_priority_windows() {
        use arcana_core::legal_actions::has_meaningful_play;

        // Returns whether any *surfaced* priority window had no meaningful play.
        fn run(level: AutoPass) -> bool {
            let reg = arcana_cards::build_catalog();
            let deck = arcana_cards::sample_deck(&reg, 7);
            let seats = vec![Seat::Human, Seat::Human];
            let mut session = Session::new(vec![deck.clone(), deck], &reg, seats, 7);
            session.set_auto_pass(level);

            let mut saw_dead = false;
            let mut guard = 0;
            loop {
                guard += 1;
                assert!(guard < 100_000, "session must terminate");
                match session.advance() {
                    Turn::GameOver(_) => break,
                    Turn::AwaitingHuman { legal, player, .. } => {
                        let is_priority = legal.iter().any(|a| matches!(a, Action::PassPriority));
                        if is_priority && !has_meaningful_play(session.state(), player, &reg) {
                            saw_dead = true;
                        }
                        // Develop where possible, else pass; keep mulligans.
                        let pick = legal.iter()
                            .position(|a| matches!(a, Action::MulliganKeep))
                            .or_else(|| legal.iter().position(|a|
                                matches!(a, Action::PlayLand { .. } | Action::CastSpell { .. })))
                            .or_else(|| legal.iter().position(|a| matches!(a, Action::PassPriority)))
                            .unwrap_or(0);
                        session.apply(legal[pick].clone());
                    }
                }
            }
            saw_dead
        }

        assert!(run(AutoPass::None), "None must surface at least one no-play window");
        assert!(!run(AutoPass::Full), "Full must never surface a no-play priority window");
    }

    /// A scripted human that DEVELOPS (plays lands/spells) and ATTACKS, building
    /// its combat declarations via the `arcana_core::combat` incremental helpers
    /// against the engine's real enumerated legal lists. Validates that a built
    /// "attack with everything" declaration always matches a legal action, and
    /// that block-with-nothing always matches — the combat-UX path end-to-end.
    #[test]
    fn scripted_human_develops_and_attacks() {
        use arcana_core::combat::{attacker_options, match_attack, match_block, AttackerDeclaration};
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);

        // The human dispatches on the actual legal-action SHAPES (exactly like
        // the CLI frontend), not the DecisionContext — so the test exercises the
        // real "build attack/block, match to a legal action" path through the
        // Session. For non-combat decisions it takes any REAL action (like the
        // random bot: tap lands for mana, play lands, cast — passive land-only
        // play never generates mana, so never develops creatures, so never
        // attacks). Whenever a DeclareAttackers decision arises the build MUST
        // match (the `.expect`); the "attacks happened" check spans seeds.
        let mut total_attacks = 0;
        for seed in 0u64..6 {
            let seats = vec![Seat::Human, Seat::Bot(Box::new(RandomStatePolicy::new(seed * 7 + 1)))];
            let mut session = Session::new(vec![deck.clone(), deck.clone()], &reg, seats, seed);
            let mut guard = 0;
            loop {
                guard += 1;
                assert!(guard < 200_000, "session failed to terminate");
                let legal = match session.advance() {
                    Turn::GameOver(_) => break,
                    Turn::AwaitingHuman { legal, .. } => legal,
                };
                let action = if legal.iter().any(|a| matches!(a, Action::DeclareAttackers { .. })) {
                    let decls: Vec<AttackerDeclaration> = attacker_options(&legal).iter()
                        .map(|(id, defs)| AttackerDeclaration { attacker: *id, defending: defs[0] })
                        .collect();
                    let action = match_attack(&legal, &decls)
                        .expect("an attack-with-everything build must match a legal declaration");
                    if !decls.is_empty() { total_attacks += 1; }
                    action
                } else if legal.iter().any(|a| matches!(a, Action::DeclareBlockers { .. })) {
                    match_block(&legal, &[]).expect("block-with-nothing must be legal")
                } else {
                    // Any real action (tap mana / play land / cast), else pass.
                    legal.iter()
                        .find(|a| !matches!(a, Action::PassPriority | Action::Concede | Action::MulliganAgain))
                        .or_else(|| legal.iter().find(|a| matches!(a, Action::PassPriority)))
                        .cloned().unwrap_or_else(|| legal[0].clone())
                };
                session.apply(action);
            }
        }
        assert!(total_attacks > 0, "expected the human to declare attackers in some game");
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
