//! A1.5bp witnesses: a permanent's effective characteristics are captured
//! as it leaves the battlefield, and an ability reads them afterward (CR
//! 113.7a, 608.2h). The registered clients are Mausoleum Wanderer and
//! Witch's Oven; six synthetic cards cover shapes no registered card has.
use arcana_core::actions::{Action, AdditionalCostPayment, ChoiceKind, ChoiceResponse};
use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::engine::step;
use arcana_core::events::MoveCause;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::legal_actions::legal_actions;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::{Characteristics, GameObject, ObjectId, ILLEGAL_TARGET_ID, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry, SpellAbilityDef,
};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{
    ColorSet, CounterKind, ManaColor, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

const WANDERER: &str = "Mausoleum Wanderer";
const EAGLE: &str = "Empyrean Eagle";
const OVEN: &str = "Witch's Oven";
const WISP: &str = "Bp Test Wisp";
const STEWARD: &str = "Bp Test Steward";
const OX: &str = "Bp Test Ox";
const BOAR: &str = "Bp Test Boar";
const ALTAR: &str = "Bp Test Altar";
const BAKER: &str = "Bp Test Baker";
const EDICT: &str = "Bp Test Edict";
const REGENT: &str = "Bp Test Regent";
const WIPE: &str = "Day of Judgment";

/// The catalog plus:
/// * Wisp, a 1/1 Spirit with no ability, to set off the Wanderer's trigger;
/// * Steward, a 0/0 whose own static gives creatures you control +1/+1,
///   itself included;
/// * Ox, a vanilla 2/3, and Boar, a vanilla 3/4, for the Oven;
/// * Altar, whose mana ability sacrifices a creature for colorless mana
///   equal to that creature's last-known power;
/// * Baker, which sacrifices itself for one Food and one more for each
///   other permanent its context lists as sacrificed;
/// * Edict, a sorcery: you sacrifice two creatures;
/// * Regent, a legendary Steward, for the legend rule.
fn registry() -> CardRegistry {
    let mut r = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut r);
    let spirit = r.interner_mut().intern("Spirit");
    let creature = |r: &mut CardRegistry, name: &str, p: i32, t: i32, subtypes: SubtypeSet| {
        let name = r.interner_mut().intern(name);
        Characteristics {
            name,
            mana_cost: Some(ManaCost::parse("{U}").unwrap()),
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(p)),
            toughness: Some(PtValue::Fixed(t)),
            ..Default::default()
        }
    };
    let mut spirits = SubtypeSet::default();
    spirits.0.insert(spirit);
    let wisp = creature(&mut r, WISP, 1, 1, spirits);
    r.register(CardDefinition::new(wisp.name, wisp));
    let ox = creature(&mut r, OX, 2, 3, SubtypeSet::default());
    r.register(CardDefinition::new(ox.name, ox));
    let boar = creature(&mut r, BOAR, 3, 4, SubtypeSet::default());
    r.register(CardDefinition::new(boar.name, boar));
    let steward = creature(&mut r, STEWARD, 0, 0, SubtypeSet::default());
    r.register(CardDefinition::new(steward.name, steward).with_static(vec![
        ContinuousEffect::filtered_pump(
            NULL_OBJECT_ID,
            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            1, 1, Duration::WhileSourceOnBattlefield),
    ]));
    let mut regent = creature(&mut r, REGENT, 0, 0, SubtypeSet::default());
    regent.supertypes = SupertypeSet(SupertypeSet::LEGENDARY);
    r.register(CardDefinition::new(regent.name, regent).with_static(vec![
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
    let ability = |cost: ActivationCost, is_mana_ability: bool,
                   effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>| {
        ActivatedAbilityDef {
            text: String::new(),
            cost,
            target_requirements: Vec::new(),
            is_mana_ability,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: true,
            face_gate: None,
            effect,
        }
    };
    let altar = artifact(&mut r, ALTAR);
    r.register(CardDefinition::new(altar.name, altar).with_activated_ability(ability(
        ActivationCost { sacrifice_other: Some(ObjectFilter::creature()), ..Default::default() },
        true, altar_mana)));
    let baker = artifact(&mut r, BAKER);
    r.register(CardDefinition::new(baker.name, baker).with_activated_ability(ability(
        ActivationCost { sacrifice: true, ..Default::default() }, false, baker_food)));
    let edict = r.interner_mut().intern(EDICT);
    r.register(CardDefinition::new(edict, Characteristics {
        name: edict,
        mana_cost: Some(ManaCost::parse("{B}").unwrap()),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    }).with_spell_ability(SpellAbilityDef {
        text: "Sacrifice two creatures.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: edict_resolve,
    }));
    r
}

fn altar_mana(state: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    let power = ctx.sacrificed.first()
        .and_then(|id| script::last_known_power(state, *id)).unwrap_or(0).max(0);
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: (0..power).map(|_| ManaUnit::plain(ManaColor::Colorless, ctx.source)).collect(),
    }]
}

fn baker_food(_: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: ctx.controller,
        kind: CommodityToken::Food,
        count: 1 + ctx.sacrificed.len() as u32,
    }]
}

fn edict_resolve(_: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Sacrifice {
        player: entry.controller, filter: ObjectFilter::creature(), count: 2,
    }]
}

