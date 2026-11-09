//! Stage-2 card sweep: registered definitions through legal engine actions.
use arcana_core::actions::{Action, ChoiceResponse};
use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::engine::step;
use arcana_core::events::MoveCause;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::{GameObject, ObjectId};
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetChoice;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CounterKind, ManaColor, PlayerId};
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
fn cast(s: GameState, r: &CardRegistry, id: ObjectId, targets: &[ObjectId]) -> GameState {
    let wanted: Vec<_> = targets.iter().map(|id| TargetChoice::Object(*id)).collect();
    let a = legal_actions(&s, r).into_iter().find(|a| matches!(a,
        Action::CastSpell { object_id, targets, .. } if *object_id == id && targets.targets == wanted)).expect("legal cast");
    finish(step(s, a, r).0, r)
}
fn activate(
    s: GameState,
    r: &CardRegistry,
    source: ObjectId,
    index: usize,
    targets: &[ObjectId],
) -> GameState {
    let wanted: Vec<_> = targets.iter().map(|id| TargetChoice::Object(*id)).collect();
    let a = legal_actions(&s, r)
        .into_iter()
        .find(|a| {
            matches!(a,
        Action::ActivateAbility { source: id, ability_index, targets, .. }
        if *id == source && *ability_index == index && targets.targets == wanted)
        })
        .expect("legal activation");
    finish(step(s, a, r).0, r)
}
fn permanent(s: &GameState, r: &CardRegistry, name: &str) -> ObjectId {
    let card = r.card_id_by_name(name).unwrap();
    s.objects
        .objects_in_zone(Zone::Battlefield)
        .find(|o| o.card_id == card)
        .expect("permanent")
        .id
}
fn clues(s: &GameState, p: PlayerId) -> Vec<ObjectId> {
    s.objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == p && o.commodity == Some(CommodityToken::Clue))
        .map(|o| o.id)
        .collect()
}

#[test]
fn inspector_investigates_with_a_clue_that_can_be_cracked() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        fund(&mut s, p);
        add(&mut s, &r, p, "Forest", Zone::Library(p));
        let card = add(&mut s, &r, p, "Thraben Inspector", Zone::Hand(p));
        let s = cast(s, &r, card, &[]);
        assert_eq!(clues(&s, p).len(), 1);
        let clue = clues(&s, p)[0];
        let s = activate(s, &r, clue, 0, &[]);
        assert!(clues(&s, p).is_empty());
        assert_eq!(s.objects.count_in_zone(Zone::Hand(p)), 1);
    }
}

#[test]
fn jadelight_explores_twice_with_lands_and_with_a_kept_nonland() {
    let r = registry();
    for p in [0, 1] {
        for lands in [true, false] {
            let mut s = table(p);
            fund(&mut s, p);
            for _ in 0..2 {
                add(
                    &mut s,
                    &r,
                    p,
                    if lands { "Forest" } else { "Grizzly Bears" },
                    Zone::Library(p),
                );
            }
            let ranger = add(&mut s, &r, p, "Jadelight Ranger", Zone::Hand(p));
            let s = cast(s, &r, ranger, &[]);
            let ranger = permanent(&s, &r, "Jadelight Ranger");
            assert_eq!(
                s.objects
                    .get(ranger)
                    .unwrap()
                    .count_counters(CounterKind::PlusOnePlusOne),
                if lands { 0 } else { 2 }
            );
            assert_eq!(
                s.objects.count_in_zone(Zone::Hand(p)),
                if lands { 2 } else { 0 }
            );
            assert_eq!(
                s.player(p).library_top_to_bottom.len(),
                if lands { 0 } else { 2 }
            );
        }
    }
}

#[test]
fn declaration_exiles_same_name_creatures_and_investigates_only_for_nontokens() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        fund(&mut s, p);
        let foe = 1 - p;
        let first = add(&mut s, &r, foe, "Grizzly Bears", Zone::Battlefield);
        let second = add(&mut s, &r, foe, "Grizzly Bears", Zone::Battlefield);
        let token = add(&mut s, &r, foe, "Grizzly Bears", Zone::Battlefield);
        s.objects.get_mut(token).unwrap().is_token = true;
        let mine = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        let other = add(&mut s, &r, foe, "Llanowar Elves", Zone::Battlefield);
        let spell = add(&mut s, &r, p, "Declaration in Stone", Zone::Hand(p));
        let s = cast(s, &r, spell, &[first]);
        for id in [first, second, token] {
            assert!(s.objects.get(id).is_none());
        }
        assert!(s.objects.get(mine).is_some() && s.objects.get(other).is_some());
        assert_eq!(clues(&s, foe).len(), 2);
        assert!(clues(&s, p).is_empty());
    }
}

