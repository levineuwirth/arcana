//! Stage 3: a card's own free alternative cost in an ordinary priority
//! window. Catalog clients always come through register_all. Synthetic
//! variants exercise the shared machinery beyond Once Upon a Time.
use arcana_core::actions::{Action, AdditionalCostPayment, CastFace, CastModifier, ChoiceResponse};
use arcana_core::effects::Effect;
use arcana_core::engine::step;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::legal_actions::{casts_without_paying, legal_actions};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, GameObject, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry, CastRestriction, FreeCastCondition};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CounterKind, ManaColor, PlayerId, TypeLine};
use arcana_core::zones::Zone;

const ONCE: &str = "Once Upon a Time";
fn registry() -> CardRegistry {
    let mut r = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut r);
    r
}
fn table(p: PlayerId) -> GameState {
    let mut s = GameState::new(2, 20261112);
    s.turn.active_player = p;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(p);
    s
}
fn add(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str, zone: Zone) -> ObjectId {
    let card = r.card_id_by_name(name).unwrap_or_else(|| panic!("unregistered {name}"));
    let id = s.allocate_object_id();
    s.objects.insert(GameObject::new(id, p, zone, card,
        r.get(card).unwrap().initial_characteristics().clone()));
    if zone == Zone::Library(p) { s.player_mut(p).library_top_to_bottom.push(id); }
    id
}
fn hand(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str) -> ObjectId {
    add(s, r, p, name, Zone::Hand(p))
}
fn free(s: &GameState, r: &CardRegistry, id: ObjectId) -> Vec<Action> {
    legal_actions(s, r).into_iter().filter(|a| matches!(a,
        Action::CastSpell { object_id, cast_modifier: CastModifier::WithoutPayingManaCost { .. }, .. }
        if *object_id == id)).collect()
}
fn paid(s: &GameState, r: &CardRegistry, id: ObjectId) -> Option<Action> {
    legal_actions(s, r).into_iter().find(|a| matches!(a,
        Action::CastSpell { object_id, cast_modifier: CastModifier::None, .. } if *object_id == id))
}
fn targets_player(a: &Action, p: PlayerId) -> bool {
    matches!(a, Action::CastSpell { targets, .. } if targets.targets.iter().any(|t|
        matches!(t, TargetChoice::Player(q) | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(q)) if *q == p)))
}
fn settle(mut s: GameState, r: &CardRegistry) -> GameState {
    for _ in 0..80 {
        if s.stack_is_empty() && s.pending_choice.is_none() { return s; }
        let legal = legal_actions(&s, r);
        let action = legal.iter().find(|a| a.is_pass())
            .or_else(|| legal.iter().find(|a| !matches!(a, Action::Concede))).unwrap().clone();
        s = step(s, action, r).0;
    }
    panic!("unsettled stack")
}
fn variant(r: &mut CardRegistry, base: &str, name: &str) {
    let mut def = r.get(r.card_id_by_name(base).unwrap()).unwrap().clone();
    let sym = r.interner_mut().intern(name);
    def.name = sym;
    def.base_characteristics.name = sym;
    r.register(def.with_free_cast_from_hand(FreeCastCondition::FirstSpellOfGame));
}
fn tax(s: &mut GameState, r: &CardRegistry, p: PlayerId, delta: i32) {
    let source = add(s, r, p, "Grizzly Bears", Zone::Battlefield);
    s.add_continuous_effect(ContinuousEffect::spell_cost_modifier(source,
        ObjectFilter::default(), ControllerConstraint::You, delta,
        Duration::WhileSourceOnBattlefield));
}

