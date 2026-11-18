//! A1.5cd: a player asked for mana while a spell or ability resolves may
//! activate mana abilities before answering (CR 605.3a, 608.2g). The
//! activations are the asked player's, whoever holds priority; each leaves
//! the choice pending, and the pay answer is offered once the pool covers
//! the cost. The registered clients are Mausoleum Wanderer and Spell
//! Pierce ("counter it unless its controller pays"), an optional mana
//! payment, and Spell Queller's free cast with a kicker.
use arcana_core::actions::{
    Action, AdditionalCostPayment, ChoiceKind, ChoiceResponse, OptionalPaymentKind,
};
use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::engine::step;
use arcana_core::events::MoveCause;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::legal_actions::{available_mana, legal_actions};
use arcana_core::mana::{ManaCost, ManaRestrictions, ManaUnit, SpendRestriction};
use arcana_core::objects::{Characteristics, GameObject, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::{GameResult, GameState};
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetSelection,
};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{ColorSet, ManaColor, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

const WANDERER: &str = "Mausoleum Wanderer";
const WISP: &str = "Pw Test Wisp";
const TOLL: &str = "Pw Test Toll";
const PIERCE: &str = "Spell Pierce";
const QUELLER: &str = "Spell Queller";
const BURST: &str = "Burst Lightning";
const BOLT: &str = "Lightning Bolt";
const TEFERI: &str = "Teferi, Time Raveler";

/// The catalog plus Wisp, a 1/1 Spirit with no ability, to set off the
/// Wanderer's trigger, and Toll, a sorcery: "You may pay {2}. If you do,
/// draw a card."
fn registry() -> CardRegistry {
    let mut r = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut r);
    let spirit = r.interner_mut().intern("Spirit");
    let mut spirits = SubtypeSet::default();
    spirits.0.insert(spirit);
    let wisp = r.interner_mut().intern(WISP);
    r.register(CardDefinition::new(wisp, Characteristics {
        name: wisp,
        mana_cost: Some(ManaCost::parse("{U}").unwrap()),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: spirits,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    }));
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

/// `active`'s first main phase, `holder` with priority, and a library of
/// Islands for each seat.
fn table(r: &CardRegistry, active: PlayerId, holder: PlayerId) -> GameState {
    let mut s = GameState::new(2, 20261114);
    s.turn.active_player = active;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(holder);
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

/// Onto the battlefield through the zone mover, so the entry hook runs.
fn enter(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str) -> ObjectId {
    let id = add(s, r, p, name, Zone::Hand(p));
    s.move_object_to_zone(id, Zone::Battlefield, MoveCause::SpellResolution)
        .expect("entered")
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

/// Arrange the board with the pending choice set aside: effects and moves
/// made under a pending choice wait for its answer.
fn arrange<T>(s: &mut GameState, f: impl FnOnce(&mut GameState) -> T) -> T {
    let choice = s.pending_choice.take();
    let out = f(s);
    s.pending_choice = choice;
    out
}

fn hand(s: &GameState, p: PlayerId) -> usize {
    s.objects.count_in_zone(Zone::Hand(p))
}

fn pool(s: &GameState, p: PlayerId) -> usize {
    s.player(p).mana_pool.total()
}

fn tapped(s: &GameState, id: ObjectId) -> bool {
    s.objects.get(id).is_some_and(|o| o.is_tapped())
}

/// The mana-ability activations a prompt offers, by source.
fn offered_taps(s: &GameState, r: &CardRegistry) -> Vec<ObjectId> {
    legal_actions(s, r).iter().filter_map(|a| match a {
        Action::ActivateAbility { source, .. } => Some(*source),
        _ => None,
    }).collect()
}

fn pay_offered(s: &GameState, r: &CardRegistry) -> bool {
    legal_actions(s, r).iter().any(|a| matches!(a, Action::SubmitResolutionChoice {
        response: ChoiceResponse::PayOrDecline { pay: true }
            | ChoiceResponse::OptionalCost { pay: true }, .. }))
}

fn tap(s: GameState, r: &CardRegistry, source: ObjectId) -> GameState {
    act(s, r, |a| matches!(a, Action::ActivateAbility { source: x, .. } if *x == source))
}

fn answer(s: GameState, r: &CardRegistry, pay: bool) -> GameState {
    act(s, r, |a| matches!(a, Action::SubmitResolutionChoice {
        response: ChoiceResponse::PayOrDecline { pay: p }
            | ChoiceResponse::OptionalCost { pay: p }, .. } if *p == pay))
}

fn asked(s: &GameState) -> (PlayerId, ChoiceKind) {
    let pc = s.pending_choice.as_ref().expect("a prompt");
    (pc.choosing_player, pc.kind.clone())
}

fn pass_until_asked(mut s: GameState, r: &CardRegistry) -> GameState {
    for _ in 0..10 {
        if s.pending_choice.is_some() { return s; }
        s = pass(s, r);
    }
    panic!("nobody was asked");
}

/// A Wanderer board at its tax. `owner` controls Mausoleum Wanderer, grown
/// to 2/2 by a Spirit that entered this turn, and one Island; the opponent
/// controls `islands` untapped Islands and casts Opt with exactly its cost
/// floating. `owner` sacrifices the Wanderer at Opt, and the opponent is
/// asked for {2}. `active` names whose turn it is: on the owner's turn the
/// payer does not hold priority while asked.
struct Tax {
    s: GameState,
    opp: PlayerId,
    islands: Vec<ObjectId>,
    owners_island: ObjectId,
}

fn wanderer_tax(r: &CardRegistry, owner: PlayerId, active: PlayerId, islands: usize) -> Tax {
    let opp = 1 - owner;
    let mut s = table(r, active, opp);
    let wanderer = enter(&mut s, r, owner, WANDERER);
    enter(&mut s, r, owner, WISP);
    let owners_island = enter(&mut s, r, owner, "Island");
    let islands: Vec<ObjectId> = (0..islands).map(|_| enter(&mut s, r, opp, "Island")).collect();
    let opt = add(&mut s, r, opp, "Opt", Zone::Hand(opp));
    s.player_mut(opp).mana_pool.add_mana(ManaColor::Blue, 1, 0);
    s = act(s, r, |a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == opt));
    // The Wanderer's own trigger resolves first; then the owner answers Opt.
    for _ in 0..10 {
        if s.stack.len() == 1 && s.priority_player() == owner && s.pending_choice.is_none() {
            break;
        }
        s = pass(s, r);
    }
    assert_eq!(s.stack.len(), 1, "only Opt is on the stack");
    let spell = s.stack.last().unwrap().id;
    s = act(s, r, |a| matches!(a, Action::ActivateAbility { source, targets, .. }
        if *source == wanderer && targets.targets == vec![TargetChoice::Object(spell)]));
    let s = pass_until_asked(s, r);
    let (who, kind) = asked(&s);
    assert_eq!(who, opp, "the spell's controller is asked");
    let ChoiceKind::PayOrDecline { cost, .. } = kind else { panic!("asked {kind:?}") };
    assert_eq!(cost.mana_value(), 2, "the Wanderer's power as it was sacrificed");
    assert_eq!(pool(&s, opp), 0, "Opt took the floating mana");
    Tax { s, opp, islands, owners_island }
}

/// Every (owner, active player) arrangement: both seats, on either turn.
fn arrangements() -> [(PlayerId, PlayerId); 4] {
    [(0, 0), (0, 1), (1, 0), (1, 1)]
}

/// The brief's probe board. The payer is offered each untapped Island's
/// mana ability and nothing of the Wanderer's owner's; pay appears once
/// two are tapped; paying lets Opt resolve.
#[test]
fn the_payer_taps_lands_in_the_window_and_pays() {
    let r = registry();
    for (owner, active) in arrangements() {
        let Tax { mut s, opp, islands, owners_island, .. } = wanderer_tax(&r, owner, active, 3);
        assert_eq!(s.priority_player(), active, "priority sits with the active player");
        assert_eq!(offered_taps(&s, &r), islands, "seat {owner}, turn {active}: the payer's Islands");
        assert!(!offered_taps(&s, &r).contains(&owners_island));
        assert!(!pay_offered(&s, &r), "nothing floats yet");

        s = tap(s, &r, islands[0]);
        assert!(s.pending_choice.is_some(), "a mana ability leaves the choice pending");
        assert_eq!((pool(&s, opp), pool(&s, owner)), (1, 0), "the mana is the payer's");
        assert!(tapped(&s, islands[0]) && !tapped(&s, owners_island));
        assert!(!pay_offered(&s, &r), "one of two");
        assert_eq!(offered_taps(&s, &r), islands[1..].to_vec());

        s = tap(s, &r, islands[1]);
        assert!(pay_offered(&s, &r), "two of two");
        assert_eq!(offered_taps(&s, &r), vec![islands[2]], "the third stays on offer");

        let before = hand(&s, opp);
        s = finish(answer(s, &r, true), &r);
        assert_eq!(pool(&s, opp), 0, "the payment spent both");
        assert_eq!(hand(&s, opp), before + 1, "Opt resolved and drew");
    }
}

/// Declining, with or without mana floated, counters the spell.
#[test]
fn declining_in_the_window_counters_the_spell() {
    let r = registry();
    for (owner, active) in arrangements() {
        for taps in [0, 2] {
            let Tax { mut s, opp, islands, .. } = wanderer_tax(&r, owner, active, 3);
            for island in &islands[..taps] { s = tap(s, &r, *island); }
            let before = hand(&s, opp);
            s = finish(answer(s, &r, false), &r);
            assert_eq!(hand(&s, opp), before, "Opt was countered: no card drawn");
            assert_eq!(pool(&s, opp), taps, "mana floated and not spent stays for the step");
        }
    }
}

/// Sources that cannot reach the cost: the one Island is still offered,
/// tapping it does not make the cost payable, and what it made is ordinary
/// floating mana, gone when the engine empties the pools. (The engine
/// empties them at cleanup, not at each step's end; this test pins neither.)
#[test]
fn sources_short_of_the_cost_never_offer_pay() {
    let r = registry();
    for (owner, active) in arrangements() {
        let Tax { mut s, opp, islands, .. } = wanderer_tax(&r, owner, active, 1);
        assert_eq!(offered_taps(&s, &r), islands);
        assert!(!pay_offered(&s, &r));
        s = tap(s, &r, islands[0]);
        assert!(offered_taps(&s, &r).is_empty(), "nothing left to tap");
        assert!(!pay_offered(&s, &r), "one mana against {{2}}");
        s = finish(answer(s, &r, false), &r);
        assert_eq!(pool(&s, opp), 1, "the stranded mana floats");
        let turn = s.turn.turn_number;
        for _ in 0..60 {
            if s.turn.turn_number != turn { break; }
            s = act(s, &r, |a| !a.is_concede());
        }
        assert_ne!(s.turn.turn_number, turn, "the turn ended");
        assert_eq!(pool(&s, opp), 0, "and the mana went with it");
    }
}

/// A mana ability with another cost pays it: a Treasure is sacrificed in
/// the window, and its mana counts.
#[test]
fn a_treasure_is_sacrificed_for_mana_in_the_window() {
    let r = registry();
    for (owner, active) in arrangements() {
        let Tax { mut s, opp, islands, .. } = wanderer_tax(&r, owner, active, 1);
        arrange(&mut s, |s| Effect::CreateCommodityToken {
            controller: opp, kind: CommodityToken::Treasure, count: 1 }.execute(s));
        let treasure = s.objects.objects_in_zone(Zone::Battlefield)
            .find(|o| o.commodity == Some(CommodityToken::Treasure)).expect("a Treasure").id;
        assert_eq!(offered_taps(&s, &r), vec![islands[0], treasure]);
        s = tap(s, &r, treasure);
        assert!(s.objects.objects_in_zone(Zone::Battlefield)
            .all(|o| o.commodity != Some(CommodityToken::Treasure)), "the Treasure was sacrificed");
        assert_eq!(pool(&s, opp), 1);
        assert!(!pay_offered(&s, &r));
        s = tap(s, &r, islands[0]);
        assert!(pay_offered(&s, &r), "the Treasure's mana and the Island's");
        let before = hand(&s, opp);
        s = finish(answer(s, &r, true), &r);
        assert_eq!(hand(&s, opp), before + 1, "Opt resolved");
    }
}

/// Conceding in the window ends the game for the player asked.
#[test]
fn conceding_in_the_window_ends_the_game() {
    let r = registry();
    for (owner, active) in arrangements() {
        let Tax { mut s, islands, .. } = wanderer_tax(&r, owner, active, 3);
        s = tap(s, &r, islands[0]);
        let (s, _) = step(s, Action::Concede, &r);
        assert_eq!(s.result, Some(GameResult::Win(owner)), "the payer conceded");
    }
}

/// Restricted mana does not pay the tax (the window changes no spending
/// rule): two creature-only mana in the pool leave pay unoffered.
#[test]
fn restricted_mana_does_not_pay_in_the_window() {
    let r = registry();
    for (owner, active) in arrangements() {
        let Tax { mut s, opp, .. } = wanderer_tax(&r, owner, active, 0);
        for _ in 0..2 {
            s.player_mut(opp).mana_pool.pool.push(ManaUnit {
                color: ManaColor::Green,
                source: 0,
                restrictions: ManaRestrictions {
                    spend_only_on: Some(SpendRestriction::OnlyCastCreatureSpells),
                    ..Default::default()
                },
                kept_until_end_of_turn: false,
            });
        }
        assert!(!pay_offered(&s, &r), "creature-only mana is not spendable here");
        s.player_mut(opp).mana_pool.add_mana(ManaColor::Green, 2, 0);
        assert!(pay_offered(&s, &r), "plain mana is");
    }
}

/// Only choices that ask for mana open the window: Opt's scry, asked of
/// the same player with an Island still untapped, offers no activation.
#[test]
fn a_choice_that_asks_for_no_mana_offers_no_activation() {
    let r = registry();
    for (owner, active) in arrangements() {
        let Tax { mut s, opp, islands, .. } = wanderer_tax(&r, owner, active, 3);
        s = tap(s, &r, islands[0]);
        s = tap(s, &r, islands[1]);
        s = pass_until_asked(answer(s, &r, true), &r);
        let (who, kind) = asked(&s);
        assert!(matches!(kind, ChoiceKind::OrderCards { .. }), "Opt's scry, not {kind:?}");
        assert_eq!(who, opp);
        assert!(!tapped(&s, islands[2]));
        assert!(offered_taps(&s, &r).is_empty(), "a scry asks for no mana");
    }
}

/// A payment of nothing asks for no mana: "pay {0}" is offered at once and
/// the untapped Island is not.
#[test]
fn a_payment_of_nothing_opens_no_window() {
    use arcana_core::actions::{ChoiceContext, DeclineConsequence};
    let r = registry();
    for kind in [
        ChoiceKind::PayOrDecline {
            cost: ManaCost::default(), on_decline: DeclineConsequence::SkipEffect },
        ChoiceKind::OptionalCost { cost: OptionalPaymentKind::Mana(ManaCost::default()) },
    ] {
        let mut s = table(&r, 0, 0);
        enter(&mut s, &r, 1, "Island");
        s.push_pending_choice(1, ChoiceContext::ResolvingStack(0), kind);
        assert!(pay_offered(&s, &r), "nothing is owed");
        assert!(offered_taps(&s, &r).is_empty(), "so nothing is tapped for it");
    }
}

/// An activation while a choice asks for no mana is off the legal list,
/// and the engine refuses it.
#[test]
#[should_panic(expected = "asks for no mana")]
fn an_activation_during_a_scry_panics() {
    let r = registry();
    let Tax { mut s, islands, .. } = wanderer_tax(&r, 0, 1, 3);
    s = tap(s, &r, islands[0]);
    s = tap(s, &r, islands[1]);
    s = pass_until_asked(answer(s, &r, true), &r);
    let _ = step(s, Action::ActivateAbility {
        source: islands[2],
        ability_index: 0,
        targets: TargetSelection::new(),
        mana_payment: arcana_core::actions::ManaPaymentPlan::empty(),
        additional_costs: Vec::new(),
    }, &r);
}

/// The window is for mana abilities only: the payer's own Wanderer cannot
/// be sacrificed in it.
#[test]
#[should_panic(expected = "only a mana ability")]
fn a_non_mana_activation_in_the_window_panics() {
    let r = registry();
    let Tax { mut s, opp, .. } = wanderer_tax(&r, 0, 1, 3);
    let second = arrange(&mut s, |s| enter(s, &r, opp, WANDERER));
    assert!(!offered_taps(&s, &r).contains(&second), "not on the list");
    let spell = s.stack.first().unwrap().id;
    let _ = step(s, Action::ActivateAbility {
        source: second,
        ability_index: 0,
        targets: TargetSelection { targets: vec![TargetChoice::Object(spell)], ..TargetSelection::new() },
        mana_payment: arcana_core::actions::ManaPaymentPlan::empty(),
        additional_costs: vec![AdditionalCostPayment::Sacrifice(second)],
    }, &r);
}

/// `available_mana` measures the player it is asked about, whatever
/// choice is pending and whoever it asks: the Wanderer's owner has one
/// Island, in the pay window and at Opt's scry alike.
#[test]
fn available_mana_measures_its_player_while_a_choice_is_pending() {
    let r = registry();
    for (owner, active) in arrangements() {
        let Tax { mut s, opp, islands, owners_island, .. } = wanderer_tax(&r, owner, active, 3);
        assert_eq!(available_mana(&s, owner, &r).total(), 1, "in the window, the owner's Island");
        assert_eq!(available_mana(&s, opp, &r).total(), 3, "in the window, the payer's three");
        assert!(!tapped(&s, owners_island) && pool(&s, opp) == 0, "nothing was really tapped");
        s = tap(s, &r, islands[0]);
        s = tap(s, &r, islands[1]);
        s = pass_until_asked(answer(s, &r, true), &r);
        assert_eq!(available_mana(&s, owner, &r).total(), 1, "at the scry, the owner's Island");
        assert_eq!(available_mana(&s, opp, &r).total(), 1, "at the scry, the payer's third");
    }
}

/// Spell Pierce against a noncreature spell: its controller taps two
/// Islands in the window and pays, or declines and loses the spell.
#[test]
fn spell_pierce_is_paid_from_lands_or_counters() {
    let r = registry();
    for (caster, active) in arrangements() {
        for pay in [true, false] {
            let piercer = 1 - caster;
            let mut s = table(&r, active, caster);
            let islands: Vec<ObjectId> =
                (0..2).map(|_| enter(&mut s, &r, caster, "Island")).collect();
            let opt = add(&mut s, &r, caster, "Opt", Zone::Hand(caster));
            let pierce = add(&mut s, &r, piercer, PIERCE, Zone::Hand(piercer));
            s.player_mut(caster).mana_pool.add_mana(ManaColor::Blue, 1, 0);
            s.player_mut(piercer).mana_pool.add_mana(ManaColor::Blue, 1, 0);
            s = act(s, &r, |a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == opt));
            s = pass(s, &r);
            s = act(s, &r, |a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == pierce));
            s = pass_until_asked(s, &r);
            let (who, kind) = asked(&s);
            assert_eq!(who, caster, "Opt's controller is asked");
            assert!(matches!(kind, ChoiceKind::PayOrDecline { .. }));
            assert_eq!(offered_taps(&s, &r), islands);
            assert!(!pay_offered(&s, &r));
            let before = hand(&s, caster);
            if pay {
                s = tap(s, &r, islands[0]);
                s = tap(s, &r, islands[1]);
                assert!(pay_offered(&s, &r));
            }
            s = finish(answer(s, &r, pay), &r);
            assert_eq!(hand(&s, caster), before + usize::from(pay),
                "seat {caster}, turn {active}: Opt {}", if pay { "resolved" } else { "was countered" });
        }
    }
}