#[test]
fn oko_makes_food_a_three_three_creature_that_outlives_oko() {
    let r = registry();
    let mut s = table(0);
    let oko = add(&mut s, &r, 0, "Oko, Thief of Crowns", Zone::Battlefield);
    s.objects
        .get_mut(oko)
        .unwrap()
        .add_counters(CounterKind::Loyalty, 4);
    Effect::CreateCommodityToken {
        controller: 0,
        kind: CommodityToken::Food,
        count: 1,
    }
    .execute(&mut s);
    let food = s
        .objects
        .objects_in_zone(Zone::Battlefield)
        .find(|o| o.commodity == Some(CommodityToken::Food))
        .unwrap()
        .id;
    let mut s = activate(s, &r, oko, 1, &[food]);
    assert!(s.is_creature_now(food));
    assert_eq!(
        (s.computed_power(food), s.computed_toughness(food)),
        (Some(3), Some(3))
    );
    s.move_object_to_zone(oko, Zone::Graveyard(0), MoveCause::SpellResolution);
    assert!(s.is_creature_now(food));
    assert_eq!(s.computed_power(food), Some(3));
}

#[test]
fn tasigur_can_pay_five_generic_mana_with_delve() {
    let r = registry();
    let mut s = table(0);
    for _ in 0..5 {
        add(&mut s, &r, 0, "Forest", Zone::Graveyard(0));
    }
    s.player_mut(0).mana_pool.add_mana(ManaColor::Black, 1, 0);
    let tasigur = add(&mut s, &r, 0, "Tasigur, the Golden Fang", Zone::Hand(0));
    let s = cast(s, &r, tasigur, &[]);
    assert_eq!(s.objects.count_in_zone(Zone::Exile), 5);
    assert_eq!(s.objects.count_in_zone(Zone::Graveyard(0)), 0);
    permanent(&s, &r, "Tasigur, the Golden Fang");
}

#[test]
fn kari_zevs_token_enters_tapped_and_attacking() {
    let r = registry();
    let mut s = table(0);
    let kari = add(&mut s, &r, 0, "Kari Zev, Skyship Raider", Zone::Battlefield);
    s.objects.get_mut(kari).unwrap().status.summoning_sick = false;
    s.turn.phase = Phase::Combat;
    s.turn.step = Step::DeclareAttackers;
    s.combat = Some(arcana_core::combat::CombatState {
        phase: arcana_core::combat::CombatPhase::DeclareAttackers,
        ..arcana_core::combat::CombatState::new()
    });
    let attack = legal_actions(&s, &r).into_iter().find(|a| matches!(a,
        Action::DeclareAttackers { attackers } if attackers.len() == 1 && attackers[0].attacker == kari)).expect("Kari can attack");
    let s = finish(step(s, attack, &r).0, &r);
    let token = s
        .objects
        .objects_in_zone(Zone::Battlefield)
        .find(|o| o.is_token)
        .expect("Ragavan");
    assert!(token.is_tapped());
    assert_eq!(
        token.characteristics.name,
        r.interner().lookup("Ragavan").unwrap()
    );
    assert!(s
        .combat
        .as_ref()
        .unwrap()
        .attackers
        .iter()
        .any(|a| a.object_id == token.id && a.defending_player == 1));
}

#[test]
fn voice_triggers_only_for_opponents_spells_during_its_controllers_turn() {
    let r = registry();
    for own_turn in [true, false] {
        let mut s = table(if own_turn { 0 } else { 1 });
        fund(&mut s, 1);
        add(&mut s, &r, 0, "Voice of Resurgence", Zone::Battlefield);
        add(&mut s, &r, 1, "Forest", Zone::Library(1));
        let spell = add(&mut s, &r, 1, "Opt", Zone::Hand(1));
        s.priority.give_to(1);
        let s = cast(s, &r, spell, &[]);
        assert_eq!(
            s.objects
                .objects_in_zone(Zone::Battlefield)
                .filter(|o| o.is_token)
                .count(),
            usize::from(own_turn)
        );
    }
}

