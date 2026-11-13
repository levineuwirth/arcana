//! A1.5bk: death replacements apply to every death, and regeneration to
//! destruction only. Before, only the state-based check consulted them,
//! so a regeneration shield or an "exile it instead" did nothing against
//! a destroy effect or a sacrifice, while a shield could be spent on a
//! creature at zero toughness, which is put into the graveyard without
//! being destroyed (CR 701.8, 704.5f-h).
//!
//! Each case checks where the creature ends up, the dies events, and the
//! shield. Murder destroys, Lightning Bolt deals lethal damage, a -2/-2
//! effect brings toughness to zero, Witch's Oven sacrifices as a cost and
//! Plaguecrafter sacrifices by effect.

use arcana_core::actions::{Action, ChoiceResponse};
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::engine::step;
use arcana_core::events::GameEvent;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::replacement::{
    ReplacementCondition, ReplacementDuration, ReplacementEffect, ReplacementKind,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetFilter, TargetSelection};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, ManaColor, PlayerId};
use arcana_core::zones::Zone;
use arcana_core::{EngineYield, ObjectId};

struct Cards { bears: CardId, murder: CardId, bolt: CardId, oven: CardId, plaguecrafter: CardId }

fn registry() -> (CardRegistry, Cards) {
    let mut reg = CardRegistry::new();
    let c = Cards {
        bears: arcana_cards::lea::grizzly_bears::register(&mut reg),
        murder: arcana_cards::isd::murder::register(&mut reg),
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        oven: arcana_cards::eld::witch_s_oven::register(&mut reg),
        plaguecrafter: arcana_cards::rvr::plaguecrafter::register(&mut reg),
    };
    (reg, c)
}

fn add(s: &mut GameState, reg: &CardRegistry, owner: PlayerId, card: CardId, zone: Zone) -> ObjectId {
    let id = s.allocate_object_id();
    let chars = reg.get(card).unwrap().initial_characteristics().clone();
    s.objects.insert(GameObject::new(id, owner, zone, card, chars));
    id
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Guard { Shield, ExileInstead }

/// Player 0's main phase with priority, plenty of mana, and Grizzly Bears
/// under `guard`.
fn table(reg: &CardRegistry, c: &Cards, guard: Guard) -> (GameState, ObjectId) {
    let mut s = GameState::new(2, 0);
    s.turn.active_player = 0;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(0);
    for color in [ManaColor::Black, ManaColor::Red] { s.player_mut(0).mana_pool.add_mana(color, 4, 0); }
    let b = add(&mut s, reg, 0, c.bears, Zone::Battlefield);
    match guard {
        Guard::Shield => Effect::Regenerate { target: b }.execute(&mut s),
        Guard::ExileInstead => s.add_replacement_effect(ReplacementEffect {
            source: b, id: 0,
            condition: ReplacementCondition::WouldDieSpecific { object_id: b },
            kind: ReplacementKind::ExileInsteadOfDying,
            is_self_replacement: false,
            duration: ReplacementDuration::EndOfTurn,
            state_gate: None,
        }),
    }
    (s, b)
}

fn shields(s: &GameState, id: ObjectId) -> usize {
    s.replacement_effects.iter().filter(|e| matches!(e.kind, ReplacementKind::RegenerateShield)
        && matches!(e.condition, ReplacementCondition::WouldDieSpecific { object_id } if object_id == id)).count()
}

fn dies(s: &GameState, id: ObjectId) -> usize {
    s.event_log.iter().filter(|e| matches!(e, GameEvent::Dies { object_id } if *object_id == id)).count()
}

fn bears_in(s: &GameState, c: &Cards) -> Vec<Zone> {
    s.objects.iter().filter(|o| o.card_id == c.bears).map(|o| o.zone).collect()
}

fn hits(sel: &TargetSelection, id: ObjectId) -> bool {
    sel.targets.iter().any(|t| matches!(t, TargetChoice::Object(o)
        | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(o)) if *o == id))
}