/// An optional mana payment opens the same window: "you may pay {2}".
#[test]
fn an_optional_mana_payment_opens_the_window() {
    let r = registry();
    for caster in [0, 1] {
        let mut s = table(&r, caster, caster);
        let islands: Vec<ObjectId> =
            (0..2).map(|_| enter(&mut s, &r, caster, "Island")).collect();
        let toll = add(&mut s, &r, caster, TOLL, Zone::Hand(caster));
        s.player_mut(caster).mana_pool.add_mana(ManaColor::Blue, 1, 0);
        s = act(s, &r, |a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == toll));
        s = pass_until_asked(s, &r);
        assert!(matches!(asked(&s).1, ChoiceKind::OptionalCost { cost: OptionalPaymentKind::Mana(_) }));
        assert_eq!(offered_taps(&s, &r), islands);
        assert!(!pay_offered(&s, &r));
        s = tap(s, &r, islands[0]);
        s = tap(s, &r, islands[1]);
        assert!(pay_offered(&s, &r));
        let before = hand(&s, caster);
        s = finish(answer(s, &r, true), &r);
        assert_eq!(hand(&s, caster), before + 1, "paid, and drew");
    }
}

/// Ward is the widest client of the window: "counter it unless that
/// player pays {2}" asks the targeting spell's controller, who taps for it
/// while the Ward trigger resolves. Paid, the Bolt kills the Honor Guard;
/// declined, the Bolt is countered and the Guard lives.
#[test]
fn a_ward_cost_is_paid_from_lands_in_the_window() {
    let r = registry();
    for (caster, active) in arrangements() {
        for pay in [true, false] {
            let warder = 1 - caster;
            let mut s = table(&r, active, caster);
            let guard = enter(&mut s, &r, warder, "Tomakul Honor Guard");
            let islands: Vec<ObjectId> =
                (0..2).map(|_| enter(&mut s, &r, caster, "Island")).collect();
            let bolt = add(&mut s, &r, caster, BOLT, Zone::Hand(caster));
            s.player_mut(caster).mana_pool.add_mana(ManaColor::Red, 1, 0);
            s = act(s, &r, |a| matches!(a, Action::CastSpell { object_id, targets, .. }
                if *object_id == bolt && hits_object(targets, guard)));
            s = pass_until_asked(s, &r);
            let (who, kind) = asked(&s);
            assert_eq!(who, caster, "the Bolt's controller is asked");
            assert!(matches!(kind, ChoiceKind::PayOrDecline { .. }));
            assert_eq!(offered_taps(&s, &r), islands);
            assert!(!pay_offered(&s, &r), "nothing floats yet");
            if pay {
                s = tap(s, &r, islands[0]);
                s = tap(s, &r, islands[1]);
                assert!(pay_offered(&s, &r));
            }
            s = finish(answer(s, &r, pay), &r);
            let lives = s.objects.get(guard).is_some_and(|o| o.zone == Zone::Battlefield);
            assert_eq!(lives, !pay, "seat {caster}, turn {active}: the Bolt {}",
                if pay { "resolved" } else { "was countered" });
            assert_eq!(s.objects.count_in_zone(Zone::Graveyard(caster)), 1, "the Bolt, either way");
            assert_eq!(s.objects.count_in_zone(Zone::Graveyard(warder)), usize::from(pay));
            assert_eq!(pool(&s, caster), 0);
        }
    }
}

