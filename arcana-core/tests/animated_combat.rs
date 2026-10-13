//! Combat with permanents that become creatures, or stop being creatures,
//! through continuous effects (A1.5x). Combat enumerates, validates and
//! resolves by an object's computed creature type (CR 613), never its
//! printed one: an animated land blocks and dies, a crewed Vehicle attacks,
//! a Vehicle that entered this turn is summoning sick once crewed (CR 302.6),
//! and an attacker that stops being a creature leaves combat (CR 506.4).

use arcana_core::actions::Action;
use arcana_core::combat::{legal_block_declaration, AttackerDeclaration, BlockerDeclaration, DefendingEntity};
use arcana_core::engine::step;
use arcana_core::events::MoveCause;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, ManaColor, PlayerId};
use arcana_core::zones::Zone;
use arcana_core::{EngineYield, ObjectId};

fn add(s: &mut GameState, reg: &CardRegistry, owner: PlayerId, card: CardId, zone: Zone) -> ObjectId {
    let id = s.allocate_object_id();
    let chars = reg.get(card).unwrap().base_characteristics.clone();
    s.objects.insert(GameObject::new(id, owner, zone, card, chars));
    id
}

fn window(s: &mut GameState, active: PlayerId, player: PlayerId, phase: Phase, step: Step) {
    s.turn.active_player = active;
    s.turn.phase = phase;
    s.turn.step = step;
    s.priority.give_to(player);
}

/// Activate `source`'s ability `index` with the first payment the engine
/// offers, then pass until the stack is empty.
fn activate(s: GameState, reg: &CardRegistry, source: ObjectId, index: usize) -> GameState {
    let act = legal_actions(&s, reg).into_iter()
        .find(|a| matches!(a, Action::ActivateAbility { source: src, ability_index, .. }
            if *src == source && *ability_index == index))
        .unwrap_or_else(|| panic!("ability {index} of {source} is offered"));
    let (mut s, _) = step(s, act, reg);
    for _ in 0..8 {
        if s.stack_is_empty() { break; }
        (s, _) = step(s, Action::PassPriority, reg);
    }
    assert!(s.stack_is_empty(), "the ability resolved");
    s
}

/// Play out the rest of combat: pass where passing is legal, otherwise take
/// the first offered answer (a may-trigger, an ordering), never conceding.
fn finish_combat(mut s: GameState, reg: &CardRegistry) -> GameState {
    for _ in 0..60 {
        if s.combat.is_none() || !s.turn.phase.is_combat() { return s; }
        let legal = legal_actions(&s, reg);
        let next = legal.iter().find(|a| a.is_pass()).or_else(|| legal.iter()
            .find(|a| !matches!(a, Action::Concede))).cloned().expect("an action");
        let (ns, yld) = step(s, next, reg);
        s = ns;
        if matches!(yld, EngineYield::GameOver(_)) { return s; }
    }
    panic!("combat did not end");
}

fn zone_of(s: &GameState, id: ObjectId) -> Option<Zone> {
    s.objects.get(id).map(|o| o.zone)
}

/// Mutavault, animated in the opponent's declare-attackers step, is offered
/// as a blocker, blocks the Bears, and both 2/2s die in combat damage.
#[test]
fn animated_land_blocks_and_dies_in_combat() {
    let mut reg = CardRegistry::new();
    let mutavault = arcana_cards::clb::mutavault::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = GameState::new(2, 0);
    let vault = add(&mut s, &reg, 0, mutavault, Zone::Battlefield);
    let attacker = add(&mut s, &reg, 1, bears, Zone::Battlefield);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Red, 1, 0);
    s.turn.active_player = 1;
    s.begin_combat();
    s.apply_declared_attackers(vec![AttackerDeclaration { attacker, defending: DefendingEntity::Player(0) }]);
    window(&mut s, 1, 0, Phase::Combat, Step::DeclareAttackers);
    let mut s = activate(s, &reg, vault, 1);

    s.enter_declare_blockers();
    window(&mut s, 1, 0, Phase::Combat, Step::DeclareBlockers);
    let block = legal_actions(&s, &reg).into_iter().find(|a| matches!(a,
        Action::DeclareBlockers { blockers }
            if blockers.len() == 1 && blockers[0].blocker == vault && blockers[0].blocking == attacker))
        .expect("the animated Mutavault is offered as a blocker");
    // The direct validator the client uses agrees with the enumerator.
    let pair = [BlockerDeclaration { blocker: vault, blocking: attacker }];
    assert!(legal_block_declaration(&s, 0, &pair).is_some(), "the direct validator accepts the block");
    let (s, _) = step(s, block, &reg);
    let s = finish_combat(s, &reg);
    assert_eq!(s.player(0).life, 20, "the Bears were blocked");
    assert_ne!(zone_of(&s, vault), Some(Zone::Battlefield), "Mutavault died to lethal damage");
    assert_ne!(zone_of(&s, attacker), Some(Zone::Battlefield), "the Bears died to Mutavault");
}