/// `active`'s first main phase with priority and a library for each seat.
fn table(r: &CardRegistry, active: PlayerId) -> GameState {
    let mut s = GameState::new(2, 20261113);
    s.turn.active_player = active;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(active);
    for p in [0, 1] {
        for _ in 0..6 { add(&mut s, r, p, "Island", Zone::Library(p)); }
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

/// Onto the battlefield through the zone mover, so the entry hook installs
/// a printed static; direct insertion would skip it.
fn enter(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str) -> ObjectId {
    let id = add(s, r, p, name, Zone::Hand(p));
    s.move_object_to_zone(id, Zone::Battlefield, MoveCause::SpellResolution)
        .expect("entered")
}

/// Treat everything that has happened so far as already seen by the
/// trigger scan: the permanents on the battlefield entered long ago. The
/// scan asks every permanent now on the battlefield about each unseen
/// event, so without this a permanent would answer for an entry that
/// preceded its own.
fn long_ago(s: &mut GameState) {
    s.trigger_event_cursor = s.event_log.len();
}

fn act(s: GameState, r: &CardRegistry, pick: impl Fn(&Action) -> bool) -> GameState {
    let chosen = legal_actions(&s, r).into_iter().find(|a| pick(a))
        .unwrap_or_else(|| panic!("no such action; pending {:?}", s.pending_choice));
    step(s, chosen, r).0
}

fn pass(s: GameState, r: &CardRegistry) -> GameState {
    act(s, r, |a| a.is_pass())
}

/// Resolve everything, taking the first non-concede answer to any prompt.
fn finish(mut s: GameState, r: &CardRegistry) -> GameState {
    for _ in 0..100 {
        if s.stack_is_empty() && s.pending_choice.is_none()
            && s.pending_trigger_queue.is_empty() { return s; }
        s = act(s, r, |a| !a.is_concede());
    }
    panic!("unsettled stack");
}

fn power(s: &GameState, id: ObjectId) -> i32 {
    s.computed_power(id).expect("a creature on the battlefield")
}

fn hand(s: &GameState, p: PlayerId) -> usize {
    s.objects.count_in_zone(Zone::Hand(p))
}

fn zone_of(s: &GameState, r: &CardRegistry, name: &str) -> Zone {
    let card = r.card_id_by_name(name).unwrap();
    s.objects.iter().find(|o| o.card_id == card).expect("the card exists").zone
}

fn food(s: &GameState, p: PlayerId) -> usize {
    s.objects.objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == p && o.commodity == Some(CommodityToken::Food))
        .count()
}

/// The opponent of `owner` casts Opt with `spare` extra mana floating;
/// priority is passed until `owner` holds it with only Opt on the stack,
/// so any trigger the board set off has resolved. Returns the state there.
fn opponent_casts_opt(mut s: GameState, r: &CardRegistry, owner: PlayerId, spare: u32)
    -> GameState
{
    let opp = 1 - owner;
    let opt = add(&mut s, r, opp, "Opt", Zone::Hand(opp));
    s.player_mut(opp).mana_pool.add_mana(ManaColor::Blue, 1, 0);
    s.player_mut(opp).mana_pool.add_mana(ManaColor::Colorless, spare, 0);
    s = act(s, r, |a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == opt));
    for _ in 0..10 {
        if s.stack.len() == 1 && s.priority_player() == owner && s.pending_choice.is_none() {
            return s;
        }
        s = pass(s, r);
    }
    panic!("the owner never got priority over Opt alone");
}

/// `owner` sacrifices the Wanderer at the one spell on the stack.
fn sacrifice_at_spell(s: GameState, r: &CardRegistry, wanderer: ObjectId) -> GameState {
    let spell = s.stack.last().expect("a spell to counter").id;
    act(s, r, |a| matches!(a, Action::ActivateAbility { source, targets, .. }
        if *source == wanderer
            && targets.targets == vec![TargetChoice::Object(spell)]))
}

/// Pass until the Wanderer's ability asks the spell's controller to pay,
/// and return the generic amount asked.
fn until_asked(mut s: GameState, r: &CardRegistry) -> (GameState, u32) {
    for _ in 0..10 {
        if let Some(pc) = &s.pending_choice {
            let ChoiceKind::PayOrDecline { cost, .. } = &pc.kind
                else { panic!("unexpected prompt {:?}", pc.kind) };
            let x = cost.mana_value();
            return (s, x);
        }
        s = pass(s, r);
    }
    panic!("the spell's controller was never asked to pay");
}

fn answer_payment(s: GameState, r: &CardRegistry, pay: bool) -> GameState {
    let s = act(s, r, |a| matches!(a, Action::SubmitResolutionChoice {
        response: ChoiceResponse::PayOrDecline { pay: p }, .. } if *p == pay));
    finish(s, r)
}

#[test]
fn a_wanderer_pumped_by_its_own_trigger_asks_for_its_pumped_power_in_either_seat() {
    let r = registry();
    for owner in [0, 1] {
        let opp = 1 - owner;
        let mut s = table(&r, opp);
        let wanderer = enter(&mut s, &r, owner, WANDERER);
        enter(&mut s, &r, owner, WISP);
        let s = opponent_casts_opt(s, &r, owner, 6);
        assert_eq!(power(&s, wanderer), 2, "seat {owner}: the trigger pumped it");
        let s = sacrifice_at_spell(s, &r, wanderer);
        // The raw snapshot stays the printed 1/1; the effective one is beside it.
        assert_eq!(s.lki(wanderer).unwrap().characteristics.power, Some(PtValue::Fixed(1)));
        assert_eq!(script::last_known_power(&s, wanderer), Some(2));
        assert_eq!(script::power_of(&s, wanderer), 0, "power_of keeps its fallback");
        let (s, x) = until_asked(s, &r);
        assert_eq!(x, 2, "seat {owner}: X is the pumped power");
        assert_eq!(s.pending_choice.as_ref().unwrap().choosing_player, opp);

        // Paying lets Opt resolve: its controller draws.
        let paid = answer_payment(s.clone(), &r, true);
        assert_eq!(hand(&paid, opp), 1, "seat {owner}: Opt resolved");
        assert_eq!(zone_of(&paid, &r, "Opt"), Zone::Graveyard(opp));
        assert_eq!(paid.player(opp).mana_pool.total(), 4, "seat {owner}: {{2}} was paid");

        // Declining counters it: no card is drawn.
        let declined = answer_payment(s, &r, false);
        assert_eq!(hand(&declined, opp), 0, "seat {owner}: Opt was countered");
        assert_eq!(zone_of(&declined, &r, "Opt"), Zone::Graveyard(opp));
        assert_eq!(declined.player(opp).mana_pool.total(), 6);
    }
}

