//! Turn structure: `TurnState`, `Phase`, `Step`, and their classification
//! helpers.
//!
//! Addendum Listing 6, Phase 1 Task #7. Depends on task 6 (state).
//!
//! The actual state-machine transitions — "when in (Combat, DeclareBlockers)
//! with no first-striking creatures, skip to CombatDamageRegular" — live in
//! [`crate::engine::advance_phase`] (Task #20). This module only defines the
//! types and the pure-data helpers that don't need the rest of the state.

use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

use crate::types::PlayerId;

// =============================================================================
// TurnState
// =============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnState {
    pub active_player: PlayerId,
    pub turn_number: u32,
    pub phase: Phase,
    pub step: Step,
    /// FIFO of players owed an extra turn (CR 500.7). Popped when the
    /// current turn ends; if empty, turn rotates normally.
    pub extra_turns: VecDeque<PlayerId>,
    /// Queue of extra combat phases for the current turn
    /// (CR 500.7; Aggravated Assault et al.). Decremented when consumed.
    pub extra_combats: u32,
}

impl TurnState {
    /// Is the current phase a main phase (pre- or post-combat)?
    pub const fn is_main_phase(&self) -> bool { self.phase.is_main() }
    /// Is the current phase the combat phase?
    pub const fn is_combat(&self) -> bool { self.phase.is_combat() }
    /// Is the current step the cleanup step?
    pub const fn is_cleanup(&self) -> bool { self.step.is_cleanup() }

    /// Whether `(phase, step)` is a legal pairing. Catches state-machine
    /// bugs that would set inconsistent phase/step values.
    ///
    /// Returns `true` for exactly the pairings enumerated in the
    /// [CR 500.1 turn structure](https://magic.wizards.com/en/rules).
    pub const fn is_valid(&self) -> bool {
        use Phase::*;
        use Step::*;
        matches!(
            (self.phase, self.step),
            (Beginning, Untap)
            | (Beginning, Upkeep)
            | (Beginning, Draw)
            | (PreCombatMain, Main)
            | (Combat, BeginCombat)
            | (Combat, DeclareAttackers)
            | (Combat, DeclareBlockers)
            | (Combat, CombatDamage)
            | (Combat, CombatDamageRegular)
            | (Combat, EndCombat)
            | (PostCombatMain, Main)
            | (Ending, End)
            | (Ending, Cleanup)
        )
    }

    /// Queue an extra turn for `player` after the current one ends.
    /// Multiple queued extras take priority in the order they were queued
    /// (FIFO per CR 500.7: "extra turns are taken in a FIFO manner" — though
    /// rulings have varied; we match the modern convention).
    pub fn queue_extra_turn(&mut self, player: PlayerId) {
        self.extra_turns.push_back(player);
    }

    /// Take the next queued extra turn, if any. Returns the player who
    /// takes it.
    pub fn take_extra_turn(&mut self) -> Option<PlayerId> {
        self.extra_turns.pop_front()
    }

    /// Queue an additional combat phase for this turn.
    pub fn queue_extra_combat(&mut self) {
        self.extra_combats = self.extra_combats.saturating_add(1);
    }

    /// Consume a queued extra combat. Returns `true` if one was consumed.
    pub fn consume_extra_combat(&mut self) -> bool {
        if self.extra_combats > 0 {
            self.extra_combats -= 1;
            true
        } else {
            false
        }
    }

    /// Reset turn state to the start of a fresh turn for `new_active_player`.
    /// Increments `turn_number`, sets phase/step to `(Beginning, Untap)`.
    /// Extra-turn / extra-combat queues are *not* touched here — callers
    /// pop from them as needed.
    pub fn start_next_turn(&mut self, new_active_player: PlayerId) {
        self.active_player = new_active_player;
        self.turn_number = self.turn_number.saturating_add(1);
        self.phase = Phase::Beginning;
        self.step = Step::Untap;
    }
}

// =============================================================================
// Phase
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Phase {
    Beginning,
    PreCombatMain,
    Combat,
    PostCombatMain,
    Ending,
}

impl Phase {
    pub const fn is_beginning(self) -> bool { matches!(self, Phase::Beginning) }
    pub const fn is_combat(self)    -> bool { matches!(self, Phase::Combat) }
    pub const fn is_ending(self)    -> bool { matches!(self, Phase::Ending) }
    pub const fn is_main(self)      -> bool {
        matches!(self, Phase::PreCombatMain | Phase::PostCombatMain)
    }
    pub const fn is_pre_combat_main(self)  -> bool { matches!(self, Phase::PreCombatMain) }
    pub const fn is_post_combat_main(self) -> bool { matches!(self, Phase::PostCombatMain) }

    /// All five phases in turn order. Useful for enumeration in tests and
    /// debug output.
    pub fn all() -> impl Iterator<Item = Phase> {
        [
            Phase::Beginning,
            Phase::PreCombatMain,
            Phase::Combat,
            Phase::PostCombatMain,
            Phase::Ending,
        ].into_iter()
    }
}

