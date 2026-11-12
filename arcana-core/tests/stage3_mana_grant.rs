//! Stage 3, tranche 1: the filtered mana-ability grant. Rishkar, Peema
//! Renegade's "each creature you control with a counter on it has
//! '{T}: Add {G}.'", with the registered card and three synthetic ones
//! where no registered card has the shape.
use arcana_core::actions::{Action, ChoiceResponse};
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::engine::{step, EngineYield};
use arcana_core::events::MoveCause;
use arcana_core::layers::{ContinuousEffect, ContinuousEffectKind, Duration};
use arcana_core::legal_actions::{available_mana, legal_actions, playable_cards};
use arcana_core::mana::{ManaCost, ManaPool, ManaUnit};
use arcana_core::objects::{Characteristics, GameObject, ObjectId, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::render::render_action;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{ColorSet, CounterKind, ManaColor, PlayerId, PtValue, TypeLine};
use arcana_core::zones::Zone;

const RISHKAR: &str = "Rishkar, Peema Renegade";
const BEARS: &str = "Grizzly Bears";
const ELVES: &str = "Llanowar Elves";
const KINDLER: &str = "Mg Test Kindler";
const ACOLYTE: &str = "Mg Test Acolyte";
const RELIC: &str = "Mg Test Relic";
const WARDEN: &str = "Mg Test Warden";

const GREEN: &str = "{T}: Add {G}.";
const RED: &str = "{T}: Add {R}.";
const COLORLESS: &str = "{T}: Add {C}.";

fn add_red(_: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)] }]
}
fn add_white(_: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)] }]
}
fn add_colorless(_: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)] }]
}

fn tap_for(text: &str, effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>)
    -> ActivatedAbilityDef
{
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost::tap_only(),
        target_requirements: Vec::new(),
        is_mana_ability: true,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: true,
        face_gate: None,
        effect,
    }
}

/// The catalog plus four synthetic cards:
/// * Kindler, a nonlegendary 1/1 with Rishkar's static in red, so two
///   grants can sit on one creature and be told apart;
/// * Acolyte, a 1/1 with a printed "{T}: Add {W}.";
/// * Relic, a plain artifact;
/// * Warden, a 1/1 with one static in two parts: each creature you
///   control with a counter on it has "{T}: Add {R}." and gets +1/+1.
fn registry() -> CardRegistry {
    let mut r = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut r);
    let creature = |r: &mut CardRegistry, name: &str| {
        let name = r.interner_mut().intern(name);
        Characteristics {
            name,
            mana_cost: Some(ManaCost::parse("{1}").unwrap()),
            colors: ColorSet::default(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            ..Default::default()
        }
    };
    let kindler = creature(&mut r, KINDLER);
    r.register(CardDefinition::new(kindler.name, kindler).with_static(vec![
        ContinuousEffect::filtered_grant_activated(
            NULL_OBJECT_ID,
            ObjectFilter::creature().controlled_by(ControllerConstraint::You)
                .with_any_counter(),
            tap_for(RED, add_red),
            Duration::WhileSourceOnBattlefield),
    ]));
    let warden = creature(&mut r, WARDEN);
    let with_counters = || ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You).with_any_counter();
    r.register(CardDefinition::new(warden.name, warden).with_static(vec![
        ContinuousEffect::filtered_grant_activated(NULL_OBJECT_ID, with_counters(),
            tap_for(RED, add_red), Duration::WhileSourceOnBattlefield),
        ContinuousEffect::filtered_pump(NULL_OBJECT_ID, with_counters(),
            1, 1, Duration::WhileSourceOnBattlefield),
    ]));
    let acolyte = creature(&mut r, ACOLYTE);
    r.register(CardDefinition::new(acolyte.name, acolyte)
        .with_activated_ability(tap_for("{T}: Add {W}.", add_white)));
    let relic = r.interner_mut().intern(RELIC);
    r.register(CardDefinition::new(relic, Characteristics {
        name: relic,
        mana_cost: Some(ManaCost::parse("{1}").unwrap()),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    }));
    r
}