#[test]
fn lava_coil_exiles_instead_of_dying_and_prevents_the_dies_trigger() {
    let r = registry();
    let mut s = table(0);
    fund(&mut s, 0);
    let traveler = add(&mut s, &r, 1, "Doomed Traveler", Zone::Battlefield);
    let coil = add(&mut s, &r, 0, "Lava Coil", Zone::Hand(0));
    let s = cast(s, &r, coil, &[traveler]);
    assert_eq!(s.objects.count_in_zone(Zone::Graveyard(1)), 0);
    assert!(s
        .objects
        .objects_in_zone(Zone::Exile)
        .any(|o| o.card_id == r.card_id_by_name("Doomed Traveler").unwrap()));
    assert!(!s
        .objects
        .objects_in_zone(Zone::Battlefield)
        .any(|o| o.is_token));
}

#[test]
fn tracker_counts_only_its_controllers_clue_sacrifices() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        fund(&mut s, p);
        let mine = add(&mut s, &r, p, "Tireless Tracker", Zone::Battlefield);
        let theirs = add(&mut s, &r, 1 - p, "Tireless Tracker", Zone::Battlefield);
        add(&mut s, &r, p, "Forest", Zone::Library(p));
        Effect::CreateCommodityToken {
            controller: p,
            kind: CommodityToken::Clue,
            count: 1,
        }
        .execute(&mut s);
        let clue = clues(&s, p)[0];
        let s = activate(s, &r, clue, 0, &[]);
        assert_eq!(
            s.objects
                .get(mine)
                .unwrap()
                .count_counters(CounterKind::PlusOnePlusOne),
            1
        );
        assert_eq!(
            s.objects
                .get(theirs)
                .unwrap()
                .count_counters(CounterKind::PlusOnePlusOne),
            0
        );
    }
}

#[test]
fn inferno_gives_each_opponent_an_emblem_that_survives_chandra() {
    let r = registry();
    let mut s = GameState::new(3, 0);
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(0);
    let chandra = add(
        &mut s,
        &r,
        0,
        "Chandra, Awakened Inferno",
        Zone::Battlefield,
    );
    s.objects
        .get_mut(chandra)
        .unwrap()
        .add_counters(CounterKind::Loyalty, 6);
    for p in 0..3 {
        for _ in 0..3 {
            add(&mut s, &r, p, "Forest", Zone::Library(p));
        }
    }
    let mut s = activate(s, &r, chandra, 0, &[]);
    assert_eq!(s.objects.count_in_zone(Zone::Command), 2);
    s.move_object_to_zone(chandra, Zone::Graveyard(0), MoveCause::SpellResolution);
    for _ in 0..300 {
        if s.active_player() == 2 && s.turn.step == Step::Upkeep && s.stack_is_empty() {
            break;
        }
        let acts = legal_actions(&s, &r);
        let a = acts
            .iter()
            .find(|a| a.is_pass())
            .or_else(|| acts.iter().find(|a| !matches!(a, Action::Concede)))
            .unwrap()
            .clone();
        s = step(s, a, &r).0;
    }
    assert_eq!(s.life_totals(), vec![20, 19, 19]);
}

fn back_jace(s: &mut GameState, r: &CardRegistry, p: PlayerId) -> ObjectId {
    let jace = add(s, r, p, "Jace, Vryn's Prodigy", Zone::Battlefield);
    let card = s.objects.get(jace).unwrap().card_id;
    let arcana_core::registry::AlternateFace::Transform(back) =
        r.get(card).unwrap().alternate_face.as_ref().unwrap()
    else {
        panic!("transform card");
    };
    s.objects.get_mut(jace).unwrap().back_face_characteristics = Some(back.characteristics.clone());
    Effect::Transform { target: jace }.execute(s);
    jace
}

#[test]
fn jace_back_plus_one_lasts_through_opponents_turn_and_expires_on_ours() {
    let r = registry();
    for p in 0..2 {
        let mut s = table(p);
        for who in 0..2 {
            for _ in 0..3 {
                add(&mut s, &r, who, "Forest", Zone::Library(who));
            }
        }
        let jace = back_jace(&mut s, &r, p);
        let bear = add(&mut s, &r, 1 - p, "Grizzly Bears", Zone::Battlefield);
        let mut s = activate(s, &r, jace, 0, &[bear]);
        assert_eq!(s.computed_power(bear), Some(0));
        let mut saw_opponent = false;
        for _ in 0..200 {
            if s.active_player() != p {
                saw_opponent = true;
                assert_eq!(s.computed_power(bear), Some(0));
            }
            if saw_opponent && s.active_player() == p {
                break;
            }
            let acts = legal_actions(&s, &r);
            let a = acts
                .iter()
                .find(|a| a.is_pass())
                .or_else(|| acts.iter().find(|a| !matches!(a, Action::Concede)))
                .unwrap()
                .clone();
            s = step(s, a, &r).0;
        }
        assert!(saw_opponent);
        assert_eq!(s.computed_power(bear), Some(2));
    }
}

