//! A1.5bq witnesses, using the registered client definitions.
use arcana_core::actions::Action;
use arcana_core::combat::{AttackerDeclaration, CombatState, DefendingEntity};
use arcana_core::effects::KeywordAbility;
use arcana_core::engine::step;
use arcana_core::events::MoveCause;
use arcana_core::layers::{ContinuousEffect, ContinuousEffectKind, Duration, Layer};
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::{GameObject, ObjectId};
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, PtCondition, TargetChoice};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CounterKind, ManaColor, PlayerId, TypeLine};
use arcana_core::zones::Zone;

fn registry() -> CardRegistry {
    let mut r = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut r);
    r
}
fn table(p: PlayerId) -> GameState {
    let mut s = GameState::new(2, 20261110);
    s.turn.active_player = p;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(p);
    for color in [ManaColor::White, ManaColor::Blue, ManaColor::Black,
        ManaColor::Red, ManaColor::Green] {
        s.player_mut(p).mana_pool.add_mana(color, 30, 0);
    }
    s
}
fn add(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str, zone: Zone) -> ObjectId {
    let card = r.card_id_by_name(name).expect("registered client");
    let id = s.allocate_object_id();
    s.objects.insert(GameObject::new(id, p, zone, card,
        r.get(card).unwrap().initial_characteristics().clone()));
    id
}
fn finish(mut s: GameState, r: &CardRegistry) -> GameState {
    for _ in 0..100 {
        if s.stack_is_empty() && s.pending_choice.is_none()
            && s.pending_trigger_queue.is_empty() { return s; }
        let actions = legal_actions(&s, r);
        let action = actions.iter().find(|a| a.is_pass())
            .or_else(|| actions.iter().find(|a| !a.is_concede()))
            .expect("decision").clone();
        s = step(s, action, r).0;
    }
    panic!("resolution did not finish");
}
fn cast(mut s: GameState, r: &CardRegistry, p: PlayerId, name: &str) -> (GameState, ObjectId) {
    let id = add(&mut s, r, p, name, Zone::Hand(p));
    let action = legal_actions(&s, r).into_iter().find(|a| matches!(a,
        Action::CastSpell { object_id, .. } if *object_id == id)).expect("legal cast");
    let s = finish(step(s, action, r).0, r);
    let new = s.objects.objects_in_zone(Zone::Battlefield)
        .filter(|o| o.card_id == r.card_id_by_name(name).unwrap())
        .map(|o| o.id).max().expect("resolved client");
    (s, new)
}

#[test]
fn phantom_excludes_only_itself_and_tracks_control() {
    let r = registry();
    for p in [0, 1] {
        let (s, first) = cast(table(p), &r, p, "Supreme Phantom");
        assert_eq!(s.computed_power(first), Some(1));
        let (mut s, second) = cast(s, &r, p, "Supreme Phantom");
        assert_eq!(s.computed_power(first), Some(2));
        assert_eq!(s.computed_power(second), Some(2));
        s.objects.get_mut(second).unwrap().controller = 1 - p;
        assert_eq!(s.computed_power(first), Some(1));
        assert_eq!(s.computed_power(second), Some(1));
    }
}

#[test]
fn eagle_excludes_only_itself_and_reads_granted_and_removed_flying() {
    let r = registry();
    for p in [0, 1] {
        let (s, first) = cast(table(p), &r, p, "Empyrean Eagle");
        assert_eq!(s.computed_power(first), Some(2));
        let (mut s, second) = cast(s, &r, p, "Empyrean Eagle");
        assert_eq!(s.computed_power(first), Some(3));
        assert_eq!(s.computed_power(second), Some(3));
        let bear = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        assert_eq!(s.computed_power(bear), Some(2));
        s.add_continuous_effect(ContinuousEffect::grant_keyword(first, bear,
            KeywordAbility::Flying, Duration::EndOfTurn));
        assert_eq!(s.computed_power(bear), Some(4));
        s.add_continuous_effect(ContinuousEffect::lose_all_abilities(first, bear, Duration::EndOfTurn));
        assert_eq!(s.computed_power(bear), Some(2));
        s.move_object_to_zone(first, Zone::Graveyard(p), MoveCause::SpellResolution);
        assert_eq!(s.computed_power(second), Some(2));
    }
}

#[test]
fn applicability_reads_prior_types_and_same_layer_timestamp_updates() {
    let r = registry();
    let mut s = table(0);
    let source = add(&mut s, &r, 0, "Empyrean Eagle", Zone::Battlefield);
    let land = add(&mut s, &r, 0, "Forest", Zone::Battlefield);
    s.add_continuous_effect(ContinuousEffect {
        source, layer: Layer::L4Type, timestamp: 0, duration: Duration::EndOfTurn,
        dependency: None, kind: ContinuousEffectKind::AddType { target: land, types: TypeLine::CREATURE.into() },
    });
    s.add_continuous_effect(ContinuousEffect::set_pt(source, land, 2, 2, Duration::EndOfTurn));
    s.add_continuous_effect(ContinuousEffect::filtered_pump(source,
        ObjectFilter::creature(), 1, 1, Duration::EndOfTurn));
    assert_eq!(s.computed_power(land), Some(3));
    s.add_continuous_effect(ContinuousEffect::anthem(source, 0, 2, 0, Duration::EndOfTurn));
    assert_eq!(s.computed_power(land), Some(5));

    let bear = add(&mut s, &r, 0, "Grizzly Bears", Zone::Battlefield);
    s.add_continuous_effect(ContinuousEffect::grant_keyword(source, bear,
        KeywordAbility::Flying, Duration::EndOfTurn));
    s.add_continuous_effect(ContinuousEffect {
        source, layer: Layer::L6Ability, timestamp: 0, duration: Duration::EndOfTurn,
        dependency: None, kind: ContinuousEffectKind::FilteredGrantKeyword {
            filter: ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
            keyword: KeywordAbility::Vigilance,
        },
    });
    assert!(s.has_keyword(bear, &KeywordAbility::Vigilance));
}