/// Smuggler's Copter, crewed in its controller's main phase, is offered as
/// an attacker and deals its 3 damage.
#[test]
fn crewed_vehicle_attacks() {
    let mut reg = CardRegistry::new();
    let copter = arcana_cards::kld::smugglers_copter::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = GameState::new(2, 0);
    let vehicle = add(&mut s, &reg, 0, copter, Zone::Battlefield);
    add(&mut s, &reg, 0, bears, Zone::Battlefield);
    // The Copter's attack trigger may draw; keep the library from running dry.
    for _ in 0..2 {
        let id = add(&mut s, &reg, 0, bears, Zone::Library(0));
        s.player_mut(0).library_top_to_bottom.push(id);
    }
    window(&mut s, 0, 0, Phase::PreCombatMain, Step::Main);
    let mut s = activate(s, &reg, vehicle, 0);

    s.begin_combat();
    s.enter_declare_attackers();
    window(&mut s, 0, 0, Phase::Combat, Step::DeclareAttackers);
    let attack = legal_actions(&s, &reg).into_iter().find(|a| matches!(a,
        Action::DeclareAttackers { attackers }
            if attackers.len() == 1 && attackers[0].attacker == vehicle))
        .expect("the crewed Copter is offered as an attacker");
    let (s, _) = step(s, attack, &reg);
    let s = finish_combat(s, &reg);
    assert_eq!(s.player(1).life, 17, "the Copter dealt 3");
}

/// A Vehicle that entered the battlefield this turn is summoning sick once
/// crewed (CR 302.6): it can't attack, though its crew could.
#[test]
fn vehicle_that_entered_this_turn_cannot_attack_when_crewed() {
    let mut reg = CardRegistry::new();
    let copter = arcana_cards::kld::smugglers_copter::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = GameState::new(2, 0);
    let in_hand = add(&mut s, &reg, 0, copter, Zone::Hand(0));
    let vehicle = s.move_object_to_zone(in_hand, Zone::Battlefield, MoveCause::SpellResolution)
        .expect("the Copter enters");
    add(&mut s, &reg, 0, bears, Zone::Battlefield);
    window(&mut s, 0, 0, Phase::PreCombatMain, Step::Main);
    let mut s = activate(s, &reg, vehicle, 0);

    s.begin_combat();
    s.enter_declare_attackers();
    window(&mut s, 0, 0, Phase::Combat, Step::DeclareAttackers);
    let offered = legal_actions(&s, &reg).iter().any(|a| matches!(a,
        Action::DeclareAttackers { attackers } if attackers.iter().any(|d| d.attacker == vehicle)));
    assert!(!offered, "a Vehicle that entered this turn can't attack");
}

/// An attacking Mutavault whose animation ends is removed from combat
/// (CR 506.4) before anyone next gets priority.
#[test]
fn attacker_that_stops_being_a_creature_leaves_combat() {
    let mut reg = CardRegistry::new();
    let mutavault = arcana_cards::clb::mutavault::register(&mut reg);
    let mut s = GameState::new(2, 0);
    let vault = add(&mut s, &reg, 0, mutavault, Zone::Battlefield);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Red, 1, 0);
    window(&mut s, 0, 0, Phase::Combat, Step::BeginCombat);
    s.begin_combat();
    let mut s = activate(s, &reg, vault, 1);
    s.apply_declared_attackers(vec![AttackerDeclaration { attacker: vault, defending: DefendingEntity::Player(1) }]);
    assert!(s.combat.as_ref().unwrap().is_attacker(vault), "the animated Mutavault attacks");

    // The animation ends: its effects leave play.
    s.continuous_effects.retain(|e| e.source != vault);
    assert!(!s.is_creature_now(vault), "Mutavault is a land again");
    arcana_core::sba::apply_state_based_actions(&mut s);
    assert!(!s.combat.as_ref().unwrap().is_attacker(vault), "Mutavault left combat");
}
