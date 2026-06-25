//! End-to-end regression for the combat-concede winner bug: a player conceding
//! while they are the DECIDER but not the priority holder must LOSE, not win.
//!
//! Seed 7 on a mirror `sample_deck` deterministically reaches a state at
//! action 13 where the decider is P1 while P0 holds priority. Before the fix,
//! `apply_concede` eliminated the priority holder (P0), so P1's concede
//! reported `Win(1)` — and `FlatMonteCarloPolicy` learned to concede instantly
//! for a free "win". This locks the corrected behavior.

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

    // The engine should now be awaiting P1 (a non-priority decider).
    let decider = match &yld {
        EngineYield::PendingDecision { player, .. } => *player,
        EngineYield::GameOver(r) => panic!("unexpected early game over: {r:?}"),
    };
    assert_eq!(decider, 1, "seed-7 fixture should put the decision on P1");

    // P1 concedes — P1 must lose, so the opponent (P0) wins.
    let (_state, yld) = step(state, Action::Concede, &reg);
    match yld {
        EngineYield::GameOver(GameResult::Win(w)) => {
            assert_eq!(w, 0, "the conceder (P1) must lose; opponent P0 wins");
        }
        other => panic!("expected GameOver/Win(0), got {other:?}"),
    }
}
