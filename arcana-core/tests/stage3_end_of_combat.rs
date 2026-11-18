//! End-of-combat token exile: Kari Zev, Skyship Raider. "Whenever Kari
//! Zev attacks, create Ragavan, a legendary 2/1 red Monkey creature token.
//! Ragavan enters tapped and attacking. Exile that token at end of
//! combat." The exile is a one-shot delayed trigger on the created token
//! at the beginning of the end of combat step (CR 511.2, 603.7b), not the
//! end step, and it follows neither Kari Zev nor a token that has left
//! (CR 603.7c).
use arcana_core::actions::{Action, ChoiceKind};
use arcana_core::combat::{AttackerDeclaration, BlockerDeclaration, DefendingEntity};
use arcana_core::effects::{DelayedWhen, Effect, TokenDefinition};
use arcana_core::engine::step;
use arcana_core::events::{GameEvent, MoveCause};
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::{GameObject, ObjectId};
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::triggers::PendingTrigger;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

const KARI: &str = "Kari Zev, Skyship Raider";
const BEARS: &str = "Grizzly Bears";

fn registry() -> CardRegistry {
    let mut r = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut r);
    r
}

fn table(r: &CardRegistry, you: PlayerId) -> GameState {
    let mut s = GameState::new(2, 20261114);
    s.turn.active_player = you;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(you);
    for p in [0, 1] {
        for _ in 0..10 { add(&mut s, r, p, "Mountain", Zone::Library(p)); }
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

/// Onto the battlefield as if long ago: no summoning sickness, no entry
/// event left for the trigger scan.
fn enter(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str) -> ObjectId {
    let id = add(s, r, p, name, Zone::Hand(p));
    let id = s.move_object_to_zone(id, Zone::Battlefield, MoveCause::SpellResolution)
        .expect("entered");
    s.objects.get_mut(id).unwrap().status.summoning_sick = false;
    s.trigger_event_cursor = s.event_log.len();
    id
}

fn act(s: GameState, r: &CardRegistry, pick: impl Fn(&Action) -> bool) -> GameState {
    let chosen = legal_actions(&s, r).into_iter().find(|a| pick(a))
        .unwrap_or_else(|| panic!("no such action; pending {:?}; legal {:?}",
            s.pending_choice, legal_actions(&s, r)));
    step(s, chosen, r).0
}

/// The quiet action at any decision: attack or block with nothing, pass,
/// or else the first answer.
fn quiet(s: GameState, r: &CardRegistry) -> GameState {
    let legal = legal_actions(&s, r);
    let pick = legal.iter().find(|a| matches!(a,
            Action::DeclareAttackers { attackers } if attackers.is_empty()))
        .or_else(|| legal.iter().find(|a| matches!(a,
            Action::DeclareBlockers { blockers } if blockers.is_empty())))
        .or_else(|| legal.iter().find(|a| a.is_pass()))
        .or_else(|| legal.iter().find(|a| !a.is_concede()))
        .expect("an action").clone();
    step(s, pick, r).0
}

fn until(mut s: GameState, r: &CardRegistry, done: impl Fn(&GameState) -> bool) -> GameState {
    for _ in 0..400 {
        if done(&s) { return s; }
        s = quiet(s, r);
    }
    panic!("never got there: turn {} {:?}", s.turn.turn_number, s.turn.step);
}

fn attack(s: GameState, r: &CardRegistry, attackers: &[ObjectId]) -> GameState {
    let foe = 1 - s.active_player();
    let want: Vec<AttackerDeclaration> = attackers.iter()
        .map(|a| AttackerDeclaration { attacker: *a, defending: DefendingEntity::Player(foe) })
        .collect();
    let s = until(s, r, |s| legal_actions(s, r).iter()
        .any(|a| matches!(a, Action::DeclareAttackers { .. })));
    act(s, r, |a| matches!(a, Action::DeclareAttackers { attackers } if *attackers == want))
}

/// The tokens `p` controls on the battlefield.
fn tokens(s: &GameState, p: PlayerId) -> Vec<ObjectId> {
    let mut ids: Vec<ObjectId> = s.objects.objects_in_zone(Zone::Battlefield)
        .filter(|o| o.is_token && o.controller == p).map(|o| o.id).collect();
    ids.sort();
    ids
}

/// No token object is left anywhere: an exiled token ceases to exist.
fn no_token_anywhere(s: &GameState) -> bool {
    s.objects.iter().all(|o| !o.is_token)
}

/// Kari Zev attacks alone and her trigger resolves: Ragavan is on the
/// battlefield tapped and attacking, in the declare attackers step.
fn kari_attacks(r: &CardRegistry, you: PlayerId, arrange: impl FnOnce(&mut GameState))
    -> (GameState, ObjectId, ObjectId)
{
    let mut s = table(r, you);
    let kari = enter(&mut s, r, you, KARI);
    arrange(&mut s);
    s = attack(s, r, &[kari]);
    assert_eq!(s.stack.len(), 1, "Kari Zev's trigger");
    s = until(s, r, |s| s.stack_is_empty());
    assert_eq!(s.turn.step, Step::DeclareAttackers);
    let made = tokens(&s, you);
    assert_eq!(made.len(), 1, "one Ragavan");
    let ragavan = made[0];
    let token = s.objects.get(ragavan).unwrap();
    assert!(token.is_tapped() && s.combat.as_ref().unwrap().is_attacker(ragavan));
    (s, kari, ragavan)
}

/// Attacking creates Ragavan, a legendary 2/1; it deals its combat damage
/// and is exiled at end of combat, in the end of combat step, not at the
/// end step.
#[test]
fn ragavan_deals_damage_and_is_exiled_at_end_of_combat() {
    let r = registry();
    for you in [0, 1] {
        let foe = 1 - you;
        let (mut s, kari, ragavan) = kari_attacks(&r, you, |_| {});
        let chars = s.compute_characteristics(ragavan).expect("characteristics");
        assert!(chars.supertypes.is_legendary(), "seat {you}: a legendary token");
        assert_eq!((s.computed_power(ragavan), s.computed_toughness(ragavan)), (Some(2), Some(1)));

        s = until(s, &r, |s| s.turn.step == Step::EndCombat);
        assert_eq!(s.player(foe).life, 17, "Kari Zev's 1 and Ragavan's 2");
        assert_eq!(tokens(&s, you), vec![ragavan], "still here as the step begins");
        assert_eq!(s.stack.len(), 1, "the delayed exile is on the stack");
        assert_eq!(s.stack[0].source, kari, "its source is the ability that made it");

        s = until(s, &r, |s| s.stack_is_empty());
        assert_eq!(s.turn.step, Step::EndCombat, "in the end of combat step");
        assert!(tokens(&s, you).is_empty() && no_token_anywhere(&s), "Ragavan is exiled and gone");
        assert!(s.objects.get(kari).is_some_and(|o| o.zone == Zone::Battlefield));
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::ZoneChange { object_id, to: Zone::Exile, .. } if *object_id == ragavan)),
            "by exile, not by death");

        // Nothing is left to fire at the end step or in a later combat.
        s = until(s, &r, |s| s.turn.active_player == foe && s.turn.phase == Phase::PostCombatMain);
        assert!(s.delayed_triggers.is_empty(), "the trigger fired once");
        assert!(s.objects.get(kari).is_some_and(|o| o.zone == Zone::Battlefield));
    }
}