/// Pass until the stack is empty; a pick of cards takes Grizzly Bears.
fn settle(mut s: GameState, mut y: EngineYield, reg: &CardRegistry, bears: ObjectId) -> GameState {
    for _ in 0..100 {
        if s.stack_is_empty() && s.pending_choice.is_none() { return s; }
        let EngineYield::PendingDecision { legal_actions, .. } = &y else { return s };
        let a = legal_actions.iter().find(|a| matches!(a, Action::SubmitResolutionChoice {
                response: ChoiceResponse::PickCards { picked }, .. } if picked.contains(&bears))).cloned()
            .or_else(|| legal_actions.iter().find(|a| a.is_pass()).cloned())
            .or_else(|| legal_actions.iter().find(|a| !matches!(a, Action::Concede)).cloned())
            .expect("an action");
        (s, y) = step(s, a, reg);
    }
    panic!("the stack did not settle");
}

fn cast_at(s: GameState, reg: &CardRegistry, card: ObjectId, target: ObjectId) -> GameState {
    let a = legal_actions(&s, reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, targets, .. } if *object_id == card && hits(targets, target)))
        .expect("castable at the target");
    let (s, y) = step(s, a, reg);
    settle(s, y, reg, target)
}

// --- destruction -----------------------------------------------------------

#[test]
fn murder_is_regenerated_and_spends_the_shield() {
    let (reg, c) = registry();
    let (mut s, b) = table(&reg, &c, Guard::Shield);
    let m = add(&mut s, &reg, 0, c.murder, Zone::Hand(0));
    let s = cast_at(s, &reg, m, b);
    let o = s.objects.get(b).expect("Grizzly Bears regenerated");
    assert!(o.zone.is_battlefield() && o.is_tapped());
    assert_eq!((shields(&s, b), dies(&s, b)), (0, 0));
}

#[test]
fn murder_is_exiled_instead() {
    let (reg, c) = registry();
    let (mut s, b) = table(&reg, &c, Guard::ExileInstead);
    let m = add(&mut s, &reg, 0, c.murder, Zone::Hand(0));
    let s = cast_at(s, &reg, m, b);
    assert_eq!(bears_in(&s, &c), vec![Zone::Exile]);
    assert_eq!(dies(&s, b), 0);
}

#[test]
fn indestructible_ignores_murder_and_keeps_the_shield() {
    let (reg, c) = registry();
    let (mut s, b) = table(&reg, &c, Guard::Shield);
    s.objects.get_mut(b).unwrap().characteristics.keywords.push(KeywordAbility::Indestructible);
    let m = add(&mut s, &reg, 0, c.murder, Zone::Hand(0));
    let s = cast_at(s, &reg, m, b);
    let o = s.objects.get(b).expect("still here");
    assert!(!o.is_tapped());
    assert_eq!(shields(&s, b), 1);
}

#[test]
fn lethal_damage_is_regenerated() {
    let (reg, c) = registry();
    let (mut s, b) = table(&reg, &c, Guard::Shield);
    let bolt = add(&mut s, &reg, 0, c.bolt, Zone::Hand(0));
    let s = cast_at(s, &reg, bolt, b);
    let o = s.objects.get(b).expect("Grizzly Bears regenerated");
    assert!(o.is_tapped() && o.damage_marked == 0);
    assert_eq!((shields(&s, b), dies(&s, b)), (0, 0));
}

/// A damage prevention ahead of the shield: no damage, no destruction,
/// and the shield is left for later.
#[test]
fn prevented_damage_keeps_the_shield() {
    let (reg, c) = registry();
    let (mut s, b) = table(&reg, &c, Guard::Shield);
    s.add_replacement_effect(ReplacementEffect {
        source: b, id: 0,
        condition: ReplacementCondition::WouldDealDamage {
            source_filter: Default::default(),
            target_filter: TargetFilter::AnyTarget,
            combat: None,
        },
        kind: ReplacementKind::PreventAllDamage,
        is_self_replacement: false,
        duration: ReplacementDuration::EndOfTurn,
        state_gate: None,
    });
    let bolt = add(&mut s, &reg, 0, c.bolt, Zone::Hand(0));
    let s = cast_at(s, &reg, bolt, b);
    let o = s.objects.get(b).expect("still here");
    assert!(!o.is_tapped() && o.damage_marked == 0);
    assert_eq!(shields(&s, b), 1);
}