#[test]
fn jace_back_plus_one_allows_no_target_and_is_hidden_on_front() {
    let r = registry();
    let mut s = table(0);
    let front = add(&mut s, &r, 0, "Jace, Vryn's Prodigy", Zone::Battlefield);
    assert!(!legal_actions(&s, &r)
        .iter()
        .any(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == front)));
    s.move_object_to_zone(front, Zone::Graveyard(0), MoveCause::SpellResolution);
    let back = back_jace(&mut s, &r, 0);
    let s = activate(s, &r, back, 0, &[]);
    assert_eq!(
        s.objects
            .get(back)
            .unwrap()
            .count_counters(CounterKind::Loyalty),
        6
    );
}

fn end_step(mut s: GameState, r: &CardRegistry, player: PlayerId, turn: u32) -> GameState {
    for _ in 0..250 {
        if s.active_player() == player
            && s.turn.turn_number >= turn
            && s.turn.step == Step::End
            && s.stack_is_empty()
            && s.pending_choice.is_none()
            && s.pending_trigger_queue.is_empty()
        {
            return s;
        }
        let acts = legal_actions(&s, r);
        let a = acts
            .iter()
            .find(|a| a.is_pass())
            .or_else(|| acts.iter().find(|a| !matches!(a, Action::Concede)))
            .expect("advance end step")
            .clone();
        s = step(s, a, r).0;
    }
    panic!("did not reach end step");
}

#[test]
fn curious_obsession_is_sacrificed_if_its_controller_did_not_attack() {
    let r = registry();
    for p in 0..2 {
        let mut s = table(p);
        fund(&mut s, p);
        let bear = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        let aura = add(&mut s, &r, p, "Curious Obsession", Zone::Hand(p));
        let s = cast(s, &r, aura, &[bear]);
        let aura = permanent(&s, &r, "Curious Obsession");
        let turn = s.turn.turn_number;
        let s = end_step(s, &r, p, turn);
        assert!(s.objects.get(aura).is_none());
        assert_eq!(s.objects.count_in_zone(Zone::Graveyard(p)), 1);
        assert_eq!(s.computed_power(bear), Some(2));
    }
}

#[test]
fn curious_obsession_remembers_a_departed_attacker_but_not_next_turn() {
    let r = registry();
    for p in 0..2 {
        let mut s = table(p);
        fund(&mut s, p);
        for who in 0..2 {
            for _ in 0..4 {
                add(&mut s, &r, who, "Forest", Zone::Library(who));
            }
        }
        let host = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        let attacker = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        let aura = add(&mut s, &r, p, "Curious Obsession", Zone::Hand(p));
        let mut s = cast(s, &r, aura, &[host]);
        let aura = permanent(&s, &r, "Curious Obsession");
        s.objects.get_mut(attacker).unwrap().status.summoning_sick = false;
        s.turn.phase = Phase::Combat;
        s.turn.step = Step::DeclareAttackers;
        s.combat = Some(arcana_core::combat::CombatState {
            phase: arcana_core::combat::CombatPhase::DeclareAttackers,
            ..arcana_core::combat::CombatState::new()
        });
        let attack=legal_actions(&s,&r).into_iter().find(|a|matches!(a,Action::DeclareAttackers{attackers} if attackers.len()==1 && attackers[0].attacker==attacker)).unwrap();
        s = step(s, attack, &r).0;
        Effect::ChangeControl {
            target: attacker,
            new_controller: 1 - p,
        }
        .execute(&mut s);
        s.move_object_to_zone(attacker, Zone::Graveyard(p), MoveCause::SpellResolution);
        let turn = s.turn.turn_number;
        let s = end_step(s, &r, p, turn);
        assert!(
            s.objects.get(aura).is_some(),
            "this turn's departed attacker still counts"
        );
        assert!(s.lki(attacker).is_none(), "the check cannot depend on LKI");
        let s = end_step(s, &r, p, turn + 1);
        assert!(
            s.objects.get(aura).is_none(),
            "last turn's attack must not count"
        );
    }
}