fn hits_player(sel: &TargetSelection, p: PlayerId) -> bool {
    sel.targets.iter().any(|t| matches!(t, TargetChoice::Player(q)
        | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(q)) if *q == p))
}

fn hits_object(sel: &TargetSelection, id: ObjectId) -> bool {
    sel.targets.iter().any(|t| matches!(t, TargetChoice::Object(o)
        | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(o)) if *o == id))
}

/// `burner` casts `spell_name` at the opponent, who exiles it with Spell
/// Queller; `burner` then Bolts the Queller, and its leave ability asks
/// `burner` whether to cast the exiled card for nothing. `burner` controls
/// four untapped Mountains. Returns the state at that prompt.
fn queller_offers_the_cast(r: &CardRegistry, burner: PlayerId, spell_name: &str)
    -> (GameState, Vec<ObjectId>)
{
    let foe = 1 - burner;
    let mut s = table(r, burner, burner);
    let mountains: Vec<ObjectId> =
        (0..4).map(|_| enter(&mut s, r, burner, "Mountain")).collect();
    let spell = add(&mut s, r, burner, spell_name, Zone::Hand(burner));
    let queller = add(&mut s, r, foe, QUELLER, Zone::Hand(foe));
    s.player_mut(burner).mana_pool.add_mana(ManaColor::Red, 1, 0);
    s.player_mut(foe).mana_pool.add_mana(ManaColor::White, 1, 0);
    s.player_mut(foe).mana_pool.add_mana(ManaColor::Blue, 2, 0);
    s = act(s, r, |a| matches!(a, Action::CastSpell { object_id, targets, additional_costs, .. }
        if *object_id == spell && hits_player(targets, foe) && additional_costs.is_empty()));
    let on_stack = s.stack.last().unwrap().id;
    s = pass(s, r);
    s = act(s, r, |a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == queller));
    // Queller resolves, its trigger takes the spell.
    for _ in 0..10 {
        if s.stack_is_empty() && s.pending_choice.is_none() { break; }
        let choosing = s.pending_choice.is_some();
        s = act(s, r, |a| match a {
            Action::SubmitResolutionChoice {
                response: ChoiceResponse::ChooseTargets { selection }, .. } =>
                hits_object(selection, on_stack),
            a => a.is_pass() && !choosing,
        });
    }
    let queller = s.objects.objects_in_zone(Zone::Battlefield)
        .find(|o| r.card_id_by_name(QUELLER) == Some(o.card_id)).expect("Queller resolved").id;
    let bolt = add(&mut s, r, burner, BOLT, Zone::Hand(burner));
    s.player_mut(burner).mana_pool.add_mana(ManaColor::Red, 1, 0);
    s.priority.give_to(burner);
    s = act(s, r, |a| matches!(a, Action::CastSpell { object_id, targets, .. }
        if *object_id == bolt && hits_object(targets, queller)));
    for _ in 0..10 {
        if matches!(s.pending_choice.as_ref().map(|c| &c.kind),
            Some(ChoiceKind::CastWithoutPaying { .. })) { break; }
        s = act(s, r, |a| !a.is_concede());
    }
    let (who, kind) = asked(&s);
    assert!(matches!(kind, ChoiceKind::CastWithoutPaying { .. }), "asked {kind:?}");
    assert_eq!(who, burner, "the card's owner is asked");
    (s, mountains)
}

