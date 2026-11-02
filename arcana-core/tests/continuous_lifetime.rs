//! A1.5ax: a continuous effect bound to its source is never installed once
//! the source has left, as A1.5ab already holds for replacement effects.
//! The catalog models statics as effects an entry trigger installs, so a
//! Glorious Anthem destroyed in response to its own trigger used to leave
//! its anthem for the rest of the game, since its kind never consults the
//! source. Effects whose printed duration outlives the source are untouched.

use arcana_core::actions::Action;
use arcana_core::engine::step;
use arcana_core::events::MoveCause;
use arcana_core::layers::{ContinuousEffect, Duration};
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
    let chars = reg.get(card).unwrap().initial_characteristics().clone();
    s.objects.insert(GameObject::new(id, owner, zone, card, chars));
    id
}

fn pass(s: GameState, y: &EngineYield, reg: &CardRegistry) -> (GameState, EngineYield) {
    let EngineYield::PendingDecision { legal_actions, .. } = y else { panic!("a decision") };
    let a = legal_actions.iter().find(|a| a.is_pass()).cloned().expect("a pass");
    step(s, a, reg)
}

/// Cast Glorious Anthem and pass until its enters trigger is on the stack;
/// if `remove`, the Anthem leaves in response. Returns Grizzly Bears' power
/// once the stack is empty.
fn bears_power(remove: bool) -> i32 {
    let mut reg = CardRegistry::new();
    let anthem = arcana_cards::po2::glorious_anthem::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = GameState::new(2, 0);
    s.turn.active_player = 0;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(0);
    s.player_mut(0).mana_pool.add_mana(ManaColor::White, 3, 0);
    let b = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    let card = add(&mut s, &reg, 0, anthem, Zone::Hand(0));
    let a = legal_actions(&s, &reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == card)).expect("castable");
    let (mut s, mut y) = step(s, a, &reg);
    // Pass until the Anthem is on the battlefield with its trigger waiting.
    for _ in 0..10 {
        let on_bf = s.objects.objects_in_zone(Zone::Battlefield).any(|o| o.card_id == anthem);
        if on_bf && !s.stack_is_empty() { break; }
        (s, y) = pass(s, &y, &reg);
    }
    let on = s.objects.objects_in_zone(Zone::Battlefield).find(|o| o.card_id == anthem).map(|o| o.id)
        .expect("the Anthem resolved");
    assert!(!s.stack_is_empty(), "its enters trigger waits on the stack");
    if remove {
        s.move_object_to_zone(on, Zone::Graveyard(0), MoveCause::SpellResolution);
    }
    for _ in 0..10 {
        if s.stack_is_empty() { break; }
        (s, y) = pass(s, &y, &reg);
    }
    arcana_core::script::power_of(&s, b)
}

#[test]
fn glorious_anthem_pumps_while_it_stays() {
    assert_eq!(bears_power(false), 3);
}

#[test]
fn glorious_anthem_removed_in_response_installs_nothing() {
    assert_eq!(bears_power(true), 2);
}

/// The guard refuses only the source-bound lifetime: an end-of-turn effect
/// from a source that has gone still installs.
#[test]
fn an_effect_that_outlives_its_source_still_installs() {
    let mut s = GameState::new(2, 0);
    let before = s.continuous_effects.len();
    s.add_continuous_effect(ContinuousEffect::anthem(999, 0, 1, 1, Duration::EndOfTurn));
    assert_eq!(s.continuous_effects.len(), before + 1);
    s.add_continuous_effect(ContinuousEffect::anthem(999, 0, 1, 1, Duration::WhileSourceOnBattlefield));
    assert_eq!(s.continuous_effects.len(), before + 1, "a phantom source's static is refused");
}