/// Ragavan dying in combat leaves nothing to exile, and nothing breaks.
#[test]
fn ragavan_dying_in_combat_leaves_nothing_to_exile() {
    let r = registry();
    for you in [0, 1] {
        let foe = 1 - you;
        let mut bears = 0;
        let (mut s, kari, ragavan) = kari_attacks(&r, you, |s| { bears = enter(s, &r, foe, BEARS); });
        s = until(s, &r, |s| legal_actions(s, &r).iter()
            .any(|a| matches!(a, Action::DeclareBlockers { .. })));
        let want = vec![BlockerDeclaration { blocker: bears, blocking: ragavan }];
        s = act(s, &r, |a| matches!(a, Action::DeclareBlockers { blockers } if *blockers == want));
        s = until(s, &r, |s| s.turn.step == Step::EndCombat && s.stack_is_empty()
            && s.pending_trigger_queue.is_empty());
        assert!(no_token_anywhere(&s), "seat {you}: Ragavan died to the Bears");
        assert!(!s.event_log.iter().any(|e| matches!(e,
            GameEvent::ZoneChange { object_id, to: Zone::Exile, .. } if *object_id == ragavan)),
            "it was never exiled");
        s = until(s, &r, |s| s.turn.phase == Phase::PostCombatMain);
        assert!(s.objects.get(kari).is_some_and(|o| o.zone == Zone::Battlefield),
            "and the exile fell on nothing else");
        assert_eq!(s.player(foe).life, 19, "Kari Zev's 1");
    }
}