/// `p`'s precombat main phase, `p` with priority, nothing in any pool.
fn table(p: PlayerId) -> GameState {
    let mut s = GameState::new(2, 20261111);
    s.turn.active_player = p;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(p);
    s
}

fn add(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str, zone: Zone) -> ObjectId {
    let card = r.card_id_by_name(name).unwrap_or_else(|| panic!("unregistered: {name}"));
    let id = s.allocate_object_id();
    s.objects.insert(GameObject::new(id, p, zone, card,
        r.get(card).unwrap().initial_characteristics().clone()));
    id
}

/// Put a card onto the battlefield through the zone mover, so the engine
/// installs its statics and stamps it, then treat the entry as old news:
/// its events are marked as seen (no entry trigger will fire) and the
/// permanent is no longer summoning sick. Tests that are about either
/// say so and use [`enter_fresh`] or a real cast.
fn enter(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str) -> ObjectId {
    let id = enter_fresh(s, r, p, name);
    s.objects.get_mut(id).unwrap().status.summoning_sick = false;
    id
}

fn enter_fresh(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str) -> ObjectId {
    let id = add(s, r, p, name, Zone::Hand(p));
    let id = s.move_object_to_zone(id, Zone::Battlefield, MoveCause::SpellResolution)
        .expect("entered");
    s.trigger_event_cursor = s.event_log.len();
    id
}

fn counter(s: &mut GameState, id: ObjectId, kind: CounterKind) {
    s.objects.get_mut(id).unwrap().add_counters(kind, 1);
}

fn lose_abilities(s: &mut GameState, target: ObjectId) {
    Effect::LoseAllAbilities { target, duration: Duration::EndOfTurn }.execute(s);
}

/// The texts of the abilities continuous effects grant `id`, in index order.
fn grants(s: &GameState, id: ObjectId) -> Vec<String> {
    s.granted_activated_for(id).iter().map(|a| a.text.clone()).collect()
}

/// The ability indices `id`'s controller may activate right now.
fn offered(s: &GameState, r: &CardRegistry, id: ObjectId) -> Vec<usize> {
    let mut out: Vec<usize> = legal_actions(s, r).iter().filter_map(|a| match a {
        Action::ActivateAbility { source, ability_index, .. } if *source == id =>
            Some(*ability_index),
        _ => None,
    }).collect();
    out.sort();
    out
}

fn colors(pool: &ManaPool) -> Vec<(ManaColor, usize)> {
    [ManaColor::White, ManaColor::Blue, ManaColor::Black, ManaColor::Red,
     ManaColor::Green, ManaColor::Colorless].into_iter()
        .map(|c| (c, pool.count_color(c)))
        .filter(|(_, n)| *n > 0).collect()
}

fn pool(s: &GameState, p: PlayerId) -> Vec<(ManaColor, usize)> {
    colors(&s.player(p).mana_pool)
}

/// Activate ability `index` of `id` and check what a mana ability must
/// do: tap the permanent, add exactly one mana of `color` to the player
/// who had priority, use no stack, ask nothing, and leave priority where
/// it was (CR 605.3).
fn tap(s: &GameState, r: &CardRegistry, id: ObjectId, index: usize, color: ManaColor)
    -> GameState
{
    let p = s.priority_player();
    let total = s.player(p).mana_pool.total();
    let of_color = s.player(p).mana_pool.count_color(color);
    let action = legal_actions(s, r).into_iter().find(|a| matches!(a,
        Action::ActivateAbility { source, ability_index, .. }
            if *source == id && *ability_index == index))
        .unwrap_or_else(|| panic!("ability {index} of {id:?} is not offered"));
    let (s, _) = step(s.clone(), action, r);
    assert!(s.objects.get(id).unwrap().is_tapped(), "the tap cost was paid");
    assert_eq!(s.player(p).mana_pool.total(), total + 1, "exactly one mana");
    assert_eq!(s.player(p).mana_pool.count_color(color), of_color + 1,
        "the mana is {color:?}");
    assert!(s.stack_is_empty(), "a mana ability uses no stack");
    assert!(s.pending_choice.is_none() && s.pending_trigger_queue.is_empty());
    assert_eq!(s.priority_player(), p, "priority stays where it was");
    s
}

