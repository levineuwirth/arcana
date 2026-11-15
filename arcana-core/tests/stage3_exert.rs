//! Exert: Glorybringer. "You may exert this creature as it attacks. When
//! you do, it deals 4 damage to target non-Dragon creature an opponent
//! controls." Exerting is chosen as the attack is declared (CR 701.43d,
//! 508.1g), before any player receives priority; the exerted creature
//! doesn't untap during its exerter's next untap step (CR 701.43a); and
//! the "when you do" ability triggers on the exert alone.
use arcana_core::actions::{Action, ChoiceKind, ChoiceResponse};
use arcana_core::combat::{AttackerDeclaration, DefendingEntity};
use arcana_core::effects::Effect;
use arcana_core::engine::step;
use arcana_core::events::{GameEvent, MoveCause};
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::{GameObject, ObjectId};
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetChoice;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::PlayerId;
use arcana_core::zones::Zone;

const GLORY: &str = "Glorybringer";
const BEARS: &str = "Grizzly Bears";
/// A 4/4 Dragon with no ability of its own that matters here.
const DRAGON: &str = "Shivan Dragon";

fn registry() -> CardRegistry {
    let mut r = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut r);
    r
}

/// `you`'s first main phase with priority and ten Mountains in each
/// library, enough for the turns these tests play.
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

/// Onto the battlefield through the zone mover, as if long ago: no
/// summoning sickness, and no entry event left for the trigger scan.
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

/// Play quietly until `done`.
fn until(mut s: GameState, r: &CardRegistry, done: impl Fn(&GameState) -> bool) -> GameState {
    for _ in 0..400 {
        if done(&s) { return s; }
        s = quiet(s, r);
    }
    panic!("never got there: turn {} {:?}", s.turn.turn_number, s.turn.step);
}

/// Play quietly to the first decision of turn `turn`: its untap step is
/// over, and nothing else has happened in it.
fn to_turn(s: GameState, r: &CardRegistry, turn: u32) -> GameState {
    until(s, r, |s| s.turn.turn_number == turn)
}

fn tapped(s: &GameState, id: ObjectId) -> bool {
    s.objects.get(id).expect("the permanent").is_tapped()
}

fn on_battlefield(s: &GameState, id: ObjectId) -> bool {
    s.objects.get(id).is_some_and(|o| o.zone == Zone::Battlefield)
}

/// `you` attack the opponent with `attackers`, from the main phase.
fn attack(s: GameState, r: &CardRegistry, attackers: &[ObjectId]) -> GameState {
    let foe = 1 - s.active_player();
    let want: Vec<AttackerDeclaration> = attackers.iter()
        .map(|a| AttackerDeclaration { attacker: *a, defending: DefendingEntity::Player(foe) })
        .collect();
    let s = until(s, r, |s| legal_actions(s, r).iter()
        .any(|a| matches!(a, Action::DeclareAttackers { .. })));
    act(s, r, |a| matches!(a, Action::DeclareAttackers { attackers } if *attackers == want))
}

/// Who is asked to exert what, if anyone is.
fn exert_asked(s: &GameState) -> Option<(PlayerId, ObjectId)> {
    match s.pending_choice.as_ref().map(|c| (c.choosing_player, &c.kind)) {
        Some((who, ChoiceKind::Exert { attacker })) => Some((who, *attacker)),
        _ => None,
    }
}

fn answer_exert(s: GameState, r: &CardRegistry, exert: bool) -> GameState {
    act(s, r, |a| matches!(a, Action::SubmitResolutionChoice {
        response: ChoiceResponse::Exert { exert: e }, .. } if *e == exert))
}

/// The objects the pending target choice offers.
fn offered_targets(s: &GameState, r: &CardRegistry) -> Vec<ObjectId> {
    legal_actions(s, r).iter().filter_map(|a| match a {
        Action::SubmitResolutionChoice {
            response: ChoiceResponse::ChooseTargets { selection }, .. } =>
            match selection.targets.as_slice() {
                [TargetChoice::Object(id)] => Some(*id),
                _ => None,
            },
        _ => None,
    }).collect()
}

