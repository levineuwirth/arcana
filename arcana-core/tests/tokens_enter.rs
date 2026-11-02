//! A1.5bf: a created token enters the battlefield (CR 603.6a), so "whenever
//! a creature enters" triggers written on a zone change see it. Before, a
//! token emitted no zone change and those triggers missed every token.

use arcana_core::actions::Action;
use arcana_core::engine::step;
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

fn settle(mut s: GameState, mut y: EngineYield, reg: &CardRegistry) -> GameState {
    for _ in 0..100 {
        if s.stack_is_empty() && s.pending_choice.is_none() { return s; }
        let EngineYield::PendingDecision { legal_actions, .. } = &y else { return s };
        let a = legal_actions.iter().find(|a| a.is_pass()).cloned()
            .or_else(|| legal_actions.iter().find(|a| !matches!(a, Action::Concede)).cloned())
            .expect("an action");
        (s, y) = step(s, a, reg);
    }
    panic!("the stack did not settle");
}

fn cast(s: GameState, reg: &CardRegistry, card: ObjectId) -> GameState {
    let a = legal_actions(&s, reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == card))
        .expect("castable");
    let (s, y) = step(s, a, reg);
    settle(s, y, reg)
}

/// Rampaging Ferocidon, player 1's: "Whenever another creature enters the
/// battlefield, Rampaging Ferocidon deals 1 damage to that creature's
/// controller." Grizzly Bears entering costs player 0 one life, as
/// before; Dragon Fodder's two Goblin tokens now cost two more.
#[test]
fn rampaging_ferocidon_sees_created_tokens_enter() {
    let mut reg = CardRegistry::new();
    let fer = arcana_cards::dsc::rampaging_ferocidon::register(&mut reg);
    let fodder = arcana_cards::fdn::dragon_fodder::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = GameState::new(2, 0);
    s.turn.active_player = 0;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(0);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Red, 3, 0);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Green, 3, 0);
    add(&mut s, &reg, 1, fer, Zone::Battlefield);
    let g = add(&mut s, &reg, 0, bears, Zone::Hand(0));
    let d = add(&mut s, &reg, 0, fodder, Zone::Hand(0));
    let s = cast(s, &reg, g);
    assert_eq!(s.player(0).life, 19, "one trigger for a cast creature, not two");
    let s = cast(s, &reg, d);
    let goblins = s.objects.objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == 0 && o.is_token).count();
    assert_eq!(goblins, 2);
    assert_eq!(s.player(0).life, 17, "one trigger for each Goblin token");
}