// --- who has the ability ---------------------------------------------------

#[test]
fn every_creature_you_control_with_any_counter_has_the_ability() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let plus = enter(&mut s, &r, p, BEARS);
        let minus = enter(&mut s, &r, p, BEARS);
        let stun = enter(&mut s, &r, p, BEARS);
        let charge = enter(&mut s, &r, p, BEARS);
        let bare = enter(&mut s, &r, p, BEARS);
        let foe = enter(&mut s, &r, 1 - p, BEARS);
        let forest = enter(&mut s, &r, p, "Forest");
        counter(&mut s, plus, CounterKind::PlusOnePlusOne);
        counter(&mut s, minus, CounterKind::MinusOneMinusOne);
        counter(&mut s, stun, CounterKind::Stun);
        counter(&mut s, charge, CounterKind::Charge);
        counter(&mut s, foe, CounterKind::PlusOnePlusOne);
        counter(&mut s, forest, CounterKind::PlusOnePlusOne);
        for id in [plus, minus, stun, charge, bare, foe, forest] {
            assert!(grants(&s, id).is_empty(), "nothing is granted without Rishkar");
        }
        // The filter's own "with a counter on it", asked directly: any
        // kind counts, and one kind does not stand for all.
        let any = ObjectFilter::creature().with_any_counter();
        let obj = |id: ObjectId| s.objects.get(id).unwrap();
        for id in [plus, minus, stun, charge, foe] { assert!(any.matches(obj(id), &s, p)); }
        assert!(!any.matches(obj(bare), &s, p));
        assert!(!any.matches(obj(forest), &s, p), "the creature half still applies");
        assert!(ObjectFilter::default().with_any_counter().matches(obj(forest), &s, p));
        let effects_before = s.continuous_effects.len();

        let rishkar = enter(&mut s, &r, p, RISHKAR);
        // One effect on Rishkar; nothing is installed on any creature.
        assert_eq!(s.continuous_effects.len(), effects_before + 1);
        assert!(matches!(s.continuous_effects.last().unwrap().kind,
            ContinuousEffectKind::FilteredGrantsActivated { .. }));
        assert_eq!(s.continuous_effects.last().unwrap().source, rishkar);

        for id in [plus, minus, stun, charge] {
            assert_eq!(grants(&s, id), vec![GREEN], "any kind of counter qualifies");
            assert!(s.objects.get(id).unwrap().intrinsic_activated_abilities.is_empty());
        }
        assert!(grants(&s, bare).is_empty(), "no counter, no ability");
        assert!(grants(&s, foe).is_empty(), "not a creature you control");
        assert!(grants(&s, forest).is_empty(), "a land with a counter is no creature");
        assert!(grants(&s, rishkar).is_empty(), "Rishkar has no counter yet");

        // A counter put on later counts at once, on Rishkar too.
        counter(&mut s, bare, CounterKind::Time);
        counter(&mut s, rishkar, CounterKind::PlusOnePlusOne);
        assert_eq!(grants(&s, bare), vec![GREEN]);
        assert_eq!(grants(&s, rishkar), vec![GREEN]);
        assert_eq!(s.continuous_effects.len(), effects_before + 1, "still one effect");

        // Each of them really taps for green, one after another. A Bears
        // has no ability of its own, so the granted one is index 0.
        let mut green = 0;
        for id in [plus, minus, stun, charge, bare, rishkar] {
            assert_eq!(offered(&s, &r, id), vec![0]);
            s = tap(&s, &r, id, 0, ManaColor::Green);
            green += 1;
            assert_eq!(pool(&s, p), vec![(ManaColor::Green, green)]);
            assert!(offered(&s, &r, id).is_empty(), "tapped: it cannot pay again");
        }
        assert!(offered(&s, &r, foe).is_empty());
        // The Forest keeps its own ability and gains none.
        assert_eq!(offered(&s, &r, forest), vec![0]);
        assert!(pool(&s, 1 - p).is_empty(), "the other player got nothing");
    }
}

