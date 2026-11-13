//! A1.5aw: clauses of several targets, or of up to N, are offered with
//! real targets, each target is rechecked against its own clause, and an
//! effect is not applied to a target that has become illegal (CR 608.2b).
//! Before, only clauses of exactly zero or one target were enumerated, so
//! Vivien, Arkbow Ranger's +1, Rishkar's entry trigger and Teferi, Time
//! Raveler's −3 always resolved with no target.

use arcana_core::actions::{Action, ChoiceResponse};
use arcana_core::engine::step;
use arcana_core::events::MoveCause;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetSelection};
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

fn objects(sel: &TargetSelection) -> Vec<ObjectId> {
    sel.targets.iter().filter_map(|t| match t {
        TargetChoice::Object(o) | TargetChoice::ObjectOrPlayer(arcana_core::targets::ObjectOrPlayer::Object(o)) => Some(*o),
        _ => None,
    }).collect()
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

/// The activations of `source`'s ability `index` on offer.
fn activations(s: &GameState, reg: &CardRegistry, source: ObjectId, index: usize) -> Vec<Action> {
    legal_actions(s, reg).into_iter()
        .filter(|a| matches!(a, Action::ActivateAbility { source: x, ability_index, .. } if *x == source && *ability_index == index))
        .collect()
}

fn targets_of(a: &Action) -> &TargetSelection {
    match a { Action::ActivateAbility { targets, .. } => targets, _ => panic!("an activation") }
}

#[test]
fn teferis_minus_three_bounces_up_to_one_target() {
    let mut reg = CardRegistry::new();
    let teferi = arcana_cards::rvr::teferi_time_raveler::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = table();
    add(&mut s, &reg, 0, bears, Zone::Library(0));
    let t = add(&mut s, &reg, 0, teferi, Zone::Battlefield);
    s.objects.get_mut(t).unwrap().add_counters(CounterKind::Loyalty, 4);
    let b = add(&mut s, &reg, 1, bears, Zone::Battlefield);
    let acts = activations(&s, &reg, t, 1);
    assert!(acts.iter().any(|a| targets_of(a).targets.is_empty()), "up to one: no target is a choice");
    let at_bears = acts.iter().find(|a| objects(targets_of(a)) == vec![b]).cloned().expect("aimed at the Bears");
    let (s, y) = step(s, at_bears, &reg);
    let s = settle(s, y, &reg);
    assert!(s.objects.objects_in_zone(Zone::Hand(1)).any(|o| o.card_id == bears), "the Bears went to their owner's hand");
    assert_eq!(s.objects.objects_in_zone(Zone::Hand(0)).count(), 1, "Teferi's controller drew");
}

#[test]
fn viviens_plus_one_reaches_up_to_two_creatures() {
    let mut reg = CardRegistry::new();
    let vivien = arcana_cards::m20::vivien_arkbow_ranger::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = table();
    let v = add(&mut s, &reg, 0, vivien, Zone::Battlefield);
    s.objects.get_mut(v).unwrap().add_counters(CounterKind::Loyalty, 4);
    let a = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    let b = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    let acts = activations(&s, &reg, v, 0);
    let mut shapes: Vec<Vec<ObjectId>> = acts.iter().map(|x| { let mut o = objects(targets_of(x)); o.sort(); o }).collect();
    shapes.sort();
    assert_eq!(shapes, vec![vec![], vec![a], vec![a, b], vec![b]]);
    let both = acts.iter().find(|x| objects(targets_of(x)).len() == 2).cloned().unwrap();
    let (s, y) = step(s, both, &reg);
    let s = settle(s, y, &reg);
    for id in [a, b] {
        assert_eq!(s.objects.get(id).unwrap().count_counters(CounterKind::PlusOnePlusOne), 1);
    }
}

/// Rishkar's entry trigger aimed at two creatures; one leaves before it
/// resolves. The other is still a legal target of the same clause, so the
/// ability resolves and puts its counter there.
#[test]
fn rishkars_trigger_survives_losing_one_of_its_two_targets() {
    let mut reg = CardRegistry::new();
    let rishkar = arcana_cards::cmm::rishkar_peema_renegade::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = table();
    s.player_mut(0).mana_pool.add_mana(ManaColor::Green, 3, 0);
    let a = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    let b = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    let r = add(&mut s, &reg, 0, rishkar, Zone::Hand(0));
    let cast = legal_actions(&s, &reg).into_iter()
        .find(|x| matches!(x, Action::CastSpell { object_id, .. } if *object_id == r)).expect("castable");
    let (mut s, mut y) = step(s, cast, &reg);
    // Pass until the trigger asks for its targets, then aim it at both Bears.
    for _ in 0..20 {
        let EngineYield::PendingDecision { legal_actions, .. } = &y else { panic!("a decision") };
        if let Some(pick) = legal_actions.iter().find(|x| matches!(x, Action::SubmitResolutionChoice {
                response: ChoiceResponse::ChooseTargets { selection }, .. } if { let mut o = objects(selection); o.sort(); o == vec![a, b] })).cloned() {
            (s, y) = step(s, pick, &reg);
            break;
        }
        let p = legal_actions.iter().find(|x| x.is_pass()).cloned().expect("a pass");
        (s, y) = step(s, p, &reg);
    }
    assert!(!s.stack_is_empty(), "the trigger waits on the stack with its two targets");
    // Remove the clause's first target: a positional recheck would find
    // every target illegal (the second has no requirement of its own) and
    // counter the ability.
    let first = objects(&s.stack.last().unwrap().targets)[0];
    let other = if first == a { b } else { a };
    s.move_object_to_zone(first, Zone::Graveyard(0), MoveCause::SpellResolution);
    let s = settle(s, y, &reg);
    assert_eq!(s.objects.get(other).unwrap().count_counters(CounterKind::PlusOnePlusOne), 1,
        "the remaining target got its counter");
}

/// An opponent's creature that gains hexproof after being chosen is an
/// illegal target at resolution; the ability resolves for its other target
/// and leaves this one alone (CR 608.2b).
#[test]
fn a_target_that_became_illegal_is_not_affected() {
    let mut reg = CardRegistry::new();
    let rishkar = arcana_cards::cmm::rishkar_peema_renegade::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = table();
    s.player_mut(0).mana_pool.add_mana(ManaColor::Green, 3, 0);
    let theirs = add(&mut s, &reg, 1, bears, Zone::Battlefield);
    let mine = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    let r = add(&mut s, &reg, 0, rishkar, Zone::Hand(0));
    let cast = legal_actions(&s, &reg).into_iter()
        .find(|x| matches!(x, Action::CastSpell { object_id, .. } if *object_id == r)).expect("castable");
    let (mut s, mut y) = step(s, cast, &reg);
    for _ in 0..20 {
        let EngineYield::PendingDecision { legal_actions, .. } = &y else { panic!("a decision") };
        if let Some(pick) = legal_actions.iter().find(|x| matches!(x, Action::SubmitResolutionChoice {
                response: ChoiceResponse::ChooseTargets { selection }, .. } if { let mut o = objects(selection); o.sort(); o == { let mut v = vec![theirs, mine]; v.sort(); v } })).cloned() {
            (s, y) = step(s, pick, &reg);
            break;
        }
        let p = legal_actions.iter().find(|x| x.is_pass()).cloned().expect("a pass");
        (s, y) = step(s, p, &reg);
    }
    s.objects.get_mut(theirs).unwrap().characteristics.keywords.push(arcana_core::effects::KeywordAbility::Hexproof);
    let s = settle(s, y, &reg);
    assert_eq!(s.objects.get(mine).unwrap().count_counters(CounterKind::PlusOnePlusOne), 1);
    assert_eq!(s.objects.get(theirs).unwrap().count_counters(CounterKind::PlusOnePlusOne), 0,
        "the target that gained hexproof was not affected");
}

// --- the review's findings: legality per occurrence, actions, information --

fn cast_with(s: &GameState, reg: &CardRegistry, card: ObjectId, want: &[ObjectId]) -> Action {
    legal_actions(s, reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, targets, .. } if *object_id == card && objects(targets) == want))
        .unwrap_or_else(|| panic!("castable at {want:?}"))
}