// --- zero toughness --------------------------------------------------------

fn shrink_to_zero(mut s: GameState, b: ObjectId) -> GameState {
    s.add_continuous_effect(ContinuousEffect::pump(b, b, -2, -2, Duration::EndOfTurn));
    arcana_core::sba::apply_state_based_actions(&mut s);
    s
}

#[test]
fn zero_toughness_passes_the_shield_by() {
    let (reg, c) = registry();
    let (s, b) = table(&reg, &c, Guard::Shield);
    let s = shrink_to_zero(s, b);
    assert_eq!(bears_in(&s, &c), vec![Zone::Graveyard(0)]);
    assert_eq!(dies(&s, b), 1);
    assert_eq!(shields(&s, b), 1, "the shield was not spent");
}

#[test]
fn zero_toughness_is_exiled_instead() {
    let (reg, c) = registry();
    let (s, b) = table(&reg, &c, Guard::ExileInstead);
    let s = shrink_to_zero(s, b);
    assert_eq!(bears_in(&s, &c), vec![Zone::Exile]);
    assert_eq!(dies(&s, b), 0);
}

#[test]
fn indestructible_does_not_stop_zero_toughness() {
    let (reg, c) = registry();
    let (mut s, b) = table(&reg, &c, Guard::Shield);
    s.objects.get_mut(b).unwrap().characteristics.keywords.push(KeywordAbility::Indestructible);
    let s = shrink_to_zero(s, b);
    assert_eq!(bears_in(&s, &c), vec![Zone::Graveyard(0)]);
}

// --- sacrifice -------------------------------------------------------------

fn sacrifice_to_the_oven(guard: Guard) -> (GameState, ObjectId, Cards) {
    let (reg, c) = registry();
    let (mut s, b) = table(&reg, &c, guard);
    let o = add(&mut s, &reg, 0, c.oven, Zone::Battlefield);
    let a = legal_actions(&s, &reg).into_iter()
        .find(|a| matches!(a, Action::ActivateAbility { source, additional_costs, .. } if *source == o
            && additional_costs.iter().any(|x| matches!(x, arcana_core::actions::AdditionalCostPayment::Sacrifice(y) if *y == b))))
        .expect("Witch's Oven can sacrifice the Bears");
    let (s, y) = step(s, a, &reg);
    (settle(s, y, &reg, b), b, c)
}

fn sacrifices(s: &GameState, id: ObjectId) -> usize {
    s.event_log.iter().filter(|e| matches!(e, GameEvent::Sacrifice { object_id, .. } if *object_id == id)).count()
}

#[test]
fn a_cost_sacrifice_passes_the_shield_by() {
    let (s, b, c) = sacrifice_to_the_oven(Guard::Shield);
    assert_eq!(bears_in(&s, &c), vec![Zone::Graveyard(0)]);
    assert_eq!((sacrifices(&s, b), dies(&s, b), shields(&s, b)), (1, 1, 1));
}

#[test]
fn a_cost_sacrifice_is_exiled_instead() {
    let (s, b, c) = sacrifice_to_the_oven(Guard::ExileInstead);
    assert_eq!(bears_in(&s, &c), vec![Zone::Exile]);
    assert_eq!((sacrifices(&s, b), dies(&s, b)), (1, 0));
}

fn sacrifice_to_plaguecrafter(guard: Guard) -> (GameState, ObjectId, Cards) {
    let (reg, c) = registry();
    let (mut s, b) = table(&reg, &c, guard);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Colorless, 2, 0);
    let p = add(&mut s, &reg, 0, c.plaguecrafter, Zone::Hand(0));
    let a = legal_actions(&s, &reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == p)).expect("castable");
    let (s, y) = step(s, a, &reg);
    (settle(s, y, &reg, b), b, c)
}

