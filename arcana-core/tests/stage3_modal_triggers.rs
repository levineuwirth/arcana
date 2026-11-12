//! Stage-3 modal triggers: registered clients, stack-placement choices,
//! per-mode targeting, captured exile identities, and replay.
use arcana_core::actions::{Action, CardDestination, ChoiceKind, ChoiceResponse};
use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
use arcana_core::engine::step;
use arcana_core::events::{GameEvent, MoveCause};
use arcana_core::layers::Duration;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::{GameObject, ObjectId};
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggerMode, TriggeredAbilityDef};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CounterKind, ManaColor, PlayerId};
use arcana_core::zones::Zone;

const PRINCE: &str = "Charming Prince";
const HYDRA: &str = "Voracious Hydra";
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
    let card = r.card_id_by_name(name).unwrap();
    let id = s.allocate_object_id();
    s.objects.insert(GameObject::new(id, p, zone, card,
        r.get(card).unwrap().initial_characteristics().clone()));
    if let Zone::Library(p) = zone { s.player_mut(p).library_top_to_bottom.push(id); }
    id
}
#[track_caller]
fn action(s: GameState, r: &CardRegistry, pred: impl Fn(&Action) -> bool) -> GameState {
    let legal = legal_actions(&s, r);
    let a = legal.iter().find(|a| pred(a)).unwrap_or_else(|| panic!("legal action; step {:?}, choice {:?}, legal {:?}", s.turn.step, s.pending_choice, legal)).clone();
    step(s, a, r).0
}
fn pass(s: GameState, r: &CardRegistry) -> GameState { action(s, r, Action::is_pass) }
fn mode(s: GameState, r: &CardRegistry, index: usize) -> GameState {
    action(s, r, |a| matches!(a, Action::SubmitResolutionChoice {
        response: ChoiceResponse::ChooseTriggerMode { mode }, .. } if *mode == index))
}
fn target(s: GameState, r: &CardRegistry, id: ObjectId) -> GameState {
    action(s, r, |a| matches!(a, Action::SubmitResolutionChoice {
        response: ChoiceResponse::ChooseTargets { selection }, .. }
        if selection.targets == vec![TargetChoice::Object(id)]))
}
fn modes(s: &GameState, r: &CardRegistry) -> Vec<usize> {
    assert!(matches!(s.pending_choice.as_ref().map(|p| &p.kind), Some(ChoiceKind::ChooseTriggerMode { .. })));
    legal_actions(s, r).into_iter().filter_map(|a| match a {
        Action::SubmitResolutionChoice { response: ChoiceResponse::ChooseTriggerMode { mode }, .. } => Some(mode),
        _ => None,
    }).collect()
}
fn targets(s: &GameState, r: &CardRegistry) -> Vec<ObjectId> {
    assert!(matches!(s.pending_choice.as_ref().map(|p| &p.kind), Some(ChoiceKind::ChooseTargets { .. })));
    legal_actions(s, r).into_iter().filter_map(|a| match a {
        Action::SubmitResolutionChoice { response: ChoiceResponse::ChooseTargets { selection }, .. } => {
            assert_eq!(selection.targets.len(), 1);
            selection.targets[0].object_id()
        },
        _ => None,
    }).collect()
}
fn cast(mut s: GameState, r: &CardRegistry, p: PlayerId, name: &str, x: Option<u32>) -> GameState {
    for color in [ManaColor::White, ManaColor::Green] { s.player_mut(p).mana_pool.add_mana(color, 12, 0); }
    let card = add(&mut s, r, p, name, Zone::Hand(p));
    s = action(s, r, |a| matches!(a, Action::CastSpell { object_id, x_value, .. }
        if *object_id == card && *x_value == x));
    for _ in 0..20 {
        if s.pending_choice.is_some() { return s; }
        s = pass(s, r);
    }
    panic!("missing modal prompt");
}
fn finish(mut s: GameState, r: &CardRegistry) -> GameState {
    for _ in 0..100 {
        if s.stack_is_empty() && s.pending_choice.is_none() && s.pending_trigger_queue.is_empty() { return s; }
        s = action(s, r, |a| !matches!(a, Action::Concede));
    }
    panic!("unsettled stack");
}
fn source(s: &GameState) -> ObjectId { s.stack.last().unwrap().source }
fn counters(s: &GameState, id: ObjectId) -> u32 {
    s.objects.get(id).unwrap().counters.get(&CounterKind::PlusOnePlusOne).copied().unwrap_or(0)
}
fn id_in(s: &GameState, r: &CardRegistry, name: &str, zone: Zone) -> ObjectId {
    let card = r.card_id_by_name(name).unwrap();
    s.objects.iter().find(|o| o.card_id == card && o.zone == zone).unwrap().id
}
fn end_step(mut s: GameState, r: &CardRegistry) -> GameState {
    // Advance real priority/turn transitions. Libraries prevent deck-out on
    // the next turn when the delayed trigger was created during an end step.
    for p in [0, 1] { for _ in 0..5 { add(&mut s, r, p, "Forest", Zone::Library(p)); } }
    let events = s.event_log.len();
    for _ in 0..100 {
        if s.event_log[events..].iter().any(|e| matches!(e,
            GameEvent::StepBegins { step: Step::End, .. })) { return s; }
        s = action(s, r, |a| matches!(a, Action::PassPriority | Action::DeclareAttackers { .. } | Action::DeclareBlockers { .. }));
    }
    panic!("end step missing");
}