#[test]
fn a_controller_who_cannot_pay_x_is_offered_only_the_decline() {
    let r = registry();
    let mut s = table(&r, 1);
    let wanderer = enter(&mut s, &r, 0, WANDERER);
    enter(&mut s, &r, 0, WISP);
    // One spare mana against X = 2.
    let s = opponent_casts_opt(s, &r, 0, 1);
    let (s, x) = until_asked(sacrifice_at_spell(s, &r, wanderer), &r);
    assert_eq!(x, 2);
    let answers: Vec<bool> = legal_actions(&s, &r).into_iter().filter_map(|a| match a {
        Action::SubmitResolutionChoice { response: ChoiceResponse::PayOrDecline { pay }, .. } =>
            Some(pay),
        _ => None,
    }).collect();
    assert_eq!(answers, vec![false]);
    let s = answer_payment(s, &r, false);
    assert_eq!(hand(&s, 1), 0, "Opt was countered");
}

#[test]
fn an_unpumped_wanderer_asks_for_one() {
    let r = registry();
    let mut s = table(&r, 1);
    let wanderer = enter(&mut s, &r, 0, WANDERER);
    let s = opponent_casts_opt(s, &r, 0, 6);
    let (_, x) = until_asked(sacrifice_at_spell(s, &r, wanderer), &r);
    assert_eq!(x, 1);
}

#[test]
fn an_anthem_counts_in_x_when_it_was_in_force_as_the_wanderer_left() {
    let r = registry();
    for owner in [0, 1] {
        let opp = 1 - owner;
        // The Eagle is on the battlefield first, so its own entry does not
        // set off the Wanderer's trigger: the bonus below is the anthem's.
        let board = |s: &mut GameState| {
            let eagle = enter(s, &r, owner, EAGLE);
            long_ago(s);
            (eagle, enter(s, &r, owner, WANDERER))
        };

        // In force at the sacrifice.
        let mut s = table(&r, opp);
        let (_, wanderer) = board(&mut s);
        let s = opponent_casts_opt(s, &r, owner, 6);
        assert_eq!(power(&s, wanderer), 2);
        let (_, x) = until_asked(sacrifice_at_spell(s, &r, wanderer), &r);
        assert_eq!(x, 2, "seat {owner}: the anthem is in X");

        // Its source removed in response, after the sacrifice: the capture
        // was taken at departure, so X still has the bonus.
        let mut s = table(&r, opp);
        let (eagle, wanderer) = board(&mut s);
        let s = opponent_casts_opt(s, &r, owner, 6);
        let mut s = sacrifice_at_spell(s, &r, wanderer);
        Effect::DestroyPermanent { target: eagle }.execute(&mut s);
        assert_ne!(s.objects.get(eagle).map(|o| o.zone), Some(Zone::Battlefield));
        let (_, x) = until_asked(s, &r);
        assert_eq!(x, 2, "seat {owner}: X is fixed as the Wanderer left");

        // Its source gone before the sacrifice: no bonus.
        let mut s = table(&r, opp);
        let (eagle, wanderer) = board(&mut s);
        let mut s = opponent_casts_opt(s, &r, owner, 6);
        Effect::DestroyPermanent { target: eagle }.execute(&mut s);
        assert_eq!(power(&s, wanderer), 1);
        let (_, x) = until_asked(sacrifice_at_spell(s, &r, wanderer), &r);
        assert_eq!(x, 1, "seat {owner}: the anthem had left first");
    }
}

#[test]
fn the_snapshot_lasts_as_long_as_the_ability_that_reads_it() {
    let r = registry();
    let mut s = table(&r, 1);
    enter(&mut s, &r, 0, EAGLE);
    long_ago(&mut s);
    let wanderer = enter(&mut s, &r, 0, WANDERER);
    let s = opponent_casts_opt(s, &r, 0, 6);
    assert!(s.last_known_characteristics(wanderer).is_none(), "nothing before it leaves");
    let s = sacrifice_at_spell(s, &r, wanderer);
    assert!(s.last_known_characteristics(wanderer).is_some(), "on the stack");
    // Each player passes; the ability resolves and parks on the payment.
    let (s, _) = until_asked(s, &r);
    assert!(s.last_known_characteristics(wanderer).is_some(), "parked on the payment");
    let mut s = answer_payment(s, &r, false);
    // The stack, the queue and the parked resolution are empty; the next
    // sweep releases the snapshot with the raw one.
    for _ in 0..4 {
        if s.last_known_characteristics(wanderer).is_none() { break; }
        s = pass(s, &r);
    }
    assert!(s.last_known_characteristics(wanderer).is_none(), "released");
    assert!(s.lki(wanderer).is_none(), "with the raw snapshot");
    assert_eq!(script::last_known_power(&s, wanderer), None);
}

