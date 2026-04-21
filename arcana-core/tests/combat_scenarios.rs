//! Engine-driven combat scenarios. Unlike the tests in
//! `seed_integration.rs` that call `apply_declared_*` / `deal_combat_damage`
//! directly, these drive the full `step()` loop so the engine's
//! phase-gating and yield shape is also under test.

use arcana_core::actions::{Action, DecisionContext};
use arcana_core::combat::{
    AttackerDeclaration, BlockerDeclaration, CombatPhase, DamageAssignment,
    DefendingEntity, PendingDamagePass,
};
use arcana_core::engine::{advance_phase, step, EngineYield};
use arcana_core::turn::{Phase, Step};
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

/// CR 510.1c end-to-end: after `advance_phase` reaches the combat-
/// damage step with a multi-blocked attacker, the engine yields a
/// `DecisionContext::DistributeDamage` and waits for the active
/// player's `Action::AssignCombatDamage`. Submitting a valid
/// distribution deals the damage and advances the step.
#[test]
fn engine_yields_distribute_damage_and_applies_assignment() {
    let reg = CardRegistry::new();
    let mut s = GameState::new(2, 0);
    // Set turn state to Combat / DeclareBlockers so advance_phase
    // transitions directly into the combat-damage step.
    s.turn.phase = Phase::Combat;
    s.turn.step = Step::DeclareBlockers;
    s.begin_combat();
    let atk = creature(&mut s, 0, 5, 5);
    let b1 = creature(&mut s, 1, 2, 2);
    let b2 = creature(&mut s, 1, 1, 4);

    s.apply_declared_attackers(vec![AttackerDeclaration {
        attacker: atk, defending: DefendingEntity::Player(1),
    }]);
    s.enter_declare_blockers();
    s.apply_declared_blockers(vec![
        BlockerDeclaration { blocker: b1, blocking: atk },
        BlockerDeclaration { blocker: b2, blocking: atk },
    ]);
    // Skip through OrderBlockers using the declared order.
    s.apply_blocker_ordering(vec![(atk, vec![b1, b2])]);

    // advance_phase runs the (phase, step) state machine one
    // transition at a time. DeclareBlockers → CombatDamageRegular is
    // step (1); the flag-setting / deal-damage logic lives on the
    // CombatDamageRegular arm which needs step (2).
    advance_phase(&mut s, &reg);
    advance_phase(&mut s, &reg);
    assert_eq!(s.turn.step, Step::CombatDamageRegular,
        "advanced into the regular damage step");
    assert_eq!(s.combat.as_ref().unwrap().pending_damage_assignment,
        Some(PendingDamagePass::Regular),
        "regular damage step is pending a CR 510.1c distribution");
    assert_eq!(s.objects.get(b1).unwrap().damage_marked, 0,
        "damage has NOT been dealt yet");

    // Submit a legal distribution via step().
    let (s, yld) = step(s, Action::AssignCombatDamage {
        distributions: vec![DamageAssignment {
            attacker: atk,
            distribution: vec![(b1, 3), (b2, 2)],
        }],
    }, &reg);

    // b1 took 3 (lethal); SBA re-id'd it into the graveyard.
    // b2 took 2 (non-lethal); still on battlefield with the marked
    // damage visible.
    assert_eq!(s.zone_count(arcana_core::zones::Zone::Graveyard(1)), 1,
        "b1 died from lethal assignment and landed in p1's graveyard");
    assert_eq!(s.objects.get(b2).unwrap().damage_marked, 2,
        "b2 took its share of damage per the submitted distribution");
    assert_eq!(s.combat.as_ref().unwrap().pending_damage_assignment, None,
        "pending flag cleared after assignment");
    assert_eq!(s.turn.step, Step::EndCombat,
        "step advanced past regular damage");
    // Next yield is a normal priority window in the next step.
    assert!(matches!(yld, EngineYield::PendingDecision { .. }));
}