fn aim(s: GameState, r: &CardRegistry, target: ObjectId) -> GameState {
    act(s, r, |a| matches!(a, Action::SubmitResolutionChoice {
        response: ChoiceResponse::ChooseTargets { selection }, .. }
        if selection.targets == vec![TargetChoice::Object(target)]))
}

/// Exerting deals 4 to the chosen non-Dragon creature an opponent
/// controls; Glorybringer stays tapped through its controller's next
/// untap step and untaps in the one after.
#[test]
fn exerting_deals_four_and_skips_one_untap_step() {
    let r = registry();
    for you in [0, 1] {
        let foe = 1 - you;
        let mut s = table(&r, you);
        let glory = enter(&mut s, &r, you, GLORY);
        let mine = enter(&mut s, &r, you, BEARS);
        let bears = enter(&mut s, &r, foe, BEARS);
        let dragon = enter(&mut s, &r, foe, DRAGON);
        let turn = s.turn.turn_number;

        s = attack(s, &r, &[glory]);
        assert_eq!(exert_asked(&s), Some((you, glory)), "seat {you}: asked as it attacks");
        assert!(s.stack_is_empty(), "before anything is on the stack");
        let view = arcana_core::view::view_state(&s, &r, you, &legal_actions(&s, &r));
        assert_eq!(view.prompt.as_deref(),
            Some("Exert Glorybringer? It won't untap during your next untap step."));
        assert_eq!(view.legal.iter().map(|a| a.label.as_str()).collect::<Vec<_>>(),
            vec!["Exert Glorybringer", "Don't exert Glorybringer", "Concede"]);

        s = answer_exert(s, &r, true);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::Exerted { object_id, player } if *object_id == glory && *player == you)));
        assert_eq!(offered_targets(&s, &r), vec![bears],
            "a non-Dragon creature an opponent controls: not {dragon} the Dragon, not {mine} its own");
        s = aim(s, &r, bears);
        assert_eq!(s.stack.len(), 1, "the ability waits on the stack; players may respond");
        s = until(s, &r, |s| s.stack_is_empty());
        assert!(!on_battlefield(&s, bears), "4 damage killed the Bears");
        assert!(on_battlefield(&s, dragon) && on_battlefield(&s, mine));

        s = to_turn(s, &r, turn + 2);
        assert_eq!(s.active_player(), you);
        assert!(tapped(&s, glory), "seat {you}: it did not untap in its controller's next untap step");
        assert_eq!(s.player(foe).life, 16, "it dealt its combat damage on the first turn");
        s = to_turn(s, &r, turn + 4);
        assert!(!tapped(&s, glory), "and untapped in the one after");
    }
}

/// Declining deals nothing, and it untaps as usual.
#[test]
fn declining_deals_nothing_and_untaps_as_usual() {
    let r = registry();
    for you in [0, 1] {
        let foe = 1 - you;
        let mut s = table(&r, you);
        let glory = enter(&mut s, &r, you, GLORY);
        let bears = enter(&mut s, &r, foe, BEARS);
        let turn = s.turn.turn_number;
        s = attack(s, &r, &[glory]);
        s = answer_exert(s, &r, false);
        assert!(s.pending_choice.is_none() && s.stack_is_empty(), "seat {you}: no ability");
        assert!(!s.event_log.iter().any(|e| matches!(e, GameEvent::Exerted { .. })));
        assert!(tapped(&s, glory), "attacking tapped it");
        s = to_turn(s, &r, turn + 2);
        assert!(on_battlefield(&s, bears), "nothing was dealt");
        assert!(!tapped(&s, glory), "it untapped in its controller's next untap step");
    }
}

