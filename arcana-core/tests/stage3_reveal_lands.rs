//! Reveal-land witnesses: "As this land enters, you may reveal a [type] or
//! [type] card from your hand. If you don't, this land enters tapped." is a
//! typed entry decision answered before the land moves. The registered
//! clients are Port Town, Game Trail, Foreboding Ruins and Choked Estuary;
//! two synthetic lands cover shapes no registered card has.
use arcana_core::actions::{Action, ChoiceContext, ChoiceFollowUp, ChoiceKind, ChoiceResponse};
use arcana_core::engine::step;
use arcana_core::events::GameEvent;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::{Characteristics, GameObject, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::render::render_action;
use arcana_core::state::{GameResult, GameState};
use arcana_core::targets::ObjectFilter;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{PlayerId, SubtypeSet, TypeLine};
use arcana_core::view::view_state;
use arcana_core::zones::Zone;
use arcana_core::EngineYield;

const TOWN: &str = "Port Town";
const POOL: &str = "Breeding Pool";
const BEARS: &str = "Grizzly Bears";
const ISLE: &str = "Reveal Test Isle";
const TOLL: &str = "Reveal Test Toll";
const MIMIC: &str = "Reveal Test Mimic";
const GLADE: &str = "Reveal Test Glade";

/// (land, the two basic types it asks for, a basic that is neither).
const CLIENTS: [(&str, &str, &str, &str); 4] = [
    ("Port Town", "Plains", "Island", "Swamp"),
    ("Game Trail", "Mountain", "Forest", "Plains"),
    ("Foreboding Ruins", "Swamp", "Mountain", "Island"),
    ("Choked Estuary", "Island", "Swamp", "Forest"),
];

/// The catalog plus:
/// * Isle, an Island that asks for an Island card: it matches its own
///   clause, and must not be offered as its own reveal;
/// * Toll, a land with both decisions, the reveal first: reveal a Plains
///   card or enter tapped, and pay 2 life or enter tapped.
/// * Mimic and Glade, land creatures that are every creature type; only
///   Glade has a land type.
fn registry() -> CardRegistry {
    let mut r = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut r);
    let island = r.interner_mut().intern("Island");
    let plains = r.interner_mut().intern("Plains");
    let isle = r.interner_mut().intern(ISLE);
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(island);
    r.register(CardDefinition::new(isle, Characteristics {
        name: isle, types: TypeLine::LAND.into(), subtypes, ..Default::default()
    }).with_enters_with(EntersWithSpec::TappedUnlessReveal {
        filter: ObjectFilter::new().with_subtypes_any(vec![island]),
    }));
    let toll = r.interner_mut().intern(TOLL);
    r.register(CardDefinition::new(toll, Characteristics {
        name: toll, types: TypeLine::LAND.into(), ..Default::default()
    }).with_enters_with(EntersWithSpec::TappedUnlessReveal {
        filter: ObjectFilter::new().with_subtypes_any(vec![plains]),
    }).with_enters_with(EntersWithSpec::TappedUnlessPayLife { life: 2 }));
    // Two land creatures that are every creature type: Mimic has no land
    // type, Glade is a Plains.
    for (card, land_types) in [(MIMIC, SubtypeSet::default()), (GLADE, {
        let mut plains_only = SubtypeSet::default();
        plains_only.0.insert(plains);
        plains_only
    })] {
        let name = r.interner_mut().intern(card);
        r.register(CardDefinition::new(name, Characteristics {
            name,
            types: TypeLine::new().with(TypeLine::LAND).with(TypeLine::CREATURE),
            subtypes: land_types,
            every_creature_type: true,
            ..Default::default()
        }));
    }
    r
}

fn table(player: PlayerId) -> GameState {
    let mut s = GameState::new(2, 20261113);
    s.turn.active_player = player;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(player);
    s
}

fn add(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str, zone: Zone) -> ObjectId {
    let card = r.card_id_by_name(name).unwrap_or_else(|| panic!("unregistered: {name}"));
    let id = s.allocate_object_id();
    s.objects.insert(GameObject::new(id, p, zone, card,
        r.get(card).unwrap().initial_characteristics().clone()));
    if let Zone::Library(owner) = zone {
        s.player_mut(owner).library_top_to_bottom.push(id);
    }
    id
}