#[test]
fn an_effect_sacrifice_passes_the_shield_by() {
    let (s, b, c) = sacrifice_to_plaguecrafter(Guard::Shield);
    assert_eq!(bears_in(&s, &c), vec![Zone::Graveyard(0)]);
    assert_eq!((sacrifices(&s, b), dies(&s, b), shields(&s, b)), (1, 1, 1));
}

#[test]
fn an_effect_sacrifice_is_exiled_instead() {
    let (s, b, c) = sacrifice_to_plaguecrafter(Guard::ExileInstead);
    assert_eq!(bears_in(&s, &c), vec![Zone::Exile]);
    assert_eq!((sacrifices(&s, b), dies(&s, b)), (1, 0));
}

// --- the review's findings -------------------------------------------------

fn exile_instead(s: &mut GameState, id: ObjectId) {
    s.add_replacement_effect(ReplacementEffect {
        source: id, id: 0,
        condition: ReplacementCondition::WouldDieSpecific { object_id: id },
        kind: ReplacementKind::ExileInsteadOfDying,
        is_self_replacement: false,
        duration: ReplacementDuration::EndOfTurn,
        state_gate: None,
    });
}

/// Thatcher Revolt's tokens are sacrificed at the next end step, a
/// sacrifice, not a destroy: one under a regeneration shield and one with
/// indestructible go too, and the shield is left unspent.
#[test]
fn a_delayed_sacrifice_passes_a_shield_and_indestructible_by() {
    let (mut reg, c) = registry();
    let revolt = arcana_cards::avr::thatcher_revolt::register(&mut reg);
    let (mut s, _) = table(&reg, &c, Guard::ExileInstead);
    let card = add(&mut s, &reg, 0, revolt, Zone::Hand(0));
    let a = legal_actions(&s, &reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == card)).expect("castable");
    let (mut s, y) = step(s, a, &reg);
    s = settle(s, y, &reg, 0);
    let humans: Vec<ObjectId> = s.objects.objects_in_zone(Zone::Battlefield).filter(|o| o.is_token).map(|o| o.id).collect();
    assert_eq!(humans.len(), 3);
    Effect::Regenerate { target: humans[0] }.execute(&mut s);
    s.objects.get_mut(humans[1]).unwrap().characteristics.keywords.push(KeywordAbility::Indestructible);
    for _ in 0..60 {
        if humans.iter().all(|h| s.objects.get(*h).is_none()) || s.turn.active_player != 0 { break; }
        let acts = legal_actions(&s, &reg);
        let a = acts.iter().find(|a| a.is_pass()).cloned()
            .or_else(|| acts.iter().find(|a| matches!(a, Action::DeclareAttackers { attackers } if attackers.is_empty())).cloned())
            .or_else(|| acts.iter().find(|a| !matches!(a, Action::Concede)).cloned()).expect("an action");
        (s, _) = step(s, a, &reg);
    }
    for h in &humans {
        assert!(!s.objects.objects_in_zone(Zone::Battlefield).any(|o| o.id == *h), "token {h} was sacrificed");
        assert_eq!(sacrifices(&s, *h), 1);
    }
    assert_eq!(shields(&s, humans[0]), 1, "the shield was not spent on a sacrifice");
}

/// The legend rule puts the other copy into its owner's graveyard: a
/// death, so its exile-instead applies.
#[test]
fn the_legend_rules_loser_is_exiled_instead() {
    let (mut reg, c) = registry();
    let rhys = arcana_cards::khc::rhys_the_exiled::register(&mut reg);
    let (mut s, _) = table(&reg, &c, Guard::Shield);
    let keep = add(&mut s, &reg, 0, rhys, Zone::Battlefield);
    let lose = add(&mut s, &reg, 0, rhys, Zone::Battlefield);
    exile_instead(&mut s, lose);
    arcana_core::sba::apply_state_based_actions(&mut s);
    let a = legal_actions(&s, &reg).into_iter()
        .find(|a| matches!(a, Action::SubmitResolutionChoice { response: ChoiceResponse::PickCards { picked }, .. } if picked == &vec![keep]))
        .expect("keep the first Rhys");
    let (s, _) = step(s, a, &reg);
    let zones: Vec<Zone> = s.objects.iter().filter(|o| o.card_id == rhys).map(|o| o.zone).collect();
    assert!(zones.contains(&Zone::Battlefield) && zones.contains(&Zone::Exile), "{zones:?}");
    assert_eq!(dies(&s, lose), 0);
}

