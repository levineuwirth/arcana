//! Priority system: [`PriorityState`], priority-passing logic, APNAP
//! ordering, and the predicate for "does this step grant priority?"
//!
//! Addendum Section 5, Phase 1 Task #10. Depends on tasks 1 (types), 6
//! (state), 7 (turn).
//!
//! **Priority model (CR 117)** — at each decision point, exactly one
//! player "has priority". They may take a game action (cast a spell,
//! activate an ability, play a land, take a special action) or pass. If
//! they act, they retain priority afterward (CR 117.3d). If they pass,
//! priority moves to the next player in turn order. When every player
//! has passed in succession without taking an action, either the
//! top-of-stack object resolves (if the stack is nonempty) or the step
//! ends (if empty) — CR 117.4.
//!
//! This module implements the mechanics of passing, tracking
//! consecutive passes, and rotating to the next player. It does *not*
//! perform state-based actions, resolve the stack, or advance the
//! phase/step — those are the engine's job (Task #20).
//!
//! Untap and cleanup are special: no player normally receives priority
//! during them (CR 502.4, 514.3). [`receives_priority_at`] encodes this.

use serde::{Serialize, Deserialize};

use crate::turn::Step;
use crate::types::PlayerId;

// =============================================================================
// PriorityState
// =============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PriorityState {
    /// Player who currently has priority.
    pub player: PlayerId,
    /// Number of consecutive passes since the last non-pass action.
    /// When this reaches `num_players`, everyone has passed in
    /// succession (CR 117.4).
    pub consecutive_passes: u32,
    /// Active special-action window (mulligan decision, discard to
    /// hand size, etc.). `Some` suspends the normal priority flow —
    /// the engine asks the relevant player to complete the special
    /// action before normal priority resumes.
    pub special_action: Option<SpecialAction>,
}

impl PriorityState {
    /// Grant priority to `player` with a fresh pass counter.
    pub fn give_to(&mut self, player: PlayerId) {
        self.player = player;
        self.consecutive_passes = 0;
    }

    /// Record a pass by the current priority-holder.
    ///
    /// Returns:
    /// - [`PriorityOutcome::PassedTo`] with the next player in turn
    ///   order if not everyone has passed yet. The state is updated to
    ///   reflect the new priority-holder and the incremented pass count.
    /// - [`PriorityOutcome::EveryonePassed`] once `consecutive_passes`
    ///   reaches `num_players`. The state leaves `self.player` at the
    ///   last passer; the caller (engine) is expected to either resolve
    ///   the top of stack and then call [`Self::give_to`] the active
    ///   player, or end the step and set up the next one.
    pub fn pass(&mut self, num_players: u8) -> PriorityOutcome {
        assert!(num_players > 0, "pass() requires at least one player");
        self.consecutive_passes = self.consecutive_passes.saturating_add(1);
        if self.consecutive_passes >= num_players as u32 {
            PriorityOutcome::EveryonePassed
        } else {
            let next = next_in_turn_order(self.player, num_players);
            self.player = next;
            PriorityOutcome::PassedTo(next)
        }
    }

    /// Record that the current priority-holder took a non-pass action
    /// (cast, activate, play-land, special action). Per CR 117.3d, the
    /// actor retains priority; the consecutive-pass counter resets
    /// because the sequence "all players passing in succession" has
    /// been interrupted.
    pub fn record_action(&mut self) {
        self.consecutive_passes = 0;
    }

    /// Convenience: `true` if every player has passed in succession
    /// for an `n`-player game.
    pub fn everyone_passed(&self, num_players: u8) -> bool {
        self.consecutive_passes >= num_players as u32
    }

    /// Begin a special-action window — mulligan decision, discard to
    /// hand size, etc. Priority mechanics are paused until the action
    /// completes via [`Self::end_special_action`].
    pub fn begin_special_action(&mut self, action: SpecialAction, player: PlayerId) {
        self.special_action = Some(action);
        self.player = player;
        self.consecutive_passes = 0;
    }

    /// Clear the active special action. Leaves `player` untouched — the
    /// caller typically follows up with [`Self::give_to`].
    pub fn end_special_action(&mut self) {
        self.special_action = None;
    }

    /// Is a special action currently pending?
    pub fn in_special_action(&self) -> bool { self.special_action.is_some() }
}

// =============================================================================
// PriorityOutcome
// =============================================================================

