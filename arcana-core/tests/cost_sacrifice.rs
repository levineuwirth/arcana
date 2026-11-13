//! A1.5be: a sacrifice paid as a cost is a sacrifice, so "whenever you
//! sacrifice" watchers see it. Before, only sacrifices made by effects and
//! state-based actions emitted the event, so cracking a Clue never grew a
//! Tireless Tracker, and Gixian Infiltrator never grew from a Witch's Oven
//! activation.

use arcana_core::actions::Action;
use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::engine::step;
use arcana_core::events::GameEvent;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, CounterKind, ManaColor, PlayerId};
use arcana_core::zones::Zone;
use arcana_core::{EngineYield, ObjectId};

fn add(s: &mut GameState, reg: &CardRegistry, owner: PlayerId, card: CardId, zone: Zone) -> ObjectId {
    let id = s.allocate_object_id();
    let chars = reg.get(card).unwrap().initial_characteristics().clone();
    s.objects.insert(GameObject::new(id, owner, zone, card, chars));
    if let Zone::Library(p) = zone { s.player_mut(p).library_top_to_bottom.push(id); }
    id
}

fn table() -> GameState {
    let mut s = GameState::new(2, 0);
    s.turn.active_player = 0;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(0);
    s
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

fn sacrifices(s: &GameState, id: ObjectId) -> usize {
    s.event_log.iter().filter(|e| matches!(e, GameEvent::Sacrifice { object_id, .. } if *object_id == id)).count()
}

#[test]
fn cracking_a_clue_grows_tireless_tracker() {
    let mut reg = CardRegistry::new();
    let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
    let tracker = arcana_cards::eoc::tireless_tracker::register(&mut reg);
    let mut s = table();
    s.player_mut(0).mana_pool.add_mana(ManaColor::Colorless, 2, 0);
    for _ in 0..3 { add(&mut s, &reg, 0, bolt, Zone::Library(0)); }
    let tr = add(&mut s, &reg, 0, tracker, Zone::Battlefield);
    Effect::CreateCommodityToken { controller: 0, kind: CommodityToken::Clue, count: 1 }.execute(&mut s);
    let clue = s.objects.objects_in_zone(Zone::Battlefield).find(|o| o.id != tr).map(|o| o.id).expect("a Clue");
    let a = legal_actions(&s, &reg).into_iter()
        .find(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == clue))
        .expect("the Clue can be cracked");
    let (s, y) = step(s, a, &reg);
    assert_eq!(sacrifices(&s, clue), 1, "one sacrifice event for the Clue");
    let s = settle(s, y, &reg);
    assert_eq!(s.objects.objects_in_zone(Zone::Hand(0)).count(), 1, "the Clue drew a card");
    assert_eq!(s.objects.get(tr).unwrap().count_counters(CounterKind::PlusOnePlusOne), 1,
        "Tireless Tracker saw the Clue sacrificed");
}

#[test]
fn gixian_infiltrator_sees_a_creature_sacrificed_to_witchs_oven() {
    let mut reg = CardRegistry::new();
    let gixian = arcana_cards::bro::gixian_infiltrator::register(&mut reg);
    let oven = arcana_cards::eld::witch_s_oven::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = table();
    let g = add(&mut s, &reg, 0, gixian, Zone::Battlefield);
    let o = add(&mut s, &reg, 0, oven, Zone::Battlefield);
    let b = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    let a = legal_actions(&s, &reg).into_iter()
        .find(|a| matches!(a, Action::ActivateAbility { source, additional_costs, .. } if *source == o
            && additional_costs.iter().any(|c| matches!(c, arcana_core::actions::AdditionalCostPayment::Sacrifice(x) if *x == b))))
        .expect("Witch's Oven can sacrifice the Bears");
    let (s, y) = step(s, a, &reg);
    assert_eq!(sacrifices(&s, b), 1, "one sacrifice event for the Bears");
    let s = settle(s, y, &reg);
    assert_eq!(s.objects.get(g).unwrap().count_counters(CounterKind::PlusOnePlusOne), 1,
        "Gixian Infiltrator saw the Bears sacrificed");
}