fn play(s: GameState, r: &CardRegistry, land: ObjectId) -> GameState {
    let action = legal_actions(&s, r).into_iter()
        .find(|a| matches!(a, Action::PlayLand { object_id, .. } if *object_id == land))
        .expect("legal land play");
    step(s, action, r).0
}

/// The reveal prompt's options in the order offered: `Some(card)` to
/// reveal it, `None` to decline.
fn reveal_options(s: &GameState, r: &CardRegistry) -> Vec<Option<ObjectId>> {
    legal_actions(s, r).into_iter().filter_map(|a| match a {
        Action::SubmitResolutionChoice { response: ChoiceResponse::PickCards { picked }, .. } => {
            assert!(picked.len() <= 1);
            Some(picked.first().copied())
        }
        _ => None,
    }).collect()
}

fn reveal(s: GameState, r: &CardRegistry, card: Option<ObjectId>) -> GameState {
    let wanted: Vec<ObjectId> = card.into_iter().collect();
    let action = legal_actions(&s, r).into_iter().find(|a| matches!(a,
        Action::SubmitResolutionChoice { response: ChoiceResponse::PickCards { picked }, .. }
            if *picked == wanted)).expect("offered reveal answer");
    step(s, action, r).0
}

fn pay(s: GameState, r: &CardRegistry, pay: bool) -> GameState {
    let action = legal_actions(&s, r).into_iter().find(|a| matches!(a,
        Action::SubmitResolutionChoice { response: ChoiceResponse::OptionalCost { pay: p }, .. }
            if *p == pay)).expect("offered payment answer");
    step(s, action, r).0
}

/// The land waits where it was, its controller is asked about it, and
/// nothing has entered.
fn assert_waiting(s: &GameState, land: ObjectId, from: Zone, player: PlayerId) {
    assert_eq!(s.objects.get(land).unwrap().zone, from, "entry has not happened");
    let pending = s.pending_choice.as_ref().expect("an entry decision");
    assert_eq!(pending.choosing_player, player);
    assert!(matches!(pending.context,
        ChoiceContext::EnteringBattlefield { object_id } if object_id == land));
    assert!(!s.event_log.iter().any(|e| matches!(e,
        GameEvent::ZoneChange { object_id, .. } if *object_id == land)));
}

fn on_battlefield<'a>(s: &'a GameState, r: &CardRegistry, name: &str) -> &'a GameObject {
    let card = r.card_id_by_name(name).unwrap();
    s.objects.objects_in_zone(Zone::Battlefield).find(|o| o.card_id == card)
        .unwrap_or_else(|| panic!("{name} is not on the battlefield"))
}

fn known_to(s: &GameState, card: ObjectId) -> Vec<bool> {
    (0..2).map(|p| s.player(p).known_cards.contains(&card)).collect()
}

#[test]
fn each_reveal_land_offers_its_two_types_to_either_seat() {
    let r = registry();
    for (land_name, first, second, neither) in CLIENTS {
        for player in [0, 1] {
            for which in [first, second] {
                let mut s = table(player);
                let other = add(&mut s, &r, player, neither, Zone::Hand(player));
                let card = add(&mut s, &r, player, which, Zone::Hand(player));
                let land = add(&mut s, &r, player, land_name, Zone::Hand(player));
                let s = play(s, &r, land);
                assert_waiting(&s, land, Zone::Hand(player), player);
                assert!(matches!(s.pending_choice.as_ref().unwrap().kind,
                    ChoiceKind::PickCards { min: 0, max: 1, .. }));
                assert_eq!(reveal_options(&s, &r), vec![Some(card), None],
                    "{land_name}, seat {player}: {which} and the decline, not {neither}");
                assert!(!legal_actions(&s, &r).iter().any(Action::is_pass));

                // Revealing: untapped, the card stays in hand, both know it.
                let shown = reveal(s.clone(), &r, Some(card));
                let entered = on_battlefield(&shown, &r, land_name);
                assert_eq!((entered.controller, entered.is_tapped()), (player, false),
                    "{land_name}, seat {player}: revealed a {which}");
                assert_eq!(shown.objects.get(card).unwrap().zone, Zone::Hand(player));
                assert_eq!(known_to(&shown, card), vec![true, true]);
                assert_eq!(known_to(&shown, other), vec![false, false],
                    "only the revealed card is shown");
                assert!(shown.pending_choice.is_none());

                // Declining: tapped, and nothing is shown.
                let kept = reveal(s, &r, None);
                assert!(on_battlefield(&kept, &r, land_name).is_tapped(),
                    "{land_name}, seat {player}: declined");
                assert_eq!(known_to(&kept, card), vec![false, false]);
            }
        }
    }
}