/// The end of combat step is also entered from a combat damage
/// assignment, when an attacker with two blockers has its damage divided
/// by hand. Ragavan is exiled as that step begins too.
#[test]
fn ragavan_is_exiled_after_an_assigned_damage_step() {
    let r = registry();
    for you in [0, 1] {
        let foe = 1 - you;
        let mut s = table(&r, you);
        let kari = enter(&mut s, &r, you, KARI);
        let mine = enter(&mut s, &r, you, BEARS);
        let blockers = [enter(&mut s, &r, foe, BEARS), enter(&mut s, &r, foe, BEARS)];
        s = attack(s, &r, &[kari, mine]);
        s = until(s, &r, |s| s.stack_is_empty() && legal_actions(s, &r).iter()
            .any(|a| matches!(a, Action::DeclareBlockers { .. })));
        let ragavan = tokens(&s, you)[0];
        s = act(s, &r, |a| matches!(a, Action::DeclareBlockers { blockers: b }
            if b.len() == 2 && b.iter().all(|d| d.blocking == mine && blockers.contains(&d.blocker))));
        let mut assigned_in = Vec::new();
        for _ in 0..60 {
            if s.turn.step == Step::EndCombat { break; }
            if s.combat.as_ref().is_some_and(|c| c.pending_damage_assignment.is_some()) {
                assigned_in.push(s.turn.step);
            }
            s = quiet(s, &r);
        }
        assert_eq!(assigned_in.last(), Some(&Step::CombatDamageRegular),
            "seat {you}: the regular damage step waited for an assignment");
        assert_eq!(s.turn.step, Step::EndCombat);
        assert_eq!(tokens(&s, you), vec![ragavan], "still here as the step begins");
        assert_eq!(s.stack.len(), 1, "the delayed exile is on the stack");
        s = until(s, &r, |s| s.stack_is_empty());
        assert_eq!(s.turn.step, Step::EndCombat);
        assert!(no_token_anywhere(&s), "Ragavan is exiled");
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::ZoneChange { object_id, to: Zone::Exile, .. } if *object_id == ragavan)));
        assert_eq!(s.player(foe).life, 17, "Kari Zev's 1 and Ragavan's 2");
    }
}

fn draw_one(_: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}

/// The end of combat joined one table of delayed timings that two effects
/// used to spell out apart. "At the beginning of the next upkeep" still
/// waits for the upkeep: it does not fire at this turn's end of combat or
/// end step.
#[test]
fn a_delayed_effect_for_the_next_upkeep_waits_for_it() {
    let r = registry();
    for you in [0, 1] {
        let foe = 1 - you;
        let mut s = table(&r, you);
        let source = enter(&mut s, &r, you, BEARS);
        Effect::ScheduleDelayedEffect {
            source, controller: you, when: DelayedWhen::NextUpkeep, effect: draw_one,
        }.execute(&mut s);
        assert_eq!(s.delayed_triggers.len(), 1);
        let hand = s.objects.count_in_zone(Zone::Hand(you));

        s = until(s, &r, |s| s.turn.phase == Phase::Ending && s.turn.step == Step::End);
        assert_eq!(s.turn.active_player, you);
        assert!(s.stack_is_empty() && s.pending_trigger_queue.is_empty(),
            "seat {you}: nothing fired in this turn's end step");
        assert_eq!(s.delayed_triggers.len(), 1, "it is still waiting");
        assert_eq!(s.objects.count_in_zone(Zone::Hand(you)), hand);

        s = until(s, &r, |s| s.turn.active_player == foe && s.turn.step == Step::Upkeep);
        s = until(s, &r, |s| s.stack_is_empty() && s.pending_trigger_queue.is_empty());
        assert_eq!(s.turn.step, Step::Upkeep, "it resolved in the upkeep");
        assert_eq!(s.objects.count_in_zone(Zone::Hand(you)), hand + 1, "and drew its card");
        assert!(s.delayed_triggers.is_empty(), "once");
    }
}

