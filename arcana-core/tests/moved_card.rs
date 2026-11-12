//! A1.5bh with A1.5az: an ability that triggers on its own source moving
//! to a public zone finds the object the card became there (CR 400.7e).
//! Before, a dies trigger's effect on its card, and a return it scheduled,
//! acted on the battlefield id the move had retired and did nothing:
//! Murderous Rider stayed in the graveyard, and The Scarab God never came
//! back. The link is scoped to that resolution and that first destination:
//! a card that leaves the graveyard first is a new object, and a return
//! scheduled before the death doesn't follow it.

use arcana_core::actions::Action;
use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::engine::step;
use arcana_core::events::MoveCause;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetSelection};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, ManaColor, PlayerId};
use arcana_core::zones::Zone;
use arcana_core::{EngineYield, ObjectId};

fn add(s: &mut GameState, reg: &CardRegistry, owner: PlayerId, card: CardId, zone: Zone) -> ObjectId {
    let id = s.allocate_object_id();
    let chars = reg.get(card).unwrap().initial_characteristics().clone();
    s.objects.insert(GameObject::new(id, owner, zone, card, chars));
    if let Zone::Library(p) = zone { s.player_mut(p).library_top_to_bottom.push(id); }
    id
}

fn table(active: PlayerId) -> GameState {
    let mut s = GameState::new(2, 0);
    s.turn.active_player = active;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(active);
    for p in [0, 1] {
        for color in [ManaColor::Black, ManaColor::Red] { s.player_mut(p).mana_pool.add_mana(color, 3, 0); }
    }
    s
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

fn settle(mut s: GameState, mut y: EngineYield, reg: &CardRegistry) -> GameState {
    for _ in 0..100 {
        if s.stack_is_empty() && s.pending_choice.is_none() { return s; }
        let EngineYield::PendingDecision { legal_actions, .. } = &y else { return s };
        let a = pick(legal_actions);
        (s, y) = step(s, a, reg);
    }
    panic!("the stack did not settle");
}

fn cast_at(s: GameState, reg: &CardRegistry, card: ObjectId, target: ObjectId) -> GameState {
    let a = legal_actions(&s, reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, targets, .. } if *object_id == card && hits(targets, target)))
        .expect("castable at the target");
    let (s, y) = step(s, a, reg);
    settle(s, y, reg)
}

/// Pass through player 0's turn until its end step is over.
fn to_end_of_turn(mut s: GameState, reg: &CardRegistry) -> GameState {
    let turn = s.turn.turn_number;
    for _ in 0..80 {
        if s.turn.turn_number != turn || (s.turn.step == Step::Cleanup && s.stack_is_empty()) { break; }
        let acts = legal_actions(&s, reg);
        (s, _) = step(s, pick(&acts), reg);
    }
    s
}

fn zones_of(s: &GameState, card: CardId) -> Vec<Zone> {
    s.objects.iter().filter(|o| o.card_id == card).map(|o| o.zone).collect()
}

#[test]
fn murderous_rider_goes_to_the_bottom_of_its_owners_library() {
    let mut reg = CardRegistry::new();
    let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
    let rider = arcana_cards::moc::murderous_rider_swift_end::register(&mut reg);
    let mut s = table(1);
    for _ in 0..2 { add(&mut s, &reg, 0, bolt, Zone::Library(0)); }
    let r = add(&mut s, &reg, 0, rider, Zone::Battlefield);
    let b = add(&mut s, &reg, 1, bolt, Zone::Hand(1));
    let s = cast_at(s, &reg, b, r);
    assert_eq!(zones_of(&s, rider), vec![Zone::Library(0)]);
    let bottom = *s.player(0).library_top_to_bottom.last().unwrap();
    assert_eq!(s.objects.get(bottom).unwrap().card_id, rider, "on the bottom");
}

fn murdered_scarab_god(exile_first: bool) -> (GameState, CardId) {
    let mut reg = CardRegistry::new();
    let murder = arcana_cards::isd::murder::register(&mut reg);
    let god = arcana_cards::cmm::the_scarab_god::register(&mut reg);
    let mut s = table(0);
    let g = add(&mut s, &reg, 0, god, Zone::Battlefield);
    let m = add(&mut s, &reg, 0, murder, Zone::Hand(0));
    let mut s = cast_at(s, &reg, m, g);
    assert_eq!(zones_of(&s, god), vec![Zone::Graveyard(0)]);
    if exile_first {
        let in_gy = s.objects.iter().find(|o| o.card_id == god).map(|o| o.id).unwrap();
        s.move_object_to_zone(in_gy, Zone::Exile, MoveCause::SpellResolution);
    }
    (to_end_of_turn(s, &reg), god)
}

#[test]
fn the_scarab_god_returns_to_hand_at_the_next_end_step() {
    let (s, god) = murdered_scarab_god(false);
    assert_eq!(zones_of(&s, god), vec![Zone::Hand(0)]);
}

#[test]
fn the_scarab_god_exiled_first_stays_exiled() {
    let (s, god) = murdered_scarab_god(true);
    assert_eq!(zones_of(&s, god), vec![Zone::Exile], "a new object; the return does not find it");
}