#[test]
fn a_placeholder_and_a_live_object_read_apart_from_a_departed_one() {
    let r = registry();
    let mut s = table(&r, 0);
    let wanderer = enter(&mut s, &r, 0, WANDERER);
    // Live: the current power and toughness.
    assert_eq!(script::last_known_power(&s, wanderer), Some(1));
    assert_eq!(script::last_known_toughness(&s, wanderer), Some(1));
    // A1.5bo's masked illegal target names nothing, and reads as nothing.
    assert_eq!(script::last_known_power(&s, ILLEGAL_TARGET_ID), None);
    assert_eq!(script::last_known_toughness(&s, ILLEGAL_TARGET_ID), None);
    assert_eq!(script::last_known_power(&s, NULL_OBJECT_ID), None);
}

#[test]
fn a_permanent_s_own_static_is_in_its_capture() {
    let r = registry();
    let mut s = table(&r, 0);
    let steward = enter(&mut s, &r, 0, STEWARD);
    assert_eq!(power(&s, steward), 1);
    s.move_object_to_zone(steward, Zone::Graveyard(0), MoveCause::SpellResolution);
    // Taken before the Steward's own anthem went with it.
    assert_eq!(script::last_known_power(&s, steward), Some(1));
    assert_eq!(script::last_known_toughness(&s, steward), Some(1));
}

/// Put `eagle_first`'s chosen order of ids on the battlefield and mark
/// lethal damage on both.
fn lord_and_spirit(r: &CardRegistry, eagle_first: bool) -> (GameState, ObjectId, ObjectId) {
    let mut s = table(r, 0);
    let (eagle, wanderer) = if eagle_first {
        let eagle = enter(&mut s, r, 0, EAGLE);
        long_ago(&mut s);
        (eagle, enter(&mut s, r, 0, WANDERER))
    } else {
        let wanderer = enter(&mut s, r, 0, WANDERER);
        (enter(&mut s, r, 0, EAGLE), wanderer)
    };
    // Entering second, the Eagle would set off the Wanderer's trigger, but
    // nothing resolves here: the Wanderer is 2/2 from the anthem alone.
    assert_eq!(power(&s, wanderer), 2);
    (s, eagle, wanderer)
}

#[test]
fn creatures_that_die_in_one_sweep_are_captured_with_the_board_before_it() {
    let r = registry();
    for eagle_first in [true, false] {
        let (mut s, eagle, wanderer) = lord_and_spirit(&r, eagle_first);
        s.objects.get_mut(eagle).unwrap().damage_marked = 3;
        s.objects.get_mut(wanderer).unwrap().damage_marked = 2;
        arcana_core::sba::apply_state_based_actions(&mut s);
        assert!(s.objects.get(eagle).is_none() && s.objects.get(wanderer).is_none());
        // The lord's bonus is in the other creature's snapshot whichever
        // of them the sweep happened to move first.
        assert_eq!(script::last_known_power(&s, wanderer), Some(2), "eagle first: {eagle_first}");
        assert_eq!(script::last_known_toughness(&s, wanderer), Some(2));
        assert_eq!(script::last_known_power(&s, eagle), Some(2));
    }
}

#[test]
fn a_board_wipe_captures_every_creature_with_the_board_before_it() {
    let r = registry();
    for eagle_first in [true, false] {
        let (mut s, eagle, wanderer) = lord_and_spirit(&r, eagle_first);
        for color in [ManaColor::White, ManaColor::Blue] {
            s.player_mut(0).mana_pool.add_mana(color, 4, 0);
        }
        // Day of Judgment says "destroy all creatures" as one instruction,
        // a `ForEach`.
        let wipe = add(&mut s, &r, 0, WIPE, Zone::Hand(0));
        let s = act(s, &r, |a| matches!(a,
            Action::CastSpell { object_id, .. } if *object_id == wipe));
        // Whatever the Eagle's entry set off resolves above the wipe and
        // is gone by then; read the Wanderer's power as the wipe finds it.
        let mut s = s;
        let mut before = None;
        for _ in 0..10 {
            if s.objects.get(wanderer).is_none() { break; }
            before = s.computed_power(wanderer);
            s = act(s, &r, |a| !a.is_concede());
        }
        assert!(s.objects.get(eagle).is_none() && s.objects.get(wanderer).is_none());
        let before = before.expect("the Wanderer was on the battlefield");
        assert!(before >= 2, "the anthem was in force");
        assert_eq!(script::last_known_power(&s, wanderer), Some(before),
            "eagle first: {eagle_first}");
    }
}

#[test]
fn a_creature_that_survives_a_board_wipe_keeps_no_capture_from_it() {
    let r = registry();
    let (mut s, eagle, wanderer) = lord_and_spirit(&r, true);
    let boar = enter(&mut s, &r, 0, BOAR);
    Effect::Regenerate { target: boar }.execute(&mut s);
    for color in [ManaColor::White, ManaColor::Blue] {
        s.player_mut(0).mana_pool.add_mana(color, 4, 0);
    }
    let wipe = add(&mut s, &r, 0, WIPE, Zone::Hand(0));
    let s = act(s, &r, |a| matches!(a,
        Action::CastSpell { object_id, .. } if *object_id == wipe));
    let mut s = finish(s, &r);
    assert!(s.objects.get(eagle).is_none() && s.objects.get(wanderer).is_none());
    assert_eq!(s.objects.get(boar).map(|o| o.zone), Some(Zone::Battlefield), "regenerated");
    assert!(s.last_known_characteristics(boar).is_none(), "it never left");
    // The wipe staged a 3/4 Boar. What leaves later is a 5/6.
    Effect::AddCounters { target: boar, kind: CounterKind::PlusOnePlusOne, count: 2 }
        .execute(&mut s);
    Effect::ExilePermanent { target: boar }.execute(&mut s);
    assert_eq!(script::last_known_power(&s, boar), Some(5));
    assert_eq!(script::last_known_toughness(&s, boar), Some(6));
}