#[test]
fn hydra_doubles_without_targets_in_either_seat_and_reads_current_counters() {
    let r = registry();
    for p in [0, 1] {
        let s = cast(table(p), &r, p, HYDRA, Some(3));
        let hydra = source(&s);
        assert_eq!(modes(&s, &r), vec![0]);
        assert_eq!(counters(&s, hydra), 3);
        let mut s = mode(s, &r, 0);
        assert!(s.pending_choice.is_none());
        assert!(s.stack.last().unwrap().target_requirements.is_empty());
        assert_eq!(s.stack.last().unwrap().modes[0].mode_indices, vec![0]);
        Effect::AddCounters { target: hydra, kind: CounterKind::PlusOnePlusOne, count: 2 }.execute(&mut s);
        Effect::AddCounters { target: hydra, kind: CounterKind::Charge, count: 1 }.execute(&mut s);
        let s = finish(s, &r);
        assert_eq!(counters(&s, hydra), 10);
        assert_eq!(s.objects.get(hydra).unwrap().counters.get(&CounterKind::Charge).copied().unwrap_or(0), 1);
    }
}

#[test]
fn hydra_fight_chooses_only_opposing_creature_and_dispatches_only_fight() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        let foe = add(&mut s, &r, 1-p, "Grizzly Bears", Zone::Battlefield);
        let s = cast(s, &r, p, HYDRA, Some(3));
        let hydra = source(&s);
        assert_eq!(modes(&s, &r), vec![0, 1]);
        let s = mode(s, &r, 1);
        assert_eq!(targets(&s, &r), vec![foe]);
        let s = finish(target(s, &r, foe), &r);
        assert!(s.objects.get(foe).is_none());
        assert_eq!(counters(&s, hydra), 3);
        assert_eq!(s.objects.get(hydra).unwrap().damage_marked, 2);
    }
}

#[test]
fn illegal_fight_target_and_departed_hydra_do_not_switch_modes_or_deal_damage() {
    let r = registry();
    for p in [0, 1] { for departed in [false, true] {
        let mut s = table(p);
        let foe = add(&mut s, &r, 1-p, "Grizzly Bears", Zone::Battlefield);
        let s = cast(s, &r, p, HYDRA, Some(3));
        let hydra = source(&s);
        let mut s = target(mode(s, &r, 1), &r, foe);
        if departed { Effect::ReturnToHand { target: hydra }.execute(&mut s); }
        else { Effect::ChangeControl { target: foe, new_controller: p }.execute(&mut s); }
        let s = finish(s, &r);
        assert_eq!(s.objects.get(foe).unwrap().damage_marked, 0);
        if !departed { assert_eq!(counters(&s, hydra), 3); }
    }}
}

