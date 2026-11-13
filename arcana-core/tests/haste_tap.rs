//! A1.5bi: haste lets a creature use its `{T}` abilities the turn it comes
//! under its controller's control (CR 302.6, 702.10b). Before, every tap
//! ability of a summoning-sick creature was refused, haste or not.

use arcana_core::actions::Action;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, PlayerId};
use arcana_core::zones::Zone;
use arcana_core::ObjectId;

fn add(s: &mut GameState, reg: &CardRegistry, owner: PlayerId, card: CardId, zone: Zone) -> ObjectId {
    let id = s.allocate_object_id();
    let chars = reg.get(card).unwrap().initial_characteristics().clone();
    s.objects.insert(GameObject::new(id, owner, zone, card, chars));
    id
}

fn activations(card: fn(&mut CardRegistry) -> CardId, sick: bool) -> usize {
    let mut reg = CardRegistry::new();
    let id = card(&mut reg);
    let mut s = GameState::new(2, 0);
    s.turn.active_player = 0;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(0);
    let o = add(&mut s, &reg, 0, id, Zone::Battlefield);
    s.objects.get_mut(o).unwrap().status.summoning_sick = sick;
    legal_actions(&s, &reg).into_iter()
        .filter(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == o))
        .count()
}

/// Fanatical Firebrand has haste and "{T}, Sacrifice: 1 damage to any
/// target": the same activations sick or not.
#[test]
fn a_hasty_creature_taps_the_turn_it_arrives() {
    let ready = activations(arcana_cards::fdn::fanatical_firebrand::register, false);
    assert!(ready > 0);
    assert_eq!(activations(arcana_cards::fdn::fanatical_firebrand::register, true), ready);
}

/// Llanowar Elves has no haste: its `{T}` mana ability waits a turn.
#[test]
fn a_creature_without_haste_still_waits() {
    assert!(activations(arcana_cards::lea::llanowar_elves::register, false) > 0);
    assert_eq!(activations(arcana_cards::lea::llanowar_elves::register, true), 0);
}
