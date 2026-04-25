//! Reward computation for RL training.
//!
//! v0 ships **sparse-only**: `+1.0` for the perspective player on a
//! win, `-1.0` on a loss (or their own elimination), `0.0` on a draw,
//! and `0.0` while the game is ongoing. Shaped rewards (life delta,
//! board diff, card advantage — see spec §14.3) are deliberately
//! deferred. Picking shaping coefficients is a research decision
//! that depends on observed training behavior, not a warm-up.
//!
//! The [`RewardFunction`] trait is shaped so additive composition
//! can land later without breakage:
//!
//! ```ignore
//! pub struct ShapedReward {
//!     pub terminal_weight: f32,
//!     pub life_diff_weight: f32,
//!     pub board_diff_weight: f32,
//!     // ...
//! }
//! ```
//!
//! Until that exists, all consumers should hold a `&dyn RewardFunction`
//! so the implementation can be swapped at runtime without changing
//! the policy/value-net plumbing.

use arcana_core::state::{GameResult, GameState};
use arcana_core::types::PlayerId;

/// Compute a scalar reward for a [`GameState`] from the perspective
/// of one player. Implementations may be sparse (only nonzero at
/// terminal states) or shaped (continuous signal during play).
///
/// Object-safe so multiple reward functions can be selected at
/// runtime — `&dyn RewardFunction` is the expected callsite type.
pub trait RewardFunction {
    fn reward(&self, state: &GameState, perspective: PlayerId) -> f32;
}

/// Sparse terminal reward.
///
/// * `Some(GameResult::Win(p))`        → `+1.0` if `p == perspective`, else `-1.0`.
/// * `Some(GameResult::Draw)`          → `0.0` for every perspective.
/// * `Some(GameResult::Eliminated(p))` → `-1.0` if `p == perspective`, else `0.0`.
///   (Multiplayer: only the eliminated player gets the loss signal;
///   surviving players receive `0.0` until their own outcome
///   resolves.)
/// * `None` (game ongoing)             → `0.0`.
#[derive(Debug, Clone, Copy, Default)]
pub struct TerminalReward;

impl RewardFunction for TerminalReward {
    fn reward(&self, state: &GameState, perspective: PlayerId) -> f32 {
        match &state.result {
            None => 0.0,
            Some(GameResult::Win(p)) => {
                if *p == perspective {
                    1.0
                } else {
                    -1.0
                }
            }
            Some(GameResult::Draw) => 0.0,
            Some(GameResult::Eliminated(p)) => {
                if *p == perspective {
                    -1.0
                } else {
                    0.0
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ongoing_game_is_zero_for_all_players() {
        let s = GameState::new(2, 0);
        assert!(s.result.is_none());
        let r = TerminalReward;
        assert_eq!(r.reward(&s, 0), 0.0);
        assert_eq!(r.reward(&s, 1), 0.0);
    }

    #[test]
    fn win_is_plus_one_for_winner_minus_one_for_loser() {
        let mut s = GameState::new(2, 0);
        s.result = Some(GameResult::Win(0));
        let r = TerminalReward;
        assert_eq!(r.reward(&s, 0), 1.0);
        assert_eq!(r.reward(&s, 1), -1.0);
    }

    #[test]
    fn draw_is_zero_for_all_players() {
        let mut s = GameState::new(2, 0);
        s.result = Some(GameResult::Draw);
        let r = TerminalReward;
        assert_eq!(r.reward(&s, 0), 0.0);
        assert_eq!(r.reward(&s, 1), 0.0);
    }

    #[test]
    fn elimination_is_loss_for_eliminated_zero_for_others() {
        let mut s = GameState::new(3, 0);
        s.result = Some(GameResult::Eliminated(2));
        let r = TerminalReward;
        assert_eq!(r.reward(&s, 0), 0.0);
        assert_eq!(r.reward(&s, 1), 0.0);
        assert_eq!(r.reward(&s, 2), -1.0);
    }

    #[test]
    fn reward_is_finite_for_every_outcome() {
        // Catches NaN/inf regressions in any future shaped impl that
        // shares this test harness — sparse v0 yields finite values
        // trivially.
        let outcomes = [
            None,
            Some(GameResult::Win(0)),
            Some(GameResult::Win(1)),
            Some(GameResult::Draw),
            Some(GameResult::Eliminated(0)),
            Some(GameResult::Eliminated(1)),
        ];
        for o in outcomes {
            let mut s = GameState::new(2, 0);
            s.result = o.clone();
            let r = TerminalReward;
            for p in 0..2 {
                let v = r.reward(&s, p);
                assert!(
                    v.is_finite(),
                    "reward must be finite; got {v} for {o:?} perspective={p}"
                );
            }
        }
    }

    #[test]
    fn trait_is_object_safe() {
        // Compile-time check: a future API change that breaks
        // object-safety fails this test loudly instead of deep in a
        // training-loop callsite.
        let _: Box<dyn RewardFunction> = Box::new(TerminalReward);
    }
}