#[test]
fn with_no_matching_card_only_the_decline_is_offered() {
    let r = registry();
    for (land_name, _, _, neither) in CLIENTS {
        for player in [0, 1] {
            let mut s = table(player);
            add(&mut s, &r, player, neither, Zone::Hand(player));
            add(&mut s, &r, player, BEARS, Zone::Hand(player));
            // The opponent's matching card is not in this player's hand.
            add(&mut s, &r, 1 - player, CLIENTS.iter()
                .find(|c| c.0 == land_name).unwrap().1, Zone::Hand(1 - player));
            let land = add(&mut s, &r, player, land_name, Zone::Hand(player));
            let s = play(s, &r, land);
            assert_waiting(&s, land, Zone::Hand(player), player);
            assert_eq!(reveal_options(&s, &r), vec![None], "{land_name}, seat {player}");
            let s = reveal(s, &r, None);
            assert!(on_battlefield(&s, &r, land_name).is_tapped());
        }
    }
}

#[test]
fn a_card_matches_by_its_own_subtypes_and_each_distinct_card_is_offered_once() {
    let r = registry();
    let mut s = table(0);
    // Two Plains are one option; a Hallowed Fountain is a Plains Island
    // card; a Stomping Ground is neither.
    let plains = add(&mut s, &r, 0, "Plains", Zone::Hand(0));
    let ground = add(&mut s, &r, 0, "Stomping Ground", Zone::Hand(0));
    let fountain = add(&mut s, &r, 0, "Hallowed Fountain", Zone::Hand(0));
    let _second_plains = add(&mut s, &r, 0, "Plains", Zone::Hand(0));
    let island = add(&mut s, &r, 0, "Island", Zone::Hand(0));
    let land = add(&mut s, &r, 0, TOWN, Zone::Hand(0));
    let _ = ground;
    let s = play(s, &r, land);
    assert_eq!(reveal_options(&s, &r),
        vec![Some(plains), Some(fountain), Some(island), None],
        "one option per distinct card, in object order");
    let s = reveal(s, &r, Some(fountain));
    assert!(!on_battlefield(&s, &r, TOWN).is_tapped());
    assert_eq!(known_to(&s, fountain), vec![true, true]);

    // Game Trail takes the Stomping Ground and not the Fountain.
    let mut s = table(0);
    add(&mut s, &r, 0, "Hallowed Fountain", Zone::Hand(0));
    let ground = add(&mut s, &r, 0, "Stomping Ground", Zone::Hand(0));
    let land = add(&mut s, &r, 0, "Game Trail", Zone::Hand(0));
    assert_eq!(reveal_options(&play(s, &r, land), &r), vec![Some(ground), None]);
}

#[test]
fn the_entering_card_is_never_its_own_reveal() {
    let r = registry();
    let mut s = table(0);
    let isle = add(&mut s, &r, 0, ISLE, Zone::Hand(0));
    let s = play(s, &r, isle);
    assert_eq!(reveal_options(&s, &r), vec![None]);
    // With another Isle in hand, that one can be shown.
    let mut s = table(0);
    let isle = add(&mut s, &r, 0, ISLE, Zone::Hand(0));
    let other = add(&mut s, &r, 0, ISLE, Zone::Hand(0));
    let s = play(s, &r, isle);
    assert_eq!(reveal_options(&s, &r), vec![Some(other), None]);
}

/// Put `ids`, in that order, onto the battlefield together through the
/// effect route: a pick whose follow-up moves the whole selection.
fn put_together(mut s: GameState, r: &CardRegistry, ids: &[ObjectId], tapped: bool)
    -> GameState
{
    s.pending_choice_follow_up = Some(ChoiceFollowUp::MoveToBattlefield {
        controller: 0, tapped, shuffle_library_owner: None,
    });
    s.push_pending_choice(0, ChoiceContext::ResolvingStack(999), ChoiceKind::PickCards {
        candidates: ids.to_vec(), min: ids.len() as u32, max: ids.len() as u32,
    });
    let action = legal_actions(&s, r).into_iter().find(|a| matches!(a,
        Action::SubmitResolutionChoice { response: ChoiceResponse::PickCards { picked }, .. }
            if picked == ids)).expect("legal selection");
    step(s, action, r).0
}

