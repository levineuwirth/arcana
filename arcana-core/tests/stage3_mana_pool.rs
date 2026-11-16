//! A1.5ce: each player's mana pool empties as every step and phase ends
//! (CR 500.5, 106.4). It is a turn-based action: no stack, no priority, no
//! event. Within a step nothing changes: floated mana pays for a later
//! spell, and a mana ability activated in a resolving effect's payment
//! window (A1.5cd) pays the cost it was tapped for.
use arcana_core::actions::{Action, ChoiceKind, ChoiceResponse, OptionalPaymentKind};
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::engine::{advance_phase, step};
use arcana_core::events::{GameEvent, MoveCause};
use arcana_core::legal_actions::legal_actions;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, GameObject, ObjectId};
use arcana_core::priority::SpecialAction;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{ColorSet, ManaColor, PlayerId, PtValue, TypeLine};
use arcana_core::zones::Zone;

const TOLL: &str = "Ce Test Toll";
const BOLT: &str = "Lightning Bolt";
const BEARS: &str = "Grizzly Bears";

/// The catalog plus Toll, a sorcery: "You may pay {2}. If you do, draw a
/// card."
fn registry() -> CardRegistry {
    let mut r = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut r);
    let toll = r.interner_mut().intern(TOLL);
    r.register(CardDefinition::new(toll, Characteristics {
        name: toll,
        mana_cost: Some(ManaCost::parse("{U}").unwrap()),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    }).with_spell_ability(SpellAbilityDef {
        text: "You may pay {2}. If you do, draw a card.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: toll_resolve,
    }));
    r
}

fn toll_resolve(_: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: entry.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}").unwrap()),
        then: Box::new(Effect::DrawCards { player: entry.controller, count: 1 }),
        else_effect: None,
    }]
}