/// Submitting an illegal distribution leaves the engine pending so
/// the agent can retry. Verifies C2's `is_legal_damage_assignment`
/// guards the C3 pipeline.
#[test]
fn assign_combat_damage_rejects_illegal_and_stays_pending() {
    let reg = CardRegistry::new();
    let mut s = GameState::new(2, 0);
    s.turn.phase = Phase::Combat;
    s.turn.step = Step::DeclareBlockers;
    s.begin_combat();
    let atk = creature(&mut s, 0, 5, 5);
    let b1 = creature(&mut s, 1, 2, 2);
    let b2 = creature(&mut s, 1, 1, 4);
    s.apply_declared_attackers(vec![AttackerDeclaration {
        attacker: atk, defending: DefendingEntity::Player(1),
    }]);
    s.enter_declare_blockers();
    s.apply_declared_blockers(vec![
        BlockerDeclaration { blocker: b1, blocking: atk },
        BlockerDeclaration { blocker: b2, blocking: atk },
    ]);
    s.apply_blocker_ordering(vec![(atk, vec![b1, b2])]);
    advance_phase(&mut s, &reg);
    advance_phase(&mut s, &reg);

    // Illegal: 1 damage to b1 (< lethal 2), 4 to b2.
    let (s, _) = step(s, Action::AssignCombatDamage {
        distributions: vec![DamageAssignment {
            attacker: atk,
            distribution: vec![(b1, 1), (b2, 4)],
        }],
    }, &reg);

    assert_eq!(s.combat.as_ref().unwrap().pending_damage_assignment,
        Some(PendingDamagePass::Regular),
        "illegal submission leaves pending flag intact");
    assert_eq!(s.objects.get(b1).unwrap().damage_marked, 0,
        "no damage dealt on rejected submission");
}

/// CR 702.19b end-to-end: a trample attacker with a single blocker
/// whose power exceeds the blocker's lethal damage still yields a
/// `DecisionContext::DistributeDamage` — the controller chooses how
/// much excess overflows to the defender. The engine respects the
/// submitted assignment rather than auto-distributing.
#[test]
fn trample_single_blocker_yields_distribute_damage_with_overflow() {
    use arcana_core::effects::KeywordAbility;
    let reg = CardRegistry::new();
    let mut s = GameState::new(2, 0);
    s.turn.phase = Phase::Combat;
    s.turn.step = Step::DeclareBlockers;
    s.begin_combat();
    let atk = creature(&mut s, 0, 5, 5);
    s.objects.get_mut(atk).unwrap().characteristics.keywords
        .push(KeywordAbility::Trample);
    let blk = creature(&mut s, 1, 2, 2);
    s.apply_declared_attackers(vec![AttackerDeclaration {
        attacker: atk, defending: DefendingEntity::Player(1),
    }]);
    s.enter_declare_blockers();
    s.apply_declared_blockers(vec![
        BlockerDeclaration { blocker: blk, blocking: atk },
    ]);

    advance_phase(&mut s, &reg);
    advance_phase(&mut s, &reg);
    assert_eq!(s.combat.as_ref().unwrap().pending_damage_assignment,
        Some(PendingDamagePass::Regular),
        "trample single-blocker with overflow capacity pends");
    assert_eq!(s.objects.get(blk).unwrap().damage_marked, 0,
        "damage waits for the controller's distribution");

    // Controller picks 4 to blocker, 1 overflow to defender — a legal
    // distribution that the auto-distribution would not have chosen.
    let (s, _yld) = step(s, Action::AssignCombatDamage {
        distributions: vec![DamageAssignment {
            attacker: atk,
            distribution: vec![(blk, 4)],
        }],
    }, &reg);
    assert_eq!(s.zone_count(arcana_core::zones::Zone::Graveyard(1)), 1,
        "blocker died (4 damage > 2 toughness)");
    assert_eq!(s.player(1).life, 20 - 1,
        "1 point of overflow reached the defender");
    assert_eq!(s.combat.as_ref().unwrap().pending_damage_assignment, None);
    assert_eq!(s.turn.step, Step::EndCombat);
}

/// When no attacker needs CR 510.1c assignment (unblocked + single-
/// blocker combat), the engine skips the yield and deals damage
/// immediately, same as before.
#[test]
fn single_blocker_combat_skips_distribute_damage_yield() {
    let reg = CardRegistry::new();
    let mut s = GameState::new(2, 0);
    s.turn.phase = Phase::Combat;
    s.turn.step = Step::DeclareBlockers;
    s.begin_combat();
    let atk = creature(&mut s, 0, 3, 3);
    let blk = creature(&mut s, 1, 2, 2);
    s.apply_declared_attackers(vec![AttackerDeclaration {
        attacker: atk, defending: DefendingEntity::Player(1),
    }]);
    s.enter_declare_blockers();
    s.apply_declared_blockers(vec![
        BlockerDeclaration { blocker: blk, blocking: atk },
    ]);

    // Two calls: DeclareBlockers → CombatDamageRegular (step 1),
    // CombatDamageRegular → deal-damage → EndCombat (step 2).
    advance_phase(&mut s, &reg);
    advance_phase(&mut s, &reg);
    assert!(s.combat.as_ref().unwrap().pending_damage_assignment.is_none(),
        "no pending flag when no multi-blocker attackers");
    assert_eq!(s.objects.get(blk).unwrap().damage_marked, 3,
        "damage dealt immediately");
    assert_eq!(s.objects.get(atk).unwrap().damage_marked, 2);
}