#[test]
fn prince_life_and_scry_modes_need_no_targets_and_keep_trigger_controller() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let top = add(&mut s, &r, p, "Forest", Zone::Library(p));
        let next = add(&mut s, &r, p, "Mountain", Zone::Library(p));
        let s = cast(s, &r, p, PRINCE, None);
        assert_eq!(modes(&s, &r), vec![0, 1]);
        let prince = source(&s);
        let mut life = mode(s.clone(), &r, 1);
        Effect::ChangeControl { target: prince, new_controller: 1-p }.execute(&mut life);
        Effect::ReturnToHand { target: prince }.execute(&mut life);
        let life = finish(life, &r);
        assert_eq!(life.player(p).life, 23);
        assert_eq!(life.player(1-p).life, 20);
        let mut s = mode(s, &r, 0);
        assert!(s.pending_choice.is_none());
        for _ in 0..5 { if s.pending_choice.is_some() { break; } s = pass(s, &r); }
        let ChoiceKind::OrderCards { cards, .. } = &s.pending_choice.as_ref().unwrap().kind else { panic!("scry") };
        assert_eq!(cards, &vec![top, next]);
        s = action(s, &r, |a| matches!(a, Action::SubmitResolutionChoice {
            response: ChoiceResponse::OrderCards { placements }, .. }
            if placements == &vec![(next, CardDestination::TopOfLibrary), (top, CardDestination::BottomOfLibrary)]));
        let s = finish(s, &r);
        assert_eq!(s.player(p).library_top_to_bottom, vec![next, top]);
        assert_eq!(s.player(p).life, 20);
    }
}

#[test]
fn prince_exile_targets_ownership_not_control_and_excludes_itself() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let mine = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        let stolen = add(&mut s, &r, p, "Giant Spider", Zone::Battlefield);
        let borrowed = add(&mut s, &r, 1-p, "Grizzly Bears", Zone::Battlefield);
        Effect::ChangeControl { target: stolen, new_controller: 1-p }.execute(&mut s);
        Effect::ChangeControl { target: borrowed, new_controller: p }.execute(&mut s);
        let s = cast(s, &r, p, PRINCE, None);
        assert_eq!(modes(&s, &r), vec![0, 1, 2]);
        let prince = source(&s);
        let s = mode(s, &r, 2);
        let mut offered = targets(&s, &r); offered.sort();
        assert_eq!(offered, vec![mine, stolen]);
        let req = &s.stack.last().unwrap().target_requirements[0];
        assert!(!req.matches_choice(&TargetChoice::Object(prince), &s, prince, p));
        assert!(!req.matches_choice(&TargetChoice::Object(borrowed), &s, prince, p));
    }
}

#[test]
fn prince_return_uses_new_exile_id_and_fires_entry_abilities_in_both_seats() {
    let r = registry();
    for p in [0, 1] { for depart_before_resolution in [false, true] {
        let mut s = table(p);
        let inspector = add(&mut s, &r, p, "Thraben Inspector", Zone::Battlefield);
        Effect::ChangeControl { target: inspector, new_controller: 1-p }.execute(&mut s);
        Effect::AddCounters { target: inspector, kind: CounterKind::PlusOnePlusOne, count: 2 }.execute(&mut s);
        let s = cast(s, &r, p, PRINCE, None);
        let prince = source(&s);
        let mut s = target(mode(s, &r, 2), &r, inspector);
        if depart_before_resolution { Effect::ReturnToHand { target: prince }.execute(&mut s); }
        let mut s = finish(s, &r);
        let exile = id_in(&s, &r, "Thraben Inspector", Zone::Exile);
        assert_ne!(exile, inspector);
        let delayed = s.delayed_triggers.last().unwrap();
        assert_eq!(delayed.source, prince);
        assert_eq!(delayed.controller, p);
        assert_eq!(delayed.affected, Some(exile));
        if !depart_before_resolution { Effect::ReturnToHand { target: prince }.execute(&mut s); }
        let s = end_step(s, &r);
        assert!(s.objects.get(exile).is_some(), "return waits for delayed trigger resolution");
        let s = finish(s, &r);
        let returned = id_in(&s, &r, "Thraben Inspector", Zone::Battlefield);
        assert_ne!(returned, exile);
        let obj = s.objects.get(returned).unwrap();
        assert_eq!(obj.controller, p);
        assert_eq!(obj.owner, p);
        assert_eq!(counters(&s, returned), 0);
        assert!(s.objects.iter().any(|o| o.controller == p && o.is_token &&
            o.commodity == Some(arcana_core::effects::CommodityToken::Clue)));
    }}
}