/// `active`'s turn at `(phase, step)`, `holder` with priority, and a
/// library of Islands for each seat.
fn table(r: &CardRegistry, active: PlayerId, holder: PlayerId, phase: Phase, at: Step) -> GameState {
    let mut s = GameState::new(2, 20261115);
    s.turn.active_player = active;
    s.turn.phase = phase;
    s.turn.step = at;
    s.priority.give_to(holder);
    for p in [0, 1] {
        for _ in 0..12 { add(&mut s, r, p, "Island", Zone::Library(p)); }
    }
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

/// Onto the battlefield through the zone mover, ready to act this turn.
fn enter(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str) -> ObjectId {
    let id = add(s, r, p, name, Zone::Hand(p));
    let id = s.move_object_to_zone(id, Zone::Battlefield, MoveCause::SpellResolution)
        .expect("entered");
    s.objects.get_mut(id).unwrap().status.summoning_sick = false;
    id
}

fn act(s: GameState, r: &CardRegistry, pick: impl Fn(&Action) -> bool) -> GameState {
    let chosen = legal_actions(&s, r).into_iter().find(|a| pick(a))
        .unwrap_or_else(|| panic!("no such action at {:?}/{:?}; pending {:?}",
            s.turn.phase, s.turn.step, s.pending_choice));
    step(s, chosen, r).0
}

/// Tap `land` for mana, as whoever holds priority (or the pending choice).
fn tap(s: GameState, r: &CardRegistry, land: ObjectId) -> GameState {
    act(s, r, |a| matches!(a, Action::ActivateAbility { source, .. } if *source == land))
}

fn here(s: &GameState) -> (u32, Phase, Step) {
    (s.turn.turn_number, s.turn.phase, s.turn.step)
}

/// Until `stop` holds: pass priority, or failing that take the first answer
/// that is not a concession (an empty declaration).
fn until(mut s: GameState, r: &CardRegistry, stop: impl Fn(&GameState) -> bool) -> GameState {
    let from = here(&s);
    for _ in 0..40 {
        if stop(&s) { return s; }
        let legal = legal_actions(&s, r);
        let pick = legal.iter().find(|a| a.is_pass())
            .or_else(|| legal.iter().find(|a| !a.is_concede()))
            .unwrap_or_else(|| panic!("nothing to do at {:?}", here(&s))).clone();
        s = step(s, pick, r).0;
    }
    panic!("from {from:?}: never reached");
}

/// To the next step, by passes and default answers.
fn next_step(s: GameState, r: &CardRegistry) -> GameState {
    let from = here(&s);
    until(s, r, |s| here(s) != from)
}

fn offered(s: &GameState, r: &CardRegistry, pick: impl Fn(&Action) -> bool) -> bool {
    legal_actions(s, r).iter().any(pick)
}

fn pools(s: &GameState) -> [usize; 2] {
    [s.player(0).mana_pool.total(), s.player(1).mana_pool.total()]
}

/// One red mana into each pool, as if each player had tapped a land.
fn float_both(s: &mut GameState) {
    for p in [0, 1] { s.player_mut(p).mana_pool.add_mana(ManaColor::Red, 1, 0); }
    assert_eq!(pools(s), [1, 1]);
}

/// Every seat for the player who floats the mana and for the active player.
fn arrangements() -> [(PlayerId, PlayerId); 4] {
    [(0, 0), (0, 1), (1, 0), (1, 1)]
}

/// The tranche-3 probe's case inverted: a Mountain tapped in the first main
/// phase gives mana that is there for the rest of that phase and gone as
/// combat begins.
#[test]
fn mana_floated_in_the_first_main_phase_is_gone_as_combat_begins() {
    let r = registry();
    for (floater, active) in arrangements() {
        let mut s = table(&r, active, floater, Phase::PreCombatMain, Step::Main);
        let mountain = enter(&mut s, &r, floater, "Mountain");
        let s = tap(s, &r, mountain);
        assert_eq!(s.player(floater).mana_pool.total(), 1, "the tap floats one mana");
        assert_eq!((s.turn.phase, s.turn.step), (Phase::PreCombatMain, Step::Main));
        let s = next_step(s, &r);
        assert_eq!((s.turn.phase, s.turn.step), (Phase::Combat, Step::BeginCombat));
        assert_eq!(pools(&s), [0, 0], "floater {floater}, active {active}: the phase's end empties the pool");
        assert!(s.objects.get(mountain).unwrap().is_tapped(), "the land stays tapped");
    }
}

#[test]
fn mana_floated_in_the_upkeep_is_gone_at_the_draw_step() {
    let r = registry();
    for (floater, active) in arrangements() {
        let mut s = table(&r, active, floater, Phase::Beginning, Step::Upkeep);
        let mountain = enter(&mut s, &r, floater, "Mountain");
        let s = tap(s, &r, mountain);
        assert_eq!(s.player(floater).mana_pool.total(), 1);
        let s = next_step(s, &r);
        assert_eq!((s.turn.phase, s.turn.step), (Phase::Beginning, Step::Draw));
        assert_eq!(pools(&s), [0, 0], "floater {floater}, active {active}: a step's end empties the pool");
    }
}

/// The cleanup step is seen through its discard: with eight cards in hand
/// the active player is asked to discard during cleanup, and mana from the
/// end step is already gone.
#[test]
fn mana_added_in_the_end_step_is_gone_at_cleanup() {
    let r = registry();
    for (floater, active) in arrangements() {
        let mut s = table(&r, active, floater, Phase::Ending, Step::End);
        for _ in 0..8 { add(&mut s, &r, active, "Island", Zone::Hand(active)); }
        let mountain = enter(&mut s, &r, floater, "Mountain");
        let s = tap(s, &r, mountain);
        assert_eq!(s.player(floater).mana_pool.total(), 1);
        let turn = s.turn.turn_number;
        let s = until(s, &r, |s| s.priority.in_special_action());
        assert_eq!(here(&s), (turn, Phase::Ending, Step::Cleanup), "still this turn's cleanup");
        assert!(matches!(s.priority.special_action, Some(SpecialAction::DiscardToHandSize)));
        assert_eq!(pools(&s), [0, 0], "floater {floater}, active {active}: the end step's end empties the pool");
    }
}

/// Nothing moves within a step: two mana floated in the main phase pay for
/// two spells, the second after the first has resolved.
#[test]
fn floated_mana_still_pays_within_its_step() {
    let r = registry();
    for me in [0, 1] {
        let mut s = table(&r, me, me, Phase::PreCombatMain, Step::Main);
        let lands = [enter(&mut s, &r, me, "Mountain"), enter(&mut s, &r, me, "Mountain")];
        let bolts = [add(&mut s, &r, me, BOLT, Zone::Hand(me)), add(&mut s, &r, me, BOLT, Zone::Hand(me))];
        let mut s = tap(tap(s, &r, lands[0]), &r, lands[1]);
        assert_eq!(s.player(me).mana_pool.total(), 2);
        for (i, bolt) in bolts.into_iter().enumerate() {
            s = act(s, &r, |a| matches!(a, Action::CastSpell { object_id, targets, .. }
                if *object_id == bolt && format!("{targets:?}").contains(&format!("Player({})", 1 - me))));
            assert_eq!(s.player(me).mana_pool.total(), 1 - i, "one mana pays for each");
            // Both players pass and the spell resolves, in the same step.
            s = act(s, &r, |a| a.is_pass());
            s = act(s, &r, |a| a.is_pass());
            assert!(s.stack_is_empty());
            assert_eq!((s.turn.phase, s.turn.step), (Phase::PreCombatMain, Step::Main));
            assert_eq!(s.player(1 - me).life, 20 - 3 * (i as i32 + 1));
            assert_eq!(s.player(me).mana_pool.total(), 1 - i, "a resolution ends no step");
        }
    }
}

/// A1.5cd's window still pays: three Islands tapped while Toll asks for
/// {2} pay it, and the third mana, left unspent, stays for the rest of the
/// step and goes when it ends.
#[test]
fn mana_tapped_in_the_payment_window_pays_and_the_rest_goes_with_the_step() {
    let r = registry();
    for me in [0, 1] {
        let mut s = table(&r, me, me, Phase::PreCombatMain, Step::Main);
        let cast_with = enter(&mut s, &r, me, "Island");
        let window: Vec<ObjectId> = (0..3).map(|_| enter(&mut s, &r, me, "Island")).collect();
        let toll = add(&mut s, &r, me, TOLL, Zone::Hand(me));
        let hand = |s: &GameState| s.objects.count_in_zone(Zone::Hand(me));
        let s = tap(s, &r, cast_with);
        let s = act(s, &r, |a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == toll));
        assert_eq!(s.player(me).mana_pool.total(), 0, "Toll took the one mana floated for it");
        let before = hand(&s);
        let mut s = act(act(s, &r, |a| a.is_pass()), &r, |a| a.is_pass());
        let asked = s.pending_choice.as_ref().expect("Toll asks").id;
        assert!(matches!(s.pending_choice.as_ref().unwrap().kind,
            ChoiceKind::OptionalCost { cost: OptionalPaymentKind::Mana(_) }));
        let pay = |a: &Action| matches!(a, Action::SubmitResolutionChoice {
            response: ChoiceResponse::OptionalCost { pay: true }, .. });
        assert!(!legal_actions(&s, &r).iter().any(pay), "nothing is floating yet");
        for land in window { s = tap(s, &r, land); }
        assert_eq!(s.pending_choice.as_ref().map(|c| c.id), Some(asked), "the choice waits");
        assert_eq!(s.player(me).mana_pool.total(), 3);
        let s = act(s, &r, pay);
        assert!(s.pending_choice.is_none());
        assert_eq!(hand(&s), before + 1, "paid, so the card is drawn");
        assert_eq!(s.player(me).mana_pool.total(), 1, "the unspent mana stays for the step");
        assert_eq!((s.turn.phase, s.turn.step), (Phase::PreCombatMain, Step::Main));
        let s = next_step(s, &r);
        assert_eq!((s.turn.phase, s.turn.step), (Phase::Combat, Step::BeginCombat));
        assert_eq!(pools(&s), [0, 0], "seat {me}: and goes when the step ends");
    }
}