#[test]
fn the_ability_follows_what_the_permanent_is_now() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        enter(&mut s, &r, p, RISHKAR);

        // An artifact with a counter is no creature; animated, it is one.
        let relic = enter(&mut s, &r, p, RELIC);
        counter(&mut s, relic, CounterKind::Charge);
        assert!(grants(&s, relic).is_empty());
        s.add_continuous_effect(ContinuousEffect::add_type(
            relic, relic, TypeLine::CREATURE.into(), Duration::EndOfTurn));
        s.add_continuous_effect(ContinuousEffect::set_pt(
            relic, relic, 2, 2, Duration::EndOfTurn));
        assert!(!s.objects.get(relic).unwrap().characteristics.types.is_creature(),
            "the printed card is still no creature");
        assert!(s.is_creature_now(relic));
        assert_eq!(grants(&s, relic), vec![GREEN]);
        s = tap(&s, &r, relic, 0, ManaColor::Green);
        s.expire_end_of_turn_effects();
        assert!(grants(&s, relic).is_empty(), "no longer a creature");

        // Xenagos is printed a creature and, short of devotion, is not
        // one: a counter on it earns no ability.
        let xenagos = enter(&mut s, &r, p, "Xenagos, God of Revels");
        counter(&mut s, xenagos, CounterKind::PlusOnePlusOne);
        assert!(s.objects.get(xenagos).unwrap().characteristics.types.is_creature());
        assert!(!s.is_creature_now(xenagos));
        assert!(grants(&s, xenagos).is_empty());
        assert!(offered(&s, &r, xenagos).is_empty());
    }
}

#[test]
fn only_a_permanent_on_the_battlefield_has_the_ability() {
    let r = registry();
    for p in [0, 1] {
        // A suspended creature card sits in exile with time counters on
        // it. It is a creature card its owner owns, with a counter, and
        // it is not a creature that player controls.
        let mut s = table(p);
        enter(&mut s, &r, p, RISHKAR);
        let suspended = add(&mut s, &r, p, BEARS, Zone::Exile);
        counter(&mut s, suspended, CounterKind::Time);
        let buried = add(&mut s, &r, p, BEARS, Zone::Graveyard(p));
        counter(&mut s, buried, CounterKind::PlusOnePlusOne);
        let on_table = enter(&mut s, &r, p, BEARS);
        counter(&mut s, on_table, CounterKind::Time);
        assert_eq!(grants(&s, on_table), vec![GREEN]);
        assert!(grants(&s, suspended).is_empty());
        assert!(grants(&s, buried).is_empty());
        assert!(offered(&s, &r, suspended).is_empty());
        assert!(offered(&s, &r, buried).is_empty());
    }
}

#[test]
fn a_grant_beside_another_part_of_one_static_leaves_that_part_working() {
    let r = registry();
    for p in [0, 1] {
        // The grant is read by its own scanner, not applied in the layer
        // pipeline. Listed first in a two-part static, it must not settle
        // which objects the static's other part applies to.
        let mut s = table(p);
        let bear = enter(&mut s, &r, p, BEARS);
        let bare = enter(&mut s, &r, p, BEARS);
        counter(&mut s, bear, CounterKind::Charge);
        enter(&mut s, &r, p, WARDEN);
        assert_eq!(grants(&s, bear), vec![RED]);
        assert_eq!((s.computed_power(bear), s.computed_toughness(bear)), (Some(3), Some(3)));
        assert!(grants(&s, bare).is_empty());
        assert_eq!(s.computed_power(bare), Some(2));
        tap(&s, &r, bear, 0, ManaColor::Red);
    }
}