/// What happens after a pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PriorityOutcome {
    /// Priority moved to `next` player; they now decide.
    PassedTo(PlayerId),
    /// All players passed in succession — the engine should resolve the
    /// top of the stack (if non-empty) or end the current step
    /// (if empty), per CR 117.4.
    EveryonePassed,
}

// =============================================================================
// SpecialAction
// =============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpecialAction {
    DiscardToHandSize,
    MulliganDecision,
    LondonMulliganBottomCards(u32),
    ChooseFirstPlayer,
    Sideboarding,
}

// =============================================================================
// Turn-order helpers
// =============================================================================

/// Next player in turn order (clockwise per CR 101.4): simple modular
/// increment. In a 2-player game this just swaps; in 3+ players it
/// advances one seat.
pub const fn next_in_turn_order(current: PlayerId, num_players: u8) -> PlayerId {
    debug_assert!(num_players > 0);
    (current + 1) % num_players
}

/// APNAP iteration (CR 101.4, 603.3): yields `active_player` first,
/// then each subsequent player in turn order, exactly once each. Used
/// by the engine for ordering simultaneous events and triggered
/// abilities.
pub fn apnap_order(
    active_player: PlayerId,
    num_players: u8,
) -> impl Iterator<Item = PlayerId> {
    assert!(num_players > 0, "apnap_order requires at least one player");
    (0..num_players).map(move |i| (active_player + i) % num_players)
}