#[test]
fn one_instruction_s_sacrifices_are_captured_with_the_board_before_them() {
    let r = registry();
    for eagle_first in [true, false] {
        let (mut s, eagle, wanderer) = lord_and_spirit(&r, eagle_first);
        s.player_mut(0).mana_pool.add_mana(ManaColor::Black, 1, 0);
        let edict = add(&mut s, &r, 0, EDICT, Zone::Hand(0));
        let s = act(s, &r, |a| matches!(a,
            Action::CastSpell { object_id, .. } if *object_id == edict));
        let mut s = s;
        let mut before = None;
        for _ in 0..12 {
            if s.objects.get(wanderer).is_none() { break; }
            before = s.computed_power(wanderer);
            s = act(s, &r, |a| !a.is_concede());
        }
        assert!(s.objects.get(eagle).is_none() && s.objects.get(wanderer).is_none());
        let before = before.expect("the Wanderer was on the battlefield");
        assert!(before >= 2);
        assert_eq!(script::last_known_power(&s, wanderer), Some(before),
            "eagle first: {eagle_first}");
    }
}

#[test]
fn an_instruction_nested_out_of_the_resolver_s_sight_is_captured_the_same_way() {
    let r = registry();
    for eagle_first in [true, false] {
        let (mut s, eagle, wanderer) = lord_and_spirit(&r, eagle_first);
        // Executed directly, as a Conditional's branch executes it: the
        // resolver's flattening never sees this list.
        let boar = enter(&mut s, &r, 0, BOAR);
        Effect::Regenerate { target: boar }.execute(&mut s);
        Effect::ForEach {
            targets: vec![eagle, wanderer, boar],
            effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
        }.execute(&mut s);
        assert!(s.objects.get(eagle).is_none() && s.objects.get(wanderer).is_none());
        assert_eq!(script::last_known_power(&s, wanderer), Some(2), "eagle first: {eagle_first}");
        // The Boar regenerated; what was staged for it is not kept for the
        // day it does leave.
        assert_eq!(s.objects.get(boar).map(|o| o.zone), Some(Zone::Battlefield));
        Effect::AddCounters { target: boar, kind: CounterKind::PlusOnePlusOne, count: 2 }
            .execute(&mut s);
        Effect::ExilePermanent { target: boar }.execute(&mut s);
        assert_eq!(script::last_known_power(&s, boar), Some(5));
    }
}

#[test]
fn the_legend_rule_s_copies_are_captured_with_the_board_before_they_went() {
    let r = registry();
    for owner in [0, 1] {
        let mut s = table(&r, owner);
        let regents: Vec<ObjectId> = (0..3).map(|_| enter(&mut s, &r, owner, REGENT)).collect();
        assert!(regents.iter().all(|id| power(&s, *id) == 3), "three anthems each");
        arcana_core::sba::apply_state_based_actions(&mut s);
        let keep = regents[1];
        let s = act(s, &r, |a| matches!(a, Action::SubmitResolutionChoice {
            response: ChoiceResponse::PickCards { picked }, .. } if *picked == vec![keep]));
        assert_eq!(power(&s, keep), 1, "the kept one stands alone");
        for gone in [regents[0], regents[2]] {
            assert!(s.objects.get(gone).is_none());
            assert_eq!(script::last_known_power(&s, gone), Some(3), "seat {owner}");
            assert_eq!(script::last_known_toughness(&s, gone), Some(3));
        }
    }
}

#[test]
fn a_creature_that_survives_a_sweep_keeps_no_capture_from_it() {
    let r = registry();
    let mut s = table(&r, 0);
    let ox = enter(&mut s, &r, 0, OX);
    let boar = enter(&mut s, &r, 0, BOAR);
    Effect::Regenerate { target: boar }.execute(&mut s);
    s.objects.get_mut(ox).unwrap().damage_marked = 3;
    s.objects.get_mut(boar).unwrap().damage_marked = 4;
    arcana_core::sba::apply_state_based_actions(&mut s);
    assert!(s.objects.get(ox).is_none(), "the Ox died");
    assert_eq!(s.objects.get(boar).map(|o| o.zone), Some(Zone::Battlefield), "regenerated");
    assert!(s.last_known_characteristics(boar).is_none(), "it never left");
    // What it becomes later is what its real departure records.
    Effect::AddCounters { target: boar, kind: CounterKind::PlusOnePlusOne, count: 2 }
        .execute(&mut s);
    s.move_object_to_zone(boar, Zone::Graveyard(0), MoveCause::SpellResolution);
    assert_eq!(script::last_known_power(&s, boar), Some(5));
    assert_eq!(script::last_known_toughness(&s, boar), Some(6));
}

