//! Cast-or-else: Chandra, Torch of Defiance's first +1. "Exile the top
//! card of your library. You may cast that card. If you don't, Chandra
//! deals 2 damage to each opponent." The card is cast during the
//! resolution (CR 608.2g), at its normal costs, by tapping for it while
//! the choice is open (A1.5cd); every way of not casting it leads to the
//! damage, and a cast made suppresses it whatever becomes of the spell.
use arcana_core::actions::{
    Action, AdditionalCostPayment, CastFace, CastModifier, ChoiceKind, ChoiceResponse,
    CostReductions, ManaPaymentPlan,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::engine::step;
use arcana_core::events::MoveCause;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::legal_actions::legal_actions;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, GameObject, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetSelection,
};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CounterKind, ManaColor, PlayerId, TypeLine};
use arcana_core::zones::Zone;

const CHANDRA: &str = "Chandra, Torch of Defiance";

const TRINKET: &str = "Coe Test Trinket";
const VISION: &str = "Coe Test Vision";

/// The catalog plus Trinket, a {0} artifact with kicker {2}, and Vision, a
/// sorcery with no mana cost (as a suspend-only card has none).
fn registry() -> CardRegistry {
    let mut r = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut r);
    let trinket = r.interner_mut().intern(TRINKET);
    r.register(CardDefinition::new(trinket, Characteristics {
        name: trinket,
        mana_cost: Some(ManaCost::parse("{0}").unwrap()),
        types: TypeLine::ARTIFACT.into(),
        keywords: vec![KeywordAbility::Kicker(ManaCost::parse("{2}").unwrap())],
        ..Default::default()
    }));
    let vision = r.interner_mut().intern(VISION);
    r.register(CardDefinition::new(vision, Characteristics {
        name: vision,
        mana_cost: None,
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    }));
    r
}

/// `you`'s first main phase with priority; each seat has a library of
/// Mountains, so `you`'s Chandra exiles whatever a test puts on top.
fn table(r: &CardRegistry, you: PlayerId) -> GameState {
    let mut s = GameState::new(2, 20261114);
    s.turn.active_player = you;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(you);
    for p in [0, 1] {
        for _ in 0..6 { add(&mut s, r, p, "Mountain", Zone::Library(p)); }
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

fn enter(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str) -> ObjectId {
    let id = add(s, r, p, name, Zone::Hand(p));
    s.move_object_to_zone(id, Zone::Battlefield, MoveCause::SpellResolution)
        .expect("entered")
}

/// Put `name` on top of `p`'s library.
fn on_top(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str) {
    let id = add(s, r, p, name, Zone::Library(p));
    let lib = &mut s.player_mut(p).library_top_to_bottom;
    lib.pop();
    lib.insert(0, id);
}

fn act(s: GameState, r: &CardRegistry, pick: impl Fn(&Action) -> bool) -> GameState {
    let chosen = legal_actions(&s, r).into_iter().find(|a| pick(a))
        .unwrap_or_else(|| panic!("no such action; pending {:?}", s.pending_choice));
    step(s, chosen, r).0
}

fn pass(s: GameState, r: &CardRegistry) -> GameState {
    act(s, r, |a| a.is_pass())
}

/// Resolve everything, taking the first answer to any prompt.
fn finish(mut s: GameState, r: &CardRegistry) -> GameState {
    for _ in 0..100 {
        if s.stack_is_empty() && s.pending_choice.is_none()
            && s.pending_trigger_queue.is_empty() { return s; }
        s = act(s, r, |a| !a.is_concede());
    }
    panic!("unsettled stack");
}

fn name_zone(s: &GameState, r: &CardRegistry, name: &str) -> Vec<Zone> {
    let card = r.card_id_by_name(name).unwrap();
    s.objects.iter().filter(|o| o.card_id == card && !matches!(o.zone, Zone::Library(_)))
        .map(|o| o.zone).collect()
}

fn loyalty(s: &GameState, chandra: ObjectId) -> u32 {
    s.objects.get(chandra).expect("Chandra").count_counters(CounterKind::Loyalty)
}

fn offered_taps(s: &GameState, r: &CardRegistry) -> Vec<ObjectId> {
    legal_actions(s, r).iter().filter_map(|a| match a {
        Action::ActivateAbility { source, .. } => Some(*source),
        _ => None,
    }).collect()
}

fn tap(s: GameState, r: &CardRegistry, source: ObjectId) -> GameState {
    act(s, r, |a| matches!(a, Action::ActivateAbility { source: x, .. } if *x == source))
}

/// The casts a pending cast choice offers.
fn offered_casts(s: &GameState, r: &CardRegistry) -> Vec<Action> {
    legal_actions(s, r).into_iter().filter_map(|a| match a {
        Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastDuringResolution { cast: Some(c) }, .. } => Some(*c),
        _ => None,
    }).collect()
}

/// The answers of the pending choice: everything but Concede and the mana
/// abilities of the window.
fn answers(s: &GameState, r: &CardRegistry) -> Vec<Action> {
    legal_actions(s, r).into_iter()
        .filter(|a| matches!(a, Action::SubmitResolutionChoice { .. })).collect()
}

fn decline(s: GameState, r: &CardRegistry) -> GameState {
    act(s, r, |a| matches!(a, Action::SubmitResolutionChoice {
        response: ChoiceResponse::CastDuringResolution { cast: None }, .. }))
}

fn cast_where(s: GameState, r: &CardRegistry, pick: impl Fn(&Action) -> bool) -> GameState {
    act(s, r, |a| matches!(a, Action::SubmitResolutionChoice {
        response: ChoiceResponse::CastDuringResolution { cast: Some(c) }, .. } if pick(c)))
}

fn hits_player(sel: &TargetSelection, p: PlayerId) -> bool {
    sel.targets.iter().any(|t| matches!(t, TargetChoice::Player(q)
        | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(q)) if *q == p))
}