// =============================================================================
// Step
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Step {
    Untap,
    Upkeep,
    Draw,
    /// Used for both pre-combat and post-combat main phases. Use
    /// `turn.phase` to disambiguate.
    Main,
    BeginCombat,
    DeclareAttackers,
    DeclareBlockers,
    /// First-strike damage sub-step (CR 510.5). Only entered if some
    /// creature in combat has first strike or double strike.
    CombatDamage,
    CombatDamageRegular,
    EndCombat,
    End,
    Cleanup,
}

impl Step {
    pub const fn is_untap(self)   -> bool { matches!(self, Step::Untap) }
    pub const fn is_upkeep(self)  -> bool { matches!(self, Step::Upkeep) }
    pub const fn is_draw(self)    -> bool { matches!(self, Step::Draw) }
    pub const fn is_main(self)    -> bool { matches!(self, Step::Main) }
    pub const fn is_end(self)     -> bool { matches!(self, Step::End) }
    pub const fn is_cleanup(self) -> bool { matches!(self, Step::Cleanup) }

    /// True if this is one of the six combat sub-steps.
    pub const fn is_combat(self) -> bool {
        matches!(
            self,
            Step::BeginCombat | Step::DeclareAttackers | Step::DeclareBlockers
            | Step::CombatDamage | Step::CombatDamageRegular | Step::EndCombat
        )
    }

    /// True for the two damage sub-steps.
    pub const fn is_combat_damage(self) -> bool {
        matches!(self, Step::CombatDamage | Step::CombatDamageRegular)
    }