/// An attack by `active`'s Bears, with mana floated by both players in
/// every step from the first main phase through the combat phase: each
/// step's end empties both pools. Returns the steps entered.
fn combat_walk(r: &CardRegistry, active: PlayerId, first_strike: bool) -> Vec<(Phase, Step)> {
    let mut s = table(r, active, active, Phase::PreCombatMain, Step::Main);
    let bears = enter(&mut s, r, active, BEARS);
    if first_strike {
        s.objects.get_mut(bears).unwrap().characteristics.keywords.push(KeywordAbility::FirstStrike);
    }
    let turn = s.turn.turn_number;
    let mut seen = Vec::new();
    for _ in 0..40 {
        float_both(&mut s);
        let from = (s.turn.phase, s.turn.step);
        // The attack is declared with the Bears; every other decision is a
        // pass or its first answer.
        s = next_step(s, r);
        assert_eq!(pools(&s), [0, 0], "active {active}: {from:?} ended with mana in both pools");
        if s.turn.step == Step::DeclareAttackers {
            s = act(s, r, |a| matches!(a, Action::DeclareAttackers { attackers } if attackers.len() == 1));
            // The declaration costs nothing, and makes no mana either.
            assert_eq!(pools(&s), [0, 0]);
        }
        seen.push((s.turn.phase, s.turn.step));
        if s.turn.phase != Phase::Combat { break; }
    }
    assert_eq!(s.turn.turn_number, turn);
    // The attack connects: no blocker was declared.
    assert_eq!(s.player(1 - active).life, 18);
    seen
}