/// The order in which objects arrived on the battlefield, by the ids they
/// left with.
fn entry_order(s: &GameState) -> Vec<ObjectId> {
    s.event_log.iter().filter_map(|e| match e {
        GameEvent::ZoneChange { object_id, to: Zone::Battlefield, .. } => Some(*object_id),
        _ => None,
    }).collect()
}

#[test]
fn a_batch_asks_every_decision_in_order_before_anything_enters() {
    let r = registry();
    for town_first in [true, false] {
        let mut s = table(0);
        let plains = add(&mut s, &r, 0, "Plains", Zone::Hand(0));
        // A selection is offered in object order, so the cards are made in
        // the order the batch is to have.
        let bears = add(&mut s, &r, 0, BEARS, Zone::Graveyard(0));
        let (town, pool) = if town_first {
            let town = add(&mut s, &r, 0, TOWN, Zone::Graveyard(0));
            (town, add(&mut s, &r, 0, POOL, Zone::Graveyard(0)))
        } else {
            let pool = add(&mut s, &r, 0, POOL, Zone::Graveyard(0));
            (add(&mut s, &r, 0, TOWN, Zone::Graveyard(0)), pool)
        };
        let last = add(&mut s, &r, 0, BEARS, Zone::Graveyard(0));
        let ids = if town_first {
            vec![bears, town, pool, last]
        } else {
            vec![bears, pool, town, last]
        };
        let s = put_together(s, &r, &ids, false);
        let life = s.player(0).life;
        let s = if town_first {
            // The reveal first, then the life payment; nothing has moved.
            assert_waiting(&s, town, Zone::Graveyard(0), 0);
            assert_eq!(reveal_options(&s, &r), vec![Some(plains), None]);
            let s = reveal(s, &r, Some(plains));
            assert_eq!(s.objects.objects_in_zone(Zone::Battlefield).count(), 0);
            assert_waiting(&s, pool, Zone::Graveyard(0), 0);
            pay(s, &r, true)
        } else {
            assert_waiting(&s, pool, Zone::Graveyard(0), 0);
            let s = pay(s, &r, true);
            assert_eq!(s.objects.objects_in_zone(Zone::Battlefield).count(), 0);
            assert_waiting(&s, town, Zone::Graveyard(0), 0);
            assert_eq!(reveal_options(&s, &r), vec![Some(plains), None]);
            reveal(s, &r, Some(plains))
        };
        assert!(s.pending_choice.is_none());
        assert_eq!(entry_order(&s), ids, "the batch keeps its order");
        assert_eq!(s.player(0).life, life - 2);
        assert!(!on_battlefield(&s, &r, TOWN).is_tapped());
        assert!(!on_battlefield(&s, &r, POOL).is_tapped());
        assert_eq!(known_to(&s, plains), vec![true, true]);
    }
}

#[test]
fn a_land_an_effect_puts_onto_the_battlefield_asks_its_controller() {
    let r = registry();
    for (land_name, first, _, _) in CLIENTS {
        let mut s = table(0);
        let card = add(&mut s, &r, 0, first, Zone::Hand(0));
        let land = add(&mut s, &r, 0, land_name, Zone::Library(0));
        let s = put_together(s, &r, &[land], false);
        assert_waiting(&s, land, Zone::Library(0), 0);
        assert_eq!(reveal_options(&s, &r), vec![Some(card), None], "{land_name}");
        let s = reveal(s, &r, None);
        assert!(on_battlefield(&s, &r, land_name).is_tapped(), "{land_name}");
    }
}