#[test]
fn first_spell_is_free_in_either_seat_without_mana_and_resolves() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(1 - p);
        s.turn.phase = Phase::Beginning;
        s.turn.step = Step::Upkeep;
        s.priority.give_to(p);
        let once = hand(&mut s, &r, p, ONCE);
        let second = hand(&mut s, &r, p, ONCE);
        let bear = add(&mut s, &r, p, "Grizzly Bears", Zone::Library(p));
        for _ in 0..5 { add(&mut s, &r, p, "Lightning Bolt", Zone::Library(p)); }
        let action = free(&s, &r, once).pop().expect("first cast with zero mana");
        // The action itself is replayable; no ephemeral permission is needed.
        let replay: Action = serde_json::from_str(&serde_json::to_string(&action).unwrap()).unwrap();
        let replayed = step(s.clone(), replay, &r).0;
        let (mut s, _) = step(s, action, &r);
        assert_eq!(s.player(p).spells_cast_this_game, 1);
        assert_eq!(s.player(1-p).spells_cast_this_game, 0);
        assert_eq!(s.player(p), replayed.player(p));
        assert_eq!(s.stack.len(), 1);
        assert_eq!(s.stack[0].mana_spent, 0);
        assert_eq!(s.stack[0].cast_from_zone, Zone::Hand(p));
        assert!(free(&s, &r, second).is_empty(), "a second copy is not a first spell");
        s = step(s, Action::PassPriority, &r).0;
        s = step(s, Action::PassPriority, &r).0;
        let pick = legal_actions(&s, &r).into_iter().find(|a| matches!(a,
            Action::SubmitResolutionChoice { response: ChoiceResponse::PickCards { picked }, .. }
                if picked == &vec![bear])).expect("the creature is offered from the top five");
        s = step(s, pick, &r).0;
        assert_eq!(s.player(p).library_top_to_bottom.len(), 5);
        assert!(s.objects.iter().any(|o| o.zone == Zone::Hand(p)
            && o.card_id == r.card_id_by_name("Grizzly Bears").unwrap()));
        assert!(s.objects.iter().any(|o| o.zone == Zone::Graveyard(p)
            && o.card_id == r.card_id_by_name(ONCE).unwrap()));
    }
}

#[test]
fn paid_cast_remains_optional_and_costs_one_generic_and_green() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let once = hand(&mut s, &r, p, ONCE);
        let later = hand(&mut s, &r, p, ONCE);
        s.player_mut(p).mana_pool.add_mana(ManaColor::Green, 2, 0);
        assert_eq!(free(&s, &r, once).len(), 1);
        let action = paid(&s, &r, once).expect("printed cost is {1}{G}");
        let mut s = step(s, action, &r).0;
        assert_eq!(s.stack[0].mana_spent, 2);
        assert_eq!(s.stack[0].characteristics().unwrap().mana_cost, Some(ManaCost::parse("{1}{G}").unwrap()));
        assert_eq!(s.player(p).mana_pool.total(), 0);
        assert_eq!(s.player(p).spells_cast_this_game, 1);
        s = settle(s, &r);
        s.priority.give_to(p);
        s.player_mut(p).mana_pool.add_mana(ManaColor::Green, 2, 0);
        assert!(free(&s, &r, later).is_empty());
        assert!(paid(&s, &r, later).is_some(), "ordinary cost works after a first spell");
    }
}

#[test]
fn another_spell_counts_at_cast_even_when_countered_and_history_survives_turns() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        for q in [0, 1] { for _ in 0..10 { add(&mut s, &r, q, "Forest", Zone::Library(q)); } }
        let once = hand(&mut s, &r, p, ONCE);
        let opponents_once = hand(&mut s, &r, 1-p, ONCE);
        let expired_offer = free(&s, &r, once).pop().unwrap();
        let bolt = hand(&mut s, &r, p, "Lightning Bolt");
        s.player_mut(p).mana_pool.add_mana(ManaColor::Red, 1, 0);
        let action = legal_actions(&s, &r).into_iter().find(|a|
            matches!(a, Action::CastSpell { object_id, .. } if *object_id == bolt)
                && targets_player(a, 1-p)).unwrap();
        s = step(s, action, &r).0;
        assert_eq!(s.player(p).spells_cast_this_game, 1);
        assert!(free(&s, &r, once).is_empty(), "cast, not resolution, spends the permission");
        let checked = step(s.clone(), expired_offer, &r).0;
        assert_eq!(checked.stack.len(), 1, "application refuses an expired alternative cost");
        s.priority.give_to(1-p);
        assert!(!free(&s, &r, opponents_once).is_empty(), "history belongs to the caster");
        let entry = s.stack.pop().unwrap();
        s.counter_resolved_spell(entry);
        assert_eq!(s.player(1-p).life, 20);
        let next_turn = s.turn.turn_number + 2;
        for _ in 0..150 {
            if s.turn.turn_number >= next_turn { break; }
            let legal = legal_actions(&s, &r);
            let action = legal.iter().find(|a| a.is_pass())
                .or_else(|| legal.iter().find(|a| !matches!(a, Action::Concede)))
                .expect("a turn decision").clone();
            s = step(s, action, &r).0;
        }
        assert!(s.turn.turn_number >= next_turn);
        s.priority.give_to(p);
        assert_eq!(s.player(p).spells_cast_this_game, 1);
        assert!(free(&s, &r, once).is_empty());
        assert!(free(&s.clone(), &r, once).is_empty());
    }
}