// --- losing it -------------------------------------------------------------

#[test]
fn losing_the_counters_the_source_or_control_removes_the_ability() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let mine = enter(&mut s, &r, p, BEARS);
        let theirs = enter(&mut s, &r, 1 - p, BEARS);
        counter(&mut s, mine, CounterKind::PlusOnePlusOne);
        counter(&mut s, theirs, CounterKind::PlusOnePlusOne);
        let rishkar = enter(&mut s, &r, p, RISHKAR);
        assert_eq!(grants(&s, mine), vec![GREEN]);
        assert_eq!(offered(&s, &r, mine), vec![0]);

        // The counters.
        Effect::RemoveCounters { target: mine, kind: CounterKind::PlusOnePlusOne, count: 1 }
            .execute(&mut s);
        assert!(grants(&s, mine).is_empty());
        assert!(offered(&s, &r, mine).is_empty());
        counter(&mut s, mine, CounterKind::PlusOnePlusOne);
        assert_eq!(offered(&s, &r, mine), vec![0]);

        // Control of the creature.
        Effect::ChangeControl { target: mine, new_controller: 1 - p }.execute(&mut s);
        assert!(grants(&s, mine).is_empty(), "it is no longer a creature you control");
        Effect::ChangeControl { target: mine, new_controller: p }.execute(&mut s);
        assert_eq!(grants(&s, mine), vec![GREEN]);

        // Control of Rishkar: "you" is whoever controls him now.
        Effect::ChangeControl { target: rishkar, new_controller: 1 - p }.execute(&mut s);
        assert!(grants(&s, mine).is_empty());
        assert_eq!(grants(&s, theirs), vec![GREEN]);
        let mut other = s.clone();
        other.priority.give_to(1 - p);
        other.objects.get_mut(theirs).unwrap().status.summoning_sick = false;
        let other = tap(&other, &r, theirs, 0, ManaColor::Green);
        assert_eq!(pool(&other, 1 - p), vec![(ManaColor::Green, 1)]);
        assert!(pool(&other, p).is_empty());
        Effect::ChangeControl { target: rishkar, new_controller: p }.execute(&mut s);
        assert_eq!(grants(&s, mine), vec![GREEN]);
        assert!(grants(&s, theirs).is_empty());

        // Rishkar's own abilities: the grant stops, and the same static
        // resumes when the loss ends.
        lose_abilities(&mut s, rishkar);
        assert!(grants(&s, mine).is_empty());
        assert!(offered(&s, &r, mine).is_empty());
        s.expire_end_of_turn_effects();
        assert_eq!(offered(&s, &r, mine), vec![0]);

        // The source.
        s.move_object_to_zone(rishkar, Zone::Graveyard(p), MoveCause::SpellResolution);
        assert!(grants(&s, mine).is_empty());
        assert!(offered(&s, &r, mine).is_empty());
        assert!(s.continuous_effects.iter().all(|e| e.source != rishkar));
    }
}

#[test]
fn a_loss_of_abilities_takes_the_grant_only_if_it_is_later() {
    let r = registry();
    for p in [0, 1] {
        // Rishkar first, the loss second: the creature loses the ability
        // until the loss ends.
        let mut s = table(p);
        let bear = enter(&mut s, &r, p, BEARS);
        counter(&mut s, bear, CounterKind::PlusOnePlusOne);
        enter(&mut s, &r, p, RISHKAR);
        assert_eq!(grants(&s, bear), vec![GREEN]);
        lose_abilities(&mut s, bear);
        assert!(grants(&s, bear).is_empty());
        assert!(offered(&s, &r, bear).is_empty());
        s.expire_end_of_turn_effects();
        assert_eq!(offered(&s, &r, bear), vec![0]);

        // The loss first, Rishkar second: his later effect gives the
        // ability to a creature that had lost its own (CR 613.7).
        let mut s = table(p);
        let bear = enter(&mut s, &r, p, BEARS);
        counter(&mut s, bear, CounterKind::PlusOnePlusOne);
        lose_abilities(&mut s, bear);
        enter(&mut s, &r, p, RISHKAR);
        assert_eq!(grants(&s, bear), vec![GREEN]);
        tap(&s, &r, bear, 0, ManaColor::Green);
    }
}