/// Sacrifice `creature` to the Oven and count the Food its ability makes.
fn bake(r: &CardRegistry, owner: PlayerId, prepare: impl Fn(&mut GameState, ObjectId), name: &str)
    -> usize
{
    let mut s = table(r, owner);
    let oven = enter(&mut s, r, owner, OVEN);
    let creature = enter(&mut s, r, owner, name);
    prepare(&mut s, creature);
    let s = act(s, r, |a| matches!(a, Action::ActivateAbility { source, additional_costs, .. }
        if *source == oven
            && additional_costs.contains(&AdditionalCostPayment::Sacrifice(creature))));
    assert!(s.objects.get(creature).is_none(), "sacrificed as the cost");
    assert_eq!(food(&s, owner), 0, "the ability is on the stack");
    food(&finish(s, r), owner)
}

#[test]
fn the_oven_reads_the_sacrificed_creature_s_last_known_toughness_in_either_seat() {
    let r = registry();
    for owner in [0, 1] {
        // Printed toughness decides when nothing changes it.
        assert_eq!(bake(&r, owner, |_, _| {}, OX), 1, "seat {owner}: a 2/3");
        assert_eq!(bake(&r, owner, |_, _| {}, BOAR), 2, "seat {owner}: a 3/4");
        // Printed 3, at 4 through a counter.
        assert_eq!(bake(&r, owner, |s, ox| Effect::AddCounters {
            target: ox, kind: CounterKind::PlusOnePlusOne, count: 1 }.execute(s), OX),
            2, "seat {owner}: a counter");
        // Printed 3, at 4 through an anthem.
        assert_eq!(bake(&r, owner, |s, _| { enter(s, &r, owner, STEWARD); }, OX),
            2, "seat {owner}: an anthem");
        // Printed 4, lowered to 3 by an effect.
        assert_eq!(bake(&r, owner, |s, boar| Effect::Pump {
            target: boar, power: 0, toughness: -1,
            duration: Duration::EndOfTurn, keywords: vec![] }.execute(s), BOAR),
            1, "seat {owner}: shrunk");
    }
}

#[test]
fn a_mana_ability_reads_the_creature_sacrificed_to_pay_for_it() {
    let r = registry();
    let mut s = table(&r, 0);
    let altar = enter(&mut s, &r, 0, ALTAR);
    enter(&mut s, &r, 0, STEWARD);
    let ox = enter(&mut s, &r, 0, OX);
    assert_eq!(power(&s, ox), 3, "the Steward's anthem");
    let s = act(s, &r, |a| matches!(a, Action::ActivateAbility { source, additional_costs, .. }
        if *source == altar
            && additional_costs.contains(&AdditionalCostPayment::Sacrifice(ox))));
    assert!(s.stack_is_empty(), "a mana ability does not use the stack");
    assert_eq!(s.player(0).mana_pool.total(), 3);
}

#[test]
fn the_source_s_own_sacrifice_is_not_listed_as_another_permanent() {
    let r = registry();
    let mut s = table(&r, 0);
    let baker = enter(&mut s, &r, 0, BAKER);
    enter(&mut s, &r, 0, OX);
    let s = act(s, &r, |a| matches!(a,
        Action::ActivateAbility { source, .. } if *source == baker));
    assert_eq!(food(&finish(s, &r), 0), 1);
}

#[test]
fn only_another_spirit_s_entry_pumps_the_wanderer() {
    let r = registry();
    for owner in [0, 1] {
        let opp = 1 - owner;
        // Its own entry: still a 1/1 once everything has resolved.
        let mut s = table(&r, opp);
        let wanderer = enter(&mut s, &r, owner, WANDERER);
        let s = opponent_casts_opt(s, &r, owner, 0);
        assert_eq!(power(&s, wanderer), 1, "seat {owner}: its own entry");

        // An opponent's Spirit is not one you control.
        let mut s = table(&r, opp);
        let wanderer = enter(&mut s, &r, owner, WANDERER);
        enter(&mut s, &r, opp, WISP);
        let s = opponent_casts_opt(s, &r, owner, 0);
        assert_eq!(power(&s, wanderer), 1, "seat {owner}: an opponent's Spirit");

        // A second Wanderer pumps the first and not itself.
        let mut s = table(&r, opp);
        let first = enter(&mut s, &r, owner, WANDERER);
        long_ago(&mut s);
        let second = enter(&mut s, &r, owner, WANDERER);
        let s = opponent_casts_opt(s, &r, owner, 0);
        assert_eq!(power(&s, first), 2, "seat {owner}: another Spirit entered");
        assert_eq!(power(&s, second), 1, "seat {owner}: the newcomer saw no other");
    }
}

/// "Destroy target creature. Destroy another target creature.": two
/// instructions, each with its own target.
fn two_strikes(_: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    let ids: Vec<_> = entry.targets.targets.iter().filter_map(|t| match t {
        TargetChoice::Object(id) => Some(*id),
        _ => None,
    }).collect();
    vec![Effect::DestroyPermanent { target: ids[0] }, Effect::DestroyPermanent { target: ids[1] }]
}

/// The same two instructions, each in a container of its own.
fn two_strikes_in_sequences(s: &GameState, entry: &StackEntry, r: &CardRegistry) -> Vec<Effect> {
    two_strikes(s, entry, r).into_iter().map(|e| Effect::Sequence(vec![e])).collect()
}