/// A planeswalker put into the graveyard at zero loyalty dies, so its
/// exile-instead applies.
#[test]
fn a_planeswalker_at_zero_loyalty_is_exiled_instead() {
    let (mut reg, c) = registry();
    let teferi = arcana_cards::rvr::teferi_time_raveler::register(&mut reg);
    let (mut s, _) = table(&reg, &c, Guard::Shield);
    let t = add(&mut s, &reg, 1, teferi, Zone::Battlefield);
    exile_instead(&mut s, t);
    arcana_core::sba::apply_state_based_actions(&mut s);
    let zones: Vec<Zone> = s.objects.iter().filter(|o| o.card_id == teferi).map(|o| o.zone).collect();
    assert_eq!(zones, vec![Zone::Exile]);
}

/// Regeneration taps the permanent, and that is an event: an untapped
/// Emmara, Soul of the Accord regenerated against Murder becomes tapped
/// once and makes her Soldier.
#[test]
fn regenerating_an_untapped_emmara_makes_a_soldier() {
    let (mut reg, c) = registry();
    let emmara = arcana_cards::grn::emmara_soul_of_the_accord::register(&mut reg);
    let (mut s, _) = table(&reg, &c, Guard::Shield);
    let e = add(&mut s, &reg, 0, emmara, Zone::Battlefield);
    Effect::Regenerate { target: e }.execute(&mut s);
    let m = add(&mut s, &reg, 0, c.murder, Zone::Hand(0));
    let s = cast_at(s, &reg, m, e);
    let o = s.objects.get(e).expect("Emmara regenerated");
    assert!(o.is_tapped());
    assert_eq!(s.event_log.iter().filter(|x| matches!(x, GameEvent::Tapped { object_id } if *object_id == e)).count(), 1);
    assert!(s.objects.objects_in_zone(Zone::Battlefield).any(|o| o.is_token && o.controller == 0),
        "her tap trigger made a Soldier");
}

// --- the re-review's findings ----------------------------------------------

/// An Aura put into the graveyard by the state-based check (here, one on
/// the battlefield attached to nothing) meets the death replacements; an
/// exile-instead aimed at it applies.
#[test]
fn an_illegal_auras_exile_instead_applies() {
    let (mut reg, c) = registry();
    let armor = arcana_cards::dsk::ethereal_armor::register(&mut reg);
    let (mut s, _) = table(&reg, &c, Guard::Shield);
    let a = add(&mut s, &reg, 0, armor, Zone::Battlefield);
    exile_instead(&mut s, a);
    arcana_core::sba::apply_state_based_actions(&mut s);
    let zones: Vec<Zone> = s.objects.iter().filter(|o| o.card_id == armor).map(|o| o.zone).collect();
    assert_eq!(zones, vec![Zone::Exile]);
}

/// A battle defeated with no back face goes to the graveyard through the
/// death replacements too.
#[test]
fn a_defeated_battles_exile_instead_applies() {
    let (reg, c) = registry();
    let (mut s, _) = table(&reg, &c, Guard::Shield);
    let id = s.allocate_object_id();
    let chars = arcana_core::objects::Characteristics {
        types: arcana_core::types::TypeLine(arcana_core::types::TypeLine::BATTLE),
        ..Default::default()
    };
    let mut obj = GameObject::new(id, 1, Zone::Battlefield, 0, chars);
    obj.controller = 1;
    s.objects.insert(obj);
    exile_instead(&mut s, id);
    arcana_core::sba::apply_state_based_actions(&mut s);
    assert_eq!(s.objects.iter().filter(|o| o.zone == Zone::Exile).count(), 1, "the battle was exiled");
    assert!(!s.objects.iter().any(|o| o.zone == Zone::Graveyard(1)));
}
