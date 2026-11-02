//! A1.5ar and A1.5as: Stonecoil Serpent and Voracious Hydra enter with X
//! +1/+1 counters, read from the cast's X (`EntersWithSpec::CountersFromX`).
//! Before, an empty enters trigger stood in for the counters: the 0/0
//! Serpent died on entering, and the Hydra stayed a 0/1. The counters go
//! through the counter replacements, so a Winding Constrictor adds one.

use arcana_core::engine::step;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, CounterKind, ManaColor, PlayerId};
use arcana_core::zones::Zone;
use arcana_core::{Action, EngineYield, ObjectId};

fn add(s: &mut GameState, reg: &CardRegistry, owner: PlayerId, card: CardId, zone: Zone) -> ObjectId {
    let id = s.allocate_object_id();
    let chars = reg.get(card).unwrap().initial_characteristics().clone();
    s.objects.insert(GameObject::new(id, owner, zone, card, chars));
    id
}

/// Player 0's main phase with priority and `green` green mana.
fn table(green: u32) -> GameState {
    let mut s = GameState::new(2, 0);
    s.turn.active_player = 0;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(0);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Green, green, 0);
    s
}

/// Answer every decision by passing until the stack is empty.
fn settle(mut s: GameState, mut yld: EngineYield, reg: &CardRegistry) -> GameState {
    for _ in 0..100 {
        if s.stack_is_empty() && s.pending_choice.is_none() { return s; }
        let EngineYield::PendingDecision { legal_actions, .. } = &yld else { return s };
        let a = legal_actions.iter().find(|a| a.is_pass()).cloned()
            .or_else(|| legal_actions.iter().find(|a| !matches!(a, Action::Concede)).cloned())
            .expect("an action");
        (s, yld) = step(s, a, reg);
    }
    panic!("the stack did not settle");
}

/// Cast `card` from player 0's hand for X = `x` and let it resolve; return
/// the permanent it became, if it is on the battlefield.
fn cast_for_x(mut s: GameState, reg: &CardRegistry, card: ObjectId, x: u32) -> (GameState, Option<ObjectId>) {
    let card_id = s.objects.get(card).unwrap().card_id;
    let a = legal_actions(&s, reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, x_value, .. }
            if *object_id == card && *x_value == Some(x)))
        .unwrap_or_else(|| panic!("castable for X = {x}"));
    let y;
    (s, y) = step(s, a, reg);
    let s = settle(s, y, reg);
    let on = s.objects.objects_in_zone(Zone::Battlefield)
        .find(|o| o.card_id == card_id).map(|o| o.id);
    (s, on)
}

fn plus_ones(s: &GameState, id: ObjectId) -> u32 {
    s.objects.get(id).unwrap().count_counters(CounterKind::PlusOnePlusOne)
}

#[test]
fn stonecoil_serpent_enters_with_x_counters_and_survives() {
    let mut reg = CardRegistry::new();
    let serpent = arcana_cards::soc::stonecoil_serpent::register(&mut reg);
    let mut s = table(3);
    let card = add(&mut s, &reg, 0, serpent, Zone::Hand(0));
    let (s, on) = cast_for_x(s, &reg, card, 3);
    let on = on.expect("Stonecoil Serpent is on the battlefield after entering");
    assert_eq!(plus_ones(&s, on), 3);
    assert_eq!((arcana_core::script::power_of(&s, on), arcana_core::script::toughness_of(&s, on)), (3, 3));
}

#[test]
fn voracious_hydra_enters_with_x_counters() {
    let mut reg = CardRegistry::new();
    let hydra = arcana_cards::m20::voracious_hydra::register(&mut reg);
    let mut s = table(4);
    let card = add(&mut s, &reg, 0, hydra, Zone::Hand(0));
    let (s, on) = cast_for_x(s, &reg, card, 2);
    let on = on.expect("Voracious Hydra is on the battlefield");
    assert_eq!(plus_ones(&s, on), 2, "a 0/1 with two counters");
    assert_eq!((arcana_core::script::power_of(&s, on), arcana_core::script::toughness_of(&s, on)), (2, 3));
}

#[test]
fn a_winding_constrictor_adds_one_to_stonecoils_counters() {
    let mut reg = CardRegistry::new();
    let serpent = arcana_cards::soc::stonecoil_serpent::register(&mut reg);
    let constrictor = arcana_cards::aer::winding_constrictor::register(&mut reg);
    let mut s = table(3);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Black, 1, 0);
    let snake = add(&mut s, &reg, 0, constrictor, Zone::Hand(0));
    let (s, on) = cast_for_x_free(s, &reg, snake);
    assert!(on.is_some(), "Winding Constrictor resolved and its trigger installed");
    let mut s = s;
    let card = add(&mut s, &reg, 0, serpent, Zone::Hand(0));
    let (s, on) = cast_for_x(s, &reg, card, 2);
    let on = on.expect("Stonecoil Serpent is on the battlefield");
    assert_eq!(plus_ones(&s, on), 3, "X = 2 plus Constrictor's one");
}

/// Cast a card with no X and let it resolve.
fn cast_for_x_free(mut s: GameState, reg: &CardRegistry, card: ObjectId) -> (GameState, Option<ObjectId>) {
    let card_id = s.objects.get(card).unwrap().card_id;
    let a = legal_actions(&s, reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == card))
        .expect("castable");
    let y;
    (s, y) = step(s, a, reg);
    let s = settle(s, y, reg);
    let on = s.objects.objects_in_zone(Zone::Battlefield)
        .find(|o| o.card_id == card_id).map(|o| o.id);
    (s, on)
}
