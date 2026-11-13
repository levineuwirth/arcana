//! A1.5br witnesses: printed statics installed by the engine as their
//! permanent enters, using the registered clients and three synthetic
//! statics where no registered card has the shape.
use arcana_core::actions::{Action, ChoiceResponse};
use arcana_core::combat::{AttackerDeclaration, CombatState, DefendingEntity};
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::engine::step;
use arcana_core::events::{GameEvent, MoveCause};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::legal_actions::legal_actions;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, GameObject, ObjectId, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{
    ColorSet, ManaColor, PlayerId, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

const WISP: &str = "Br Test Wisp";
const STEWARD: &str = "Br Test Steward";
const ANIMATOR: &str = "Br Test Animator";
const RELIC: &str = "Br Test Relic";

/// The catalog plus four synthetic cards:
/// * Wisp, a 0/0 Spirit that lives only under a Spirit lord;
/// * Steward, a 0/0 whose own static gives creatures you control +1/+1,
///   itself included, so it lives only if the static exists on entry;
/// * Animator, one static in two layers: noncreature artifacts you control
///   are creatures (layer 4) with base power and toughness 2/2 (layer 7b);
/// * Relic, a plain artifact for the Animator to act on.
fn registry() -> CardRegistry {
    let mut r = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut r);
    let spirit = r.interner_mut().intern("Spirit");
    let creature = |r: &mut CardRegistry, name: &str, subtypes: SubtypeSet| {
        let name = r.interner_mut().intern(name);
        Characteristics {
            name,
            mana_cost: Some(ManaCost::parse("{U}").unwrap()),
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            ..Default::default()
        }
    };
    let mut spirits = SubtypeSet::default();
    spirits.0.insert(spirit);
    let wisp = creature(&mut r, WISP, spirits);
    r.register(CardDefinition::new(wisp.name, wisp));
    let steward = creature(&mut r, STEWARD, SubtypeSet::default());
    r.register(CardDefinition::new(steward.name, steward).with_static(vec![
        ContinuousEffect::filtered_pump(
            NULL_OBJECT_ID,
            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            1, 1, Duration::WhileSourceOnBattlefield),
    ]));
    let artifact = |r: &mut CardRegistry, name: &str| {
        let name = r.interner_mut().intern(name);
        Characteristics {
            name,
            mana_cost: Some(ManaCost::parse("{1}").unwrap()),
            types: TypeLine::ARTIFACT.into(),
            ..Default::default()
        }
    };
    let relic = artifact(&mut r, RELIC);
    r.register(CardDefinition::new(relic.name, relic));
    let animator = artifact(&mut r, ANIMATOR);
    let noncreature_artifacts = || ObjectFilter::default()
        .with_types(TypeLine::ARTIFACT.into())
        .without_types(TypeLine::CREATURE.into())
        .controlled_by(ControllerConstraint::You);
    r.register(CardDefinition::new(animator.name, animator).with_static(vec![
        ContinuousEffect::filtered_add_type(NULL_OBJECT_ID, noncreature_artifacts(),
            TypeLine::CREATURE.into(), Duration::WhileSourceOnBattlefield),
        ContinuousEffect::filtered_set_base_pt(NULL_OBJECT_ID, noncreature_artifacts(),
            2, 2, Duration::WhileSourceOnBattlefield),
    ]));
    r
}

fn table(p: PlayerId) -> GameState {
    let mut s = GameState::new(2, 20261111);
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
    let card = r.card_id_by_name(name).unwrap_or_else(|| panic!("unregistered: {name}"));
    let id = s.allocate_object_id();
    s.objects.insert(GameObject::new(id, p, zone, card,
        r.get(card).unwrap().initial_characteristics().clone()));
    if let Zone::Library(owner) = zone {
        s.player_mut(owner).library_top_to_bottom.push(id);
    }
    id
}

/// Put a card onto the battlefield through the zone mover, as a
/// reanimation or a blink does: no spell, no stack.
fn enter(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str) -> ObjectId {
    let id = add(s, r, p, name, Zone::Hand(p));
    s.move_object_to_zone(id, Zone::Battlefield, MoveCause::SpellResolution)
        .expect("entered")
}

fn attach(s: &mut GameState, aura: ObjectId, host: ObjectId) {
    Effect::Attach { equipment_or_aura: aura, target: host }.execute(s);
    assert_eq!(s.objects.get(aura).unwrap().attached_to, Some(host));
}

fn lose_abilities(s: &mut GameState, target: ObjectId) {
    Effect::LoseAllAbilities { target, duration: Duration::EndOfTurn }.execute(s);
}

