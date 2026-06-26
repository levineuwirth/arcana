//! End-to-end regression for the combat-concede winner bug: conceding must make
//! the CONCEDER lose — never win.
//!
//! Before the fix, `Action::Concede` was attributed to `priority_player()` while
//! `compute_next_decision` yielded a different, per-combat-phase "decider"; at an
//! in-combat priority window those disagreed, so the conceder could eliminate
//! the OTHER player and "win" (a `FlatMonteCarloPolicy` learned to concede
//! instantly for a free win). The decider is now always the priority holder, so
//! conceding eliminates the conceder. Seed 7 on a mirror `sample_deck`
//! deterministically reaches such an in-combat decision at action 13.

use arcana_core::actions::Action;
use arcana_core::engine::{new_game, step, EngineYield};
use arcana_core::state::GameResult;

#[test]
fn concede_when_decider_is_not_priority_holder_loses() {
    let reg = arcana_cards::build_catalog();
    let deck = arcana_cards::sample_deck(&reg, 1);
    let pre = vec![
        Action::MulliganKeep, Action::MulliganKeep,
        Action::PassPriority, Action::PassPriority, Action::PassPriority,
        Action::PassPriority, Action::PassPriority, Action::PassPriority,
        Action::PassPriority, Action::PassPriority,
        Action::DeclareAttackers { attackers: vec![] },
        Action::PassPriority, Action::PassPriority,
    ];
    let (mut state, mut yld) = new_game(vec![deck.clone(), deck], &reg, 7);
    for a in &pre {
        let (s, y) = step(state, a.clone(), &reg);
        state = s; yld = y;
    }

    // An in-combat decision that offers Concede.
    let decider = match &yld {
        EngineYield::PendingDecision { player, legal_actions, .. } => {
            assert!(legal_actions.iter().any(|a| matches!(a, Action::Concede)),
                "the decision must offer Concede");
            *player
        }
        EngineYield::GameOver(r) => panic!("unexpected early game over: {r:?}"),
    };

    // The decider concedes — the conceder must LOSE (the bug made them WIN).
    let (_state, yld) = step(state, Action::Concede, &reg);
    match yld {
        EngineYield::GameOver(GameResult::Win(w)) => {
            assert_ne!(w, decider, "the conceder (P{decider}) must lose, not win");
        }
        other => panic!("expected a Win result after concede, got {other:?}"),
    }
}