#[test]
fn a_later_instruction_reads_the_board_the_earlier_one_left() {
    // CR 608.2c: the second destruction happens after the first. A lord
    // destroyed by the first instruction is not in the capture of a
    // creature the second one destroys, however alike the two effects
    // look in the resolving list; destroyed in the other order, it is.
    let mut r = registry();
    let creature = || TargetRequirement {
        filter: TargetFilter::Permanent(ObjectFilter::creature()),
        count: TargetCount::Exactly(1),
        controller: None,
    };
    for (card, effect) in [
        ("Bp Test Two Strikes", two_strikes as fn(&GameState, &StackEntry, &CardRegistry) -> Vec<Effect>),
        ("Bp Test Two Strikes Apart", two_strikes_in_sequences),
    ] {
        let name = r.interner_mut().intern(card);
        r.register(CardDefinition::new(name, Characteristics {
            name, types: TypeLine::SORCERY.into(), mana_cost: Some(ManaCost::empty()),
            ..Default::default()
        }).with_spell_ability(SpellAbilityDef {
            text: "Destroy target creature. Destroy another target creature.".into(),
            target_requirements: vec![creature(), creature()],
            modal: None,
            effect,
        }));
    }
    for card in ["Bp Test Two Strikes", "Bp Test Two Strikes Apart"] {
        for lord_first in [true, false] {
            let mut s = table(&r, 0);
            let steward = enter(&mut s, &r, 0, STEWARD);
            let ox = enter(&mut s, &r, 0, OX);
            long_ago(&mut s);
            assert_eq!(s.computed_toughness(ox), Some(4));
            let spell = add(&mut s, &r, 0, card, Zone::Hand(0));
            let order = if lord_first { vec![steward, ox] } else { vec![ox, steward] };
            let wanted: Vec<_> = order.iter().map(|id| TargetChoice::Object(*id)).collect();
            let s = act(s, &r, |a| matches!(a, Action::CastSpell { object_id, targets, .. }
                if *object_id == spell && targets.targets == wanted));
            let s = finish(s, &r);
            assert!(s.objects.get(steward).is_none() && s.objects.get(ox).is_none());
            assert_eq!(script::last_known_toughness(&s, ox), Some(if lord_first { 3 } else { 4 }),
                "{card}, the lord destroyed first: {lord_first}");
            assert_eq!(script::last_known_toughness(&s, steward), Some(1), "{card}");
        }
    }
}

/// "Sacrifice two creatures: Add {C} for each point of their total power."
fn feast(state: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    let power: i32 = ctx.sacrificed.iter()
        .map(|id| script::last_known_power(state, *id).unwrap())
        .sum();
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: (0..power).map(|_| ManaUnit::plain(ManaColor::Colorless, ctx.source)).collect(),
    }]
}

#[test]
fn the_creatures_one_cost_sacrifices_are_captured_with_the_board_before_them() {
    // "Sacrifice two creatures" is one cost, paid whole (CR 601.2h,
    // 602.2b): a lord among the two is in the other's capture whichever
    // of them the payment happens to move first.
    let mut r = registry();
    let name = r.interner_mut().intern("Bp Test Feast");
    r.register(CardDefinition::new(name, Characteristics {
        name, types: TypeLine::ARTIFACT.into(), ..Default::default()
    }).with_activated_ability(ActivatedAbilityDef {
        text: "Sacrifice two creatures: Add {C} for each point of their total power.".into(),
        cost: ActivationCost {
            sacrifice_other: Some(ObjectFilter::creature()),
            sacrifice_other_count: 2,
            ..Default::default()
        },
        target_requirements: Vec::new(), is_mana_ability: true, is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield, is_instant_speed: true, face_gate: None,
        effect: feast,
    }));
    for player in [0, 1] {
        for lord_first in [true, false] {
            let mut s = table(&r, player);
            let feast = enter(&mut s, &r, player, "Bp Test Feast");
            let (steward, ox) = if lord_first {
                (enter(&mut s, &r, player, STEWARD), enter(&mut s, &r, player, OX))
            } else {
                let ox = enter(&mut s, &r, player, OX);
                (enter(&mut s, &r, player, STEWARD), ox)
            };
            long_ago(&mut s);
            assert_eq!((s.computed_power(steward), s.computed_power(ox)), (Some(1), Some(3)));
            let s = act(s, &r, |a| matches!(a,
                Action::ActivateAbility { source, .. } if *source == feast));
            assert!(s.objects.get(steward).is_none() && s.objects.get(ox).is_none());
            assert_eq!(s.player(player).mana_pool.total(), 4,
                "seat {player}, the lord allocated first: {lord_first}");
            assert_eq!(script::last_known_power(&s, ox), Some(3));
            assert_eq!(script::last_known_toughness(&s, ox), Some(4));
        }
    }
}

fn wipe_group(state: &GameState, player: PlayerId) -> Effect {
    Effect::ForEach {
        targets: state.objects.ids_in_zone_sorted(Zone::Battlefield).into_iter()
            .filter(|id| state.objects.get(*id).is_some_and(|o|
                o.controller == player && o.characteristics.types.is_creature()))
            .collect(),
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }
}

fn scry_then_wipe(state: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Scry { player: entry.controller, count: 1 }, wipe_group(state, entry.controller)]
}

fn two_parked_wipes(state: &GameState, entry: &StackEntry, r: &CardRegistry) -> Vec<Effect> {
    // Two pauses before the first batch and another before the second;
    // nested containers must retain the same instruction boundaries.
    vec![Effect::Sequence(vec![
        Effect::Scry { player: entry.controller, count: 1 },
        Effect::Sequence(scry_then_wipe(state, entry, r)),
        Effect::Scry { player: entry.controller, count: 1 },
        wipe_group(state, 1 - entry.controller),
    ])]
}