fn settle(mut s: GameState, r: &CardRegistry) -> GameState {
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

/// Cast `name` and pass priority only until the permanent is on the
/// battlefield, so the caller sees the state at the first decision after
/// it resolved.
fn cast_until_entered(mut s: GameState, r: &CardRegistry, p: PlayerId, name: &str,
                      targets: &[ObjectId]) -> (GameState, ObjectId) {
    let card = r.card_id_by_name(name).unwrap();
    let before: Vec<ObjectId> = s.objects.objects_in_zone(Zone::Battlefield)
        .map(|o| o.id).collect();
    let id = add(&mut s, r, p, name, Zone::Hand(p));
    let wanted: Vec<_> = targets.iter().map(|t| TargetChoice::Object(*t)).collect();
    let action = legal_actions(&s, r).into_iter().find(|a| matches!(a,
        Action::CastSpell { object_id, targets, .. }
            if *object_id == id && targets.targets == wanted)).expect("legal cast");
    s = step(s, action, r).0;
    for _ in 0..10 {
        let entered = s.objects.objects_in_zone(Zone::Battlefield)
            .find(|o| o.card_id == card && !before.contains(&o.id)).map(|o| o.id);
        if let Some(entered) = entered { return (s, entered); }
        s = step(s, Action::PassPriority, r).0;
    }
    panic!("{name} did not resolve");
}

/// (static index, timestamp) of every installed part of `id`'s printed
/// statics.
fn tags(s: &GameState, id: ObjectId) -> Vec<(u16, u64)> {
    s.continuous_effects.iter().filter(|e| e.source == id)
        .filter_map(|e| e.printed.map(|tag| (tag.index, e.timestamp))).collect()
}

fn timestamp(s: &GameState, id: ObjectId) -> u64 {
    s.objects.get(id).unwrap().timestamp
}

fn hand(s: &GameState, p: PlayerId) -> usize {
    s.objects.count_in_zone(Zone::Hand(p))
}

/// Attack the other player with `attacker`, unblocked, answer yes to every
/// "you may", and stop once combat is over.
fn combat_hit(s: GameState, r: &CardRegistry, attacker: ObjectId) -> GameState {
    combat_hit_with_blocker(s, r, attacker, None)
}

fn combat_hit_with_blocker(mut s: GameState, r: &CardRegistry, attacker: ObjectId,
                           blocker: Option<ObjectId>) -> GameState {
    let p = s.active_player();
    for who in 0..2 {
        for _ in 0..3 { add(&mut s, r, who, "Forest", Zone::Library(who)); }
    }
    s.objects.get_mut(attacker).unwrap().status.summoning_sick = false;
    s.turn.phase = Phase::Combat;
    s.turn.step = Step::DeclareAttackers;
    s.combat = Some(CombatState::new());
    s.enter_declare_attackers();
    let action = legal_actions(&s, r).into_iter().find(|a| matches!(a,
        Action::DeclareAttackers { attackers } if attackers == &vec![AttackerDeclaration {
            attacker, defending: DefendingEntity::Player(1 - p),
        }])).expect("legal attack");
    s = step(s, action, r).0;
    for _ in 0..60 {
        if s.turn.phase != Phase::Combat { return s; }
        let actions = legal_actions(&s, r);
        let action = actions.iter().find(|a| matches!(a, Action::SubmitResolutionChoice {
                response: ChoiceResponse::OptionalCost { pay: true }, .. }))
            .or_else(|| actions.iter().find(|a| matches!(a,
                Action::DeclareBlockers { blockers } if blockers == &blocker.into_iter()
                    .map(|blocker| arcana_core::combat::BlockerDeclaration {
                        blocker, blocking: attacker }).collect::<Vec<_>>())))
            .or_else(|| actions.iter().find(|a| matches!(a,
                Action::AssignCombatDamage { distributions }
                    if blocker.is_some_and(|b| distributions.iter()
                        .any(|d| d.distribution == vec![(b, 1)])))))
            .or_else(|| actions.iter().find(|a| a.is_pass()))
            .or_else(|| actions.iter().find(|a| !a.is_concede()))
            .expect("combat decision").clone();
        s = step(s, action, r).0;
    }
    panic!("combat did not end");
}

// --- entry paths -----------------------------------------------------------

#[test]
fn a_resolving_permanent_has_its_statics_before_sba_and_priority() {
    let r = registry();
    for p in [0, 1] {
        // The Steward is 0/0 without its own static: it survives the
        // state-based actions that follow its resolution only if the
        // static is already there.
        let (s, steward) = cast_until_entered(table(p), &r, p, STEWARD, &[]);
        assert!(s.stack_is_empty(), "no trigger installs a static");
        assert_eq!(s.computed_toughness(steward), Some(1));
        assert_eq!(tags(&s, steward).len(), 1);
        let s = settle(step(s, Action::PassPriority, &r).0, &r);
        assert!(s.objects.get(steward).is_some_and(|o| o.zone.is_battlefield()));

        // The Eagle's anthem is in force at the first priority after it
        // resolves, with nothing on the stack to respond to.
        let mut s = table(p);
        let flier = enter(&mut s, &r, p, "Storm Crow");
        assert_eq!(s.computed_power(flier), Some(1));
        let (s, eagle) = cast_until_entered(s, &r, p, "Empyrean Eagle", &[]);
        assert!(s.stack_is_empty());
        assert!(s.pending_trigger_queue.is_empty());
        assert_eq!(s.computed_power(flier), Some(2));
        assert_eq!(s.computed_power(eagle), Some(2));
    }
}

#[test]
fn a_reanimated_permanent_has_its_statics_before_sba() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let card = add(&mut s, &r, p, STEWARD, Zone::Graveyard(p));
        let steward = s.move_object_to_zone(card, Zone::Battlefield,
            MoveCause::SpellResolution).unwrap();
        assert_eq!(s.computed_toughness(steward), Some(1));
        let s = settle(step(s, Action::PassPriority, &r).0, &r);
        assert!(s.objects.get(steward).is_some_and(|o| o.zone.is_battlefield()),
            "the 0/0 Steward lives on its own static");
    }
}

