//! A1.5bg, completed: a token's own "when this dies" trigger fires. The
//! granted-ability scan read live objects only, and a dead token has left
//! its id, so the trigger never fired: Garruk, Cursed Huntsman's Wolf died
//! without putting a loyalty counter on him. The scan now reads the
//! departed source's last-known information, as printed triggers do.

use arcana_core::actions::Action;
use arcana_core::engine::step;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetSelection};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, CounterKind, ManaColor, PlayerId};
use arcana_core::zones::Zone;
use arcana_core::{EngineYield, ObjectId};

fn add(s: &mut GameState, reg: &CardRegistry, owner: PlayerId, card: CardId, zone: Zone) -> ObjectId {
    let id = s.allocate_object_id();
    let chars = reg.get(card).unwrap().initial_characteristics().clone();
    s.objects.insert(GameObject::new(id, owner, zone, card, chars));
    id
}

fn hits(sel: &TargetSelection, id: ObjectId) -> bool {
    sel.targets.iter().any(|t| matches!(t, TargetChoice::Object(o)
        | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(o)) if *o == id))
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

#[test]
fn garruks_wolf_dying_adds_a_loyalty_counter() {
    let mut reg = CardRegistry::new();
    let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
    let garruk = arcana_cards::eld::garruk_cursed_huntsman::register(&mut reg);
    let mut s = GameState::new(2, 0);
    s.turn.active_player = 0;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(0);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Red, 1, 0);
    let g = add(&mut s, &reg, 0, garruk, Zone::Battlefield);
    s.objects.get_mut(g).unwrap().add_counters(CounterKind::Loyalty, 5);
    let b = add(&mut s, &reg, 0, bolt, Zone::Hand(0));
    // Garruk's 0: two Wolves, each with "When this creature dies, put a
    // loyalty counter on each Garruk you control."
    let zero = legal_actions(&s, &reg).into_iter()
        .find(|a| matches!(a, Action::ActivateAbility { source, ability_index: 0, .. } if *source == g))
        .expect("Garruk's 0 ability");
    let (s, y) = step(s, zero, &reg);
    let s = settle(s, y, &reg);
    let loyalty = |s: &GameState| s.objects.get(g).unwrap().count_counters(CounterKind::Loyalty);
    assert_eq!(loyalty(&s), 5);
    let wolf = s.objects.objects_in_zone(Zone::Battlefield).find(|o| o.is_token).map(|o| o.id)
        .expect("a Wolf");
    let a = legal_actions(&s, &reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, targets, .. } if *object_id == b && hits(targets, wolf)))
        .expect("Bolt the Wolf");
    let (s, y) = step(s, a, &reg);
    let s = settle(s, y, &reg);
    assert!(s.objects.get(wolf).is_none(), "the Wolf died");
    assert_eq!(loyalty(&s), 6, "the Wolf's dies trigger put a loyalty counter on Garruk");
}