fn kicked_offered(s: &GameState, r: &CardRegistry) -> bool {
    legal_actions(s, r).iter().any(|a| matches!(a, Action::SubmitResolutionChoice {
        response: ChoiceResponse::CastWithoutPaying { cast: Some(c) }, .. }
        if matches!(c.as_ref(), Action::CastSpell { additional_costs, .. }
            if additional_costs.contains(&AdditionalCostPayment::Kicker))))
}

/// A free cast still pays its kicker (CR 118.9d), so it asks for mana:
/// the owner taps four Mountains in the window and casts Burst Lightning
/// kicked.
#[test]
fn a_free_cast_with_a_kicker_opens_the_window() {
    let r = registry();
    for burner in [0, 1] {
        let (mut s, mountains) = queller_offers_the_cast(&r, burner, BURST);
        assert_eq!(offered_taps(&s, &r), mountains);
        assert!(!kicked_offered(&s, &r), "no mana floats for the {{4}} kicker");
        for m in &mountains { s = tap(s, &r, *m); }
        assert!(kicked_offered(&s, &r));
        let foe = 1 - burner;
        let life = s.player(foe).life;
        s = act(s, &r, |a| matches!(a, Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastWithoutPaying { cast: Some(c) }, .. }
            if matches!(c.as_ref(), Action::CastSpell { additional_costs, targets, .. }
                if additional_costs.contains(&AdditionalCostPayment::Kicker)
                    && hits_player(targets, foe))));
        s = finish(s, &r);
        assert_eq!(s.player(foe).life, life - 4, "seat {burner}: the kicked Burst Lightning");
    }
}