#[test]
fn revealing_never_untaps_what_another_requirement_taps() {
    use arcana_core::replacement::*;
    let r = registry();

    // The instruction itself puts the land onto the battlefield tapped.
    let mut s = table(0);
    let plains = add(&mut s, &r, 0, "Plains", Zone::Hand(0));
    let town = add(&mut s, &r, 0, TOWN, Zone::Library(0));
    let s = reveal(put_together(s, &r, &[town], true), &r, Some(plains));
    assert!(on_battlefield(&s, &r, TOWN).is_tapped(), "the instruction's own tap");

    // Another permanent's "lands enter tapped".
    let mut s = table(0);
    s.add_replacement_effect(ReplacementEffect {
        source: 0,
        id: 0,
        condition: ReplacementCondition::WouldEnterBattlefield {
            object_filter: ObjectFilter::default(),
        },
        kind: ReplacementKind::EtbTapped,
        is_self_replacement: false,
        duration: ReplacementDuration::Permanent,
        state_gate: None,
    });
    let plains = add(&mut s, &r, 0, "Plains", Zone::Hand(0));
    let town = add(&mut s, &r, 0, TOWN, Zone::Hand(0));
    let s = reveal(play(s, &r, town), &r, Some(plains));
    assert!(on_battlefield(&s, &r, TOWN).is_tapped(), "an external replacement");
}

#[test]
fn each_clause_of_one_land_is_answered_on_its_own() {
    let r = registry();
    for (show, life, tapped) in [
        (true, true, false),
        (true, false, true),
        (false, true, true),
        (false, false, true),
    ] {
        let mut s = table(0);
        let plains = add(&mut s, &r, 0, "Plains", Zone::Hand(0));
        let toll = add(&mut s, &r, 0, TOLL, Zone::Hand(0));
        let s = play(s, &r, toll);
        assert_waiting(&s, toll, Zone::Hand(0), 0);
        let s = reveal(s, &r, show.then_some(plains));
        // The second decision is asked before the land moves, whatever the
        // first answer was.
        assert_waiting(&s, toll, Zone::Hand(0), 0);
        assert!(matches!(s.pending_choice.as_ref().unwrap().kind,
            ChoiceKind::OptionalCost { .. }));
        let s = pay(s, &r, life);
        assert_eq!(on_battlefield(&s, &r, TOLL).is_tapped(), tapped,
            "reveal {show}, pay {life}");
        assert_eq!(s.player(0).life, if life { 18 } else { 20 });
    }
}

#[test]
fn conceding_at_the_reveal_ends_the_game() {
    let r = registry();
    for player in [0, 1] {
        let mut s = table(player);
        let plains = add(&mut s, &r, player, "Plains", Zone::Hand(player));
        let land = add(&mut s, &r, player, TOWN, Zone::Hand(player));
        let s = play(s, &r, land);
        assert_waiting(&s, land, Zone::Hand(player), player);
        let action = legal_actions(&s, &r).into_iter().find(Action::is_concede).unwrap();
        let (s, result) = step(s, action, &r);
        assert_eq!(s.result, Some(GameResult::Win(1 - player)));
        assert!(matches!(result,
            EngineYield::GameOver(GameResult::Win(winner)) if winner == 1 - player));
        assert_eq!(s.objects.get(land).unwrap().zone, Zone::Hand(player), "no entry");
        assert_eq!(known_to(&s, plains), vec![false, false], "nothing was shown");
    }
}

#[test]
fn the_prompt_and_the_labels_say_what_the_reveal_is_for() {
    let r = registry();
    let expected = [
        ("Port Town", "Reveal a Plains or Island card from your hand? \
            If you don't, Port Town enters tapped."),
        ("Game Trail", "Reveal a Mountain or Forest card from your hand? \
            If you don't, Game Trail enters tapped."),
        ("Foreboding Ruins", "Reveal a Swamp or Mountain card from your hand? \
            If you don't, Foreboding Ruins enters tapped."),
        ("Choked Estuary", "Reveal an Island or Swamp card from your hand? \
            If you don't, Choked Estuary enters tapped."),
    ];
    for ((land_name, first, _, _), (name, prompt)) in CLIENTS.into_iter().zip(expected) {
        assert_eq!(land_name, name);
        let mut s = table(0);
        add(&mut s, &r, 0, first, Zone::Hand(0));
        let land = add(&mut s, &r, 0, land_name, Zone::Hand(0));
        let s = play(s, &r, land);
        let legal = legal_actions(&s, &r);
        let view = view_state(&s, &r, 0, &legal);
        assert_eq!(view.prompt.as_deref(), Some(prompt));
        assert!(view.choice.is_none(), "a hand pick stays a text choice");
        let labels: Vec<String> = legal.iter()
            .filter(|a| !a.is_concede())
            .map(|a| render_action(a, &s, &r)).collect();
        assert_eq!(labels, vec![format!("Reveal {first}"), "Don't reveal".to_string()]);
        // The other seat is not the one deciding.
        assert_eq!(view_state(&s, &r, 1, &legal).prompt, None);
    }
}