#[test]
fn a_token_has_its_characteristic_defining_ability_as_it_enters() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let voice = enter(&mut s, &r, p, "Voice of Resurgence");
        let bear = enter(&mut s, &r, p, "Grizzly Bears");
        enter(&mut s, &r, 1 - p, "Grizzly Bears");
        s = settle(step(s, Action::PassPriority, &r).0, &r);
        Effect::SacrificePermanent { target: voice }.execute(&mut s);
        s = settle(step(s, Action::PassPriority, &r).0, &r);
        // 0/0 without the ability: alive after state-based actions means
        // it had the ability as it entered.
        let token = s.objects.objects_in_zone(Zone::Battlefield)
            .find(|o| o.is_token).map(|o| o.id).expect("the Elemental survives");
        assert_eq!(s.objects.get(token).unwrap().controller, p);
        // The token and the Bears; the other player's Bears do not count.
        assert_eq!((s.computed_power(token), s.computed_toughness(token)), (Some(2), Some(2)));
        let second = enter(&mut s, &r, p, "Grizzly Bears");
        assert_eq!((s.computed_power(token), s.computed_toughness(token)), (Some(3), Some(3)));
        s.move_object_to_zone(second, Zone::Graveyard(p), MoveCause::SpellResolution);
        s.move_object_to_zone(bear, Zone::Graveyard(p), MoveCause::SpellResolution);
        assert_eq!((s.computed_power(token), s.computed_toughness(token)), (Some(1), Some(1)));
        // The ability is the token's own: losing it leaves a 0/0.
        lose_abilities(&mut s, token);
        assert_eq!(s.computed_toughness(token), Some(0));
    }
}

#[test]
fn permanents_entering_together_have_statics_before_sba_and_apnap_timestamps() {
    let r = registry();
    // Both seats and both exile orders: in some of them the batch lists
    // the other player's card ahead of the active player's.
    let mut other_player_listed_first = false;
    for (p, theirs_first) in [(0, true), (0, false), (1, true), (1, false)] {
        let mut s = table(p);
        let jailer = enter(&mut s, &r, p, "Forest");
        let theirs = enter(&mut s, &r, 1 - p, "Supreme Phantom");
        let phantom = enter(&mut s, &r, p, "Supreme Phantom");
        let wisp = add(&mut s, &r, p, WISP, Zone::Hand(p));
        let order = if theirs_first { [theirs, phantom, wisp] } else { [phantom, wisp, theirs] };
        for target in order {
            Effect::ExileUntilSourceLeaves { source: jailer, target }.execute(&mut s);
        }
        assert_eq!(s.objects.count_in_zone(Zone::Exile), 3);
        let clock = s.timestamp_counter;
        s.move_object_to_zone(jailer, Zone::Graveyard(p), MoveCause::SpellResolution);

        let phantom_card = r.card_id_by_name("Supreme Phantom").unwrap();
        let wisp_card = r.card_id_by_name(WISP).unwrap();
        let on_field = |s: &GameState, card, who| s.objects.objects_in_zone(Zone::Battlefield)
            .find(|o| o.card_id == card && o.controller == who).map(|o| o.id).expect("returned");
        let (mine, wisp, theirs) = (on_field(&s, phantom_card, p),
            on_field(&s, wisp_card, p), on_field(&s, phantom_card, 1 - p));
        // The Wisp is 1/1 at once, from the Phantom that entered with it.
        assert_eq!(s.computed_toughness(wisp), Some(1));
        assert_eq!(tags(&s, mine), vec![(0, timestamp(&s, mine))]);
        // CR 613.7m: the active player's permanents get the earlier
        // timestamps, wherever the batch lists them. Entries are finished
        // in batch order, so the lower new id was listed first.
        other_player_listed_first |= theirs < mine;
        assert!(timestamp(&s, mine) >= clock && timestamp(&s, wisp) >= clock);
        assert!(timestamp(&s, mine) < timestamp(&s, theirs));
        assert!(timestamp(&s, wisp) < timestamp(&s, theirs));
        let s = settle(step(s, Action::PassPriority, &r).0, &r);
        assert!(s.objects.get(wisp).is_some_and(|o| o.zone.is_battlefield()),
            "the Wisp survives state-based actions");
    }
    assert!(other_player_listed_first, "no run put the other player's card first");
}

// --- identity, suppression, lifetime ------------------------------------------

#[test]
fn installing_twice_or_resuming_after_suppression_gives_no_duplicate() {
    let r = registry();
    for p in [0, 1] {
        let (mut s, phantom) = cast_until_entered(table(p), &r, p, "Supreme Phantom", &[]);
        let other = enter(&mut s, &r, p, "Supreme Phantom");
        let installed = tags(&s, phantom);
        assert_eq!(installed, vec![(0, timestamp(&s, phantom))]);
        assert_eq!(s.computed_power(other), Some(2));

        // A recheck installs nothing and refreshes nothing.
        s.install_printed_statics(phantom);
        assert_eq!(tags(&s, phantom), installed);
        assert_eq!(s.computed_power(other), Some(2));

        // Suppression keeps the tag; the same static resumes.
        lose_abilities(&mut s, phantom);
        assert_eq!(s.computed_power(other), Some(1), "the Phantom's anthem is suppressed");
        assert_eq!(s.computed_power(phantom), Some(2), "the other Phantom's is not");
        assert_eq!(tags(&s, phantom), installed);
        s.install_printed_statics(phantom);
        s.expire_end_of_turn_effects();
        assert_eq!(tags(&s, phantom), installed);
        assert_eq!(s.computed_power(other), Some(2));

        // Leaving removes the static; the returning card is a new object
        // with new tags.
        let gone = s.move_object_to_zone(phantom, Zone::Graveyard(p),
            MoveCause::SpellResolution).unwrap();
        assert!(s.continuous_effects.iter().all(|e| e.source != phantom));
        assert_eq!(s.computed_power(other), Some(1));
        let back = s.move_object_to_zone(gone, Zone::Battlefield,
            MoveCause::SpellResolution).unwrap();
        assert_eq!(tags(&s, back), vec![(0, timestamp(&s, back))]);
        assert!(timestamp(&s, back) > installed[0].1);
        assert_eq!(s.computed_power(other), Some(2));
    }
}