/// Does `step` normally grant priority to the active player at its
/// start? Returns `false` for [`Step::Untap`] (CR 502.4) and
/// [`Step::Cleanup`] (CR 514.3) — those skip the normal priority flow.
///
/// Cleanup may still *temporarily* grant priority if an ability
/// triggered during cleanup (CR 514.3a). That's an engine-level
/// fixup; this predicate describes the default.
pub const fn receives_priority_at(step: Step) -> bool {
    !matches!(step, Step::Untap | Step::Cleanup)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh(player: PlayerId) -> PriorityState {
        PriorityState { player, consecutive_passes: 0, special_action: None }
    }

    // --- next_in_turn_order ---------------------------------------------------

    #[test]
    fn next_in_turn_order_wraps() {
        assert_eq!(next_in_turn_order(0, 2), 1);
        assert_eq!(next_in_turn_order(1, 2), 0);
        assert_eq!(next_in_turn_order(2, 3), 0);
        assert_eq!(next_in_turn_order(0, 4), 1);
    }

    // --- apnap_order ----------------------------------------------------------

    #[test]
    fn apnap_starts_with_active_player() {
        let order: Vec<_> = apnap_order(0, 2).collect();
        assert_eq!(order, vec![0, 1]);

        let order: Vec<_> = apnap_order(1, 2).collect();
        assert_eq!(order, vec![1, 0]);
    }

    #[test]
    fn apnap_order_4_players() {
        let order: Vec<_> = apnap_order(2, 4).collect();
        assert_eq!(order, vec![2, 3, 0, 1]);
    }

    #[test]
    fn apnap_visits_every_player_once() {
        use std::collections::HashSet;
        let order: Vec<_> = apnap_order(1, 5).collect();
        let unique: HashSet<_> = order.iter().copied().collect();
        assert_eq!(order.len(), 5);
        assert_eq!(unique.len(), 5);
    }

    #[test]
    #[should_panic(expected = "at least one player")]
    fn apnap_zero_players_panics() {
        let _: Vec<_> = apnap_order(0, 0).collect();
    }

    // --- receives_priority_at -------------------------------------------------

    #[test]
    fn receives_priority_excludes_untap_and_cleanup() {
        assert!(!receives_priority_at(Step::Untap));
        assert!(!receives_priority_at(Step::Cleanup));
        assert!( receives_priority_at(Step::Upkeep));
        assert!( receives_priority_at(Step::Draw));
        assert!( receives_priority_at(Step::Main));
        assert!( receives_priority_at(Step::BeginCombat));
        assert!( receives_priority_at(Step::DeclareAttackers));
        assert!( receives_priority_at(Step::DeclareBlockers));
        assert!( receives_priority_at(Step::CombatDamage));
        assert!( receives_priority_at(Step::CombatDamageRegular));
        assert!( receives_priority_at(Step::EndCombat));
        assert!( receives_priority_at(Step::End));
    }

    // --- PriorityState::give_to ----------------------------------------------

    #[test]
    fn give_to_sets_player_and_clears_passes() {
        let mut ps = PriorityState {
            player: 0,
            consecutive_passes: 2,
            special_action: None,
        };
        ps.give_to(1);
        assert_eq!(ps.player, 1);
        assert_eq!(ps.consecutive_passes, 0);
    }

    // --- PriorityState::pass -------------------------------------------------

    #[test]
    fn pass_rotates_to_next_in_two_player() {
        let mut ps = fresh(0);
        let out = ps.pass(2);
        assert_eq!(out, PriorityOutcome::PassedTo(1));
        assert_eq!(ps.player, 1);
        assert_eq!(ps.consecutive_passes, 1);
    }

    #[test]
    fn second_pass_reports_everyone_passed_in_two_player() {
        let mut ps = fresh(0);
        ps.pass(2); // passed → 1
        let out = ps.pass(2);
        assert_eq!(out, PriorityOutcome::EveryonePassed);
        assert_eq!(ps.consecutive_passes, 2);
        // The current holder stays at whoever passed last (player 1);
        // the engine is expected to re-grant priority via give_to() after
        // resolving / ending the step.
        assert_eq!(ps.player, 1);
    }

    #[test]
    fn pass_rotation_in_three_player() {
        let mut ps = fresh(0);
        let a = ps.pass(3);
        let b = ps.pass(3);
        let c = ps.pass(3);
        assert_eq!(a, PriorityOutcome::PassedTo(1));
        assert_eq!(b, PriorityOutcome::PassedTo(2));
        assert_eq!(c, PriorityOutcome::EveryonePassed);
        assert_eq!(ps.consecutive_passes, 3);
    }

    #[test]
    fn one_player_game_one_pass_is_everyone() {
        // Degenerate but worth pinning down.
        let mut ps = fresh(0);
        let out = ps.pass(1);
        assert_eq!(out, PriorityOutcome::EveryonePassed);
    }

    #[test]
    #[should_panic(expected = "at least one player")]
    fn pass_with_zero_players_panics() {
        let mut ps = fresh(0);
        let _ = ps.pass(0);
    }

    // --- record_action -------------------------------------------------------

    #[test]
    fn record_action_resets_pass_counter_keeps_player() {
        let mut ps = fresh(0);
        ps.pass(2); // now at player 1, passes = 1
        // Player 1 now acts instead of passing.
        ps.record_action();
        assert_eq!(ps.consecutive_passes, 0);
        assert_eq!(ps.player, 1);
    }

    #[test]
    fn action_then_full_round_of_passes_still_ends() {
        // Classic response flow: P0 casts → resets, then everyone passes.
        let mut ps = fresh(0);
        ps.record_action(); // P0 cast a spell
        assert_eq!(ps.consecutive_passes, 0);
        let a = ps.pass(2); // P0 passes; priority to P1
        let b = ps.pass(2); // P1 passes; everyone passed
        assert_eq!(a, PriorityOutcome::PassedTo(1));
        assert_eq!(b, PriorityOutcome::EveryonePassed);
    }

    // --- everyone_passed -----------------------------------------------------

    #[test]
    fn everyone_passed_predicate_mirrors_counter() {
        let mut ps = fresh(0);
        assert!(!ps.everyone_passed(2));
        ps.pass(2);
        assert!(!ps.everyone_passed(2));
        ps.pass(2);
        assert!(ps.everyone_passed(2));
    }

    // --- Special actions -----------------------------------------------------

    #[test]
    fn begin_special_action_sets_flags() {
        let mut ps = fresh(0);
        ps.consecutive_passes = 3;
        ps.begin_special_action(SpecialAction::MulliganDecision, 1);
        assert_eq!(ps.player, 1);
        assert_eq!(ps.consecutive_passes, 0);
        assert_eq!(ps.special_action, Some(SpecialAction::MulliganDecision));
        assert!(ps.in_special_action());
    }

    #[test]
    fn end_special_action_clears_flag_only() {
        let mut ps = fresh(0);
        ps.begin_special_action(SpecialAction::DiscardToHandSize, 1);
        ps.end_special_action();
        assert!(ps.special_action.is_none());
        assert!(!ps.in_special_action());
        // `player` is left where it was — caller handles follow-up.
        assert_eq!(ps.player, 1);
    }

    // --- Serde roundtrip -----------------------------------------------------

    #[test]
    fn priority_state_roundtrip() {
        let ps = PriorityState {
            player: 1,
            consecutive_passes: 2,
            special_action: Some(SpecialAction::LondonMulliganBottomCards(3)),
        };
        let json = serde_json::to_string(&ps).unwrap();
        let back: PriorityState = serde_json::from_str(&json).unwrap();
        assert_eq!(ps, back);
    }
}