#[test]
fn applicability_uses_in_flight_power_without_double_counting_counters() {
    let r = registry();
    let mut s = table(0);
    let bear = add(&mut s, &r, 0, "Grizzly Bears", Zone::Battlefield);
    s.objects.get_mut(bear).unwrap().add_counters(CounterKind::PlusOnePlusOne, 1);
    s.add_continuous_effect(ContinuousEffect::pump(bear, bear, 2, 0, Duration::EndOfTurn));
    s.add_continuous_effect(ContinuousEffect::filtered_pump(bear,
        ObjectFilter { power_condition: Some(PtCondition::Eq(4)), ..ObjectFilter::creature() },
        1, 0, Duration::EndOfTurn));
    // 2 base + 2 pump + 1 filtered pump + 1 counter; the filter sees 4.
    assert_eq!(s.computed_power(bear), Some(6));
    assert_eq!(s.computed_toughness(bear), Some(3));
}

#[test]
fn hazoret_hand_changes_gate_attacks_and_blocks_only_for_itself() {
    let r = registry();
    for p in [0, 1] {
        let (mut s, hazoret) = cast(table(p), &r, p, "Hazoret the Fervent");
        let bear = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        let foe = add(&mut s, &r, 1 - p, "Grizzly Bears", Zone::Battlefield);
        let a = add(&mut s, &r, p, "Forest", Zone::Hand(p));
        assert!(!s.cant_attack(hazoret));
        let b = add(&mut s, &r, p, "Forest", Zone::Hand(p));
        assert!(s.cant_attack(hazoret));
        assert!(s.cant_block(hazoret));
        assert!(!s.blocker_eligible(hazoret, foe));
        assert!(!s.cant_attack(bear));
        s.move_object_to_zone(b, Zone::Graveyard(p), MoveCause::SpellResolution);
        assert!(!s.cant_attack(hazoret));
        assert!(!s.cant_block(hazoret));
        assert!(s.blocker_eligible(hazoret, foe));
        s.move_object_to_zone(a, Zone::Graveyard(p), MoveCause::SpellResolution);
        assert!(!s.cant_attack(hazoret));
    }
}

#[test]
fn adanto_pump_tracks_attacking_and_ends_with_combat() {
    let r = registry();
    for p in [0, 1] {
        let (mut s, adanto) = cast(table(p), &r, p, "Adanto Vanguard");
        assert_eq!(s.computed_power(adanto), Some(1));
        s.objects.get_mut(adanto).unwrap().status.summoning_sick = false;
        s.turn.phase = Phase::Combat;
        s.turn.step = Step::DeclareAttackers;
        s.combat = Some(CombatState::new());
        s.enter_declare_attackers();
        let action = legal_actions(&s, &r).into_iter().find(|a| matches!(a,
            Action::DeclareAttackers { attackers } if attackers == &vec![AttackerDeclaration {
                attacker: adanto, defending: DefendingEntity::Player(1 - p),
            }])).expect("Adanto attack");
        let mut s = step(s, action, &r).0;
        assert_eq!(s.computed_power(adanto), Some(3));
        s.combat = None;
        assert_eq!(s.computed_power(adanto), Some(1));
    }
}

#[test]
fn xenagos_devotion_changes_its_type_and_another_excludes_the_source() {
    let r = registry();
    for p in [0, 1] {
        let (mut s, god) = cast(table(p), &r, p, "Xenagos, God of Revels");
        assert!(!s.is_creature_now(god));
        let other = add(&mut s, &r, 1 - p, "Xenagos, God of Revels", Zone::Hand(1 - p));
        let other = s.move_object_to_zone(other, Zone::Battlefield, MoveCause::SpellResolution).unwrap();
        s = finish(step(s, Action::PassPriority, &r).0, &r);
        assert!(!s.is_creature_now(other));
        let mut bears = Vec::new();
        for _ in 0..5 { bears.push(add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield)); }
        assert!(s.is_creature_now(god));
        assert!(!s.is_creature_now(other));
        for _ in 0..5 { add(&mut s, &r, 1 - p, "Grizzly Bears", Zone::Battlefield); }
        assert!(s.is_creature_now(other));
        let req = &r.get(r.card_id_by_name("Xenagos, God of Revels").unwrap())
            .unwrap().triggered_abilities.iter().find(|t| t.id == 1).unwrap().target_requirements[0];
        assert!(!req.matches_choice(&TargetChoice::Object(god), &s, god, p));
        assert!(req.matches_choice(&TargetChoice::Object(bears[0]), &s, god, p));
        let candidates = req.filter.enumerate_legal(&s, god, p);
        assert!(!candidates.contains(&TargetChoice::Object(god)));
        assert!(candidates.contains(&TargetChoice::Object(bears[0])));
        s.move_object_to_zone(bears[0], Zone::Graveyard(p), MoveCause::SpellResolution);
        assert!(!s.is_creature_now(god));
        assert!(s.is_creature_now(other));
    }
}