fn hits_object(sel: &TargetSelection, id: ObjectId) -> bool {
    sel.targets.iter().any(|t| matches!(t, TargetChoice::Object(o)
        | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(o)) if *o == id))
}

/// A board at Chandra's question. `you` controls Chandra at loyalty 4 and
/// `lands` untapped lands named `land`; `top` is the top card of the
/// library. The first +1 is activated and resolves to its choice; with an
/// empty library (`top` is `None`, the library emptied) there is none.
struct Board {
    s: GameState,
    foe: PlayerId,
    chandra: ObjectId,
    lands: Vec<ObjectId>,
    /// The exiled card, when one was exiled.
    exiled: Option<ObjectId>,
}

fn chandra_asks(r: &CardRegistry, you: PlayerId, top: Option<&str>, land: &str, lands: usize)
    -> Board
{
    chandra_asks_with(r, you, top, land, lands, |_, _| {})
}

fn chandra_asks_with(
    r: &CardRegistry, you: PlayerId, top: Option<&str>, land: &str, lands: usize,
    arrange: impl FnOnce(&mut GameState, PlayerId),
) -> Board {
    let foe = 1 - you;
    let mut s = table(r, you);
    let chandra = enter(&mut s, r, you, CHANDRA);
    if loyalty(&s, chandra) == 0 {
        s.objects.get_mut(chandra).unwrap().add_counters(CounterKind::Loyalty, 4);
    }
    let lands: Vec<ObjectId> = (0..lands).map(|_| enter(&mut s, r, you, land)).collect();
    match top {
        Some(name) => on_top(&mut s, r, you, name),
        None => {
            let ids = std::mem::take(&mut s.player_mut(you).library_top_to_bottom);
            for id in ids { s.objects.remove(id); }
        }
    }
    arrange(&mut s, you);
    s.trigger_event_cursor = s.event_log.len();
    assert_eq!(loyalty(&s, chandra), 4);
    s = act(s, r, |a| matches!(a, Action::ActivateAbility { source, ability_index: 0, .. }
        if *source == chandra));
    assert_eq!(loyalty(&s, chandra), 5, "the +1 is paid");
    for _ in 0..4 {
        if s.stack_is_empty() || s.pending_choice.is_some() { break; }
        s = pass(s, r);
    }
    let exiled = match s.pending_choice.as_ref().map(|c| (c.choosing_player, c.kind.clone())) {
        Some((who, ChoiceKind::CastDuringResolution { object })) => {
            assert_eq!(who, you, "Chandra's controller is asked");
            assert_eq!(s.objects.get(object).map(|o| o.zone), Some(Zone::Exile));
            Some(object)
        }
        Some((_, kind)) => panic!("asked {kind:?}"),
        None => None,
    };
    Board { s, foe, chandra, lands, exiled }
}

