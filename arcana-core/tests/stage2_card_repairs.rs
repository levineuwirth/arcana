//! Stage 2 card sweep: exercise the definitions used by the full registry.

use arcana_core::actions::Action;
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
    let mut reg = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut reg);
    reg
}

fn table(player: PlayerId) -> GameState {
    let mut s = GameState::new(2, 0);
    s.turn.active_player = player;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(player);
    s
}

fn add(s: &mut GameState, reg: &CardRegistry, owner: PlayerId, name: &str, zone: Zone) -> ObjectId {
    let card = reg.card_id_by_name(name).expect("registered card");
    let id = s.allocate_object_id();
    s.objects.insert(GameObject::new(
        id,
        owner,
        zone,
        card,
        reg.get(card).unwrap().initial_characteristics().clone(),
    ));
    id
}

fn activate(
    s: GameState,
    reg: &CardRegistry,
    source: ObjectId,
    index: usize,
    target: Option<ObjectId>,
) -> GameState {
    let a = legal_actions(&s, reg)
        .into_iter()
        .find(|a| {
            matches!(a,
            Action::ActivateAbility { source: id, ability_index, targets, .. }
            if *id == source && *ability_index == index && match target {
                Some(t) => targets.targets == vec![TargetChoice::Object(t)],
                None => targets.targets.is_empty(),
            })
        })
        .expect("legal activation");
    step(s, a, reg).0
}

fn resolve(mut s: GameState, reg: &CardRegistry) -> GameState {
    for _ in 0..20 {
        if s.stack_is_empty() && s.pending_choice.is_none() {
            return s;
        }
        let pass = legal_actions(&s, reg)
            .into_iter()
            .find(Action::is_pass)
            .expect("no resolution choice expected");
        s = step(s, pass, reg).0;
    }
    panic!("stack did not resolve");
}

#[test]
fn chandra_mana_plus_one_uses_the_stack_and_produces_two_red() {
    let reg = registry();
    for player in [0, 1] {
        let mut s = table(player);
        let chandra = add(
            &mut s,
            &reg,
            player,
            "Chandra, Torch of Defiance",
            Zone::Battlefield,
        );
        s.objects
            .get_mut(chandra)
            .unwrap()
            .add_counters(CounterKind::Loyalty, 4);
        let s = activate(s, &reg, chandra, 1, None);
        assert!(!s.stack_is_empty(), "loyalty ability allows responses");
        assert!(
            s.player(player).mana_pool.is_empty(),
            "no mana before resolution"
        );
        assert_eq!(
            s.objects
                .get(chandra)
                .unwrap()
                .count_counters(CounterKind::Loyalty),
            5
        );
        let s = resolve(s, &reg);
        assert_eq!(
            s.player(player)
                .mana_pool
                .count_unrestricted_color(ManaColor::Red),
            2
        );
        assert_eq!(s.player(player).mana_pool.total(), 2);
        assert!(s.player(1 - player).mana_pool.is_empty());
        assert!(s
            .player(player)
            .mana_pool
            .iter()
            .all(|m| m.source == chandra));
        assert!(
            !legal_actions(&s, &reg).iter().any(|a| matches!(a,
            Action::ActivateAbility { source, .. } if *source == chandra)),
            "loyalty ability cannot be activated twice this turn"
        );
    }
}

#[test]
fn chandra_mana_plus_one_survives_its_source_leaving() {
    let reg = registry();
    let mut s = table(0);
    let chandra = add(
        &mut s,
        &reg,
        0,
        "Chandra, Torch of Defiance",
        Zone::Battlefield,
    );
    s.objects
        .get_mut(chandra)
        .unwrap()
        .add_counters(CounterKind::Loyalty, 4);
    let mut s = activate(s, &reg, chandra, 1, None);
    s.move_object_to_zone(chandra, Zone::Graveyard(0), MoveCause::SpellResolution);
    let s = resolve(s, &reg);
    assert_eq!(
        s.player(0)
            .mana_pool
            .count_unrestricted_color(ManaColor::Red),
        2
    );
}

fn ooze_activation(
    reg: &CardRegistry,
    player: PlayerId,
    card: &str,
) -> (GameState, ObjectId, ObjectId) {
    let mut s = table(player);
    let ooze = add(&mut s, reg, player, "Scavenging Ooze", Zone::Battlefield);
    let target = add(&mut s, reg, 1 - player, card, Zone::Graveyard(1 - player));
    s.player_mut(player)
        .mana_pool
        .add_mana(ManaColor::Green, 1, ooze);
    (activate(s, reg, ooze, 0, Some(target)), ooze, target)
}

#[test]
fn ooze_exiles_noncreatures_without_counter_or_life() {
    let reg = registry();
    for player in [0, 1] {
        for name in ["Forest", "Opt"] {
            let (s, ooze, target) = ooze_activation(&reg, player, name);
            let s = resolve(s, &reg);
            assert!(s.objects.get(target).is_none());
            assert!(s
                .objects
                .objects_in_zone(Zone::Exile)
                .any(|o| o.card_id == reg.card_id_by_name(name).unwrap()));
            assert_eq!(
                s.objects
                    .get(ooze)
                    .unwrap()
                    .count_counters(CounterKind::PlusOnePlusOne),
                0
            );
            assert_eq!(s.player(player).life, 20);
        }
    }
}

#[test]
fn ooze_rewards_exiling_a_creature_card_including_an_artifact_creature() {
    let reg = registry();
    for player in [0, 1] {
        for name in ["Grizzly Bears", "Stonecoil Serpent"] {
            let (s, ooze, target) = ooze_activation(&reg, player, name);
            let s = resolve(s, &reg);
            assert!(s.objects.get(target).is_none());
            assert!(s
                .objects
                .objects_in_zone(Zone::Exile)
                .any(|o| o.card_id == reg.card_id_by_name(name).unwrap()));
            assert_eq!(
                s.objects
                    .get(ooze)
                    .unwrap()
                    .count_counters(CounterKind::PlusOnePlusOne),
                1
            );
            assert_eq!(s.player(player).life, 21);
            assert_eq!(s.player(1 - player).life, 20);
        }
    }
}

#[test]
fn ooze_gets_no_reward_if_the_target_leaves_before_resolution() {
    let reg = registry();
    let (mut s, ooze, target) = ooze_activation(&reg, 0, "Grizzly Bears");
    s.move_object_to_zone(target, Zone::Hand(1), MoveCause::SpellResolution);
    let s = resolve(s, &reg);
    assert_eq!(
        s.objects
            .get(ooze)
            .unwrap()
            .count_counters(CounterKind::PlusOnePlusOne),
        0
    );
    assert_eq!(s.player(0).life, 20);
    assert_eq!(s.objects.objects_in_zone(Zone::Exile).count(), 0);
}

#[test]
fn ooze_still_exiles_and_gains_life_after_its_source_leaves() {
    let reg = registry();
    let (mut s, ooze, target) = ooze_activation(&reg, 0, "Grizzly Bears");
    s.move_object_to_zone(ooze, Zone::Graveyard(0), MoveCause::SpellResolution);
    let s = resolve(s, &reg);
    assert!(s.objects.get(target).is_none());
    assert_eq!(s.objects.objects_in_zone(Zone::Exile).count(), 1);
    assert_eq!(s.player(0).life, 21);
}