#[test]
fn lands_mana_abilities_and_stack_copies_are_not_casts() {
    let r = registry();
    let mut s = table(0);
    let forest = hand(&mut s, &r, 0, "Forest");
    let once = hand(&mut s, &r, 0, ONCE);
    let play = legal_actions(&s, &r).into_iter().find(|a|
        matches!(a, Action::PlayLand { object_id, .. } if *object_id == forest)).unwrap();
    s = step(s, play, &r).0;
    let mana = legal_actions(&s, &r).into_iter().find(|a|
        matches!(a, Action::ActivateAbility { .. })).unwrap();
    s = step(s, mana, &r).0;
    assert_eq!(s.player(0).spells_cast_this_game, 0);
    let a = free(&s, &r, once).pop().unwrap();
    s = step(s, a, &r).0;
    let original = s.stack[0].id;
    Effect::CopySpell { target: original }.execute(&mut s);
    assert_eq!(s.stack.len(), 2);
    assert_eq!(s.player(0).spells_cast_this_game, 1, "a stack copy is not cast");
}

#[test]
fn a_free_cast_from_exile_also_spends_the_first_spell_permission() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let once = hand(&mut s, &r, p, ONCE);
        let bears = add(&mut s, &r, p, "Grizzly Bears", Zone::Exile);
        Effect::CastFromExileFree { player: p, target: bears }.execute(&mut s);
        assert_eq!(s.player(p).spells_cast_this_game, 1);
        assert!(free(&s, &r, once).is_empty());
    }
}

#[test]
fn ordinary_free_cast_keeps_card_type_timing() {
    let mut r = registry();
    variant(&mut r, "Lava Spike", "Free Sorcery");
    for p in [0, 1] {
        let mut s = table(p);
        let sorcery = hand(&mut s, &r, p, "Free Sorcery");
        let once = hand(&mut s, &r, p, ONCE);
        assert!(!free(&s, &r, sorcery).is_empty());
        s.turn.active_player = 1-p;
        assert!(free(&s, &r, sorcery).is_empty());
        assert!(!free(&s, &r, once).is_empty());
        s.turn.active_player = p;
        s.turn.phase = Phase::Beginning;
        s.turn.step = Step::Upkeep;
        assert!(free(&s, &r, sorcery).is_empty());
        assert!(!free(&s, &r, once).is_empty());
        s.turn.phase = Phase::PreCombatMain;
        s.turn.step = Step::Main;
        let bolt = hand(&mut s, &r, 1-p, "Lightning Bolt");
        s.player_mut(1-p).mana_pool.add_mana(ManaColor::Red, 1, 0);
        s.priority.give_to(1-p);
        let a = paid(&s, &r, bolt).unwrap();
        s = step(s, a, &r).0;
        s.priority.give_to(p);
        assert!(free(&s, &r, sorcery).is_empty(), "nonempty stack");
        assert!(!free(&s, &r, once).is_empty(), "an instant can respond");
    }
}

#[test]
fn teferi_restricts_the_free_instant_in_both_seats() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let once = hand(&mut s, &r, p, ONCE);
        let teferi = add(&mut s, &r, 1-p, "Teferi, Time Raveler", Zone::Battlefield);
        s.objects.get_mut(teferi).unwrap().add_counters(CounterKind::Loyalty, 4);
        assert!(!free(&s, &r, once).is_empty(), "own main with empty stack");
        s.turn.active_player = 1-p;
        assert!(free(&s, &r, once).is_empty());
        s.turn.active_player = p;
        s.turn.phase = Phase::Beginning;
        s.turn.step = Step::Upkeep;
        assert!(free(&s, &r, once).is_empty());
        s.objects.get_mut(teferi).unwrap().controller = p;
        assert!(!free(&s, &r, once).is_empty(), "own Teferi does not restrict");
    }
}