fn pass_until_empty(mut s: GameState, mut y: EngineYield, reg: &CardRegistry) -> GameState {
    for _ in 0..40 {
        if s.stack_is_empty() && s.pending_choice.is_none() { return s; }
        let EngineYield::PendingDecision { legal_actions, .. } = &y else { return s };
        let a = legal_actions.iter().find(|a| a.is_pass()).cloned()
            .or_else(|| legal_actions.iter().find(|a| !matches!(a, Action::Concede)).cloned()).unwrap();
        (s, y) = step(s, a, reg);
    }
    s
}

/// Plague Spores at one creature land for both clauses. It turns black:
/// illegal as the nonblack creature, still legal as the land, so it is
/// destroyed (CR 608.2b's own example, and CR 115.3).
#[test]
fn plague_spores_destroys_a_land_still_legal_for_its_second_clause() {
    let mut reg = CardRegistry::new();
    let spores = arcana_cards::inv::plague_spores::register(&mut reg);
    let mut s = table();
    for c in [ManaColor::Black, ManaColor::Red, ManaColor::Colorless] { s.player_mut(0).mana_pool.add_mana(c, 4, 0); }
    let land = s.allocate_object_id();
    let chars = arcana_core::objects::Characteristics {
        types: arcana_core::types::TypeLine(arcana_core::types::TypeLine::CREATURE | arcana_core::types::TypeLine::LAND),
        colors: arcana_core::types::ColorSet::green(),
        power: Some(arcana_core::types::PtValue::Fixed(1)),
        toughness: Some(arcana_core::types::PtValue::Fixed(1)),
        ..Default::default()
    };
    let mut obj = GameObject::new(land, 1, Zone::Battlefield, 0, chars);
    obj.controller = 1;
    s.objects.insert(obj);
    let card = add(&mut s, &reg, 0, spores, Zone::Hand(0));
    let a = cast_with(&s, &reg, card, &[land, land]);
    let (mut s, y) = step(s, a, &reg);
    s.objects.get_mut(land).unwrap().characteristics.colors = arcana_core::types::ColorSet::black();
    let s = pass_until_empty(s, y, &reg);
    assert!(!s.objects.objects_in_zone(Zone::Battlefield).any(|o| o.id == land), "destroyed as the land target");
}