#[test]
fn a_static_spanning_layers_keeps_one_set_and_survives_its_source_losing_abilities() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let relic = enter(&mut s, &r, p, RELIC);
        let foreign = enter(&mut s, &r, 1 - p, RELIC);
        assert_eq!(s.computed_power(relic), None);
        let animator = enter(&mut s, &r, p, ANIMATOR);
        // One ability, two installed parts under one tag.
        assert_eq!(tags(&s, animator).iter().map(|t| t.0).collect::<Vec<_>>(), vec![0, 0]);
        // The layer-7b part applies to the objects the layer-4 part made
        // creatures, although "noncreature" no longer describes them there.
        assert!(s.is_creature_now(relic));
        assert_eq!((s.computed_power(relic), s.computed_toughness(relic)), (Some(2), Some(2)));
        assert!(!s.is_creature_now(foreign));
        assert_eq!(s.computed_power(foreign), None);

        // CR 613.6: the effect started to apply in layer 4, before the
        // Animator's abilities are removed in layer 6, so it goes on
        // applying to the same objects in layer 7b.
        lose_abilities(&mut s, animator);
        assert!(s.is_creature_now(relic));
        assert_eq!((s.computed_power(relic), s.computed_toughness(relic)), (Some(2), Some(2)));

        // A static that only starts in layer 7 has no such claim.
        let steward = enter(&mut s, &r, p, STEWARD);
        assert_eq!(s.computed_power(relic), Some(3));
        lose_abilities(&mut s, steward);
        assert_eq!(s.computed_power(relic), Some(2));
    }
}

// --- clients -------------------------------------------------------------------

#[test]
fn domri_pumps_its_controllers_creatures_and_tracks_control_loss_and_departure() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let mine = enter(&mut s, &r, p, "Grizzly Bears");
        let theirs = enter(&mut s, &r, 1 - p, "Grizzly Bears");
        let (mut s, domri) = cast_until_entered(s, &r, p, "Domri, Anarch of Bolas", &[]);
        assert!(s.stack_is_empty());
        assert_eq!((s.computed_power(mine), s.computed_toughness(mine)), (Some(3), Some(2)));
        assert_eq!(s.computed_power(theirs), Some(2));

        let stamped = timestamp(&s, domri);
        Effect::ChangeControl { target: domri, new_controller: 1 - p }.execute(&mut s);
        assert_eq!(s.computed_power(mine), Some(2));
        assert_eq!(s.computed_power(theirs), Some(3));
        assert_eq!(timestamp(&s, domri), stamped, "a control change is no new timestamp");
        assert_eq!(tags(&s, domri), vec![(0, stamped)]);
        Effect::ChangeControl { target: domri, new_controller: p }.execute(&mut s);

        lose_abilities(&mut s, domri);
        assert_eq!(s.computed_power(mine), Some(2));
        s.expire_end_of_turn_effects();
        assert_eq!(s.computed_power(mine), Some(3));

        s.move_object_to_zone(domri, Zone::Graveyard(p), MoveCause::SpellResolution);
        assert_eq!(s.computed_power(mine), Some(2));
        assert!(s.continuous_effects.iter().all(|e| e.source != domri));
    }
}

#[test]
fn warchief_grants_haste_and_a_discount_and_ability_loss_stops_both() {
    let r = registry();
    let haste = KeywordAbility::Haste;
    let piker_chars = |r: &CardRegistry| r.get(r.card_id_by_name("Goblin Piker").unwrap())
        .unwrap().initial_characteristics().clone();
    for p in [0, 1] {
        let mut s = table(p);
        let piker = enter(&mut s, &r, p, "Goblin Piker");
        let bear = enter(&mut s, &r, p, "Grizzly Bears");
        let foe = enter(&mut s, &r, 1 - p, "Goblin Piker");
        assert!(!s.has_keyword(piker, &haste));
        let (mut s, chief) = cast_until_entered(s, &r, p, "Goblin Warchief", &[]);
        assert!(s.stack_is_empty());
        assert!(s.has_keyword(piker, &haste));
        assert!(s.has_keyword(chief, &haste), "the Warchief is a Goblin you control");
        assert!(!s.has_keyword(bear, &haste));
        assert!(!s.has_keyword(foe, &haste));
        assert_eq!(s.spell_cost_delta(&piker_chars(&r), p), -1);
        assert_eq!(s.spell_cost_delta(&piker_chars(&r), 1 - p), 0);
        let bear_chars = s.objects.get(bear).unwrap().characteristics.clone();
        assert_eq!(s.spell_cost_delta(&bear_chars, p), 0);
        // Two abilities, two tags.
        assert_eq!(tags(&s, chief).iter().map(|t| t.0).collect::<Vec<_>>(), vec![0, 1]);

        // The discount is real: one red mana casts a {1}{R} Goblin.
        let castable = |s: &GameState| {
            let mut s = s.clone();
            s.player_mut(p).mana_pool = Default::default();
            s.player_mut(p).mana_pool.add_mana(ManaColor::Red, 1, 0);
            let card = add(&mut s, &r, p, "Goblin Piker", Zone::Hand(p));
            legal_actions(&s, &r).iter().any(|a| matches!(a,
                Action::CastSpell { object_id, .. } if *object_id == card))
        };
        assert!(castable(&s));

        // Ability loss on the Warchief stops the layer-6 grant and the
        // cost reduction, and both resume when it ends.
        lose_abilities(&mut s, chief);
        assert!(!s.has_keyword(piker, &haste));
        assert!(!s.has_keyword(chief, &haste));
        assert_eq!(s.spell_cost_delta(&piker_chars(&r), p), 0);
        assert!(!castable(&s));
        s.expire_end_of_turn_effects();
        assert!(s.has_keyword(piker, &haste));
        assert_eq!(s.spell_cost_delta(&piker_chars(&r), p), -1);

        // Two copies neither duplicate nor leak.
        let second = enter(&mut s, &r, p, "Goblin Warchief");
        assert_eq!(s.spell_cost_delta(&piker_chars(&r), p), -2);
        assert_eq!(s.effective_keywords(piker).iter().filter(|k| **k == haste).count(), 1);
        s.move_object_to_zone(chief, Zone::Graveyard(p), MoveCause::SpellResolution);
        assert_eq!(s.spell_cost_delta(&piker_chars(&r), p), -1);
        assert!(s.has_keyword(piker, &haste));
        s.move_object_to_zone(second, Zone::Graveyard(p), MoveCause::SpellResolution);
        assert_eq!(s.spell_cost_delta(&piker_chars(&r), p), 0);
        assert!(!s.has_keyword(piker, &haste));
    }
}

