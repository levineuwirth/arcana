//! A1.5bw: target prompts use the stack entry’s controller and real source.
use arcana_core::actions::{Action, ChoiceKind, ChoiceResponse};
use arcana_core::effects::{Effect, KeywordAbility, ProtectionQuality};
use arcana_core::engine::step;
use arcana_core::events::MoveCause;
use arcana_core::layers::Duration;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::{GameObject, ObjectId};
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetChoice;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{Color, ManaColor, PlayerId};
use arcana_core::zones::Zone;

fn registry() -> CardRegistry {
    let mut r = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut r);
    r
}
fn table(p: PlayerId) -> GameState {
    let mut s = GameState::new(2, 0);
    s.turn.active_player = p;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(p);
    s
}
fn add(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str, zone: Zone) -> ObjectId {
    let card = r
        .card_id_by_name(name)
        .unwrap_or_else(|| panic!("unregistered: {name}"));
    let id = s.allocate_object_id();
    s.objects.insert(GameObject::new(
        id,
        p,
        zone,
        card,
        r.get(card).unwrap().initial_characteristics().clone(),
    ));
    if let Zone::Library(p) = zone {
        s.player_mut(p).library_top_to_bottom.push(id);
    }
    id
}
fn fund(s: &mut GameState, p: PlayerId) {
    for c in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ] {
        s.player_mut(p).mana_pool.add_mana(c, 8, 0);
    }
}
fn finish(mut s: GameState, r: &CardRegistry) -> GameState {
    for _ in 0..100 {
        if s.stack_is_empty() && s.pending_choice.is_none() && s.pending_trigger_queue.is_empty() {
            return s;
        }
        let acts = legal_actions(&s, r);
        let a = acts
            .iter()
            .find(|a| a.is_pass())
            .or_else(|| {
                acts.iter().find(|a| {
                    matches!(
                        a,
                        Action::SubmitResolutionChoice {
                            response: ChoiceResponse::YesNo { answer: false },
                            ..
                        }
                    )
                })
            })
            .or_else(|| acts.iter().find(|a| !matches!(a, Action::Concede)))
            .expect("decision")
            .clone();
        s = step(s, a, r).0;
    }
    panic!("resolution did not finish");
}