/// Skarrgan Hellkite divides 2 damage between two targets; one gains
/// hexproof. The other keeps its share of 1; the illegal one takes none.
#[test]
fn skarrgans_surviving_target_keeps_its_share() {
    let mut reg = CardRegistry::new();
    let hellkite = arcana_cards::tdc::skarrgan_hellkite::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = table();
    s.player_mut(0).mana_pool.add_mana(ManaColor::Red, 4, 0);
    let h = add(&mut s, &reg, 0, hellkite, Zone::Battlefield);
    s.objects.get_mut(h).unwrap().add_counters(CounterKind::PlusOnePlusOne, 1);
    let a = add(&mut s, &reg, 1, bears, Zone::Battlefield);
    let b = add(&mut s, &reg, 1, bears, Zone::Battlefield);
    let act = legal_actions(&s, &reg).into_iter()
        .find(|x| matches!(x, Action::ActivateAbility { source, targets, .. } if *source == h && { let mut o = objects(targets); o.sort(); o == vec![a, b] }))
        .expect("the divided-damage ability at both Bears");
    let (mut s, y) = step(s, act, &reg);
    s.objects.get_mut(a).unwrap().characteristics.keywords.push(arcana_core::effects::KeywordAbility::Hexproof);
    let s = pass_until_empty(s, y, &reg);
    assert_eq!((s.objects.get(a).unwrap().damage_marked, s.objects.get(b).unwrap().damage_marked), (0, 1));
}