// --- the cost --------------------------------------------------------------

#[test]
fn summoning_sickness_applies_to_the_tap_cost() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let old = enter(&mut s, &r, p, BEARS);
        let new = enter_fresh(&mut s, &r, p, BEARS);
        let rishkar = enter_fresh(&mut s, &r, p, RISHKAR);
        for id in [old, new, rishkar] { counter(&mut s, id, CounterKind::PlusOnePlusOne); }
        assert!(s.objects.get(new).unwrap().status.summoning_sick);
        assert!(s.objects.get(rishkar).unwrap().status.summoning_sick);

        // All three have the ability; only the one that began the turn
        // under its controller's control can pay {T} (CR 302.6).
        for id in [old, new, rishkar] { assert_eq!(grants(&s, id), vec![GREEN]); }
        assert_eq!(offered(&s, &r, old), vec![0]);
        assert!(offered(&s, &r, new).is_empty());
        assert!(offered(&s, &r, rishkar).is_empty(), "Rishkar came in this turn too");
        assert_eq!(colors(&available_mana(&s, p, &r)), vec![(ManaColor::Green, 1)]);

        // Haste pays it.
        let mut hasty = s.clone();
        hasty.add_continuous_effect(ContinuousEffect::grant_keyword(
            new, new, KeywordAbility::Haste, Duration::EndOfTurn));
        assert_eq!(offered(&hasty, &r, new), vec![0]);

        // So does a turn's wait.
        s.objects.get_mut(new).unwrap().status.summoning_sick = false;
        s.objects.get_mut(rishkar).unwrap().status.summoning_sick = false;
        assert_eq!(offered(&s, &r, new), vec![0]);
        assert_eq!(offered(&s, &r, rishkar), vec![0]);
    }
}