/// An exiled instant with mana available: tapped for in the window, cast
/// and paid during the resolution, and no damage.
#[test]
fn an_exiled_instant_is_cast_from_lands_and_no_damage_follows() {
    let r = registry();
    for you in [0, 1] {
        let Board { mut s, foe, lands, exiled, .. } = chandra_asks(&r, you, Some("Opt"), "Island", 2);
        let opt = exiled.expect("Opt is exiled");
        assert_eq!(offered_taps(&s, &r), lands, "seat {you}: the window is open");
        assert!(offered_casts(&s, &r).is_empty(), "nothing floats yet");
        s = tap(s, &r, lands[0]);
        let casts = offered_casts(&s, &r);
        assert!(matches!(casts.as_slice(), [Action::CastSpell {
            object_id, cast_modifier: CastModifier::DuringResolution { face: CastFace::Main }, ..
        }] if *object_id == opt), "one cast of Opt: {casts:?}");
        let hand = s.objects.count_in_zone(Zone::Hand(you));
        s = cast_where(s, &r, |_| true);
        assert_eq!(s.player(you).mana_pool.total(), 0, "the cast spent the mana");
        assert!(s.pending_choice.is_none() && s.stack.len() == 1, "Opt is on the stack");
        assert_eq!(s.priority_player(), you, "priority comes only after the resolution");
        s = finish(s, &r);
        assert_eq!(s.objects.count_in_zone(Zone::Hand(you)), hand + 1, "Opt resolved");
        assert_eq!(name_zone(&s, &r, "Opt"), vec![Zone::Graveyard(you)]);
        assert_eq!(s.player(foe).life, 20, "the card was cast: no damage");
    }
}

/// Declining: 2 damage to the opponent, and the card stays in exile with
/// no permission to cast it later.
#[test]
fn declining_deals_two_and_leaves_the_card_in_exile() {
    let r = registry();
    for you in [0, 1] {
        let Board { mut s, foe, lands, exiled, chandra, .. } =
            chandra_asks(&r, you, Some("Opt"), "Island", 2);
        let opt = exiled.unwrap();
        s = tap(s, &r, lands[0]);
        s = finish(decline(s, &r), &r);
        assert_eq!(s.player(foe).life, 18, "seat {you}: Chandra deals 2");
        assert_eq!(s.player(you).life, 20);
        assert_eq!(loyalty(&s, chandra), 5);
        let card = s.objects.get(opt).expect("the card is where it was");
        assert_eq!(card.zone, Zone::Exile);
        assert!(!card.status.face_down && !card.impulse_play_pending, "face up, no permission");
        // Priority, an empty stack, a blue mana floating: nothing casts it.
        assert_eq!(s.priority_player(), you);
        assert_eq!(s.player(you).mana_pool.total(), 1);
        assert!(!legal_actions(&s, &r).iter().any(|a| matches!(a,
            Action::CastSpell { object_id, .. } if *object_id == opt)));
        // A forged cast under the resolution's modifier is refused.
        let (s, _) = step(s, Action::CastSpell {
            object_id: opt,
            targets: TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: ManaPaymentPlan::empty(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: CastModifier::DuringResolution { face: CastFace::Main },
            cost_reductions: CostReductions::default(),
        }, &r);
        assert_eq!(s.objects.get(opt).map(|o| o.zone), Some(Zone::Exile), "still in exile");
        assert!(s.stack_is_empty());
        assert_eq!(s.player(you).mana_pool.total(), 1, "and nothing was spent");
    }
}

/// An exiled land is not cast: declining is the only answer, the untapped
/// lands open no window, and the damage follows.
#[test]
fn an_exiled_land_deals_two() {
    let r = registry();
    for you in [0, 1] {
        let Board { mut s, foe, exiled, .. } = chandra_asks(&r, you, Some("Mountain"), "Island", 2);
        let land = exiled.expect("the Mountain is exiled");
        assert!(offered_taps(&s, &r).is_empty(), "nothing to pay for");
        assert!(matches!(answers(&s, &r).as_slice(), [Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastDuringResolution { cast: None }, .. }]));
        s = finish(decline(s, &r), &r);
        assert_eq!(s.player(foe).life, 18, "seat {you}");
        assert_eq!(s.objects.get(land).map(|o| o.zone), Some(Zone::Exile));
        assert!(!legal_actions(&s, &r).iter().any(|a| matches!(a,
            Action::PlayLand { object_id, .. } if *object_id == land)), "nor is it played");
    }
}