/// Exerting with no legal target is allowed: the ability is removed for
/// want of a target, and Glorybringer is exerted all the same.
#[test]
fn exerting_with_no_legal_target_still_exerts() {
    let r = registry();
    for you in [0, 1] {
        let foe = 1 - you;
        let mut s = table(&r, you);
        let glory = enter(&mut s, &r, you, GLORY);
        let dragon = enter(&mut s, &r, foe, DRAGON);
        let turn = s.turn.turn_number;
        s = attack(s, &r, &[glory]);
        assert_eq!(exert_asked(&s), Some((you, glory)));
        s = answer_exert(s, &r, true);
        assert!(s.pending_choice.is_none() && s.stack_is_empty(),
            "seat {you}: no target, so no ability on the stack");
        s = to_turn(s, &r, turn + 2);
        assert!(on_battlefield(&s, dragon));
        assert!(tapped(&s, glory), "exerted: it stays tapped");
    }
}

/// Two Glorybringers attack: each is asked in turn, one is exerted and
/// one is not, and only the exerted one triggers and stays tapped.
#[test]
fn two_glorybringers_are_asked_separately() {
    let r = registry();
    for you in [0, 1] {
        let foe = 1 - you;
        let mut s = table(&r, you);
        let first = enter(&mut s, &r, you, GLORY);
        let second = enter(&mut s, &r, you, GLORY);
        let bears = enter(&mut s, &r, foe, BEARS);
        let turn = s.turn.turn_number;
        s = attack(s, &r, &[first, second]);
        assert_eq!(exert_asked(&s), Some((you, first)));
        s = answer_exert(s, &r, false);
        assert_eq!(exert_asked(&s), Some((you, second)), "seat {you}: then the second");
        s = answer_exert(s, &r, true);
        assert_eq!(offered_targets(&s, &r), vec![bears]);
        s = aim(s, &r, bears);
        assert_eq!(s.stack.len(), 1, "one ability, the second's");
        assert_eq!(s.stack[0].source, second);
        s = to_turn(s, &r, turn + 2);
        assert!(!on_battlefield(&s, bears));
        assert!(!tapped(&s, first) && tapped(&s, second));
    }
}

/// An untap effect still untaps an exerted creature: exert stops only the
/// untap step's untap.
#[test]
fn an_untap_effect_untaps_an_exerted_creature() {
    let r = registry();
    for you in [0, 1] {
        let mut s = table(&r, you);
        let glory = enter(&mut s, &r, you, GLORY);
        s = attack(s, &r, &[glory]);
        s = answer_exert(s, &r, true);
        assert!(tapped(&s, glory));
        Effect::Untap { target: glory }.execute(&mut s);
        assert!(!tapped(&s, glory), "seat {you}");
    }
}

/// The effect is the exerting player's (CR 701.43a, "your next untap
/// step"): under another player's control the creature untaps in that
/// player's untap step, and the effect is spent in the exerter's own next
/// untap step even though the creature is not theirs then.
#[test]
fn exert_is_the_exerting_players_untap_step() {
    let r = registry();
    for you in [0, 1] {
        let foe = 1 - you;
        let mut s = table(&r, you);
        let glory = enter(&mut s, &r, you, GLORY);
        let turn = s.turn.turn_number;
        s = attack(s, &r, &[glory]);
        s = answer_exert(s, &r, true);
        s = until(s, &r, |s| s.turn.phase == Phase::PostCombatMain);
        Effect::ChangeControl { target: glory, new_controller: foe }.execute(&mut s);
        assert!(tapped(&s, glory));

        s = to_turn(s, &r, turn + 1);
        assert_eq!(s.active_player(), foe);
        assert!(!tapped(&s, glory), "seat {you}: it untaps in its new controller's untap step");
        assert_eq!(s.objects.get(glory).unwrap().exerting_players(), vec![you], "the exerter's step is still ahead");

        // The new controller attacks with it, declining to exert, so it is
        // tapped again; control then returns to the exerter after the
        // exerter's untap step has spent the effect.
        s = attack(s, &r, &[glory]);
        s = answer_exert(s, &r, false);
        s = to_turn(s, &r, turn + 2);
        assert_eq!(s.active_player(), you);
        assert!(tapped(&s, glory), "not the active player's permanent, so it did not untap");
        assert!(s.objects.get(glory).unwrap().exerting_players().is_empty(), "the effect ended in the exerter's untap step");
        Effect::ChangeControl { target: glory, new_controller: you }.execute(&mut s);
        s = to_turn(s, &r, turn + 4);
        assert!(!tapped(&s, glory), "and nothing holds it down in the next one");
    }
}