#[test]
fn obsession_host_draws_once_and_the_grant_follows_the_attachment() {
    let r = registry();
    for p in [0, 1] {
        // Cast on a creature: +1/+1 at once, one draw per combat hit.
        let mut s = table(p);
        let host = enter(&mut s, &r, p, "Grizzly Bears");
        let (s, aura) = cast_until_entered(s, &r, p, "Curious Obsession", &[host]);
        assert!(s.stack_is_empty());
        assert_eq!(s.computed_power(host), Some(3));
        let before = hand(&s, p);
        let s = combat_hit(s, &r, host);
        assert_eq!(hand(&s, p), before + 1, "exactly one draw");
        assert_eq!(hand(&s, 1 - p), 0);
        assert_eq!(tags(&s, aura).len(), 2, "one ability, two parts");

        // Two Obsessions are two abilities: two draws, +2/+2.
        let mut s = table(p);
        let host = enter(&mut s, &r, p, "Grizzly Bears");
        for _ in 0..2 {
            let aura = enter(&mut s, &r, p, "Curious Obsession");
            attach(&mut s, aura, host);
        }
        assert_eq!(s.computed_power(host), Some(4));
        let s = combat_hit(s, &r, host);
        assert_eq!(hand(&s, p), 2);

        // The ability is the enchanted creature's: its controller draws,
        // not the Aura's.
        let mut s = table(p);
        let host = enter(&mut s, &r, p, "Grizzly Bears");
        let aura = enter(&mut s, &r, 1 - p, "Curious Obsession");
        attach(&mut s, aura, host);
        let s = combat_hit(s, &r, host);
        assert_eq!((hand(&s, p), hand(&s, 1 - p)), (1, 0));

        // Moved to another creature, the grant goes with it.
        let mut s = table(p);
        let old = enter(&mut s, &r, p, "Grizzly Bears");
        let new = enter(&mut s, &r, p, "Grizzly Bears");
        let aura = enter(&mut s, &r, p, "Curious Obsession");
        attach(&mut s, aura, old);
        attach(&mut s, aura, new);
        assert_eq!((s.computed_power(old), s.computed_power(new)), (Some(2), Some(3)));
        let s = combat_hit(s, &r, old);
        assert_eq!(hand(&s, p), 0, "the old host no longer has the ability");

        // The Aura gone, the grant is gone.
        let mut s = table(p);
        let host = enter(&mut s, &r, p, "Grizzly Bears");
        let aura = enter(&mut s, &r, p, "Curious Obsession");
        attach(&mut s, aura, host);
        s.move_object_to_zone(aura, Zone::Graveyard(p), MoveCause::SpellResolution);
        assert_eq!(s.computed_power(host), Some(2));
        let s = combat_hit(s, &r, host);
        assert_eq!(hand(&s, p), 0);
    }
}

#[test]
fn obsession_under_ability_loss_on_the_aura_and_on_the_host_in_both_orders() {
    let r = registry();
    for p in [0, 1] {
        // Loss on the Aura removes the pump and the granted trigger.
        let mut s = table(p);
        let host = enter(&mut s, &r, p, "Grizzly Bears");
        let aura = enter(&mut s, &r, p, "Curious Obsession");
        attach(&mut s, aura, host);
        lose_abilities(&mut s, aura);
        assert_eq!(s.computed_power(host), Some(2));
        let s = combat_hit(s, &r, host);
        assert_eq!(hand(&s, p), 0);

        // Loss on the host after the attachment: the host loses the
        // granted trigger. The +1/+1 is the Aura's effect on it, not an
        // ability of the host, and stays.
        let mut s = table(p);
        let host = enter(&mut s, &r, p, "Grizzly Bears");
        let aura = enter(&mut s, &r, p, "Curious Obsession");
        attach(&mut s, aura, host);
        lose_abilities(&mut s, host);
        assert_eq!(s.computed_power(host), Some(3));
        let s = combat_hit(s, &r, host);
        assert_eq!(hand(&s, p), 0);

        // The Aura attached after the loss: its timestamp is the later
        // one (CR 613.7e), so the host has the ability. The Aura was on
        // the battlefield before the loss; only becoming attached is new.
        let mut s = table(p);
        let first = enter(&mut s, &r, p, "Grizzly Bears");
        let host = enter(&mut s, &r, p, "Grizzly Bears");
        let aura = enter(&mut s, &r, p, "Curious Obsession");
        attach(&mut s, aura, first);
        lose_abilities(&mut s, host);
        attach(&mut s, aura, host);
        assert_eq!(s.computed_power(host), Some(3));
        let s = combat_hit(s, &r, host);
        assert_eq!(hand(&s, p), 1);
    }
}