fn pay_to_grow_then_wipe(state: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    let ox = state.objects.ids_in_zone_sorted(Zone::Battlefield).into_iter()
        .find(|id| state.computed_power(*id) == Some(3)).unwrap();
    vec![
        Effect::OptionalPayment {
            chooser: entry.controller,
            cost: arcana_core::actions::OptionalPaymentKind::Life(1),
            then: Box::new(Effect::Sequence(vec![
                Effect::AddCounters { target: ox, kind: CounterKind::PlusOnePlusOne, count: 1 },
                Effect::Scry { player: entry.controller, count: 1 },
            ])),
            else_effect: None,
        },
        wipe_group(state, entry.controller),
    ]
}

fn departure_spell(
    r: &mut CardRegistry,
    card: &str,
    effect: fn(&GameState, &StackEntry, &CardRegistry) -> Vec<Effect>,
) {
    let name = r.interner_mut().intern(card);
    r.register(CardDefinition::new(name, Characteristics {
        name, types: TypeLine::SORCERY.into(), mana_cost: Some(ManaCost::empty()),
        ..Default::default()
    }).with_spell_ability(SpellAbilityDef {
        text: card.into(), target_requirements: Vec::new(), modal: None, effect,
    }));
}

fn lord_and_ox(s: &mut GameState, r: &CardRegistry, player: PlayerId, lord_first: bool)
    -> (ObjectId, ObjectId)
{
    if lord_first {
        (enter(s, r, player, STEWARD), enter(s, r, player, OX))
    } else {
        let ox = enter(s, r, player, OX);
        (enter(s, r, player, STEWARD), ox)
    }
}

#[test]
fn a_choice_before_a_wipe_preserves_its_simultaneous_departure() {
    let mut r = registry();
    let card = "Bp Scry Then Wipe";
    departure_spell(&mut r, card, scry_then_wipe);
    for player in [0, 1] {
        for lord_first in [true, false] {
            let mut s = table(&r, player);
            let (lord, ox) = lord_and_ox(&mut s, &r, player, lord_first);
            let survivor = enter(&mut s, &r, player, BOAR);
            Effect::Regenerate { target: survivor }.execute(&mut s);
            long_ago(&mut s);
            let spell = add(&mut s, &r, player, card, Zone::Hand(player));
            let s = act(s, &r, |a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == spell));
            let s = pass(pass(s, &r), &r);
            assert!(s.pending_choice.is_some());
            assert!(s.objects.get(lord).is_some() && s.objects.get(ox).is_some());
            let s = finish(s, &r);
            assert!(s.objects.get(lord).is_none() && s.objects.get(ox).is_none());
            assert_eq!(script::last_known_toughness(&s, ox), Some(4),
                "seat {player}, lord first {lord_first}");
            // Reconstructing the group must replace its flattened
            // children, not execute them again after the regenerated wipe.
            assert!(s.objects.get(survivor).is_some(), "the wipe ran only once");
            assert!(s.last_known_characteristics(survivor).is_none());
        }
    }
}

#[test]
fn repeated_choices_preserve_each_later_departure_group() {
    let mut r = registry();
    let card = "Bp Two Parked Wipes";
    departure_spell(&mut r, card, two_parked_wipes);
    for player in [0, 1] {
        for lord_first in [true, false] {
            let mut s = table(&r, player);
            let first = lord_and_ox(&mut s, &r, player, lord_first);
            let second = lord_and_ox(&mut s, &r, 1 - player, lord_first);
            long_ago(&mut s);
            let spell = add(&mut s, &r, player, card, Zone::Hand(player));
            let s = act(s, &r, |a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == spell));
            let mut s = pass(pass(s, &r), &r);
            for _ in 0..2 {
                assert!(s.pending_choice.is_some());
                s = act(s, &r, |a| !a.is_concede());
            }
            // The third scry is after the first wipe. Read its held LKI
            // while resolution is parked; completing the spell may retire it.
            assert!(s.pending_choice.is_some());
            assert!(s.objects.get(first.0).is_none() && s.objects.get(first.1).is_none());
            assert_eq!(script::last_known_toughness(&s, first.1), Some(4));
            assert!(s.objects.get(second.0).is_some() && s.objects.get(second.1).is_some());
            let s = finish(s, &r);
            assert!(s.objects.get(second.0).is_none() && s.objects.get(second.1).is_none());
            assert_eq!(script::last_known_toughness(&s, second.1), Some(4),
                "seat {player}, lord first {lord_first}");
        }
    }
}

#[test]
fn a_resumed_departure_captures_changes_made_by_the_earlier_choice() {
    let mut r = registry();
    let card = "Bp Pay To Grow Then Wipe";
    departure_spell(&mut r, card, pay_to_grow_then_wipe);
    for player in [0, 1] {
        for lord_first in [true, false] {
            for pay in [true, false] {
                let mut s = table(&r, player);
                let (lord, ox) = lord_and_ox(&mut s, &r, player, lord_first);
                long_ago(&mut s);
                let spell = add(&mut s, &r, player, card, Zone::Hand(player));
                let s = act(s, &r, |a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == spell));
                let s = pass(pass(s, &r), &r);
                let s = act(s, &r, |a| matches!(a, Action::SubmitResolutionChoice {
                    response: ChoiceResponse::OptionalCost { pay: answer }, .. } if *answer == pay));
                let s = finish(s, &r);
                assert!(s.objects.get(lord).is_none() && s.objects.get(ox).is_none());
                assert_eq!(script::last_known_toughness(&s, ox), Some(if pay { 5 } else { 4 }),
                    "seat {player}, lord first {lord_first}, paid {pay}");
            }
        }
    }
}