#[test]
fn cant_cast_and_per_turn_limits_still_apply() {
    let mut r = registry();
    let name = r.interner_mut().intern("Free Cast Test Restriction");
    r.register(CardDefinition::new(name, Characteristics {
        name, types: TypeLine::ARTIFACT.into(), ..Default::default()
    }).with_cant_cast(CastRestriction::Noncreature));
    for p in [0, 1] {
        let mut s = table(p);
        let once = hand(&mut s, &r, p, ONCE);
        let blocker = add(&mut s, &r, p, "Free Cast Test Restriction", Zone::Battlefield);
        assert!(free(&s, &r, once).is_empty());
        s.objects.get_mut(blocker).unwrap().controller = 1-p;
        assert!(!free(&s, &r, once).is_empty());
        let source = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        s.add_continuous_effect(ContinuousEffect::spell_cast_limit(source,
            ObjectFilter::default(), ControllerConstraint::You, 0,
            Duration::WhileSourceOnBattlefield));
        assert!(free(&s, &r, once).is_empty());
    }
}

#[test]
fn free_cast_pays_taxes_and_applies_reductions() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let once = hand(&mut s, &r, p, ONCE);
        tax(&mut s, &r, p, 2);
        assert!(free(&s, &r, once).is_empty(), "free does not bypass a tax");
        s.player_mut(p).mana_pool.add_mana(ManaColor::Red, 1, 0);
        assert!(free(&s, &r, once).is_empty());
        tax(&mut s, &r, p, -1);
        let a = free(&s, &r, once).pop().expect("tax reduced to one");
        s = step(s, a, &r).0;
        assert_eq!(s.stack[0].mana_spent, 1);
        assert_eq!(s.player(p).mana_pool.total(), 0);
    }
}

#[test]
fn optional_kicker_is_offered_paid_and_reduced_after_the_base_cost_is_waived() {
    let mut r = registry();
    variant(&mut r, "Burst Lightning", "Free Burst");
    for p in [0, 1] {
        let mut s = table(p);
        let burst = hand(&mut s, &r, p, "Free Burst");
        let kicked = |a: &Action| matches!(a, Action::CastSpell { additional_costs, .. }
            if additional_costs.contains(&AdditionalCostPayment::Kicker));
        assert!(free(&s, &r, burst).iter().any(|a| !kicked(a)));
        assert!(!free(&s, &r, burst).iter().any(kicked));
        s.player_mut(p).mana_pool.add_mana(ManaColor::Green, 3, 0);
        assert!(!free(&s, &r, burst).iter().any(kicked));
        tax(&mut s, &r, p, -1);
        let offers = free(&s, &r, burst);
        assert!(offers.iter().any(|a| !kicked(a)));
        let a = offers.into_iter().find(|a| kicked(a) && targets_player(a, 1-p)).unwrap();
        s = step(s, a, &r).0;
        assert_eq!(s.stack[0].mana_spent, 3);
        assert!(s.stack[0].kicked);
        assert_eq!(s.player(p).mana_pool.total(), 0);
        s = settle(s, &r);
        assert_eq!(s.player(1-p).life, 16);
    }
}

#[test]
fn x_is_zero_and_the_printed_mana_cost_is_not_required() {
    let mut r = registry();
    variant(&mut r, "Stonecoil Serpent", "Free Serpent");
    // A missing mana cost is unpayable normally but may be bypassed.
    let name = r.interner_mut().intern("Free Costless Spell");
    r.register(CardDefinition::new(name, Characteristics {
        name, types: TypeLine::ARTIFACT.into(), ..Default::default()
    }).with_free_cast_from_hand(FreeCastCondition::FirstSpellOfGame));
    let mut s = table(0);
    let serpent = hand(&mut s, &r, 0, "Free Serpent");
    let costless = hand(&mut s, &r, 0, "Free Costless Spell");
    s.player_mut(0).mana_pool.add_mana(ManaColor::Green, 5, 0);
    assert!(!free(&s, &r, costless).is_empty());
    assert!(paid(&s, &r, costless).is_none());
    let offers = free(&s, &r, serpent);
    assert_eq!(offers.len(), 1);
    assert!(matches!(&offers[0], Action::CastSpell { x_value: Some(0), mana_payment, cost_reductions, .. }
        if mana_payment.assignments.is_empty() && cost_reductions == &Default::default()));
    s = step(s, offers[0].clone(), &r).0;
    assert_eq!(s.stack[0].x_value, Some(0));
    assert_eq!(s.player(0).mana_pool.total(), 5);
}

