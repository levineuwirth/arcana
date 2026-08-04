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
use arcana_core::engine::{new_game_first_player, step, EngineYield};
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

/// How a HUMAN's priority windows are surfaced. Bots are unaffected (they choose
/// via their policy). A "dead" window is one where the human has no meaningful
/// play — only passing or tapping mana with nothing to spend it on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AutoPass {
    /// Smart default: auto-pass a dead window; stop ONLY when the human actually
    /// holds a decision — a castable spell or non-mana ability, including
    /// instant-speed responses on the opponent's turn ([`has_meaningful_play`]).
    /// What the opponent did flows into the passive action feed
    /// ([`Session::recent_actions`]) without halting the human to acknowledge
    /// something they cannot act on. This is the fix for the "why did it stop me
    /// here?" false stop.
    ///
    /// [`has_meaningful_play`]: arcana_core::legal_actions::has_meaningful_play
    #[default]
    Default,
    /// Full manual control: stop at EVERY priority window that offers any legal
    /// non-pass option (including mana-only windows) — for bluffing, holding up
    /// mana, or acting at an unusual window. Toggled by the full-control key.
    FullControl,
}

/// Overhaul step 2 — the "pass until \<phase\>" one-shot skip directive
/// (MTGO/Forge model). While armed for a seat, that human's priority windows
/// are auto-passed EVEN when they hold a meaningful play — that's the point:
/// skip your own dead-but-castable main phase — until the checkpoint arrives
/// or a safety interrupt takes over:
///
/// * **Checkpoint reached** → the window SURFACES (even a dead one — you asked
///   to be stopped there, e.g. to act at end of turn), and the directive
///   disarms.
/// * **Non-empty stack** → the skip pauses and the normal [`AutoPass`] rules
///   decide (a window with a real response surfaces; a dead one auto-passes
///   and the skip resumes once the stack clears). Surfacing disarms.
/// * **Your own declare-attackers while skipping past combat** → declares no
///   attackers automatically when legal; a must-attack creature makes that
///   illegal, which disarms and surfaces the declaration.
/// * **Any other non-priority decision** (declare blockers, a resolution
///   choice, …) → disarms and surfaces. Never skips a real decision.
///
/// Whatever the exit, ANY surfaced decision disarms — the directive is
/// one-shot, never a standing mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PassUntil {
    /// Stop at this turn's combat phase (the beginning-of-combat window).
    Combat,
    /// Stop at this turn's end step.
    EndOfTurn,
    /// Stop at this seat's own next turn, skipping the rest of this one and
    /// the opponent's turn.
    MyNextTurn,
}

/// A [`PassUntil`] armed for a seat, remembering the turn it was issued on so
/// the checkpoint test can't wrap around ("this turn's combat" armed after
/// combat degrades to "start of next turn", never a whole extra cycle).
#[derive(Clone, Copy, Debug)]
struct ArmedPassUntil {
    target: PassUntil,
    armed_turn: u32,
}