/// Not enough mana after the window: with one Island against Divination's
/// {2}{U}, tapping offers no cast, declining is the only answer, and the
/// damage follows.
#[test]
fn too_little_mana_offers_only_decline() {
    let r = registry();
    for you in [0, 1] {
        let Board { mut s, foe, lands, .. } = chandra_asks(&r, you, Some("Divination"), "Island", 1);
        assert_eq!(offered_taps(&s, &r), lands, "the window opens: a cast exists to pay for");
        s = tap(s, &r, lands[0]);
        assert!(offered_taps(&s, &r).is_empty());
        assert!(matches!(answers(&s, &r).as_slice(), [Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastDuringResolution { cast: None }, .. }]),
            "one blue against {{2}}{{U}}");
        s = finish(decline(s, &r), &r);
        assert_eq!(s.player(foe).life, 18, "seat {you}");
        assert_eq!(name_zone(&s, &r, "Divination"), vec![Zone::Exile]);
    }
}

/// Cast and then countered: the card was cast, so no damage.
#[test]
fn a_cast_that_is_countered_still_suppresses_the_damage() {
    let r = registry();
    for you in [0, 1] {
        let Board { mut s, foe, lands, .. } = chandra_asks(&r, you, Some("Opt"), "Island", 1);
        s = tap(s, &r, lands[0]);
        s = cast_where(s, &r, |_| true);
        let spell = s.stack.last().unwrap().id;
        let negate = add(&mut s, &r, foe, "Negate", Zone::Hand(foe));
        s.player_mut(foe).mana_pool.add_mana(ManaColor::Blue, 2, 0);
        s = pass(s, &r);
        let hand = s.objects.count_in_zone(Zone::Hand(you));
        s = act(s, &r, |a| matches!(a, Action::CastSpell { object_id, targets, .. }
            if *object_id == negate && hits_object(targets, spell)));
        s = finish(s, &r);
        assert_eq!(s.objects.count_in_zone(Zone::Hand(you)), hand, "Opt was countered");
        assert_eq!(name_zone(&s, &r, "Opt"), vec![Zone::Graveyard(you)]);
        assert_eq!(s.player(foe).life, 20, "seat {you}: cast, so no damage");
    }
}

/// With no card to exile nothing is cast, and the damage follows at once.
#[test]
fn an_empty_library_deals_two() {
    let r = registry();
    for you in [0, 1] {
        let Board { s, foe, exiled, .. } = chandra_asks(&r, you, None, "Island", 2);
        assert!(exiled.is_none() && s.pending_choice.is_none(), "nobody is asked");
        assert_eq!(s.player(foe).life, 18, "seat {you}");
    }
}

/// A spell with no legal target is not cast: Fatal Push with no creature
/// out offers only decline and opens no window.
#[test]
fn a_spell_with_no_legal_target_deals_two() {
    let r = registry();
    for you in [0, 1] {
        let Board { mut s, foe, .. } = chandra_asks(&r, you, Some("Fatal Push"), "Swamp", 2);
        assert!(offered_taps(&s, &r).is_empty(), "no cast to pay for");
        assert!(matches!(answers(&s, &r).as_slice(), [Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastDuringResolution { cast: None }, .. }]));
        s = finish(decline(s, &r), &r);
        assert_eq!(s.player(foe).life, 18, "seat {you}");
    }
}

