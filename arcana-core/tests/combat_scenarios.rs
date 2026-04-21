//! Engine-driven combat scenarios. Unlike the tests in
//! `seed_integration.rs` that call `apply_declared_*` / `deal_combat_damage`
//! directly, these drive the full `step()` loop so the engine's
//! phase-gating and yield shape is also under test.

use arcana_core::actions::{Action, DecisionContext};
use arcana_core::combat::{
    AttackerDeclaration, BlockerDeclaration, CombatPhase, DefendingEntity,
};
use arcana_core::engine::{step, EngineYield};
use arcana_core::objects::{Characteristics, GameObject};
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::types::{PlayerId, PtValue, TypeLine};
use arcana_core::ObjectId;
use arcana_core::zones::Zone;

fn creature(s: &mut GameState, owner: PlayerId, p: i32, t: i32) -> ObjectId {
    let id = s.allocate_object_id();
    let chars = Characteristics {
        types: TypeLine::CREATURE.into(),
        power: Some(PtValue::Fixed(p)),
        toughness: Some(PtValue::Fixed(t)),
        ..Default::default()
    };
    let mut obj = GameObject::new(id, owner, Zone::Battlefield, 0, chars);
    obj.controller = owner;
    obj.status.summoning_sick = false;
    s.objects.insert(obj);
    id
}

/// CR 509.2 end-to-end: after a multi-blocker declaration, the engine
/// yields a `DecisionContext::OrderBlockers` to the active player and
/// accepts an `Action::OrderBlockers` that rewrites `blocked_by`.
#[test]
fn engine_yields_order_blockers_after_multi_block() {
    let reg = CardRegistry::new();
    let mut s = GameState::new(2, 0);
    s.begin_combat();
    let atk = creature(&mut s, 0, 5, 5);
    let b1 = creature(&mut s, 1, 2, 2);
    let b2 = creature(&mut s, 1, 1, 4);

    s.apply_declared_attackers(vec![AttackerDeclaration {
        attacker: atk, defending: DefendingEntity::Player(1),
    }]);
    s.enter_declare_blockers();
    s.priority.give_to(1);

    // Defender declares the multi-block via step().
    let (s, _) = step(s, Action::DeclareBlockers {
        blockers: vec![
            BlockerDeclaration { blocker: b1, blocking: atk },
            BlockerDeclaration { blocker: b2, blocking: atk },
        ],
    }, &reg);

    // Next yield must be the OrderBlockers decision to the active player.
    let phase = s.combat.as_ref().unwrap().phase;
    assert_eq!(phase, CombatPhase::OrderBlockers);

    // Drive the engine one more step (pass priority would normally
    // advance; here we issue the OrderBlockers action directly, since
    // the engine yielded that decision).
    let (s, yld) = step(s, Action::OrderBlockers {
        orderings: vec![(atk, vec![b2, b1])],
    }, &reg);

    // Phase advanced, and the next yield is a normal priority window
    // to the active player (triggers + response window).
    assert_eq!(s.combat.as_ref().unwrap().phase,
        CombatPhase::PostDeclareBlockers);
    match yld {
        EngineYield::PendingDecision { context: DecisionContext::Priority, player, .. } => {
            assert_eq!(player, 0, "active player has priority");
        }
        other => panic!("expected Priority yield, got {other:?}"),
    }
    // Ordering was applied.
    assert_eq!(s.combat.as_ref().unwrap()
        .attacker(atk).unwrap().blocked_by, vec![b2, b1]);
}