impl ArmedPassUntil {
    /// Has the skip reached its stop point?
    fn reached(&self, turn: &arcana_core::turn::TurnState, seat: PlayerId) -> bool {
        match self.target {
            // Past the armed turn counts as reached: the backstop that keeps
            // a late-armed directive from skipping into the NEXT cycle.
            PassUntil::Combat =>
                turn.turn_number > self.armed_turn || turn.phase.is_combat(),
            PassUntil::EndOfTurn =>
                turn.turn_number > self.armed_turn || turn.phase.is_ending(),
            PassUntil::MyNextTurn =>
                turn.active_player == seat && turn.turn_number > self.armed_turn,
        }
    }
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
    /// How a HUMAN's priority windows are surfaced ([`AutoPass`]). Defaults to
    /// [`AutoPass::Default`] (the smart middle ground). Bots are unaffected.
    auto_pass: AutoPass,
    /// Per-seat armed "pass until \<phase\>" directive ([`PassUntil`]); `None`
    /// when idle. One-shot: consumed by reaching its checkpoint or by ANY
    /// decision surfacing to that seat.
    pass_until: Vec<Option<ArmedPassUntil>>,
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
        Self::new_first_player(decks, registry, seats, seed, 0)
    }

    /// Like [`Session::new`] but `first` (a seat index) takes turn 1 — the
    /// play/draw choice. `first == 0` is exactly [`Session::new`].
    pub fn new_first_player(
        decks: Vec<Vec<CardId>>,
        registry: &'a CardRegistry,
        seats: Vec<Seat>,
        seed: u64,
        first: PlayerId,
    ) -> Self {
        assert_eq!(decks.len(), seats.len(), "one seat per deck");
        let record = GameRecord::new(decks.clone(), seed);
        let (state, yld) = new_game_first_player(decks, registry, seed, first);
        let pass_until = vec![None; seats.len()];
        Self { state, yld, registry, seats, record, log: Vec::new(),
               auto_pass: AutoPass::default(), pass_until }
    }

    /// Set how the human's priority windows surface (default
    /// [`AutoPass::Default`]). See [`AutoPass`].
    pub fn set_auto_pass(&mut self, level: AutoPass) { self.auto_pass = level; }

    /// Arm (or clear, with `None`) a one-shot [`PassUntil`] skip for `seat`.
    /// Call [`advance`](Self::advance) afterward to let it run; in a
    /// multi-human game the directive persists on the session and consumes
    /// `seat`'s windows as opponents drive the game forward.
    pub fn set_pass_until(&mut self, seat: PlayerId, target: Option<PassUntil>) {
        if let Some(slot) = self.pass_until.get_mut(seat as usize) {
            *slot = target.map(|t| ArmedPassUntil {
                target: t,
                armed_turn: self.state.turn.turn_number,
            });
        }
    }

    /// The armed [`PassUntil`] for `seat`, if any — lets a UI show "skipping
    /// to …" and offer a cancel.
    pub fn pass_until(&self, seat: PlayerId) -> Option<PassUntil> {
        self.pass_until.get(seat as usize).copied().flatten().map(|a| a.target)
    }

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

            // Overhaul step 2: an armed "pass until <phase>" skip for this
            // seat. Runs BEFORE the dead-window logic because it passes even
            // windows that hold a meaningful play — that's what "skip my main
            // phase" means. See [`PassUntil`] for the interrupt contract.
            let mut at_checkpoint = false;
            if matches!(self.seats[player as usize], Seat::Human) {
                if let Some(armed) = self.pass_until[player as usize] {
                    if armed.reached(&self.state.turn, player) {
                        // Surface this window even if it's dead — the player
                        // asked to be stopped here. Disarm; fall through.
                        self.pass_until[player as usize] = None;
                        at_checkpoint = true;
                    } else if !self.state.stack.is_empty() {
                        // Something is on the stack: let the normal AutoPass
                        // rules decide this window (a real response surfaces —
                        // and surfacing disarms below). The skip resumes if
                        // the window was dead and the stack clears.
                    } else if matches!(context, DecisionContext::DeclareAttackers) {
                        // Skipping past our own combat: declare no attackers
                        // when that's legal; a must-attack creature makes it
                        // illegal → disarm and surface the declaration.
                        let none = legal.iter().find(|a| matches!(
                            a, Action::DeclareAttackers { attackers } if attackers.is_empty()
                        )).cloned();
                        match none {
                            Some(a) => {
                                self.apply_internal(a);
                                continue;
                            }
                            None => self.pass_until[player as usize] = None,
                        }
                    } else if legal.iter().any(|a| matches!(a, Action::PassPriority)) {
                        self.apply_internal(Action::PassPriority);
                        continue;
                    } else {
                        // A decision the skip can't answer (blockers, a
                        // choice, a cast sub-step): disarm and surface.
                        self.pass_until[player as usize] = None;
                    }
                }
            }

            // Auto-pass a HUMAN's dead priority window (no meaningful play — only
            // passing or tapping mana with nothing to spend it on), per the
            // configured level. Bots are unaffected. Gated on a priority window so
            // it never short-circuits mulligans, combat declarations, or choices.
            // A just-reached PassUntil checkpoint window always surfaces.
            if !at_checkpoint
                && matches!(self.seats[player as usize], Seat::Human)
                && legal.iter().any(|a| matches!(a, Action::PassPriority))
                && !arcana_core::legal_actions::has_meaningful_play(
                    &self.state, player, self.registry)
            {
                // We only get here with NO meaningful play (checked above), so a
                // legal instant/ability response — which would make this a real
                // decision — has already surfaced the window via
                // `has_meaningful_play`. The default therefore auto-passes every
                // remaining (dead) window; what the opponent did is still recorded
                // in `log` for the passive feed, it just no longer HALTS the human
                // to acknowledge something they can't act on. Full control surfaces
                // every window regardless.
                let skip = match self.auto_pass {
                    AutoPass::Default => true,
                    AutoPass::FullControl => false,
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
                    // Any surfaced decision consumes the seat's PassUntil —
                    // it's a one-shot directive, never a standing mode.
                    self.pass_until[player as usize] = None;
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

    /// `new_first_player(first)` puts that seat on the play: it takes the opening
    /// mulligan decision (turn 1), and `first == 0` is the default.
    #[test]
    fn first_player_choice_starts_the_chosen_seat() {
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        for first in [0u8, 1] {
            let seats = vec![Seat::Human, Seat::Human];
            let mut s = Session::new_first_player(
                vec![deck.clone(), deck.clone()], &reg, seats, 3, first);
            // The chosen seat owns turn 1 (the definitive "on the play" signal —
            // holds even when a bot's mulligan auto-resolves in the solo path).
            assert_eq!(s.state().turn.active_player, first,
                "seat {first} chosen → owns turn 1");
            // …and its mulligan is the first decision surfaced.
            match s.advance() {
                Turn::AwaitingHuman { player, .. } =>
                    assert_eq!(player, first, "seat {first} on the play acts first"),
                Turn::GameOver(_) => panic!("game shouldn't be over at the opening mulligan"),
            }
        }
    }

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

        assert!(run(AutoPass::FullControl),
            "full control must surface at least one no-play window");
        assert!(!run(AutoPass::Default),
            "the default must never surface a no-play priority window");
    }

    /// Overhaul step 2: "pass until <phase>" skips windows the player COULD
    /// act in (that's the point — the human below holds castable Bolts the
    /// whole way), stops at the first window at-or-after the checkpoint, and
    /// disarms once consumed. With an instant in hand the engine yields the
    /// combat/end-step windows, so the stops land EXACTLY on the checkpoint;
    /// with nothing to do there the engine never yields such a window and the
    /// stop degrades to the next real one (consistent with the no-false-stop
    /// model — never present a decision-less halt).
    #[test]
    fn pass_until_skips_to_the_checkpoint_and_disarms() {
        use arcana_core::turn::{Phase, Step};
        let reg = arcana_cards::build_catalog();
        let find = |n: &str| reg.iter()
            .find(|(_, d)| reg.interner().resolve(d.name) == Some(n))
            .map(|(id, _)| id)
            .unwrap_or_else(|| panic!("{n} in catalog"));
        let mountain = find("Mountain");
        let bolt = find("Lightning Bolt");
        // Human: Bolts to hold up (meaningful play at every window, so the
        // engine yields the checkpoint windows). Bot: all lands (can't win,
        // so the skip-my-next-turn run can't be cut short by a loss).
        let human_deck: Vec<CardId> = std::iter::repeat(mountain).take(24)
            .chain(std::iter::repeat(bolt).take(36)).collect();
        let bot_deck: Vec<CardId> = vec![mountain; 60];

        // Drive to the human's turn-1 main-phase window AFTER a land drop
        // (so a Bolt is genuinely castable from then on), then arm + advance.
        let run = |target: PassUntil| {
            let seats = vec![Seat::Human, Seat::Bot(Box::new(
                crate::search::RandomStatePolicy::new(11)))];
            let mut session = Session::new(
                vec![human_deck.clone(), bot_deck.clone()], &reg, seats, 7);
            let mut guard = 0;
            loop {
                guard += 1;
                assert!(guard < 10_000, "must reach the post-land main window");
                match session.advance() {
                    Turn::GameOver(r) => panic!("game ended during setup: {r:?}"),
                    Turn::AwaitingHuman { legal, .. } => {
                        if let Some(i) = legal.iter()
                            .position(|a| matches!(a, Action::MulliganKeep)) {
                            session.apply(legal[i].clone());
                            continue;
                        }
                        if let Some(i) = legal.iter()
                            .position(|a| matches!(a, Action::PlayLand { .. })) {
                            session.apply(legal[i].clone());
                            continue;
                        }
                        let t = &session.state().turn;
                        assert!(t.active_player == 0 && t.phase.is_pre_combat_main(),
                            "setup should still be in our first main");
                        break;
                    }
                }
            }
            let armed_turn = session.state().turn.turn_number;
            session.set_pass_until(0, Some(target));
            match session.advance() {
                Turn::GameOver(r) => panic!("mono-land bot can't end the game: {r:?}"),
                Turn::AwaitingHuman { player, .. } => {
                    assert_eq!(player, 0, "the bot never surfaces");
                    assert_eq!(session.pass_until(0), None,
                        "surfacing a decision must disarm the directive");
                    (session.state().turn.clone(), armed_turn)
                }
            }
        };

        // Castable Bolt in hand → the engine yields the exact checkpoint
        // windows, so each stop lands precisely.
        let (t, armed) = run(PassUntil::Combat);
        assert_eq!((t.turn_number, t.phase), (armed, Phase::Combat),
            "stopped at this turn's combat");
        let (t, armed) = run(PassUntil::EndOfTurn);
        assert_eq!((t.turn_number, t.phase, t.step), (armed, Phase::Ending, Step::End),
            "stopped at this turn's end step");
        let (t, armed) = run(PassUntil::MyNextTurn);
        assert!(t.active_player == 0 && t.turn_number > armed,
            "stopped on our own later turn (got t{} ap={})", t.turn_number, t.active_player);
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

    /// Repro for "can't declare attackers/blockers under Full control": Full
    /// control surfaces EXTRA (dead) priority windows, but combat declarations
    /// are not priority windows, so they must still surface. If the human can
    /// attack under the default they must be able to under full control too.
    #[test]
    fn full_control_still_surfaces_combat_declarations() {
        // Full control surfaces EXTRA (dead) priority windows, but combat
        // declarations are not priority windows, so they must still surface.
        // Uses the SAME "take any real action" script as the default combat test.
        use arcana_core::combat::{attacker_options, match_attack, match_block, AttackerDeclaration};
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        let mut total_attacks = 0;
        for seed in 0u64..6 {
            let seats = vec![Seat::Human, Seat::Bot(Box::new(RandomStatePolicy::new(seed*7+1)))];
            let mut session = Session::new(vec![deck.clone(), deck.clone()], &reg, seats, seed);
            session.set_auto_pass(AutoPass::FullControl);
            let mut guard = 0;
            loop {
                guard += 1; assert!(guard < 400_000, "session must terminate");
                let legal = match session.advance() {
                    Turn::GameOver(_) => break,
                    Turn::AwaitingHuman { legal, .. } => legal,
                };
                let action = if legal.iter().any(|a| matches!(a, Action::DeclareAttackers{..})) {
                    let decls: Vec<AttackerDeclaration> = attacker_options(&legal).iter()
                        .map(|(id,defs)| AttackerDeclaration{attacker:*id,defending:defs[0]}).collect();
                    if !decls.is_empty() { total_attacks += 1; }
                    match_attack(&legal,&decls).expect("attack build must match a legal action")
                } else if legal.iter().any(|a| matches!(a, Action::DeclareBlockers{..})) {
                    match_block(&legal,&[]).expect("block-with-nothing must be legal")
                } else {
                    legal.iter()
                        .find(|a| !matches!(a, Action::PassPriority | Action::Concede | Action::MulliganAgain))
                        .or_else(|| legal.iter().find(|a| matches!(a, Action::PassPriority)))
                        .cloned().unwrap_or_else(|| legal[0].clone())
                };
                session.apply(action);
            }
        }
        assert!(total_attacks > 0, "the human must be able to attack under full control");
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