/// Casting restrictions still apply (CR 608.2g lifts card-type timing
/// only): under an opposing Teferi, Time Raveler nothing is cast during a
/// resolution, so declining is the only answer.
#[test]
fn an_opposing_teferi_refuses_the_cast() {
    let r = registry();
    for you in [0, 1] {
        let Board { mut s, foe, .. } = chandra_asks_with(&r, you, Some("Opt"), "Island", 2,
            |s, you| { enter(s, &r, 1 - you, "Teferi, Time Raveler"); });
        assert!(offered_taps(&s, &r).is_empty());
        assert!(matches!(answers(&s, &r).as_slice(), [Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastDuringResolution { cast: None }, .. }]));
        s = finish(decline(s, &r), &r);
        assert_eq!(s.player(foe).life, 18, "seat {you}");
    }
}

/// The card's own costs and choices: X is chosen up to the mana floating,
/// and the spell is paid for it. Stonecoil Serpent at X = 2 enters with
/// two counters.
#[test]
fn x_is_chosen_and_paid() {
    let r = registry();
    for you in [0, 1] {
        let Board { mut s, foe, lands, .. } =
            chandra_asks(&r, you, Some("Stonecoil Serpent"), "Mountain", 3);
        assert_eq!(offered_taps(&s, &r), lands, "an X cost can owe mana");
        let xs = |s: &GameState| -> Vec<u32> {
            let mut xs: Vec<u32> = offered_casts(s, &r).iter().filter_map(|a| match a {
                Action::CastSpell { x_value, .. } => *x_value, _ => None }).collect();
            xs.sort(); xs.dedup(); xs
        };
        assert_eq!(xs(&s), vec![0], "with nothing floating, X = 0");
        s = tap(s, &r, lands[0]);
        s = tap(s, &r, lands[1]);
        assert_eq!(xs(&s), vec![0, 1, 2]);
        let labels: Vec<String> = legal_actions(&s, &r).iter()
            .map(|a| arcana_core::render::render_action(a, &s, &r)).collect();
        assert_eq!(labels[0], "Don't cast it");
        assert!(labels.iter().any(|l| l == "Cast Stonecoil Serpent (X=2)"), "{labels:?}");
        s = cast_where(s, &r, |a| matches!(a, Action::CastSpell { x_value: Some(2), .. }));
        assert_eq!(s.player(you).mana_pool.total(), 0, "X = 2 took both");
        s = finish(s, &r);
        let serpent = s.objects.objects_in_zone(Zone::Battlefield)
            .find(|o| r.card_id_by_name("Stonecoil Serpent") == Some(o.card_id))
            .expect("the Serpent resolved");
        assert_eq!(serpent.count_counters(CounterKind::PlusOnePlusOne), 2);
        assert_eq!(s.player(foe).life, 20, "seat {you}");
    }
}

/// Modes and targets are chosen as for any cast: Abrade offers its damage
/// at the creature and its destruction at the artifact.
#[test]
fn modes_and_targets_are_chosen() {
    let r = registry();
    for you in [0, 1] {
        let mut bears = 0;
        let mut thopter = 0;
        let Board { mut s, foe, lands, .. } = chandra_asks_with(&r, you, Some("Abrade"), "Mountain", 2,
            |s, you| {
                bears = enter(s, &r, 1 - you, "Grizzly Bears");
                thopter = enter(s, &r, 1 - you, "Ornithopter");
            });
        s = tap(s, &r, lands[0]);
        s = tap(s, &r, lands[1]);
        let casts = offered_casts(&s, &r);
        let offers = |mode: usize, target: ObjectId| casts.iter().any(|a| matches!(a,
            Action::CastSpell { modes, targets, .. }
            if modes.len() == 1 && modes[0].mode_indices == vec![mode] && hits_object(targets, target)));
        assert!(offers(0, bears) && offers(0, thopter), "3 damage to either creature");
        assert!(offers(1, thopter) && !offers(1, bears), "destroy the artifact only");
        s = cast_where(s, &r, |a| matches!(a, Action::CastSpell { modes, targets, .. }
            if modes[0].mode_indices == vec![0] && hits_object(targets, bears)));
        s = finish(s, &r);
        assert!(s.objects.get(bears).is_none_or(|o| o.zone != Zone::Battlefield), "the Bears died");
        assert!(s.objects.get(thopter).is_some_and(|o| o.zone == Zone::Battlefield));
        assert_eq!(s.player(foe).life, 20, "seat {you}");
    }
}