// --- timestamps ------------------------------------------------------------------

#[test]
fn timestamps_are_given_on_entry_and_attachment_and_statics_take_them() {
    let r = registry();
    for p in [0, 1] {
        let (s, first) = cast_until_entered(table(p), &r, p, "Supreme Phantom", &[]);
        let (mut s, second) = cast_until_entered(s, &r, p, "Supreme Phantom", &[]);
        assert!(timestamp(&s, first) < timestamp(&s, second));
        // CR 613.7a: a static's effect has its object's timestamp.
        assert_eq!(tags(&s, first), vec![(0, timestamp(&s, first))]);
        assert_eq!(tags(&s, second), vec![(0, timestamp(&s, second))]);
        // An effect made between them sorts between them.
        let a = enter(&mut s, &r, p, "Grizzly Bears");
        Effect::Pump { target: a, power: 1, toughness: 1, duration: Duration::EndOfTurn,
            keywords: vec![] }.execute(&mut s);
        let pump = s.continuous_effects.last().unwrap().timestamp;
        let third = enter(&mut s, &r, p, "Supreme Phantom");
        assert!(timestamp(&s, a) < pump && pump < timestamp(&s, third));

        // CR 613.7e: an Aura gets a new timestamp each time it becomes
        // attached, and its statics' effects with it.
        let b = enter(&mut s, &r, p, "Grizzly Bears");
        let aura = enter(&mut s, &r, p, "Curious Obsession");
        let entered = timestamp(&s, aura);
        attach(&mut s, aura, a);
        let attached = timestamp(&s, aura);
        assert!(attached > entered);
        assert_eq!(tags(&s, aura), vec![(0, attached), (0, attached)]);
        // Attached to the creature it is already on: nothing happens.
        attach(&mut s, aura, a);
        assert_eq!(timestamp(&s, aura), attached);
        attach(&mut s, aura, b);
        assert!(timestamp(&s, aura) > attached);
        assert_eq!(tags(&s, aura).iter().map(|t| t.1).collect::<Vec<_>>(),
            vec![timestamp(&s, aura); 2]);
        // The host's timestamp is its own.
        assert!(timestamp(&s, b) < timestamp(&s, aura));
    }
}

// --- copies and tokens -----------------------------------------------------------

#[test]
fn a_copy_carries_the_statics_with_its_own_identity_and_timestamp() {
    let r = registry();
    for p in [0, 1] {
        let (mut s, eagle) = cast_until_entered(table(p), &r, p, "Empyrean Eagle", &[]);
        Effect::CopyPermanent { target: eagle }.execute(&mut s);
        let copy = s.objects.objects_in_zone(Zone::Battlefield)
            .find(|o| o.is_token).map(|o| o.id).expect("the copy");
        // The copy pumps the original, and each excludes only itself.
        assert_eq!(s.computed_power(eagle), Some(3));
        assert_eq!(s.computed_power(copy), Some(3));
        assert_eq!(tags(&s, copy), vec![(0, timestamp(&s, copy))]);
        assert!(timestamp(&s, copy) > timestamp(&s, eagle));
        s.move_object_to_zone(eagle, Zone::Graveyard(p), MoveCause::SpellResolution);
        assert_eq!(s.computed_power(copy), Some(2));

        // A copied Voice token has the N/N ability.
        let mut s = table(p);
        let voice = enter(&mut s, &r, p, "Voice of Resurgence");
        s = settle(step(s, Action::PassPriority, &r).0, &r);
        Effect::SacrificePermanent { target: voice }.execute(&mut s);
        s = settle(step(s, Action::PassPriority, &r).0, &r);
        let token = s.objects.objects_in_zone(Zone::Battlefield)
            .find(|o| o.is_token).map(|o| o.id).expect("the Elemental");
        assert_eq!(s.computed_power(token), Some(1));
        Effect::CopyPermanent { target: token }.execute(&mut s);
        let copy = s.objects.objects_in_zone(Zone::Battlefield)
            .find(|o| o.is_token && o.id != token).map(|o| o.id).expect("the copy");
        assert_eq!(tags(&s, copy), vec![(0, timestamp(&s, copy))]);
        assert_eq!((s.computed_power(token), s.computed_toughness(token)), (Some(2), Some(2)));
        assert_eq!((s.computed_power(copy), s.computed_toughness(copy)), (Some(2), Some(2)));
        let s = settle(step(s, Action::PassPriority, &r).0, &r);
        assert!(s.objects.get(copy).is_some_and(|o| o.zone.is_battlefield()));
    }
}

fn gain_a_life(_: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife { player: trig.controller, amount: 1 }]
}