/// Kari Zev leaving the battlefield before end of combat does not save
/// Ragavan.
#[test]
fn kari_zev_leaving_does_not_save_ragavan() {
    let r = registry();
    for you in [0, 1] {
        let (mut s, kari, ragavan) = kari_attacks(&r, you, |_| {});
        s.move_object_to_zone(kari, Zone::Graveyard(you), MoveCause::SpellResolution);
        s = until(s, &r, |s| s.turn.step == Step::EndCombat);
        assert_eq!(tokens(&s, you), vec![ragavan], "seat {you}");
        s = until(s, &r, |s| s.turn.phase == Phase::PostCombatMain);
        assert!(no_token_anywhere(&s), "Ragavan was exiled all the same");
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::ZoneChange { object_id, to: Zone::Exile, .. } if *object_id == ragavan)));
    }
}

/// An extra combat makes a second Ragavan, exiled at its own end of
/// combat; the first is gone before the second is made.
#[test]
fn an_extra_combat_makes_and_exiles_a_second_ragavan() {
    let r = registry();
    for you in [0, 1] {
        let foe = 1 - you;
        let (mut s, kari, first) = kari_attacks(&r, you, |_| {});
        Effect::AdditionalCombatPhase.execute(&mut s);
        s = until(s, &r, |s| s.turn.step == Step::BeginCombat);
        assert!(no_token_anywhere(&s), "seat {you}: the first Ragavan left with the first combat");
        assert_eq!(s.player(foe).life, 17);
        Effect::Untap { target: kari }.execute(&mut s);

        s = attack(s, &r, &[kari]);
        s = until(s, &r, |s| s.stack_is_empty());
        let made = tokens(&s, you);
        assert_eq!(made.len(), 1);
        let second = made[0];
        assert_ne!(second, first, "a new object");
        s = until(s, &r, |s| s.turn.step == Step::EndCombat);
        assert_eq!(tokens(&s, you), vec![second]);
        s = until(s, &r, |s| s.turn.phase == Phase::PostCombatMain);
        assert!(no_token_anywhere(&s), "the second left with the second combat");
        assert_eq!(s.player(foe).life, 14, "3 more");
    }
}

/// Ragavan is legendary (CR 205.4d): a second Ragavan under the same
/// controller meets the legend rule, and each is exiled only by its own
/// delayed trigger.
#[test]
fn a_second_ragavan_meets_the_legend_rule() {
    let r = registry();
    for you in [0, 1] {
        let (mut s, kari, first) = kari_attacks(&r, you, |_| {});
        Effect::CreateTokenTappedAttackingExileEoc {
            source: kari,
            controller: you,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            token: TokenDefinition {
                name: r.interner().lookup("Ragavan").expect("interned"),
                colors: ColorSet::red(),
                types: TypeLine::CREATURE.into(),
                subtypes: SubtypeSet::default(),
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        }.execute(&mut s);
        let both = tokens(&s, you);
        assert_eq!(both.len(), 2);
        s = quiet(s, &r);
        let Some(ChoiceKind::PickCards { candidates, .. }) =
            s.pending_choice.as_ref().map(|c| c.kind.clone())
            else { panic!("seat {you}: the legend rule asks; pending {:?}", s.pending_choice) };
        let mut candidates = candidates;
        candidates.sort();
        assert_eq!(candidates, both);
        s = quiet(s, &r);
        assert_eq!(tokens(&s, you).len(), 1, "one Ragavan is kept");
        let _ = first;
        s = until(s, &r, |s| s.turn.phase == Phase::PostCombatMain);
        assert!(no_token_anywhere(&s), "and it is exiled at end of combat");
        assert!(s.objects.get(kari).is_some_and(|o| o.zone == Zone::Battlefield));
    }
}
