//! A1.5bg: a token's triggered ability that targets gets its target. A
//! token's printed triggers live on the object, in the granted range, and
//! the engine looked target requirements up only in the registry, so the
//! ability went on the stack untargeted and did nothing. Rekindling
//! Phoenix's Elemental never brought the Phoenix back.
//!
//! Seat 0 only: the card file's own seat defect is A1.5n, a separate item.

use arcana_core::actions::Action;
use arcana_core::engine::step;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetSelection};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, ManaColor, PlayerId};
use arcana_core::zones::Zone;
use arcana_core::ObjectId;

fn add(s: &mut GameState, reg: &CardRegistry, owner: PlayerId, card: CardId, zone: Zone) -> ObjectId {
    let id = s.allocate_object_id();
    let chars = reg.get(card).unwrap().initial_characteristics().clone();
    s.objects.insert(GameObject::new(id, owner, zone, card, chars));
    if let Zone::Library(p) = zone { s.player_mut(p).library_top_to_bottom.push(id); }
    id
}

fn hits(sel: &TargetSelection, id: ObjectId) -> bool {
    sel.targets.iter().any(|t| matches!(t, TargetChoice::Object(o)
        | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(o)) if *o == id))
}

fn pick(acts: &[Action]) -> Action {
    acts.iter().find(|a| a.is_pass()).cloned()
        .or_else(|| acts.iter().find(|a| matches!(a, Action::DeclareAttackers { attackers } if attackers.is_empty())).cloned())
        .or_else(|| acts.iter().find(|a| matches!(a, Action::DeclareBlockers { blockers } if blockers.is_empty())).cloned())
        .or_else(|| acts.iter().find(|a| !matches!(a, Action::Concede)).cloned())
        .expect("an action")
}

#[test]
fn rekindling_phoenix_returns_at_its_controllers_next_upkeep() {
    let mut reg = CardRegistry::new();
    let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
    let phoenix = arcana_cards::ncc::rekindling_phoenix::register(&mut reg);
    let mut s = GameState::new(2, 0);
    s.turn.active_player = 0;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(0);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Red, 1, 0);
    for p in [0, 1] { for _ in 0..6 { add(&mut s, &reg, p, bolt, Zone::Library(p)); } }
    let ph = add(&mut s, &reg, 0, phoenix, Zone::Battlefield);
    let b = add(&mut s, &reg, 0, bolt, Zone::Hand(0));
    let a = legal_actions(&s, &reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, targets, .. } if *object_id == b && hits(targets, ph)))
        .expect("Bolt the Phoenix");
    (s, _) = step(s, a, &reg);
    let start = s.turn.turn_number;
    for _ in 0..400 {
        if s.turn.active_player == 0 && s.turn.turn_number > start
            && !matches!(s.turn.step, Step::Untap | Step::Upkeep) && s.stack_is_empty() { break; }
        let acts = legal_actions(&s, &reg);
        (s, _) = step(s, pick(&acts), &reg);
    }
    assert!(s.turn.turn_number > start && s.turn.active_player == 0, "reached player 0's next turn");
    let back: Vec<_> = s.objects.objects_in_zone(Zone::Battlefield)
        .filter(|o| o.card_id == phoenix).collect();
    assert_eq!(back.len(), 1, "the Phoenix is back on the battlefield");
    assert!(!s.objects.objects_in_zone(Zone::Battlefield).any(|o| o.is_token),
        "the Elemental sacrificed itself");
}
