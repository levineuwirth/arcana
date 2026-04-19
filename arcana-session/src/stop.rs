//! Auto-pass controls.
//!
//! By default a [`GameSession`] auto-passes priority on a player's
//! behalf whenever the only meaningful options are `PassPriority` and
//! `Concede` — the trivial case. [`StopSettings`] lets the caller
//! expand that: add [`StopCondition`]s to force a real prompt at
//! specific phases / events, or set `full_control = true` to disable
//! auto-pass entirely.
//!
//! # Information-leakage caveat
//!
//! Auto-pass is observable. If a client's auto-pass policy depends on
//! hidden state (e.g., "pause in my upkeep iff I have an instant to
//! cast"), opponents can learn about that hidden state from the
//! pause pattern. Arcana's session layer deliberately keeps the
//! auto-pass rule simple and state-independent; mitigating leaks via
//! deliberate latency or false stops is a UI-layer concern, not ours.
//!
//! [`GameSession`]: crate::GameSession

use std::collections::HashSet;

use arcana_core::Action;

/// Per-player auto-pass configuration.
///
/// The default is "auto-pass on trivial priority" — equivalent to
/// tabletop / Arena without custom stops.
#[derive(Clone, Debug, Default)]
pub struct StopSettings {
    /// Extra stop points where auto-pass is suppressed. Empty by
    /// default.
    pub stops: HashSet<StopCondition>,
    /// When `true`, auto-pass is disabled entirely — every priority
    /// point reaches the agent. Equivalent to Arena's "full control"
    /// mode.
    pub full_control: bool,
}

/// Where to suppress auto-pass. Additive: each added condition only
/// *opts out* of auto-pass for the matching priority point; it does
/// not synthesize any new action.
///
/// Interpretation of "own" / "opponent" resolves against the player
/// who owns the [`StopSettings`] this condition lives in.
///
/// Phase-2 scaffolding note: only the conditions whose triggers are
/// easy to detect from `(legal_actions, context)` without inspecting
/// deep state are wired today. The rest are accepted into the set
/// but have no effect until the corresponding detector lands —
/// callers can start recording their intended stop points now.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StopCondition {
    /// Stop during your own upkeep step.
    OwnUpkeep,
    /// Stop during your own main phases (pre-combat and post-combat).
    OwnMain,
    /// Stop when you have a non-trivial response available (anything
    /// beyond pass/concede).
    HasResponse,
    /// Stop when an opponent casts a spell you could respond to.
    OpponentCastsSpell,
    /// Stop at the `DeclareAttackers` sub-step when it's your turn.
    DeclareAttackers,
    /// Stop at the `DeclareBlockers` sub-step when an opponent is
    /// attacking you.
    DeclareBlockers,
    /// Stop at the end step of your turn.
    EndStep,
    /// Stop at the end step of an opponent's turn.
    OpponentEndStep,
}

/// Default auto-pass rule: auto-pass when the agent's only legal
/// moves are `PassPriority` and/or `Concede`. Returns `true` if the
/// session should inject `PassPriority` automatically.
///
/// Callers override this by setting `full_control`; higher-level
/// session integration layers can also consult `StopSettings::stops`
/// once the corresponding detectors are in place.
pub fn should_auto_pass(
    settings: &StopSettings,
    legal_actions: &[Action],
) -> bool {
    if settings.full_control {
        return false;
    }
    // Any action that isn't pass-or-concede is "meaningful" — break
    // the auto-pass and let the agent decide.
    let has_meaningful = legal_actions.iter()
        .any(|a| !a.is_pass() && !a.is_concede());
    !has_meaningful
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_passes_when_only_pass_and_concede_are_legal() {
        let s = StopSettings::default();
        assert!(should_auto_pass(&s, &[Action::PassPriority, Action::Concede]));
    }

    #[test]
    fn does_not_auto_pass_when_meaningful_action_available() {
        let s = StopSettings::default();
        let actions = vec![
            Action::PassPriority,
            Action::Concede,
            Action::MulliganKeep, // stand-in for any non-pass action
        ];
        assert!(!should_auto_pass(&s, &actions));
    }

    #[test]
    fn full_control_disables_auto_pass() {
        let s = StopSettings { full_control: true, ..Default::default() };
        assert!(!should_auto_pass(&s, &[Action::PassPriority, Action::Concede]));
    }
}