/// A return scheduled before the death refers to the creature on the
/// battlefield and does not follow it into the graveyard.
#[test]
fn an_earlier_end_step_return_does_not_follow_a_death() {
    let mut reg = CardRegistry::new();
    let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = table(0);
    let b = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    Effect::DelayedAction { source: b, controller: 0, when: DelayedWhen::NextEndStep,
        action: DelayedAction::ReturnToHand }.execute(&mut s);
    let bolt_id = add(&mut s, &reg, 0, bolt, Zone::Hand(0));
    let s = cast_at(s, &reg, bolt_id, b);
    assert_eq!(zones_of(&s, bears), vec![Zone::Graveyard(0)]);
    let s = to_end_of_turn(s, &reg);
    assert_eq!(zones_of(&s, bears), vec![Zone::Graveyard(0)]);
}

// --- the review's findings -------------------------------------------------

/// Tenacious Dead: "When this creature dies, you may pay {1}{B}. If you
/// do, return it to the battlefield tapped." The return sits inside the
/// optional payment, and the link reaches it.
#[test]
fn tenacious_dead_returns_when_its_owner_pays() {
    let mut reg = CardRegistry::new();
    let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
    let dead = arcana_cards::bbd::tenacious_dead::register(&mut reg);
    let mut s = table(1);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Colorless, 1, 0);
    let d = add(&mut s, &reg, 0, dead, Zone::Battlefield);
    let b = add(&mut s, &reg, 1, bolt, Zone::Hand(1));
    // `pick` answers the payment prompt with its first non-pass action,
    // which is to pay when payment is offered.
    let s = cast_at(s, &reg, b, d);
    assert_eq!(zones_of(&s, dead), vec![Zone::Battlefield], "paid for and returned");
}

/// Undying Malice grants "When this creature dies, return it to the
/// battlefield tapped under its owner's control with a +1/+1 counter on
/// it" — a return with counters, which the link follows.
#[test]
fn undying_malice_returns_the_creature_with_a_counter() {
    let mut reg = CardRegistry::new();
    let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
    let malice = arcana_cards::fdn::undying_malice::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = table(0);
    let g = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    let m = add(&mut s, &reg, 0, malice, Zone::Hand(0));
    let s = cast_at(s, &reg, m, g);
    let b = { let mut s2 = s; let id = add(&mut s2, &reg, 0, bolt, Zone::Hand(0)); (s2, id) };
    let (s, bolt_id) = b;
    let s = cast_at(s, &reg, bolt_id, g);
    let back: Vec<_> = s.objects.objects_in_zone(Zone::Battlefield).filter(|o| o.card_id == bears).collect();
    assert_eq!(back.len(), 1, "Grizzly Bears returned");
    assert_eq!(back[0].count_counters(arcana_core::types::CounterKind::PlusOnePlusOne), 1);
}

/// The Scarab God's end-step return keeps the dies trigger's source as its
/// own (CR 603.7e) and carries the graveyard card apart from it.
#[test]
fn the_scarab_gods_return_keeps_its_source_and_carries_the_card() {
    let mut reg = CardRegistry::new();
    let murder = arcana_cards::isd::murder::register(&mut reg);
    let god = arcana_cards::cmm::the_scarab_god::register(&mut reg);
    let mut s = table(0);
    let g = add(&mut s, &reg, 0, god, Zone::Battlefield);
    let m = add(&mut s, &reg, 0, murder, Zone::Hand(0));
    let s = cast_at(s, &reg, m, g);
    let in_gy = s.objects.iter().find(|o| o.card_id == god).map(|o| o.id).unwrap();
    let returns: Vec<_> = s.delayed_triggers.iter().filter(|t| t.affected.is_some()).collect();
    assert_eq!(returns.len(), 1);
    assert_eq!((returns[0].source, returns[0].affected), (g, Some(in_gy)),
        "source kept, the graveyard card carried apart");
}

/// A dies trigger whose return loops over its own card by placeholder:
/// each iteration is specialized to the old id before the link, so the
/// return finds the card in the graveyard.
#[test]
fn a_looped_return_over_the_dying_card_finds_it() {
    use arcana_core::triggers::{GrantedTrigger, PendingTrigger, TriggerCondition, TriggerFrequency,
        TriggeredAbilityDef, GRANTED_TRIGGER_ID_BASE};
    fn loop_return(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
        vec![Effect::ForEach { targets: vec![trig.source],
            effect: Box::new(Effect::ReturnFromGraveyardToHand { target: arcana_core::objects::NULL_OBJECT_ID }) }]
    }
    let mut reg = CardRegistry::new();
    let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = table(0);
    let g = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    s.objects.get_mut(g).unwrap().granted_triggered_abilities.push(GrantedTrigger {
        def: TriggeredAbilityDef {
            id: GRANTED_TRIGGER_ID_BASE + 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: loop_return,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        },
        duration: arcana_core::layers::Duration::EndOfTurn,
    });
    let b = add(&mut s, &reg, 0, bolt, Zone::Hand(0));
    let s = cast_at(s, &reg, b, g);
    assert_eq!(zones_of(&s, bears), vec![Zone::Hand(0)]);
}