/// Kicker is offered and paid: Burst Lightning unkicked for {R}, kicked
/// for {4}{R}.
#[test]
fn kicker_is_offered_and_paid() {
    let r = registry();
    for you in [0, 1] {
        let Board { mut s, foe, lands, .. } =
            chandra_asks(&r, you, Some("Burst Lightning"), "Mountain", 5);
        let kicked = |s: &GameState| offered_casts(s, &r).iter().any(|a| matches!(a,
            Action::CastSpell { additional_costs, .. }
            if additional_costs.contains(&AdditionalCostPayment::Kicker)));
        s = tap(s, &r, lands[0]);
        assert!(!offered_casts(&s, &r).is_empty() && !kicked(&s), "one red: unkicked only");
        for land in &lands[1..] { s = tap(s, &r, *land); }
        assert!(kicked(&s));
        s = cast_where(s, &r, |a| matches!(a, Action::CastSpell { additional_costs, targets, .. }
            if additional_costs.contains(&AdditionalCostPayment::Kicker) && hits_player(targets, foe)));
        s = finish(s, &r);
        assert_eq!(s.player(foe).life, 16, "seat {you}: 4 from the kicked Burst Lightning, none from Chandra");
    }
}

/// Either face of the card may be cast (CR 709.3): Bonecrusher Giant is
/// offered as the Giant and as Stomp, and Stomp cast this way goes on an
/// adventure like any other.
#[test]
fn an_adventure_card_is_cast_as_either_face() {
    let r = registry();
    for you in [0, 1] {
        let Board { mut s, foe, lands, .. } =
            chandra_asks(&r, you, Some("Bonecrusher Giant"), "Mountain", 3);
        for land in &lands { s = tap(s, &r, *land); }
        let faces: Vec<CastFace> = offered_casts(&s, &r).iter().filter_map(|a| match a {
            Action::CastSpell { cast_modifier: CastModifier::DuringResolution { face }, .. } => Some(*face),
            _ => None }).collect();
        assert!(faces.contains(&CastFace::Main) && faces.contains(&CastFace::Adventure), "{faces:?}");
        s = cast_where(s, &r, |a| matches!(a, Action::CastSpell {
            cast_modifier: CastModifier::DuringResolution { face: CastFace::Adventure }, targets, .. }
            if hits_player(targets, foe)));
        s = finish(s, &r);
        assert_eq!(s.player(foe).life, 18, "seat {you}: Stomp's 2, none from Chandra");
        let giant = s.objects.iter()
            .find(|o| r.card_id_by_name("Bonecrusher Giant") == Some(o.card_id)).unwrap();
        assert!(giant.zone == Zone::Exile && giant.adventure_exile_pending, "on an adventure");
    }
}

/// Cost changes apply (CR 601.2f): with noncreature spells taxed {1}, Opt
/// needs two mana.
#[test]
fn a_cost_increase_is_paid() {
    let r = registry();
    for you in [0, 1] {
        let Board { mut s, lands, .. } = chandra_asks_with(&r, you, Some("Opt"), "Island", 2,
            |s, you| {
                let bears = enter(s, &r, 1 - you, "Grizzly Bears");
                s.add_continuous_effect(ContinuousEffect::spell_cost_modifier(
                    bears,
                    ObjectFilter::default().without_types(TypeLine::CREATURE.into()),
                    ControllerConstraint::Any, 1, Duration::WhileSourceOnBattlefield));
            });
        s = tap(s, &r, lands[0]);
        assert!(offered_casts(&s, &r).is_empty(), "seat {you}: {{U}} is one short of {{1}}{{U}}");
        s = tap(s, &r, lands[1]);
        assert_eq!(offered_casts(&s, &r).len(), 1);
    }
}