#[test]
fn leaving_exile_even_if_reexiled_breaks_the_delayed_link() {
    let r = registry();
    for reexile in [false, true] {
        let mut s = table(0);
        let bear = add(&mut s, &r, 0, "Grizzly Bears", Zone::Battlefield);
        let s = cast(s, &r, 0, PRINCE, None);
        let mut s = finish(target(mode(s, &r, 2), &r, bear), &r);
        let exile = id_in(&s, &r, "Grizzly Bears", Zone::Exile);
        let hand = s.move_object_to_zone(exile, Zone::Hand(0), MoveCause::SpellResolution).unwrap();
        let survivor = if reexile { s.move_object_to_zone(hand, Zone::Exile, MoveCause::SpellResolution).unwrap() } else { hand };
        let s = finish(end_step(s, &r), &r);
        assert_eq!(s.objects.get(survivor).unwrap().zone, if reexile { Zone::Exile } else { Zone::Hand(0) });
        assert!(s.delayed_triggers.is_empty());
    }
}

#[test]
fn blink_created_during_an_end_step_waits_for_the_next_end_step() {
    let r = registry();
    let mut s = table(0);
    s.turn.phase = Phase::Ending; s.turn.step = Step::End;
    let bear = add(&mut s, &r, 0, "Grizzly Bears", Zone::Battlefield);
    // Put Prince in with the engine's entry hook; casting a creature in
    // the end step would need a separate flash permission.
    let prince = add(&mut s, &r, 0, PRINCE, Zone::Hand(0));
    s.move_object_to_zone(prince, Zone::Battlefield, MoveCause::SpellResolution);
    let s = pass(s, &r);
    let s = finish(target(mode(s, &r, 2), &r, bear), &r);
    assert_eq!(s.turn.step, Step::End);
    let exile = id_in(&s, &r, "Grizzly Bears", Zone::Exile);
    let s = end_step(s, &r);
    assert_eq!(s.turn.active_player, 1);
    let s = finish(s, &r);
    assert!(s.objects.get(exile).is_none());
    id_in(&s, &r, "Grizzly Bears", Zone::Battlefield);
}

#[test]
fn prince_targets_recheck_ownership_and_creature_type_at_resolution() {
    let r = registry();
    for p in [0, 1] { for ownership in [false, true] {
        let mut s = table(p);
        let bear = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        let s = cast(s, &r, p, PRINCE, None);
        let mut s = target(mode(s, &r, 2), &r, bear);
        if ownership { s.objects.get_mut(bear).unwrap().owner = 1-p; }
        else { s.objects.get_mut(bear).unwrap().characteristics.types = arcana_core::types::TypeLine::ARTIFACT.into(); }
        let s = finish(s, &r);
        assert!(s.objects.get(bear).unwrap().zone.is_battlefield());
        assert!(s.delayed_triggers.is_empty());
    }}
}

#[test]
fn unavailable_modes_respect_hexproof_and_no_target_is_chosen_early() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let stolen = add(&mut s, &r, p, "Grizzly Bears", Zone::Battlefield);
        Effect::ChangeControl { target: stolen, new_controller: 1-p }.execute(&mut s);
        Effect::GrantKeyword { target: stolen, keyword: KeywordAbility::Hexproof, duration: Duration::Permanent }.execute(&mut s);
        let s = cast(s, &r, p, PRINCE, None);
        assert_eq!(modes(&s, &r), vec![0, 1]);
        assert!(!s.event_log.iter().any(|e| matches!(e, GameEvent::BecomesTarget { .. })));
        assert!(s.stack.last().unwrap().targets.targets.is_empty());
    }
}