/// The timestamp rule also holds for a filtered grant ("creatures you
/// control have …"): a later loss removes it, an earlier one does not.
#[test]
fn a_filtered_trigger_grant_is_lost_only_to_a_later_ability_loss() {
    let r = registry();
    for (p, loss_first) in [(0, false), (0, true), (1, false), (1, true)] {
        let mut s = table(p);
        let source = enter(&mut s, &r, p, RELIC);
        let host = enter(&mut s, &r, p, "Grizzly Bears");
        if loss_first { lose_abilities(&mut s, host); }
        s.add_continuous_effect(ContinuousEffect::filtered_grant_triggered(
            source,
            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            TriggeredAbilityDef {
                id: GRANTED_TRIGGER_ID_BASE + 9,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: gain_a_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
            Duration::WhileSourceOnBattlefield));
        if !loss_first { lose_abilities(&mut s, host); }
        let life = s.player(p).life;
        s.emit(GameEvent::CreatureAttacks { attacker: host,
            defending: DefendingEntity::Player(1 - p) });
        let s = settle(step(s, Action::PassPriority, &r).0, &r);
        assert_eq!(s.player(p).life, life + i32::from(loss_first));
    }
}

#[test]
fn a_tokens_static_rides_beside_its_triggers_without_becoming_one() {
    let r = registry();
    let mut s = table(0);
    let name = r.interner().lookup("Spirit").unwrap();
    let token = TokenDefinition {
        name,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: SubtypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        abilities: vec![TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: gain_a_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }],
    }
    .with_static(vec![ContinuousEffect::self_pt_from_match(
        NULL_OBJECT_ID,
        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        Duration::WhileSourceOnBattlefield,
    )]);
    assert_eq!(token.triggered_abilities().count(), 1);
    Effect::CreateToken { controller: 0, token }.execute(&mut s);
    let id = s.objects.objects_in_zone(Zone::Battlefield)
        .find(|o| o.is_token).map(|o| o.id).unwrap();
    let obj = s.objects.get(id).unwrap();
    assert_eq!(obj.granted_triggered_abilities.len(), 1, "the static is no trigger");
    assert_eq!(obj.characteristics.static_abilities.as_ref().map(|d| d.len()), Some(1));
    assert_eq!(tags(&s, id), vec![(0, timestamp(&s, id))]);
    assert_eq!(s.computed_toughness(id), Some(1));
    let life = s.player(0).life;
    let s = settle(step(s, Action::PassPriority, &r).0, &r);
    assert_eq!(s.player(0).life, life + 1, "its real trigger still fires once");
    assert!(s.objects.get(id).is_some_and(|o| o.zone.is_battlefield()));
}

#[test]
fn a_face_down_permanent_installs_no_statics() {
    let r = registry();
    let mut s = table(0);
    let spirit = enter(&mut s, &r, 0, "Storm Crow");
    add(&mut s, &r, 0, "Empyrean Eagle", Zone::Library(0));
    Effect::Manifest { player: 0 }.execute(&mut s);
    let manifested = s.objects.objects_in_zone(Zone::Battlefield)
        .find(|o| o.status.face_down).map(|o| o.id).expect("manifested");
    assert!(tags(&s, manifested).is_empty());
    assert_eq!(s.computed_power(spirit), Some(1));
}

// --- attachment grants at the event, before SBA and later instructions -----

/// A small instruction sequence may change the grant before the next
/// priority. Flush its events through step, then accept every optional draw.
fn resolve_event_draws(mut s: GameState, r: &CardRegistry) -> GameState {
    s = step(s, Action::PassPriority, r).0;
    for _ in 0..50 {
        if s.stack_is_empty() && s.pending_choice.is_none()
            && s.pending_trigger_queue.is_empty() { return s; }
        let actions = legal_actions(&s, r);
        let action = actions.iter().find(|a| matches!(a, Action::SubmitResolutionChoice {
                response: ChoiceResponse::OptionalCost { pay: true }, .. }))
            .or_else(|| actions.iter().find(|a| a.is_pass()))
            .expect("draw or priority decision").clone();
        s = step(s, action, r).0;
    }
    panic!("event triggers did not settle");
}

fn draw_library(s: &mut GameState, r: &CardRegistry, p: PlayerId) {
    for _ in 0..5 { add(s, r, p, "Forest", Zone::Library(p)); }
}

#[test]
fn attachment_event_combat_trigger_survives_its_host_and_aura_dying() {
    let r = registry();
    for p in [0, 1] {
        for lethal in [false, true] {
            for count in [1, 2] {
                let mut s = table(p);
                let host = enter(&mut s, &r, p, "Grizzly Bears");
                s.objects.get_mut(host).unwrap().characteristics.keywords
                    .push(KeywordAbility::Trample);
                let blocker = enter(&mut s, &r, 1 - p, "Typhoid Rats");
                assert!(s.effective_keywords(blocker).contains(&KeywordAbility::Deathtouch));
                if !lethal { s.objects.get_mut(blocker).unwrap().characteristics.keywords.clear(); }
                let mut auras = Vec::new();
                for _ in 0..count {
                    let aura = enter(&mut s, &r, p, "Curious Obsession");
                    attach(&mut s, aura, host);
                    auras.push(aura);
                }
                let s = combat_hit_with_blocker(s, &r, host, Some(blocker));
                assert_eq!(s.player(1 - p).life, 20 - (1 + count) as i32,
                    "one damage to the blocker, the rest to the player");
                assert_eq!(s.objects.get(host).is_none(), lethal);
                for aura in auras { assert_eq!(s.objects.get(aura).is_none(), lethal); }
                assert_eq!(hand(&s, p), count, "each grant triggers even when its host dies");
                assert_eq!(hand(&s, 1 - p), 0);
            }
        }
    }
}