#[test]
fn mana_floated_in_combat_is_gone_at_each_step_without_first_strike() {
    let r = registry();
    for active in [0, 1] {
        assert_eq!(combat_walk(&r, active, false), [
            (Phase::Combat, Step::BeginCombat),
            (Phase::Combat, Step::DeclareAttackers),
            (Phase::Combat, Step::DeclareBlockers),
            // No first-strike damage step: the skip is still a step's end.
            (Phase::Combat, Step::CombatDamageRegular),
            (Phase::Combat, Step::EndCombat),
            (Phase::PostCombatMain, Step::Main),
        ]);
    }
}

#[test]
fn mana_floated_in_combat_is_gone_at_each_step_with_first_strike() {
    let r = registry();
    for active in [0, 1] {
        assert_eq!(combat_walk(&r, active, true), [
            (Phase::Combat, Step::BeginCombat),
            (Phase::Combat, Step::DeclareAttackers),
            (Phase::Combat, Step::DeclareBlockers),
            (Phase::Combat, Step::CombatDamage),
            (Phase::Combat, Step::CombatDamageRegular),
            (Phase::Combat, Step::EndCombat),
            (Phase::PostCombatMain, Step::Main),
        ]);
    }
}

/// An extra combat phase begins as the first one's end of combat step
/// ends: mana floated there does not reach it, nor does mana floated in
/// the extra combat reach the second main phase.
#[test]
fn mana_floated_at_end_of_combat_does_not_reach_an_extra_combat() {
    let r = registry();
    for active in [0, 1] {
        let one = [
            (Phase::Combat, Step::BeginCombat),
            (Phase::Combat, Step::DeclareAttackers),
            (Phase::Combat, Step::DeclareBlockers),
            (Phase::Combat, Step::CombatDamageRegular),
            (Phase::Combat, Step::EndCombat),
        ];
        let mut expected = one.to_vec();
        expected.extend(one);
        expected.push((Phase::PostCombatMain, Step::Main));
        // No attack is declared in either combat; the steps are the same.
        let mut s = table(&r, active, active, Phase::PreCombatMain, Step::Main);
        s.turn.queue_extra_combat();
        let mut seen = Vec::new();
        for _ in 0..40 {
            float_both(&mut s);
            let from = (s.turn.phase, s.turn.step);
            s = next_step(s, &r);
            assert_eq!(pools(&s), [0, 0], "active {active}: {from:?} ended with mana in both pools");
            seen.push((s.turn.phase, s.turn.step));
            if s.turn.phase != Phase::Combat { break; }
        }
        assert_eq!(seen, expected);
    }
}

/// A trampler's damage is assigned by its controller, and the step ends
/// only once it is: mana floated in the damage step is still there while
/// the assignment is asked for, and gone when it has been dealt.
#[test]
fn a_damage_assignment_ends_its_step_and_empties_the_pools() {
    let r = registry();
    for active in [0, 1] {
        let defender = 1 - active;
        let mut s = table(&r, active, active, Phase::Combat, Step::BeginCombat);
        s.begin_combat();
        let attacker = enter(&mut s, &r, active, BEARS);
        {
            let c = &mut s.objects.get_mut(attacker).unwrap().characteristics;
            c.power = Some(PtValue::Fixed(5));
            c.toughness = Some(PtValue::Fixed(5));
            c.keywords.push(KeywordAbility::Trample);
        }
        let blocker = enter(&mut s, &r, defender, BEARS);
        let s = next_step(s, &r);
        assert_eq!(s.turn.step, Step::DeclareAttackers);
        let s = act(s, &r, |a| matches!(a, Action::DeclareAttackers { attackers } if attackers.len() == 1));
        let s = until(s, &r, |s| offered(s, &r, |a| matches!(a, Action::DeclareBlockers { .. })));
        assert_eq!(s.turn.step, Step::DeclareBlockers);
        let s = act(s, &r, |a| matches!(a, Action::DeclareBlockers { blockers }
            if blockers.len() == 1 && blockers[0].blocker == blocker));
        let mut s = next_step(s, &r);
        assert_eq!(s.turn.step, Step::CombatDamageRegular);
        float_both(&mut s);
        let assign = |a: &Action| matches!(a, Action::AssignCombatDamage { .. });
        let s = until(s, &r, |s| offered(s, &r, assign));
        assert_eq!(s.turn.step, Step::CombatDamageRegular, "the assignment is asked within the step");
        assert_eq!(pools(&s), [1, 1], "active {active}: the step has not ended");
        let s = act(s, &r, assign);
        assert_eq!(s.turn.step, Step::EndCombat);
        assert_eq!(pools(&s), [0, 0], "active {active}: dealing the damage ended the step");
    }
}