fn cast_to_prompt(mut s: GameState, r: &CardRegistry, p: PlayerId, name: &str) -> GameState {
    fund(&mut s, p);
    let card = add(&mut s, r, p, name, Zone::Hand(p));
    let a = legal_actions(&s, r)
        .into_iter()
        .find(|a| matches!(a,Action::CastSpell{object_id,..} if *object_id==card))
        .expect("cast");
    s = step(s, a, r).0;
    for _ in 0..20 {
        if s.pending_choice.is_some() {
            return s;
        }
        let a = legal_actions(&s, r)
            .into_iter()
            .find(|a| a.is_pass())
            .expect("priority");
        s = step(s, a, r).0;
    }
    panic!("target prompt missing");
}
fn offered(s: &GameState, r: &CardRegistry) -> Vec<Vec<TargetChoice>> {
    assert!(matches!(
        s.pending_choice.as_ref().unwrap().kind,
        ChoiceKind::ChooseTargets { .. }
    ));
    legal_actions(s, r)
        .into_iter()
        .filter_map(|a| match a {
            Action::SubmitResolutionChoice {
                response: ChoiceResponse::ChooseTargets { selection },
                ..
            } => Some(selection.targets),
            _ => None,
        })
        .collect()
}
fn choose_object(s: GameState, r: &CardRegistry, id: ObjectId) -> GameState {
    let a=legal_actions(&s,r).into_iter().find(|a|matches!(a,Action::SubmitResolutionChoice{response:ChoiceResponse::ChooseTargets{selection},..} if selection.targets.len()==1 && selection.targets[0].object_id()==Some(id))).expect("offered object");
    step(s, a, r).0
}
fn has_target(s: &GameState, r: &CardRegistry, id: ObjectId) -> bool {
    offered(s, r)
        .iter()
        .any(|ts| ts.iter().any(|t| t.object_id() == Some(id)))
}
#[test]
fn reflector_mage_offers_and_bounces_opponents_creature_from_each_seat() {
    let r = registry();
    for p in 0..2 {
        let mut s = table(p);
        let mine = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        let foe = add(&mut s, &r, 1 - p, "Grizzly Bears", Zone::Battlefield);
        let s = cast_to_prompt(s, &r, p, "Reflector Mage");
        assert_eq!(offered(&s, &r), vec![vec![TargetChoice::Object(foe)]]);
        let s = finish(choose_object(s, &r, foe), &r);
        assert!(s.objects.get(mine).is_some());
        assert!(s.objects.get(foe).is_none());
        assert_eq!(s.objects.count_in_zone(Zone::Hand(1 - p)), 1);
    }
}
#[test]
fn gearhulk_offers_only_its_controllers_creatures_from_each_seat() {
    let r = registry();
    for p in 0..2 {
        let mut s = table(p);
        let mine = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        let foe = add(&mut s, &r, 1 - p, "Grizzly Bears", Zone::Battlefield);
        let s = cast_to_prompt(s, &r, p, "Verdurous Gearhulk");
        assert!(has_target(&s, &r, mine));
        assert!(!has_target(&s, &r, foe));
        assert!(offered(&s, &r).iter().flatten().all(|t| s
            .objects
            .get(t.object_id().unwrap())
            .unwrap()
            .controller
            == p));
        // Its existing counter-distribution approximation is outside this repair.
    }
}
#[test]
fn hexproof_is_relative_to_trigger_controller_in_both_seats() {
    let r = registry();
    for p in 0..2 {
        let mut s = table(p);
        let mine = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        let foe = add(&mut s, &r, 1 - p, "Grizzly Bears", Zone::Battlefield);
        for id in [mine, foe] {
            Effect::GrantKeyword {
                target: id,
                keyword: KeywordAbility::Hexproof,
                duration: Duration::Permanent,
            }
            .execute(&mut s);
        }
        let s = cast_to_prompt(s, &r, p, "Earthshaker Khenra");
        assert!(has_target(&s, &r, mine));
        assert!(!has_target(&s, &r, foe));
    }
}
#[test]
fn protection_uses_the_real_trigger_source_in_both_seats() {
    let r = registry();
    for p in 0..2 {
        let mut s = table(p);
        let protected = add(&mut s, &r, 1 - p, "Grizzly Bears", Zone::Battlefield);
        let legal = add(&mut s, &r, 1 - p, "Grizzly Bears", Zone::Battlefield);
        Effect::GrantKeyword {
            target: protected,
            keyword: KeywordAbility::Protection(ProtectionQuality::Color(Color::Blue)),
            duration: Duration::Permanent,
        }
        .execute(&mut s);
        let s = cast_to_prompt(s, &r, p, "Reflector Mage");
        assert!(!has_target(&s, &r, protected));
        assert_eq!(offered(&s, &r), vec![vec![TargetChoice::Object(legal)]]);
    }
}
#[test]
fn trigger_keeps_its_controller_after_source_control_change_or_departure() {
    let r = registry();
    for p in 0..2 {
        for leave in [false, true] {
            let mut s = table(p);
            let mine = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
            let foe = add(&mut s, &r, 1 - p, "Grizzly Bears", Zone::Battlefield);
            let mut s = cast_to_prompt(s, &r, p, "Reflector Mage");
            let source = s.stack.last().unwrap().source;
            if leave {
                s.move_object_to_zone(source, Zone::Graveyard(p), MoveCause::SpellResolution);
            } else {
                Effect::ChangeControl {
                    target: source,
                    new_controller: 1 - p,
                }
                .execute(&mut s);
            }
            assert!(has_target(&s, &r, foe));
            assert!(!has_target(&s, &r, mine));
            let s = finish(choose_object(s, &r, foe), &r);
            assert!(s.objects.get(foe).is_none());
        }
    }
}
#[test]
fn ward_prompts_the_trigger_controller_and_respects_pay_or_decline() {
    let r = registry();
    for p in 0..2 {
        for pay in [false, true] {
            let mut s = table(p);
            let foe = add(&mut s, &r, 1 - p, "Grizzly Bears", Zone::Battlefield);
            Effect::GrantKeyword {
                target: foe,
                keyword: KeywordAbility::Ward(arcana_core::mana::ManaCost::parse("{2}").unwrap()),
                duration: Duration::Permanent,
            }
            .execute(&mut s);
            let s = cast_to_prompt(s, &r, p, "Reflector Mage");
            let mut s = choose_object(s, &r, foe);
            for _ in 0..8 {
                if s.pending_choice.is_some() {
                    break;
                }
                let a = legal_actions(&s, &r)
                    .into_iter()
                    .find(|a| a.is_pass())
                    .unwrap();
                s = step(s, a, &r).0;
            }
            let pc = s.pending_choice.as_ref().expect("ward payment");
            assert_eq!(pc.choosing_player, p);
            assert!(matches!(pc.kind, ChoiceKind::PayOrDecline { .. }));
            let mana = s.player(p).mana_pool.total();
            let a=legal_actions(&s,&r).into_iter().find(|a|matches!(a,Action::SubmitResolutionChoice{response:ChoiceResponse::PayOrDecline{pay:q},..} if *q==pay)).unwrap();
            let s = finish(step(s, a, &r).0, &r);
            assert_eq!(s.objects.get(foe).is_none(), pay);
            assert_eq!(
                s.player(p).mana_pool.total(),
                mana - if pay { 2 } else { 0 }
            );
        }
    }
}
#[test]
fn source_relative_trigger_finds_only_its_own_blocker() {
    use arcana_core::combat::{AttackerInfo, BlockerInfo, CombatState};
    use arcana_core::targets::{
        ControllerConstraint, TargetCount, TargetFilter, TargetRequirement,
    };
    use arcana_core::triggers::{TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
    let r = registry();
    for p in 0..2 {
        let mut s = table(p);
        let source = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        let blocker = add(&mut s, &r, 1 - p, "Grizzly Bears", Zone::Battlefield);
        add(&mut s, &r, 1 - p, "Grizzly Bears", Zone::Battlefield);
        add(&mut s, &r, p, "Forest", Zone::Library(p));
        let mut combat = CombatState::new();
        combat.attackers.push(AttackerInfo {
            object_id: source,
            defending_player: 1 - p,
            defending_planeswalker: None,
            blocked_by: vec![blocker],
            is_blocked: true,
        });
        combat.blockers.push(BlockerInfo {
            object_id: blocker,
            blocking: source,
        });
        s.combat = Some(combat);
        Effect::GrantTriggeredAbility {
            target: source,
            duration: Duration::Permanent,
            ability: Box::new(TriggeredAbilityDef {
                id: arcana_core::triggers::GRANTED_TRIGGER_ID_BASE + 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: |_, trig, _| {
                    vec![Effect::Tap {
                        target: trig.targets.targets[0].object_id().unwrap(),
                    }]
                },
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::CreatureBlockingSource,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
        }
        .execute(&mut s);
        let s = cast_to_prompt(s, &r, p, "Opt");
        assert_eq!(offered(&s, &r), vec![vec![TargetChoice::Object(blocker)]]);
        let s = finish(choose_object(s, &r, blocker), &r);
        assert!(s.objects.get(blocker).unwrap().is_tapped());
    }
}
#[test]
fn copied_spell_can_still_choose_new_targets_from_each_seat() {
    let r = registry();
    for p in 0..2 {
        let mut s = table(p);
        fund(&mut s, p);
        let bolt = add(&mut s, &r, p, "Lightning Bolt", Zone::Hand(p));
        let a=legal_actions(&s,&r).into_iter().find(|a|matches!(a,Action::CastSpell{object_id,targets,..} if *object_id==bolt && targets.targets.len()==1 && targets.targets[0].player_id()==Some(1-p))).unwrap();
        s = step(s, a, &r).0;
        let original = s.stack.last().unwrap().id;
        Effect::CopySpell { target: original }.execute(&mut s);
        let a=legal_actions(&s,&r).into_iter().find(|a|matches!(a,Action::SubmitResolutionChoice{response:ChoiceResponse::ChooseTargets{selection},..} if selection.targets.len()==1 && selection.targets[0].player_id()==Some(p))).unwrap();
        let s = finish(step(s, a, &r).0, &r);
        assert_eq!(s.life_totals(), vec![17, 17]);
    }
}
#[test]
fn multiplayer_trigger_offers_both_opponents() {
    let r = registry();
    let mut s = GameState::new(3, 0);
    s.turn.active_player = 2;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(2);
    let a = add(&mut s, &r, 0, "Grizzly Bears", Zone::Battlefield);
    let b = add(&mut s, &r, 1, "Grizzly Bears", Zone::Battlefield);
    let mine = add(&mut s, &r, 2, "Grizzly Bears", Zone::Battlefield);
    let s = cast_to_prompt(s, &r, 2, "Reflector Mage");
    assert!(has_target(&s, &r, a));
    assert!(has_target(&s, &r, b));
    assert!(!has_target(&s, &r, mine));
    assert_eq!(offered(&s, &r).len(), 2);
}