fn gain(_s: &GameState, pt: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife { player: pt.controller, amount: 1 }]
}
fn synthetic(r: &mut CardRegistry, name: &str, targeted: bool, modal: bool) {
    let mut def = r.get(r.card_id_by_name("Grizzly Bears").unwrap()).unwrap().clone();
    let symbol = r.interner_mut().intern(name); def.name = symbol; def.base_characteristics.name = symbol;
    let req = vec![TargetRequirement { filter: TargetFilter::Permanent(ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent)), count: TargetCount::Exactly(1), controller: None }];
    def = def.with_triggered_ability(TriggeredAbilityDef { id: 1,
        trigger_condition: TriggerCondition::SelfEntersBattlefield, intervening_if: None,
        effect: gain, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime,
        target_requirements: if targeted && !modal { req.clone() } else { vec![] },
    });
    if modal { def = def.with_trigger_modes(1, vec![TriggerMode { description: "One".into(),
        effect: gain, target_requirements: if targeted { req } else { vec![] } }]); }
    r.register(def);
}
fn enter(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str) -> ObjectId {
    let id = add(s, r, p, name, Zone::Hand(p));
    s.move_object_to_zone(id, Zone::Battlefield, MoveCause::SpellResolution).unwrap()
}
#[test]
fn apnap_queue_continues_through_modes_targets_and_plain_triggers() {
    let mut r = registry();
    synthetic(&mut r, "Plain", false, false);
    synthetic(&mut r, "Targeted", true, false);
    for active in [0, 1] {
        let mut s = table(active);
        let prince = enter(&mut s, &r, active, PRINCE);
        enter(&mut s, &r, active, "Targeted");
        enter(&mut s, &r, 1-active, "Plain");
        let other_prince = enter(&mut s, &r, 1-active, PRINCE);
        let mut s = pass(s, &r);
        let mut seen = Vec::new();
        for _ in 0..8 {
            let Some(pc) = &s.pending_choice else { break; };
            seen.push(pc.choosing_player);
            s = match pc.kind {
                ChoiceKind::ChooseTriggerMode { .. } => mode(s, &r, 1),
                ChoiceKind::ChooseTargets { .. } => target(s, &r, other_prince),
                _ => panic!("unexpected prompt"),
            };
        }
        assert_eq!(seen, vec![active, active, 1-active]);
        assert_eq!(s.stack.iter().map(|e| e.controller).collect::<Vec<_>>(), vec![active, active, 1-active, 1-active]);
        assert_eq!(s.stack.first().unwrap().source, prince);
        assert!(s.pending_trigger_queue.is_empty());
        let s = finish(s, &r);
        assert_eq!(s.player(active).life, 24);
        assert_eq!(s.player(1-active).life, 24);
    }
}

#[test]
fn all_modes_unavailable_drops_only_that_trigger_and_large_plain_batches_drain() {
    let mut r = registry();
    synthetic(&mut r, "Unavailable", true, true);
    synthetic(&mut r, "Plain", false, false);
    let mut s = table(0);
    let unavailable = enter(&mut s, &r, 0, "Unavailable");
    for _ in 0..70 { enter(&mut s, &r, 0, "Plain"); }
    let s = pass(s, &r);
    assert!(s.pending_choice.is_none());
    assert_eq!(s.stack.len(), 70);
    assert!(s.stack.iter().all(|e| e.source != unavailable));
}

#[test]
fn mode_and_target_actions_round_trip_and_replay_to_the_same_outcome() {
    let r = registry();
    let mut s = table(1);
    let bear = add(&mut s, &r, 1, "Grizzly Bears", Zone::Battlefield);
    s = cast(s, &r, 1, PRINCE, None);
    let mut replay = s.clone();
    for desired in [2, usize::MAX] {
        let a = legal_actions(&s, &r).into_iter().find(|a| match a {
            Action::SubmitResolutionChoice { response: ChoiceResponse::ChooseTriggerMode { mode }, .. } => *mode == desired,
            Action::SubmitResolutionChoice { response: ChoiceResponse::ChooseTargets { selection }, .. } => selection.targets == vec![TargetChoice::Object(bear)],
            _ => false,
        }).unwrap();
        let label = arcana_core::render::render_action(&a, &s, &r);
        if desired == 2 { assert!(label.contains("Exile another creature")); }
        let restored = serde_json::from_str(&serde_json::to_string(&a).unwrap()).unwrap();
        s = step(s, a, &r).0;
        replay = step(replay, restored, &r).0;
        assert_eq!(legal_actions(&s, &r), legal_actions(&replay, &r));
    }
    assert_eq!(s.stack.last().unwrap().modes[0].mode_indices, vec![2]);
    let s = finish(end_step(finish(s, &r), &r), &r);
    let replay = finish(end_step(finish(replay, &r), &r), &r);
    assert_eq!(format!("{:?}", s.event_log), format!("{:?}", replay.event_log));
}