/// Every transition of the step machine, walked with `advance_phase` from
/// the untap step to the next turn's upkeep with mana in both pools at
/// each: none leaves any. This includes the untap step, where no player
/// could have added mana by an action, and the cleanup step.
#[test]
fn every_transition_of_a_turn_empties_the_pools() {
    let r = registry();
    for active in [0, 1] {
        let mut s = table(&r, active, active, Phase::Beginning, Step::Untap);
        let turn = s.turn.turn_number;
        let mut left = Vec::new();
        for _ in 0..14 {
            if s.turn.turn_number != turn && s.turn.step == Step::Upkeep { break; }
            float_both(&mut s);
            let from = (s.turn.phase, s.turn.step);
            advance_phase(&mut s, &r);
            assert_ne!((s.turn.phase, s.turn.step), from);
            assert_eq!(pools(&s), [0, 0], "active {active}: leaving {from:?}");
            left.push(from);
        }
        assert_eq!(left, [
            (Phase::Beginning, Step::Untap),
            (Phase::Beginning, Step::Upkeep),
            (Phase::Beginning, Step::Draw),
            (Phase::PreCombatMain, Step::Main),
            (Phase::Combat, Step::BeginCombat),
            (Phase::Combat, Step::DeclareAttackers),
            (Phase::Combat, Step::DeclareBlockers),
            (Phase::Combat, Step::CombatDamageRegular),
            (Phase::Combat, Step::EndCombat),
            (Phase::PostCombatMain, Step::Main),
            (Phase::Ending, Step::End),
            (Phase::Ending, Step::Cleanup),
            (Phase::Beginning, Step::Untap),
        ]);
        assert_eq!(s.turn.turn_number, turn + 1);
        assert_eq!(s.active_player(), 1 - active);
    }
}

/// An extra turn starts with empty pools like any other.
#[test]
fn mana_from_the_end_step_does_not_reach_an_extra_turn() {
    let r = registry();
    for active in [0, 1] {
        let mut s = table(&r, active, active, Phase::Ending, Step::End);
        s.turn.queue_extra_turn(active);
        float_both(&mut s);
        let turn = s.turn.turn_number;
        let s = next_step(s, &r);
        assert_eq!(here(&s), (turn + 1, Phase::Beginning, Step::Upkeep));
        assert_eq!(s.active_player(), active, "the extra turn is the same player's");
        assert_eq!(pools(&s), [0, 0]);
    }
}

/// Emptying is silent. A step that ends with empty pools emits the new
/// step's two events and nothing else, and a game that differs from it only
/// in mana floating in both pools is, after that step's end, the same game:
/// same events, same everything.
#[test]
fn an_empty_pool_at_a_step_end_changes_nothing() {
    let r = registry();
    for active in [0, 1] {
        let plain = table(&r, active, active, Phase::PreCombatMain, Step::Main);
        let mut floating = plain.clone();
        float_both(&mut floating);
        let logged = plain.event_log.len();
        let (plain, floating) = (next_step(plain, &r), next_step(floating, &r));
        assert_eq!(plain.event_log[logged..], [
            GameEvent::PhaseBegins { phase: Phase::Combat },
            GameEvent::StepBegins { step: Step::BeginCombat },
        ]);
        assert_eq!(pools(&floating), [0, 0]);
        assert_eq!(format!("{plain:?}"), format!("{floating:?}"),
            "active {active}: the emptied game is the game that never floated");
    }
}