#[test]
fn every_creature_type_is_no_land_type() {
    // CR 702.73a, 205.3: a changeling has every creature type, and
    // Plains is a land type. Neither registered changeling answers any
    // of the four lands, in either seat; the land has only the decline
    // and enters tapped.
    let r = registry();
    for owner in [0, 1] {
        for (land_name, first, _, _) in CLIENTS {
            for changeling in ["Changeling Outcast", "Universal Automaton", MIMIC] {
                let mut s = table(owner);
                let impostor = add(&mut s, &r, owner, changeling, Zone::Hand(owner));
                let land = add(&mut s, &r, owner, land_name, Zone::Hand(owner));
                let s = play(s, &r, land);
                assert_eq!(reveal_options(&s, &r), vec![None],
                    "seat {owner}: {changeling} for {land_name}");
                let s = reveal(s, &r, None);
                assert!(on_battlefield(&s, &r, land_name).is_tapped());
                assert!(!s.player(1 - owner).known_cards.contains(&impostor));
            }
            // The real thing beside the impostor is still the one option.
            let mut s = table(owner);
            add(&mut s, &r, owner, "Changeling Outcast", Zone::Hand(owner));
            let basic = add(&mut s, &r, owner, first, Zone::Hand(owner));
            let land = add(&mut s, &r, owner, land_name, Zone::Hand(owner));
            let s = play(s, &r, land);
            assert_eq!(reveal_options(&s, &r), vec![Some(basic), None],
                "seat {owner}: {first} for {land_name}");
        }
    }
}

#[test]
fn a_land_creature_of_every_creature_type_answers_by_its_land_types() {
    // The matcher's wildcard is for creature types. A land creature that
    // is every creature type is a Plains only if it has that land type.
    let r = registry();
    for owner in [0, 1] {
        let mut s = table(owner);
        add(&mut s, &r, owner, MIMIC, Zone::Hand(owner));
        let glade = add(&mut s, &r, owner, GLADE, Zone::Hand(owner));
        let town = add(&mut s, &r, owner, TOWN, Zone::Hand(owner));
        let s = play(s, &r, town);
        assert_eq!(reveal_options(&s, &r), vec![Some(glade), None]);
        let s = reveal(s, &r, Some(glade));
        assert!(!on_battlefield(&s, &r, TOWN).is_tapped());
    }
}

#[test]
fn a_card_entering_from_the_same_hand_can_be_the_reveal() {
    // Shadows over Innistrad release notes, on this cycle: a land card
    // entering from the same hand at the same time may be revealed. It
    // is in hand as the choice is made (CR 614.12a).
    let r = registry();
    for owner in [0, 1] {
        for town_first in [true, false] {
            let mut s = table(owner);
            let (plains, town) = if town_first {
                let town = add(&mut s, &r, owner, TOWN, Zone::Hand(owner));
                (add(&mut s, &r, owner, "Plains", Zone::Hand(owner)), town)
            } else {
                let plains = add(&mut s, &r, owner, "Plains", Zone::Hand(owner));
                (plains, add(&mut s, &r, owner, TOWN, Zone::Hand(owner)))
            };
            s.pending_choice_follow_up = Some(ChoiceFollowUp::MoveToBattlefield {
                controller: owner, tapped: false, shuffle_library_owner: None,
            });
            s.push_pending_choice(owner, ChoiceContext::ResolvingStack(999), ChoiceKind::PickCards {
                candidates: vec![plains, town], min: 2, max: 2,
            });
            let both = legal_actions(&s, &r).into_iter().find(|a| matches!(a,
                Action::SubmitResolutionChoice { response: ChoiceResponse::PickCards { picked }, .. }
                    if picked.len() == 2)).unwrap();
            let s = step(s, both, &r).0;
            assert!(reveal_options(&s, &r).contains(&Some(plains)), "seat {owner}");
            let s = reveal(s, &r, Some(plains));
            assert!(!on_battlefield(&s, &r, TOWN).is_tapped());
            assert_eq!(s.objects.count_in_zone(Zone::Battlefield), 2);
        }
    }
}