#[test]
fn exile_capture_returns_under_explicit_controller_even_when_not_owner() {
    let r = registry();
    let mut s = table(0);
    let bear = add(&mut s, &r, 1, "Grizzly Bears", Zone::Battlefield);
    let source = add(&mut s, &r, 0, "Forest", Zone::Battlefield);
    Effect::ExileAndSchedule { target: bear, source, controller: 0,
        when: DelayedWhen::NextEndStep, action: DelayedAction::ReturnFromExileUnderYourControl }.execute(&mut s);
    let s = finish(end_step(s, &r), &r);
    let returned = id_in(&s, &r, "Grizzly Bears", Zone::Battlefield);
    assert_eq!(s.objects.get(returned).unwrap().owner, 1);
    assert_eq!(s.objects.get(returned).unwrap().controller, 0);
}

#[test]
fn a_prince_that_dies_before_stack_placement_still_offers_and_resolves_modes() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let prince = enter(&mut s, &r, p, PRINCE);
        Effect::SacrificePermanent { target: prince }.execute(&mut s);
        let s = pass(s, &r);
        assert_eq!(modes(&s, &r), vec![0, 1]);
        let s = finish(mode(s, &r, 1), &r);
        assert_eq!(s.player(p).life, 23);
    }
}

#[test]
fn exiled_tokens_do_not_return_and_invalid_capture_targets_schedule_nothing() {
    let r = registry();
    let mut s = table(0);
    let token = add(&mut s, &r, 0, "Grizzly Bears", Zone::Battlefield);
    s.objects.get_mut(token).unwrap().is_token = true;
    let s = cast(s, &r, 0, PRINCE, None);
    let mut s = finish(target(mode(s, &r, 2), &r, token), &r);
    assert!(s.objects.iter().all(|o| !o.is_token));
    s = finish(end_step(s, &r), &r);
    assert!(s.objects.iter().all(|o| !o.is_token));
    assert!(s.delayed_triggers.is_empty());
    let hand = add(&mut s, &r, 0, "Grizzly Bears", Zone::Hand(0));
    for target in [hand, arcana_core::objects::NULL_OBJECT_ID, arcana_core::objects::ILLEGAL_TARGET_ID] {
        Effect::ExileAndSchedule { target, source: token, controller: 0,
            when: DelayedWhen::NextEndStep, action: DelayedAction::ReturnFromExileUnderYourControl }.execute(&mut s);
    }
    assert_eq!(s.objects.get(hand).unwrap().zone, Zone::Hand(0));
    assert!(s.delayed_triggers.is_empty());
    Effect::ReturnFromExileUnderControl { target: hand, controller: 1 }.execute(&mut s);
    assert_eq!(s.objects.get(hand).unwrap().zone, Zone::Hand(0));
}

#[test]
fn exile_capture_composes_with_for_each_and_keeps_fixed_sources() {
    let r = registry();
    let mut s = table(0);
    let bear = add(&mut s, &r, 1, "Grizzly Bears", Zone::Battlefield);
    let spider = add(&mut s, &r, 1, "Giant Spider", Zone::Battlefield);
    let source = add(&mut s, &r, 0, "Forest", Zone::Battlefield);
    Effect::ForEach { targets: vec![bear, spider], effect: Box::new(Effect::ExileAndSchedule {
        target: arcana_core::objects::NULL_OBJECT_ID, source, controller: 0,
        when: DelayedWhen::NextEndStep, action: DelayedAction::ReturnFromExileUnderYourControl,
    }) }.execute(&mut s);
    assert_eq!(s.objects.count_in_zone(Zone::Exile), 2);
    assert_eq!(s.delayed_triggers.len(), 2);
    assert!(s.delayed_triggers.iter().all(|t| t.source == source && t.affected.is_some()));
    let s = finish(end_step(s, &r), &r);
    for name in ["Grizzly Bears", "Giant Spider"] {
        let id = id_in(&s, &r, name, Zone::Battlefield);
        assert_eq!(s.objects.get(id).unwrap().controller, 0);
    }
}
