//! A1.5bd: an activated ability's life cost is paid once. Before, the
//! engine deducted the cost itself and then paid the action's declared
//! `PayLife` again, so each Mana Confluence tap cost 2 life.

use arcana_core::actions::Action;
use arcana_core::engine::step;
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

/// Every offered activation of `source`, applied to a copy of `s`, costs
/// exactly `life` life.
fn each_activation_costs(s: &GameState, reg: &CardRegistry, source: ObjectId, life: i32) -> usize {
    let acts: Vec<Action> = legal_actions(s, reg).into_iter()
        .filter(|a| matches!(a, Action::ActivateAbility { source: x, .. } if *x == source))
        .collect();
    for a in &acts {
        let before = s.player(0).life;
        let (after, _) = step(s.clone(), a.clone(), reg);
        assert_eq!(before - after.player(0).life, life, "one activation of {source} costs {life} life");
    }
    acts.len()
}

#[test]
fn mana_confluence_costs_one_life_a_tap() {
    let mut reg = CardRegistry::new();
    let conf = arcana_cards::jou::mana_confluence::register(&mut reg);
    let mut s = table();
    let c = add(&mut s, &reg, 0, conf, Zone::Battlefield);
    assert_eq!(each_activation_costs(&s, &reg, c, 1), 5, "one activation per color");
}

#[test]
fn ramunap_ruins_red_costs_one_life() {
    let mut reg = CardRegistry::new();
    let ruins = arcana_cards::hou::ramunap_ruins::register(&mut reg);
    let mut s = table();
    let r = add(&mut s, &reg, 0, ruins, Zone::Battlefield);
    // The colorless ability is free and the red one costs 1 life; check
    // the red one by its mana.
    let acts: Vec<Action> = legal_actions(&s, &reg).into_iter()
        .filter(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == r))
        .collect();
    let lives: Vec<i32> = acts.iter().map(|a| 20 - step(s.clone(), a.clone(), &reg).0.player(0).life).collect();
    assert!(lives.contains(&1), "the red ability costs 1 life: {lives:?}");
    assert!(lives.iter().all(|&l| l <= 1), "no activation costs more than 1 life: {lives:?}");
}

/// A fetchland pays 1 life and sacrifices itself once, before its search
/// resolves.
#[test]
fn flooded_strand_pays_one_life_and_leaves() {
    let mut reg = CardRegistry::new();
    let strand = arcana_cards::mh3::flooded_strand::register(&mut reg);
    let island = arcana_cards::lea::island::register(&mut reg);
    let mut s = table();
    add(&mut s, &reg, 0, island, Zone::Library(0));
    let f = add(&mut s, &reg, 0, strand, Zone::Battlefield);
    let a = legal_actions(&s, &reg).into_iter()
        .find(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == f))
        .expect("Flooded Strand can be activated");
    let (after, _) = step(s, a, &reg);
    assert_eq!(after.player(0).life, 19);
    assert!(after.objects.get(f).is_none(), "the Strand was sacrificed");
    assert!(after.objects.iter().any(|o| o.card_id == strand && o.zone == Zone::Graveyard(0)));
}