/// Pit Fight: "Target creature you control fights another target
/// creature." The first changes controller before it resolves; a fight
/// with an illegal participant doesn't happen (CR 701.14b).
#[test]
fn pit_fight_with_an_illegal_participant_does_nothing() {
    let mut reg = CardRegistry::new();
    let pit = arcana_cards::clu::pit_fight::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = table();
    s.player_mut(0).mana_pool.add_mana(ManaColor::Red, 2, 0);
    let mine = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    s.objects.get_mut(mine).unwrap().controller = 0;
    let theirs = add(&mut s, &reg, 1, bears, Zone::Battlefield);
    s.objects.get_mut(theirs).unwrap().controller = 1;
    let card = add(&mut s, &reg, 0, pit, Zone::Hand(0));
    let a = cast_with(&s, &reg, card, &[mine, theirs]);
    let (mut s, y) = step(s, a, &reg);
    s.objects.get_mut(mine).unwrap().controller = 1;
    let s = pass_until_empty(s, y, &reg);
    assert_eq!((s.objects.get(mine).unwrap().damage_marked, s.objects.get(theirs).unwrap().damage_marked), (0, 0));
}

/// Vivien's −3: "Target creature you control deals damage equal to its
/// power to target creature or planeswalker." Its first target changes
/// controller: it is illegal, performs no action and supplies no power,
/// so the second target takes no damage.
#[test]
fn viviens_minus_three_with_an_illegal_source_deals_nothing() {
    let mut reg = CardRegistry::new();
    let vivien = arcana_cards::m20::vivien_arkbow_ranger::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = table();
    let v = add(&mut s, &reg, 0, vivien, Zone::Battlefield);
    s.objects.get_mut(v).unwrap().add_counters(CounterKind::Loyalty, 4);
    let mine = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    let theirs = add(&mut s, &reg, 1, bears, Zone::Battlefield);
    s.objects.get_mut(theirs).unwrap().controller = 1;
    let act = activations(&s, &reg, v, 1).into_iter()
        .find(|x| objects(targets_of(x)) == vec![mine, theirs]).expect("−3 from my Bears at theirs");
    let (mut s, y) = step(s, act, &reg);
    s.objects.get_mut(mine).unwrap().controller = 1;
    let s = pass_until_empty(s, y, &reg);
    assert_eq!(s.objects.get(theirs).unwrap().damage_marked, 0);
}