/// A cost increase reaches the kicker too: taxed {1}, Burst Lightning is
/// {1}{R} unkicked and {5}{R} kicked.
#[test]
fn a_cost_increase_reaches_the_kicker() {
    let r = registry();
    let kicked = |s: &GameState| offered_casts(s, &r).iter().any(|a| matches!(a,
        Action::CastSpell { additional_costs, .. }
        if additional_costs.contains(&AdditionalCostPayment::Kicker)));
    for you in [0, 1] {
        let Board { mut s, lands, .. } = chandra_asks_with(&r, you, Some("Burst Lightning"), "Mountain", 6,
            |s, you| {
                let bears = enter(s, &r, 1 - you, "Grizzly Bears");
                s.add_continuous_effect(ContinuousEffect::spell_cost_modifier(
                    bears,
                    ObjectFilter::default().without_types(TypeLine::CREATURE.into()),
                    ControllerConstraint::Any, 1, Duration::WhileSourceOnBattlefield));
            });
        for land in &lands[..5] { s = tap(s, &r, *land); }
        assert!(!offered_casts(&s, &r).is_empty() && !kicked(&s),
            "seat {you}: five mana pay {{1}}{{R}}, not {{5}}{{R}}");
        s = tap(s, &r, lands[5]);
        assert!(kicked(&s));
    }
}

/// A kicker alone can be owed: Trinket costs {0}, so it is offered at
/// once, and its {2} kicker still opens the window.
#[test]
fn a_kicker_on_a_free_spell_opens_the_window() {
    let r = registry();
    for you in [0, 1] {
        let Board { mut s, lands, .. } = chandra_asks(&r, you, Some(TRINKET), "Mountain", 2);
        assert_eq!(offered_casts(&s, &r).len(), 1, "seat {you}: unkicked, for nothing");
        assert_eq!(offered_taps(&s, &r), lands, "the kicker can be paid for");
        for land in &lands { s = tap(s, &r, *land); }
        assert_eq!(offered_casts(&s, &r).len(), 2, "kicked too");
    }
}

/// A card with no mana cost cannot be paid for (CR 118.6), so it is not
/// cast: declining is the only answer, and no window opens.
#[test]
fn a_card_with_no_mana_cost_is_not_cast() {
    let r = registry();
    for you in [0, 1] {
        let Board { mut s, foe, .. } = chandra_asks(&r, you, Some(VISION), "Mountain", 2);
        assert!(offered_taps(&s, &r).is_empty());
        assert!(matches!(answers(&s, &r).as_slice(), [Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastDuringResolution { cast: None }, .. }]));
        s = finish(decline(s, &r), &r);
        assert_eq!(s.player(foe).life, 18, "seat {you}");
    }
}

/// A split card is cast as either half, and on the stack it is that half:
/// Fire // Ice has mana value 4 in exile and 2 as Fire or as Ice.
#[test]
fn a_split_card_is_cast_as_either_half() {
    let r = registry();
    for you in [0, 1] {
        for face in [CastFace::Main, CastFace::SplitRight] {
            let mut islands = Vec::new();
            let Board { mut s, lands, exiled, .. } = chandra_asks_with(&r, you, Some("Fire"), "Mountain", 2,
                |s, you| {
                    islands = (0..2).map(|_| enter(s, &r, you, "Island")).collect();
                    enter(s, &r, 1 - you, "Grizzly Bears");
                });
            let card = exiled.unwrap();
            assert_eq!(s.mana_value_of(s.objects.get(card).unwrap()), 4, "the whole card in exile");
            for land in lands.iter().chain(&islands) { s = tap(s, &r, *land); }
            let faces: Vec<CastFace> = offered_casts(&s, &r).iter().filter_map(|a| match a {
                Action::CastSpell { cast_modifier: CastModifier::DuringResolution { face }, .. } => Some(*face),
                _ => None }).collect();
            assert!(faces.contains(&CastFace::Main) && faces.contains(&CastFace::SplitRight), "{faces:?}");
            s = cast_where(s, &r, |a| matches!(a, Action::CastSpell {
                cast_modifier: CastModifier::DuringResolution { face: f }, .. } if *f == face));
            let on_stack = s.stack.iter().find(|e| e.is_spell()).expect("the spell").id;
            assert_eq!(s.mana_value_of(s.objects.get(on_stack).unwrap()), 2,
                "seat {you}: {face:?} on the stack is one half");
        }
    }
}

/// A spell that costs nothing is cast without a window: Ornithopter.
#[test]
fn a_free_spell_is_offered_without_a_window() {
    let r = registry();
    for you in [0, 1] {
        let Board { mut s, foe, .. } = chandra_asks(&r, you, Some("Ornithopter"), "Mountain", 2);
        assert!(offered_taps(&s, &r).is_empty(), "nothing is owed");
        assert_eq!(offered_casts(&s, &r).len(), 1);
        s = finish(cast_where(s, &r, |_| true), &r);
        assert!(name_zone(&s, &r, "Ornithopter") == vec![Zone::Battlefield]);
        assert_eq!(s.player(foe).life, 20, "seat {you}");
    }
}