/// Where no attacker can be exerted nothing is asked, and a Glorybringer
/// that stays home is not asked either.
#[test]
fn only_an_attacking_exert_creature_is_asked() {
    let r = registry();
    for you in [0, 1] {
        let mut s = table(&r, you);
        let glory = enter(&mut s, &r, you, GLORY);
        let bears = enter(&mut s, &r, you, BEARS);
        s = attack(s, &r, &[bears]);
        assert!(s.pending_choice.is_none(), "seat {you}: the Bears have no exert");
        assert!(s.pending_exerts.is_empty());
        assert!(!tapped(&s, glory));
        assert!(legal_actions(&s, &r).iter().any(|a| a.is_pass()), "the priority window is open");
    }
}

/// The choice comes before any attack trigger goes on the stack: Kari Zev
/// and Glorybringer attack together, the stack is empty while the exert
/// is decided, and both abilities are put on it afterward.
#[test]
fn exert_is_chosen_before_attack_triggers_go_on_the_stack() {
    let r = registry();
    for you in [0, 1] {
        let foe = 1 - you;
        let mut s = table(&r, you);
        let kari = enter(&mut s, &r, you, "Kari Zev, Skyship Raider");
        let glory = enter(&mut s, &r, you, GLORY);
        let bears = enter(&mut s, &r, foe, BEARS);
        s = attack(s, &r, &[kari, glory]);
        assert_eq!(exert_asked(&s), Some((you, glory)));
        assert!(s.stack_is_empty(), "seat {you}: Kari Zev's trigger waits for the choice");
        s = answer_exert(s, &r, true);
        s = aim(s, &r, bears);
        let sources: Vec<ObjectId> = s.stack.iter().map(|e| e.source).collect();
        assert_eq!(sources.len(), 2);
        assert!(sources.contains(&kari) && sources.contains(&glory), "{sources:?}");
    }
}

/// The log names the exert from each side of the table.
#[test]
fn the_log_names_the_exert() {
    let r = registry();
    let mut s = table(&r, 0);
    let glory = enter(&mut s, &r, 0, GLORY);
    let event = GameEvent::Exerted { object_id: glory, player: 0 };
    assert_eq!(arcana_core::render::describe_event(&event, &s, &r, 0).as_deref(),
        Some("You exert Glorybringer"));
    assert_eq!(arcana_core::render::describe_event(&event, &s, &r, 1).as_deref(),
        Some("Opponent exerts Glorybringer"));
}

/// The exert trigger and the abilities that trigger on the attack go on
/// the stack together, in one APNAP batch (CR 508.1m, 603.3b): the
/// attacking player's below the defending player's. Against Revenge of
/// Ravens the drain resolves before Glorybringer's damage, and an attacker
/// at 1 life loses before the damage is dealt.
#[test]
fn the_exert_trigger_joins_the_attack_triggers_in_one_apnap_batch() {
    let r = registry();
    for you in [0, 1] {
        for life in [20, 1] {
            let foe = 1 - you;
            let mut s = table(&r, you);
            let glory = enter(&mut s, &r, you, GLORY);
            let ravens = enter(&mut s, &r, foe, "Revenge of Ravens");
            let bears = enter(&mut s, &r, foe, BEARS);
            s.player_mut(you).life = life;
            s = attack(s, &r, &[glory]);
            assert!(s.stack_is_empty() && s.pending_trigger_queue.is_empty(),
                "seat {you}: nothing triggers before the exert decision");
            s = answer_exert(s, &r, true);
            s = aim(s, &r, bears);
            let sources: Vec<ObjectId> = s.stack.iter().map(|e| e.source).collect();
            assert_eq!(sources, vec![glory, ravens],
                "seat {you}: the active player's trigger is below the defender's");
            // The top of the stack resolves first: the drain.
            for _ in 0..4 {
                if s.stack.len() < 2 { break; }
                s = act(s, &r, |a| a.is_pass());
            }
            assert_eq!(s.player(foe).life, 21, "seat {you}: the Ravens' drain resolved first");
            assert!(on_battlefield(&s, bears), "before Glorybringer's damage");
            if life == 1 {
                assert!(s.is_game_over(), "seat {you}: the attacker at 1 life lost to the drain");
            } else {
                assert_eq!(s.player(you).life, 19);
                s = until(s, &r, |s| s.stack_is_empty());
                assert!(!on_battlefield(&s, bears), "then the 4 damage");
            }
        }
    }
}