#[test]
fn a_hand_permission_does_not_grant_other_cards_zones_or_faces() {
    let mut r = registry();
    variant(&mut r, "Bonecrusher Giant", "Free Giant");
    for p in [0, 1] {
        let mut s = table(p);
        let once = hand(&mut s, &r, p, ONCE);
        let template = free(&s, &r, once).pop().unwrap();
        let giant = hand(&mut s, &r, p, "Free Giant");
        assert!(free(&s, &r, giant).iter().all(|a| matches!(a,
            Action::CastSpell { cast_modifier: CastModifier::WithoutPayingManaCost { face: CastFace::Main }, .. })));
        assert!(!free(&s, &r, giant).is_empty());
        let vanilla = hand(&mut s, &r, p, "Grizzly Bears");
        for id in [vanilla,
            add(&mut s, &r, p, ONCE, Zone::Exile),
            add(&mut s, &r, p, ONCE, Zone::Graveyard(p)),
            add(&mut s, &r, p, ONCE, Zone::Library(p)),
            hand(&mut s, &r, 1-p, ONCE)]
        {
            assert!(free(&s, &r, id).is_empty());
            let mut forged = template.clone();
            if let Action::CastSpell { object_id, .. } = &mut forged { *object_id = id; }
            let checked = step(s.clone(), forged, &r).0;
            assert!(checked.stack_is_empty(), "application also refuses this permission for {id}");
        }
        let mut forged = template.clone();
        if let Action::CastSpell { object_id, cast_modifier, .. } = &mut forged {
            *object_id = giant;
            *cast_modifier = CastModifier::WithoutPayingManaCost { face: CastFace::Adventure };
        }
        assert!(step(s.clone(), forged, &r).0.stack_is_empty());
        assert!(casts_without_paying(&s, &r, p, once).is_empty(), "Queller helper stays exile-only");
        let exiled = add(&mut s, &r, p, "Grizzly Bears", Zone::Exile);
        let exiled_once = add(&mut s, &r, p, ONCE, Zone::Exile);
        // A resolution permission still works after this player has cast:
        // free-hand eligibility does not govern the existing Queller path.
        s = step(s, template, &r).0;
        assert!(!casts_without_paying(&s, &r, p, exiled).is_empty());
        assert!(!casts_without_paying(&s, &r, p, exiled_once).is_empty());
        assert!(casts_without_paying(&s, &r, 1-p, exiled).is_empty());
    }
}

#[test]
fn a_free_modal_spell_chooses_only_its_modes_targets() {
    let mut r = registry();
    variant(&mut r, "Boros Charm", "Free Charm");
    for p in [0, 1] {
        let mut s = table(p);
        let charm = hand(&mut s, &r, p, "Free Charm");
        let mode = |a: &Action| match a {
            Action::CastSpell { modes, .. } => modes[0].mode_indices[0],
            _ => panic!("not a cast"),
        };
        let offers = free(&s, &r, charm);
        assert!(offers.iter().any(|a| mode(a) == 0 && targets_player(a, 1-p)));
        assert!(offers.iter().any(|a| matches!(a, Action::CastSpell { targets, .. }
            if mode(a) == 1 && targets.is_empty())));
        assert!(!offers.iter().any(|a| mode(a) == 2), "no creature for that mode");
        let bear = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        assert!(free(&s, &r, charm).iter().any(|a| matches!(a, Action::CastSpell { targets, .. }
            if mode(a) == 2 && targets.targets == vec![TargetChoice::Object(bear)])));
        let a = offers.into_iter().find(|a| mode(a) == 0 && targets_player(a, 1-p)).unwrap();
        s = settle(step(s, a, &r).0, &r);
        assert_eq!(s.player(1-p).life, 16);
    }
}