#[test]
fn the_mana_is_there_at_any_time_and_pays_for_a_spell() {
    let r = registry();
    for p in [0, 1] {
        // A real cast: Rishkar resolves, his trigger puts a counter on
        // each of two Bears, and the Bears tap for the next spell.
        let mut s = table(p);
        s.player_mut(p).mana_pool.add_mana(ManaColor::Green, 3, 0);
        let a = enter(&mut s, &r, p, BEARS);
        let b = enter(&mut s, &r, p, BEARS);
        let card = add(&mut s, &r, p, RISHKAR, Zone::Hand(p));
        let cub = add(&mut s, &r, p, BEARS, Zone::Hand(p));
        let cast = legal_actions(&s, &r).into_iter().find(|x| matches!(x,
            Action::CastSpell { object_id, .. } if *object_id == card)).expect("castable");
        let (mut s, mut y) = step(s, cast, &r);
        let mut aimed = false;
        for _ in 0..20 {
            if aimed && s.stack_is_empty() && s.pending_choice.is_none() { break; }
            let EngineYield::PendingDecision { legal_actions: acts, .. } = &y
                else { panic!("a decision") };
            let both = acts.iter().find(|x| matches!(x, Action::SubmitResolutionChoice {
                response: ChoiceResponse::ChooseTargets { selection }, .. }
                    if { let mut o: Vec<ObjectId> = selection.targets.iter()
                            .filter_map(|t| match t {
                                TargetChoice::Object(id) => Some(*id), _ => None }).collect();
                         o.sort(); o == vec![a, b] })).cloned();
            aimed |= both.is_some();
            let next = both.or_else(|| acts.iter().find(|x| x.is_pass()).cloned())
                .expect("a pass");
            (s, y) = step(s, next, &r);
        }
        assert!(aimed, "the entry trigger asked for its targets");
        for id in [a, b] {
            assert_eq!(s.objects.get(id).unwrap().count_counters(CounterKind::PlusOnePlusOne), 1);
            assert_eq!(grants(&s, id), vec![GREEN]);
        }
        assert_eq!(s.priority_player(), p);
        assert_eq!(s.active_player(), p);
        assert!(pool(&s, p).is_empty(), "Rishkar took all three");
        assert!(!legal_actions(&s, &r).iter().any(|x| matches!(x,
            Action::CastSpell { object_id, .. } if *object_id == cub)),
            "nothing floating, so the cub is not castable yet");
        assert_eq!(colors(&available_mana(&s, p, &r)), vec![(ManaColor::Green, 2)]);
        assert!(playable_cards(&s, p, &r).contains(&cub));

        let s = tap(&s, &r, a, 0, ManaColor::Green);
        let s = tap(&s, &r, b, 0, ManaColor::Green);
        let cast = legal_actions(&s, &r).into_iter().find(|x| matches!(x,
            Action::CastSpell { object_id, .. } if *object_id == cub))
            .expect("two green pay for {1}{G}");
        let (s, _) = step(s, cast, &r);
        assert!(pool(&s, p).is_empty());
        assert!(!s.stack_is_empty(), "the cub is a spell on the stack");

        // No timing: in the other player's turn, with a spell on the
        // stack, the creature still taps for mana (CR 605.3a).
        let mut s = table(1 - p);
        let bear = enter(&mut s, &r, p, BEARS);
        counter(&mut s, bear, CounterKind::PlusOnePlusOne);
        enter(&mut s, &r, p, RISHKAR);
        s.player_mut(1 - p).mana_pool.add_mana(ManaColor::Green, 2, 0);
        let theirs = add(&mut s, &r, 1 - p, BEARS, Zone::Hand(1 - p));
        let cast = legal_actions(&s, &r).into_iter().find(|x| matches!(x,
            Action::CastSpell { object_id, .. } if *object_id == theirs)).expect("castable");
        let (s, _) = step(s, cast, &r);
        let (s, _) = step(s, Action::PassPriority, &r);
        assert_eq!(s.priority_player(), p);
        assert_eq!(s.stack.len(), 1);
        let after = {
            let p0 = s.player(p).mana_pool.total();
            let action = legal_actions(&s, &r).into_iter().find(|x| matches!(x,
                Action::ActivateAbility { source, .. } if *source == bear))
                .expect("offered in response");
            let (after, _) = step(s.clone(), action, &r);
            assert_eq!(after.player(p).mana_pool.total(), p0 + 1);
            after
        };
        assert_eq!(after.stack.len(), 1, "the mana ability added nothing to the stack");
        assert_eq!(after.priority_player(), p);
    }
}

// --- the flat index --------------------------------------------------------

/// What activating each offered index of `id` adds, read off the game.
fn colors_by_index(s: &GameState, r: &CardRegistry, id: ObjectId) -> Vec<(usize, ManaColor)> {
    let p = s.priority_player();
    offered(s, r, id).into_iter().map(|index| {
        let action = legal_actions(s, r).into_iter().find(|a| matches!(a,
            Action::ActivateAbility { source, ability_index, .. }
                if *source == id && *ability_index == index)).unwrap();
        let (after, _) = step(s.clone(), action, r);
        let made = pool(&after, p);
        assert_eq!(made.len(), 1, "one color from one activation: {made:?}");
        assert_eq!(made[0].1, 1);
        (index, made[0].0)
    }).collect()
}