/// Declining leaves the same single batch: the defender's attack trigger
/// alone, put on the stack after the decision.
#[test]
fn attack_triggers_wait_for_a_declined_exert_too() {
    let r = registry();
    for you in [0, 1] {
        let foe = 1 - you;
        let mut s = table(&r, you);
        let glory = enter(&mut s, &r, you, GLORY);
        let ravens = enter(&mut s, &r, foe, "Revenge of Ravens");
        s = attack(s, &r, &[glory]);
        assert!(s.stack_is_empty() && s.pending_trigger_queue.is_empty());
        s = answer_exert(s, &r, false);
        let sources: Vec<ObjectId> = s.stack.iter().map(|e| e.source).collect();
        assert_eq!(sources, vec![ravens], "seat {you}");
    }
}

/// "You may exert this creature as it attacks" is an ability of the
/// creature. One that has lost its abilities (Oko, Thief of Crowns' +1)
/// is not offered it, so nothing can keep it from untapping.
#[test]
fn a_creature_that_lost_its_abilities_is_not_offered_exert() {
    let r = registry();
    for you in [0, 1] {
        let mut s = table(&r, you);
        let glory = enter(&mut s, &r, you, GLORY);
        let oko = enter(&mut s, &r, you, "Oko, Thief of Crowns");
        s = act(s, &r, |a| matches!(a, Action::ActivateAbility {
            source, ability_index: 1, targets, ..
        } if *source == oko && targets.targets == vec![TargetChoice::Object(glory)]));
        s = until(s, &r, |s| s.stack_is_empty());
        assert!(s.has_lost_all_abilities(glory), "Oko's +1 resolved");
        let turn = s.turn.turn_number;
        s = attack(s, &r, &[glory]);
        assert!(exert_asked(&s).is_none(), "seat {you}: an Elk with no abilities is not asked");
        assert!(s.pending_exerts.is_empty());
        assert!(legal_actions(&s, &r).iter().all(|a| !matches!(a,
            Action::SubmitResolutionChoice { response: ChoiceResponse::Exert { .. }, .. })));
        s = to_turn(s, &r, turn + 2);
        assert!(!tapped(&s, glory), "and it untaps as usual");
    }
}

/// A face-down permanent has no abilities (CR 708.2): a manifested
/// Glorybringer attacks as a 2/2 and is not asked.
#[test]
fn a_face_down_glorybringer_is_not_offered_exert() {
    let r = registry();
    for you in [0, 1] {
        let mut s = table(&r, you);
        let top = add(&mut s, &r, you, GLORY, Zone::Library(you));
        let lib = &mut s.player_mut(you).library_top_to_bottom;
        lib.pop();
        lib.insert(0, top);
        Effect::Manifest { player: you }.execute(&mut s);
        let manifested = s.objects.objects_in_zone(Zone::Battlefield)
            .find(|o| o.status.face_down).expect("the manifested card").id;
        assert_eq!(s.objects.get(manifested).unwrap().card_id,
            r.card_id_by_name(GLORY).unwrap(), "it is the Glorybringer");
        s.objects.get_mut(manifested).unwrap().status.summoning_sick = false;
        s.trigger_event_cursor = s.event_log.len();
        s = attack(s, &r, &[manifested]);
        assert!(exert_asked(&s).is_none(), "seat {you}: a face-down creature is not asked");
    }
}