/// One authorization never substitutes for another: Chandra's choice
/// refuses a cast "without paying its mana cost".
#[test]
#[should_panic(expected = "must cast during the resolution")]
fn chandras_choice_refuses_a_free_cast() {
    let r = registry();
    let Board { s, exiled, .. } = chandra_asks(&r, 0, Some("Opt"), "Island", 2);
    let id = s.pending_choice.as_ref().unwrap().id;
    let _ = step(s, Action::SubmitResolutionChoice {
        id,
        response: ChoiceResponse::CastDuringResolution { cast: Some(Box::new(Action::CastSpell {
            object_id: exiled.unwrap(),
            targets: TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: ManaPaymentPlan::empty(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: CastModifier::WithoutPayingManaCost { face: CastFace::Main },
            cost_reductions: CostReductions::default(),
        })) },
    }, &r);
}

/// An answer the engine cannot carry out is a bug, not a silent decline:
/// a cast with no payment is refused loudly, and "if you don't" does not
/// quietly vanish.
#[test]
#[should_panic(expected = "was refused")]
fn an_unpaid_cast_answer_panics() {
    let r = registry();
    let Board { s, exiled, .. } = chandra_asks(&r, 0, Some("Mountain"), "Island", 2);
    let id = s.pending_choice.as_ref().unwrap().id;
    let _ = step(s, Action::SubmitResolutionChoice {
        id,
        response: ChoiceResponse::CastDuringResolution { cast: Some(Box::new(Action::CastSpell {
            object_id: exiled.unwrap(),
            targets: TargetSelection::new(),
            modes: Vec::new(),
            mana_payment: ManaPaymentPlan::empty(),
            additional_costs: Vec::new(),
            x_value: None,
            cast_modifier: CastModifier::DuringResolution { face: CastFace::Main },
            cost_reductions: CostReductions::default(),
        })) },
    }, &r);
}

/// Chandra's other three abilities are as they were: the second +1 adds
/// {R}{R}, the −3 deals 4 to a creature, the −7 makes the emblem.
#[test]
fn chandras_other_abilities_are_unchanged() {
    let r = registry();
    for you in [0, 1] {
        let foe = 1 - you;
        let setup = |loyalty_now: u32| {
            let mut s = table(&r, you);
            let chandra = enter(&mut s, &r, you, CHANDRA);
            let have = loyalty(&s, chandra);
            if have < loyalty_now {
                s.objects.get_mut(chandra).unwrap()
                    .add_counters(CounterKind::Loyalty, loyalty_now - have);
            }
            let bears = enter(&mut s, &r, foe, "Grizzly Bears");
            s.trigger_event_cursor = s.event_log.len();
            (s, chandra, bears)
        };
        let (s, chandra, _) = setup(4);
        let s = finish(act(s, &r, |a| matches!(a, Action::ActivateAbility {
            source, ability_index: 1, .. } if *source == chandra)), &r);
        assert_eq!(s.player(you).mana_pool.total(), 2, "seat {you}: {{R}}{{R}}");
        assert_eq!(loyalty(&s, chandra), 5);

        let (s, chandra, bears) = setup(4);
        let s = finish(act(s, &r, |a| matches!(a, Action::ActivateAbility {
            source, ability_index: 2, targets, .. }
            if *source == chandra && hits_object(targets, bears))), &r);
        assert!(s.objects.get(bears).is_none_or(|o| o.zone != Zone::Battlefield), "4 to the Bears");
        assert_eq!(loyalty(&s, chandra), 1);

        let (s, chandra, _) = setup(7);
        let s = finish(act(s, &r, |a| matches!(a, Action::ActivateAbility {
            source, ability_index: 3, .. } if *source == chandra)), &r);
        assert_eq!(s.objects.objects_in_zone(Zone::Command).count(), 1, "the emblem");
        assert_eq!(s.player(foe).life, 20);
    }
}