#[test]
fn several_grants_keep_the_flat_index_consistent() {
    let r = registry();
    use ManaColor::{Colorless, Green, Red};
    for p in [0, 1] {
        // Llanowar Elves has a printed ability at index 0; grants follow
        // it in the order their effects were installed.
        let mut s = table(p);
        let elves = enter(&mut s, &r, p, ELVES);
        counter(&mut s, elves, CounterKind::PlusOnePlusOne);
        assert_eq!(colors_by_index(&s, &r, elves), vec![(0, Green)]);

        let rishkar = enter(&mut s, &r, p, RISHKAR);
        // An attachment's grant, installed between the two statics: the
        // two kinds of grant share the one list and its order.
        let relic = enter(&mut s, &r, p, RELIC);
        s.objects.get_mut(relic).unwrap().attached_to = Some(elves);
        s.objects.get_mut(elves).unwrap().attachments.push(relic);
        s.add_continuous_effect(ContinuousEffect::attached_activated(
            relic, tap_for(COLORLESS, add_colorless), Duration::WhileSourceOnBattlefield));
        enter(&mut s, &r, p, KINDLER);

        assert_eq!(grants(&s, elves), vec![GREEN, COLORLESS, RED]);
        assert_eq!(colors_by_index(&s, &r, elves),
            vec![(0, Green), (1, Green), (2, Colorless), (3, Red)]);
        // The label an interface shows is the granted ability's own text.
        for (index, text) in [(1, GREEN), (2, COLORLESS), (3, RED)] {
            let action = legal_actions(&s, &r).into_iter().find(|a| matches!(a,
                Action::ActivateAbility { source, ability_index, .. }
                    if *source == elves && *ability_index == index)).unwrap();
            let label = render_action(&action, &s, &r);
            assert!(label.ends_with(text), "{label}");
        }
        // One tap spends the creature for every one of them.
        let tapped = tap(&s, &r, elves, 3, Red);
        assert!(offered(&tapped, &r, elves).is_empty());

        // Take the first grant away and the rest move down together, in
        // what is offered and in what each index does.
        let mut fewer = s.clone();
        fewer.move_object_to_zone(rishkar, Zone::Graveyard(p), MoveCause::SpellResolution);
        assert_eq!(grants(&fewer, elves), vec![COLORLESS, RED]);
        assert_eq!(colors_by_index(&fewer, &r, elves),
            vec![(0, Green), (1, Colorless), (2, Red)]);

        // The other order of entry gives the other order of indices.
        let mut s = table(p);
        let elves = enter(&mut s, &r, p, ELVES);
        counter(&mut s, elves, CounterKind::PlusOnePlusOne);
        enter(&mut s, &r, p, KINDLER);
        enter(&mut s, &r, p, RISHKAR);
        assert_eq!(colors_by_index(&s, &r, elves), vec![(0, Green), (1, Red), (2, Green)]);

        // Two copies of one static are two abilities, not one and not
        // three, and losing a copy loses one.
        let mut s = table(p);
        let bear = enter(&mut s, &r, p, BEARS);
        counter(&mut s, bear, CounterKind::PlusOnePlusOne);
        let first = enter(&mut s, &r, p, KINDLER);
        enter(&mut s, &r, p, KINDLER);
        assert_eq!(grants(&s, bear), vec![RED, RED]);
        assert_eq!(colors_by_index(&s, &r, bear), vec![(0, Red), (1, Red)]);
        s.move_object_to_zone(first, Zone::Graveyard(p), MoveCause::SpellResolution);
        assert_eq!(colors_by_index(&s, &r, bear), vec![(0, Red)]);
    }
}

#[test]
fn a_granted_second_color_makes_a_flexible_source() {
    let r = registry();
    for p in [0, 1] {
        // The Acolyte taps for white. With a counter and Rishkar it can
        // tap for green instead, which is what the Elves in hand needs;
        // the "playable if you tap out" reading has to notice the choice.
        let mut s = table(p);
        let acolyte = enter(&mut s, &r, p, ACOLYTE);
        counter(&mut s, acolyte, CounterKind::PlusOnePlusOne);
        let elves = add(&mut s, &r, p, ELVES, Zone::Hand(p));
        assert!(!playable_cards(&s, p, &r).contains(&elves), "white alone does not cast it");
        enter(&mut s, &r, p, RISHKAR);
        assert_eq!(colors_by_index(&s, &r, acolyte),
            vec![(0, ManaColor::White), (1, ManaColor::Green)]);
        assert!(playable_cards(&s, p, &r).contains(&elves));
    }
}