/// Each player's exert is its own effect (CR 701.43a). Exerted by one
/// player, then by another who took control of it, the creature skips
/// each of their next untap steps: the second exert does not erase the
/// first.
#[test]
fn a_second_players_exert_keeps_the_first_players() {
    let r = registry();
    for you in [0, 1] {
        let foe = 1 - you;
        let mut s = table(&r, you);
        let glory = enter(&mut s, &r, you, GLORY);
        let turn = s.turn.turn_number;
        s = answer_exert(attack(s, &r, &[glory]), &r, true);
        s = until(s, &r, |s| s.turn.phase == Phase::PostCombatMain);
        Effect::ChangeControl { target: glory, new_controller: foe }.execute(&mut s);
        s = to_turn(s, &r, turn + 1);
        assert!(!tapped(&s, glory), "seat {you}: it untaps in the other player's untap step");
        s = answer_exert(attack(s, &r, &[glory]), &r, true);
        let mut both = vec![you, foe];
        both.sort();
        assert_eq!(s.objects.get(glory).unwrap().exerting_players(), both, "both effects are in place");
        s = until(s, &r, |s| s.turn.phase == Phase::PostCombatMain);
        Effect::ChangeControl { target: glory, new_controller: you }.execute(&mut s);
        s = to_turn(s, &r, turn + 2);
        assert!(tapped(&s, glory), "seat {you}: the first player's exert still holds it");
        assert_eq!(s.objects.get(glory).unwrap().exerting_players(), vec![foe],
            "that effect ended; the other player's waits for their untap step");
        // Back under the second player for their next untap step: held
        // once more, then free.
        Effect::ChangeControl { target: glory, new_controller: foe }.execute(&mut s);
        s = to_turn(s, &r, turn + 3);
        assert!(tapped(&s, glory), "seat {you}: the second player's exert holds it in their step");
        assert!(s.objects.get(glory).unwrap().exerting_players().is_empty());
        s = to_turn(s, &r, turn + 5);
        assert!(!tapped(&s, glory), "and nothing holds it after that");
    }
}

/// The exert marks are a bit per player on the object: marking twice is
/// marking once, ending one player's leaves the others, and a player id
/// past the mark's width is never marked, whatever is asked about it.
#[test]
fn exert_marks_are_kept_per_player() {
    let r = registry();
    let mut s = table(&r, 0);
    let glory = enter(&mut s, &r, 0, GLORY);
    let obj = s.objects.get_mut(glory).unwrap();
    assert!(obj.exerting_players().is_empty());
    obj.mark_exerted_by(0);
    obj.mark_exerted_by(3);
    obj.mark_exerted_by(3);
    assert!(obj.is_exerted_by(0) && obj.is_exerted_by(3) && !obj.is_exerted_by(1));
    assert_eq!(obj.exerting_players(), vec![0, 3]);
    obj.end_exert_of(0);
    assert_eq!(obj.exerting_players(), vec![3]);
    assert!(!obj.is_exerted_by(GameObject::MAX_EXERTING_PLAYERS) && !obj.is_exerted_by(200));
    obj.end_exert_of(200);
    assert_eq!(obj.exerting_players(), vec![3]);
    let last = GameObject::MAX_EXERTING_PLAYERS - 1;
    obj.mark_exerted_by(last);
    assert_eq!(obj.exerting_players(), vec![3, last]);
}

/// Marking a player the mark has no bit for is a programmer bug, and says so.
#[test]
#[should_panic(expected = "past the exert mark")]
fn marking_a_player_past_the_marks_width_panics() {
    let r = registry();
    let mut s = table(&r, 0);
    let glory = enter(&mut s, &r, 0, GLORY);
    s.objects.get_mut(glory).unwrap().mark_exerted_by(GameObject::MAX_EXERTING_PLAYERS);
}