#[test]
fn attachment_event_keeps_the_original_host_and_controller() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        draw_library(&mut s, &r, p);
        draw_library(&mut s, &r, 1 - p);
        let old = enter(&mut s, &r, p, "Grizzly Bears");
        let new = enter(&mut s, &r, p, "Grizzly Bears");
        let aura = enter(&mut s, &r, 1 - p, "Curious Obsession");
        attach(&mut s, aura, old);
        s.deal_damage(old, arcana_core::events::DamageTarget::Player(1 - p), 1, true);
        Effect::ChangeControl { target: old, new_controller: 1 - p }.execute(&mut s);
        attach(&mut s, aura, new);
        s.deal_damage(old, arcana_core::events::DamageTarget::Player(p), 1, true);
        s.deal_damage(new, arcana_core::events::DamageTarget::Player(1 - p), 1, true);
        let s = resolve_event_draws(s, &r);
        assert_eq!((hand(&s, p), hand(&s, 1 - p)), (2, 0),
            "one draw from each host, both controlled by p when they triggered");
    }
}

#[test]
fn attachment_event_later_grants_do_not_see_earlier_damage() {
    let r = registry();
    for p in [0, 1] {
        for already_installed in [false, true] {
            let mut s = table(p);
            draw_library(&mut s, &r, p);
            let first = enter(&mut s, &r, p, "Grizzly Bears");
            let host = enter(&mut s, &r, p, "Grizzly Bears");
            let early_aura = already_installed.then(|| {
                let aura = enter(&mut s, &r, p, "Curious Obsession");
                attach(&mut s, aura, first);
                aura
            });
            s.deal_damage(host, arcana_core::events::DamageTarget::Player(1 - p), 1, true);
            let aura = early_aura.unwrap_or_else(|| enter(&mut s, &r, p, "Curious Obsession"));
            attach(&mut s, aura, host);
            let s = resolve_event_draws(s, &r);
            assert_eq!(hand(&s, p), 0, "neither entry nor reattachment grants a past trigger");
        }
    }
}

#[test]
fn attachment_event_ability_loss_is_checked_when_damage_happens() {
    let r = registry();
    for p in [0, 1] {
        for loss_on_aura in [false, true] {
            for lost_at_event in [false, true] {
                let mut s = table(p);
                draw_library(&mut s, &r, p);
                let host = enter(&mut s, &r, p, "Grizzly Bears");
                let aura = enter(&mut s, &r, p, "Curious Obsession");
                attach(&mut s, aura, host);
                let target = if loss_on_aura { aura } else { host };
                if lost_at_event { lose_abilities(&mut s, target); }
                s.deal_damage(host, arcana_core::events::DamageTarget::Player(1 - p), 1, true);
                if lost_at_event {
                    s.remove_continuous_effects(|e| matches!(e.kind,
                        arcana_core::layers::ContinuousEffectKind::LoseAllAbilities { .. }));
                } else { lose_abilities(&mut s, target); }
                let s = resolve_event_draws(s, &r);
                assert_eq!(hand(&s, p), usize::from(!lost_at_event),
                    "later loss cannot erase a trigger; later restoration cannot create one");
            }
        }
    }
}

fn tapped_host_deals_damage(event: &GameEvent, s: &GameState, host: ObjectId) -> bool {
    matches!(event, GameEvent::DamageDealt { source, .. } if *source == host)
        && s.objects.get(host).is_some_and(|o| o.status.tapped)
}

#[test]
fn attachment_event_matches_the_condition_before_later_changes() {
    let r = registry();
    for was_tapped in [false, true] {
        let mut s = table(0);
        let host = enter(&mut s, &r, 0, "Grizzly Bears");
        let source = enter(&mut s, &r, 0, RELIC);
        attach(&mut s, source, host);
        s.add_continuous_effect(ContinuousEffect::attached_grant_triggered(source,
            TriggeredAbilityDef {
                id: GRANTED_TRIGGER_ID_BASE + 12,
                trigger_condition: TriggerCondition::Custom(tapped_host_deals_damage),
                intervening_if: None,
                effect: gain_a_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }, Duration::WhileSourceOnBattlefield));
        s.objects.get_mut(host).unwrap().status.tapped = was_tapped;
        // emit_all must capture just like emit; no later event may replace
        // the state in which a predicate matched.
        s.emit_all([GameEvent::DamageDealt { source: host,
            target: arcana_core::events::DamageTarget::Player(1), amount: 1, is_combat: true }]);
        s.objects.get_mut(host).unwrap().status.tapped = !was_tapped;
        let s = resolve_event_draws(s, &r);
        assert_eq!(s.player(0).life, 20 + i32::from(was_tapped));
    }
}

#[test]
fn attachment_event_capture_clones_and_is_consumed_once() {
    let r = registry();
    let mut s = table(0);
    draw_library(&mut s, &r, 0);
    let host = enter(&mut s, &r, 0, "Grizzly Bears");
    let aura = enter(&mut s, &r, 0, "Curious Obsession");
    attach(&mut s, aura, host);
    s.deal_damage(host, arcana_core::events::DamageTarget::Player(1), 1, true);
    s.move_object_to_zone(aura, Zone::Graveyard(0), MoveCause::SpellResolution);
    for fork in [s.clone(), s] {
        let mut fork = resolve_event_draws(fork, &r);
        assert_eq!(hand(&fork, 0), 1, "each search branch retains the trigger");
        fork.deal_damage(host, arcana_core::events::DamageTarget::Player(1), 1, true);
        let fork = resolve_event_draws(fork, &r);
        assert_eq!(hand(&fork, 0), 1, "no stale capture or departed grant fires again");
    }
}