// Fixes 7: retain the independent review witnesses and controls.
fn fixed_source_damage_effect(
    _state: &GameState,
    entry: &arcana_core::stack::StackEntry,
    _reg: &CardRegistry,
) -> Vec<arcana_core::effects::Effect> {
    use arcana_core::effects::Effect;
    let TargetChoice::Object(source) = entry.targets.targets[0] else {
        panic!()
    };
    let TargetChoice::Object(target) = entry.targets.targets[1] else {
        panic!()
    };
    vec![
        Effect::DealDamage {
            source,
            target: arcana_core::events::DamageTarget::Object(target),
            amount: 1,
        },
        Effect::AddCounters {
            target,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}

fn run_fixed_source_probe(make_illegal: bool) -> (u32, u32) {
    use arcana_core::registry::{CardDefinition, SpellAbilityDef};
    use arcana_core::targets::{ControllerConstraint, TargetRequirement};
    let mut reg = CardRegistry::new();
    let name = reg.interner_mut().intern("Fixed damage source probe");
    let spell=reg.register(CardDefinition::new(name,arcana_core::objects::Characteristics{
        name, types:arcana_core::types::TypeLine::SORCERY.into(),
        mana_cost:Some(arcana_core::mana::ManaCost::parse("{0}").unwrap()),..Default::default()
    }).with_spell_ability(SpellAbilityDef{
        text:"Target creature you control deals 1 damage to target creature. Put a +1/+1 counter on the second creature.".into(),
        target_requirements:vec![TargetRequirement{controller:Some(ControllerConstraint::You),..TargetRequirement::target_creature()},TargetRequirement::target_creature()],
        modal:None,effect:fixed_source_damage_effect,
    }));
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = table();
    let source = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    let target = add(&mut s, &reg, 1, bears, Zone::Battlefield);
    let card = add(&mut s, &reg, 0, spell, Zone::Hand(0));
    let action = cast_with(&s, &reg, card, &[source, target]);
    let (mut s, y) = step(s, action, &reg);
    if make_illegal {
        s.objects.get_mut(source).unwrap().controller = 1;
    }
    let s = pass_until_empty(s, y, &reg);
    let o = s.objects.get(target).unwrap();
    (
        o.damage_marked,
        o.count_counters(CounterKind::PlusOnePlusOne),
    )
}

#[test]
fn fixed_damage_from_legal_target_still_happens() {
    assert_eq!(run_fixed_source_probe(false), (1, 1));
}
#[test]
fn fixed_damage_from_illegal_target_must_not_happen() {
    assert_eq!(
        run_fixed_source_probe(true),
        (0, 1),
        "illegal source must not deal fixed damage; independent counter still resolves"
    );
}
#[test]
fn ordinary_departed_damage_source_still_deals_damage() {
    use arcana_core::effects::Effect;
    let mut reg = CardRegistry::new();
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = table();
    let source = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    let target = add(&mut s, &reg, 1, bears, Zone::Battlefield);
    s.move_object_to_zone(source, Zone::Graveyard(0), MoveCause::SpellResolution);
    Effect::DealDamage {
        source,
        target: arcana_core::events::DamageTarget::Object(target),
        amount: 1,
    }
    .execute(&mut s);
    assert_eq!(
        s.objects.get(target).unwrap().damage_marked,
        1,
        "do not treat every absent source as an illegal target"
    );
}

fn run_stiltzkin_probe(concede: bool) -> PlayerId {
    let mut reg = CardRegistry::new();
    let merchant = arcana_cards::fin::stiltzkin_moogle_merchant::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = GameState::new(3, 0);
    s.turn.active_player = 0;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(0);
    let m = add(&mut s, &reg, 0, merchant, Zone::Battlefield);
    s.objects.get_mut(m).unwrap().status.summoning_sick = false;
    let b = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    add(&mut s, &reg, 0, bears, Zone::Library(0));
    s.player_mut(0).mana_pool.add_mana(ManaColor::White, 2, 0);
    let action = activations(&s, &reg, m, 0)
        .into_iter()
        .find(|a| targets_of(a).targets == vec![TargetChoice::Player(1), TargetChoice::Object(b)])
        .expect("donation to player 1 is legal");
    let (mut s, mut y) = step(s, action, &reg);
    if concede {
        for _ in 0..5 {
            let EngineYield::PendingDecision { legal_actions, .. } = &y else {
                panic!("priority")
            };
            if s.priority_player() == 1 {
                let a = legal_actions
                    .iter()
                    .find(|a| matches!(a, Action::Concede))
                    .unwrap()
                    .clone();
                (s, y) = step(s, a, &reg);
                break;
            }
            let a = legal_actions.iter().find(|a| a.is_pass()).unwrap().clone();
            (s, y) = step(s, a, &reg);
        }
        assert!(s.player(1).has_lost);
    }
    let s = pass_until_empty(s, y, &reg);
    assert!(s.stack_is_empty());
    s.objects.get(b).unwrap().controller
}
#[test]
fn stiltzkin_legal_player_receives_creature() {
    assert_eq!(run_stiltzkin_probe(false), 1);
}
#[test]
fn stiltzkin_departed_player_does_not_become_sentinel_controller() {
    assert_eq!(
        run_stiltzkin_probe(true),
        0,
        "donation to the illegal player cannot give the creature controller 255"
    );
}

/// Control effects must reject a masked player before moving the object,
/// changing its controller, emitting events, or scheduling a revert.
#[test]
fn control_effects_require_a_living_recipient() {
    use arcana_core::effects::Effect;
    use arcana_core::objects::ILLEGAL_TARGET_PLAYER;
    for kind in 0..3 {
        for (recipient, departed) in [(1, false), (1, true), (ILLEGAL_TARGET_PLAYER, false)] {
            let mut reg = CardRegistry::new();
            let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
            let mut s = table();
            let zone = if kind == 2 {
                Zone::Hand(0)
            } else {
                Zone::Battlefield
            };
            let target = add(&mut s, &reg, 0, bears, zone);
            if departed {
                s.player_mut(1).has_lost = true;
            }
            let effect = match kind {
                0 => Effect::ChangeControl {
                    target,
                    new_controller: recipient,
                },
                1 => Effect::ChangeControlEot {
                    target,
                    new_controller: recipient,
                },
                _ => Effect::PutOntoBattlefieldTappedAttacking {
                    target,
                    controller: recipient,
                },
            };
            effect.execute(&mut s);
            if recipient == 1 && !departed {
                assert!(s
                    .objects
                    .objects_in_zone(Zone::Battlefield)
                    .any(|o| o.controller == 1));
                assert_eq!(s.delayed_triggers.len(), usize::from(kind == 1));
            } else {
                let object = s
                    .objects
                    .get(target)
                    .expect("illegal transfer must not even move the card");
                assert_eq!((object.controller, object.zone), (0, zone), "{effect:?}");
                assert!(s.delayed_triggers.is_empty(), "{effect:?}");
                assert!(s.event_log.is_empty(), "{effect:?}");
            }
        }
    }
}

fn delayed_probe_effect(
    _s: &GameState,
    _pt: &arcana_core::triggers::PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<arcana_core::effects::Effect> {
    vec![arcana_core::effects::Effect::GainLife {
        player: 0,
        amount: 7,
    }]
}

/// A missing player must not become a choice maker or the controller of
/// stored effects. An independent subsequent effect still runs.
#[test]
fn masked_players_do_not_create_choices_or_persistent_effects() {
    use arcana_core::effects::{
        DelayedAction, DelayedWhen, EachCastRider, Effect, FloatingUntil, NextCastKind,
        NextCastRider,
    };
    use arcana_core::layers::Duration;
    use arcana_core::objects::ILLEGAL_TARGET_PLAYER as missing;
    use arcana_core::triggers::TriggerCondition;
    let cases = vec![
        Effect::Anthem {
            controller: missing,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantFlashbackToInstantOrSorceryInGraveyard {
            source: 1,
            controller: missing,
            duration: Duration::EndOfTurn,
        },
        Effect::WardPrompt {
            caster: missing,
            cost: arcana_core::mana::ManaCost::parse("{1}").unwrap(),
            counter_target: 1,
        },
        Effect::NameCardAndExile {
            chooser: missing,
            target: 0,
        },
        Effect::ScheduleFloatingTrigger {
            source: 1,
            controller: missing,
            condition: TriggerCondition::SelfDies,
            effect: delayed_probe_effect,
            until: FloatingUntil::EndOfTurn,
        },
        Effect::ScheduleDelayedEffect {
            source: 1,
            controller: missing,
            when: DelayedWhen::NextEndStep,
            effect: delayed_probe_effect,
        },
        Effect::DelayedAction {
            source: 1,
            controller: missing,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnToHand,
        },
        Effect::DelayedActionOn {
            source: 1,
            affected: 1,
            controller: missing,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnToHand,
        },
        Effect::NextCastThisTurn {
            controller: missing,
            kind: NextCastKind::Any,
            rider: NextCastRider::GainsHaste,
        },
        Effect::EachCastThisTurn {
            controller: missing,
            kind: NextCastKind::Any,
            rider: EachCastRider::DrawTwoDiscardOne,
        },
        Effect::Goad {
            target: 1,
            goader: missing,
            duration: Duration::EndOfTurn,
        },
    ];
    for effect in cases {
        let mut reg = CardRegistry::new();
        let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
        let mut s = table();
        let target = add(&mut s, &reg, 0, bears, Zone::Hand(0));
        // NameCardAndExile must have something to exile if its guard fails.
        let before = format!("{s:?}");
        effect.execute(&mut s);
        assert_eq!(format!("{s:?}"), before, "{effect:?}");
        Effect::GainLife {
            player: 0,
            amount: 3,
        }
        .execute(&mut s);
        assert_eq!(s.player(0).life, 23);
        assert!(s.objects.get(target).is_some());
    }
}