/// A free cast that can owe nothing asks for no mana and opens no window:
/// Lightning Bolt, with the same four Mountains untapped. A cost increase
/// makes it owe: once the opponent's Bears tax noncreature spells {1}, the
/// window opens, and the cast is offered once a Mountain is tapped.
#[test]
fn a_free_cast_opens_the_window_only_when_it_can_owe_mana() {
    let r = registry();
    let cast_offered = |s: &GameState| legal_actions(s, &r).iter().any(|a| matches!(a,
        Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastWithoutPaying { cast: Some(_) }, .. }));
    for burner in [0, 1] {
        let (mut s, mountains) = queller_offers_the_cast(&r, burner, BOLT);
        assert!(mountains.iter().all(|m| !tapped(&s, *m)));
        assert!(offered_taps(&s, &r).is_empty(), "nothing to pay for");
        assert!(cast_offered(&s));

        arrange(&mut s, |s| {
            let bears = enter(s, &r, 1 - burner, "Grizzly Bears");
            s.add_continuous_effect(ContinuousEffect::spell_cost_modifier(
                bears,
                ObjectFilter::default().without_types(TypeLine::CREATURE.into()),
                ControllerConstraint::Any, 1, Duration::WhileSourceOnBattlefield));
        });
        assert_eq!(offered_taps(&s, &r), mountains, "the Bolt now costs {{1}}");
        assert!(!cast_offered(&s), "and nothing floats");
        s = tap(s, &r, mountains[0]);
        assert!(cast_offered(&s));
    }
}

/// A card that can't be cast asks for nothing: once an opposing Teferi,
/// Time Raveler is out, Burst Lightning's kicker opens no window, and the
/// only answer declines.
#[test]
fn a_free_cast_teferi_refuses_opens_no_window() {
    let r = registry();
    for burner in [0, 1] {
        let (mut s, mountains) = queller_offers_the_cast(&r, burner, BURST);
        assert_eq!(offered_taps(&s, &r), mountains);
        arrange(&mut s, |s| enter(s, &r, 1 - burner, TEFERI));
        assert!(offered_taps(&s, &r).is_empty(), "no cast, no payment");
        let answers: Vec<Action> = legal_actions(&s, &r).into_iter()
            .filter(|a| !a.is_concede()).collect();
        assert!(matches!(answers.as_slice(), [Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastWithoutPaying { cast: None }, .. }]),
            "only \"don't cast\": {answers:?}");
    }
}