    /// All twelve steps in turn order (treating the single `Main` step as
    /// one entry). For separate pre/post main steps, pair each with its
    /// phase via [`TurnState::is_valid`].
    pub fn all() -> impl Iterator<Item = Step> {
        [
            Step::Untap, Step::Upkeep, Step::Draw,
            Step::Main,
            Step::BeginCombat, Step::DeclareAttackers, Step::DeclareBlockers,
            Step::CombatDamage, Step::CombatDamageRegular, Step::EndCombat,
            Step::End, Step::Cleanup,
        ].into_iter()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Phase classification ------------------------------------------------

    #[test]
    fn phase_predicates() {
        assert!(Phase::Beginning.is_beginning());
        assert!(Phase::Combat.is_combat());
        assert!(Phase::Ending.is_ending());
        assert!(Phase::PreCombatMain.is_main());
        assert!(Phase::PostCombatMain.is_main());

        // Main phases are mutually exclusive from combat/beginning/ending.
        for p in Phase::all() {
            if p.is_main() {
                assert!(!p.is_combat() && !p.is_beginning() && !p.is_ending());
            }
        }
    }

    #[test]
    fn phase_all_enumerates_five() {
        let phases: Vec<_> = Phase::all().collect();
        assert_eq!(phases.len(), 5);
    }

    // --- Step classification -------------------------------------------------

    #[test]
    fn step_beginning_predicates() {
        assert!(Step::Untap.is_untap());
        assert!(Step::Upkeep.is_upkeep());
        assert!(Step::Draw.is_draw());
        assert!(!Step::Untap.is_combat());
    }

    #[test]
    fn step_combat_predicates() {
        for step in [
            Step::BeginCombat, Step::DeclareAttackers, Step::DeclareBlockers,
            Step::CombatDamage, Step::CombatDamageRegular, Step::EndCombat,
        ] {
            assert!(step.is_combat(), "expected {step:?} to be a combat step");
        }
        assert!(!Step::Main.is_combat());
        assert!(!Step::End.is_combat());
    }

    #[test]
    fn step_combat_damage_predicates() {
        assert!(Step::CombatDamage.is_combat_damage());
        assert!(Step::CombatDamageRegular.is_combat_damage());
        assert!(!Step::DeclareAttackers.is_combat_damage());
    }

    #[test]
    fn step_ending_predicates() {
        assert!(Step::End.is_end());
        assert!(Step::Cleanup.is_cleanup());
        assert!(!Step::End.is_cleanup());
    }

    #[test]
    fn step_all_enumerates_twelve() {
        let steps: Vec<_> = Step::all().collect();
        assert_eq!(steps.len(), 12);
    }

    // --- TurnState::is_valid -------------------------------------------------

    #[test]
    fn initial_turn_state_is_valid() {
        let t = TurnState::new_initial(0);
        assert!(t.is_valid());
    }

    #[test]
    fn all_valid_phase_step_pairs() {
        // The 13 legal pairings per CR 500.1.
        let valid: &[(Phase, Step)] = &[
            (Phase::Beginning,      Step::Untap),
            (Phase::Beginning,      Step::Upkeep),
            (Phase::Beginning,      Step::Draw),
            (Phase::PreCombatMain,  Step::Main),
            (Phase::Combat,         Step::BeginCombat),
            (Phase::Combat,         Step::DeclareAttackers),
            (Phase::Combat,         Step::DeclareBlockers),
            (Phase::Combat,         Step::CombatDamage),
            (Phase::Combat,         Step::CombatDamageRegular),
            (Phase::Combat,         Step::EndCombat),
            (Phase::PostCombatMain, Step::Main),
            (Phase::Ending,         Step::End),
            (Phase::Ending,         Step::Cleanup),
        ];
        let mut t = TurnState::new_initial(0);
        for &(phase, step) in valid {
            t.phase = phase;
            t.step = step;
            assert!(t.is_valid(), "pair ({phase:?}, {step:?}) should be valid");
        }
    }

    #[test]
    fn invalid_phase_step_pairs_rejected() {
        // Sample a handful of nonsense pairings.
        let invalid: &[(Phase, Step)] = &[
            (Phase::Beginning,     Step::Main),
            (Phase::Beginning,     Step::DeclareAttackers),
            (Phase::Combat,        Step::Main),
            (Phase::Combat,        Step::Untap),
            (Phase::PreCombatMain, Step::Cleanup),
            (Phase::Ending,        Step::Untap),
            (Phase::Ending,        Step::Main),
        ];
        let mut t = TurnState::new_initial(0);
        for &(phase, step) in invalid {
            t.phase = phase;
            t.step = step;
            assert!(!t.is_valid(), "pair ({phase:?}, {step:?}) should be invalid");
        }
    }

    #[test]
    fn valid_pair_count_matches_cr_500_1() {
        // Sanity check: sweep the product of Phase × Step and count valid
        // pairs. Should be exactly 13.
        let mut t = TurnState::new_initial(0);
        let mut count = 0;
        for p in Phase::all() {
            for s in Step::all() {
                t.phase = p;
                t.step = s;
                if t.is_valid() { count += 1; }
            }
        }
        assert_eq!(count, 13);
    }

    // --- Extra turns ---------------------------------------------------------

    #[test]
    fn extra_turn_queue_is_fifo() {
        let mut t = TurnState::new_initial(0);
        assert!(t.take_extra_turn().is_none());

        t.queue_extra_turn(1);
        t.queue_extra_turn(0);
        t.queue_extra_turn(1);

        assert_eq!(t.take_extra_turn(), Some(1));
        assert_eq!(t.take_extra_turn(), Some(0));
        assert_eq!(t.take_extra_turn(), Some(1));
        assert_eq!(t.take_extra_turn(), None);
    }

    // --- Extra combats -------------------------------------------------------

    #[test]
    fn extra_combat_queue_and_consume() {
        let mut t = TurnState::new_initial(0);
        assert!(!t.consume_extra_combat());
        assert_eq!(t.extra_combats, 0);

        t.queue_extra_combat();
        t.queue_extra_combat();
        assert_eq!(t.extra_combats, 2);

        assert!(t.consume_extra_combat());
        assert_eq!(t.extra_combats, 1);
        assert!(t.consume_extra_combat());
        assert_eq!(t.extra_combats, 0);
        assert!(!t.consume_extra_combat());
    }

    #[test]
    fn extra_combat_queue_saturates() {
        let mut t = TurnState::new_initial(0);
        t.extra_combats = u32::MAX;
        t.queue_extra_combat(); // saturates, no panic
        assert_eq!(t.extra_combats, u32::MAX);
    }

    // --- start_next_turn -----------------------------------------------------

    #[test]
    fn start_next_turn_bumps_and_resets() {
        let mut t = TurnState::new_initial(0);
        t.phase = Phase::Ending;
        t.step = Step::Cleanup;
        t.turn_number = 5;

        t.start_next_turn(1);

        assert_eq!(t.active_player, 1);
        assert_eq!(t.turn_number, 6);
        assert_eq!(t.phase, Phase::Beginning);
        assert_eq!(t.step, Step::Untap);
    }

    #[test]
    fn start_next_turn_does_not_touch_queues() {
        // Extra turns and extra combats persist across turn boundaries;
        // they're popped explicitly by engine logic.
        let mut t = TurnState::new_initial(0);
        t.queue_extra_turn(1);
        t.queue_extra_combat();

        t.start_next_turn(1);

        assert_eq!(t.extra_turns.len(), 1);
        assert_eq!(t.extra_combats, 1);
    }

    // --- TurnState forwarding helpers ---------------------------------------

    #[test]
    fn turn_state_forwards_phase_predicates() {
        let mut t = TurnState::new_initial(0);
        t.phase = Phase::PreCombatMain;
        t.step = Step::Main;
        assert!(t.is_main_phase());
        assert!(!t.is_combat());
        assert!(!t.is_cleanup());

        t.phase = Phase::Combat;
        t.step = Step::DeclareAttackers;
        assert!(t.is_combat());
        assert!(!t.is_main_phase());

        t.phase = Phase::Ending;
        t.step = Step::Cleanup;
        assert!(t.is_cleanup());
    }
}
