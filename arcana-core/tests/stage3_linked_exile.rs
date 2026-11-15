//! Linked exile: Bomat Courier. "Whenever this creature attacks, exile the
//! top card of your library face down. {R}, Discard your hand, Sacrifice
//! this creature: Put all cards exiled with this creature into their
//! owners' hands." The two abilities are linked (CR 607.2a): the second
//! returns only what the first exiled, for this object. A card exiled face
//! down may be looked at by no player (CR 406.3), its owner included.
use arcana_core::actions::Action;
use arcana_core::combat::{AttackerDeclaration, DefendingEntity};
use arcana_core::effects::Effect;
use arcana_core::engine::step;
use arcana_core::events::{GameEvent, MoveCause};
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::{GameObject, ObjectId};
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, ManaColor, PlayerId};
use arcana_core::view::view_state;
use arcana_core::zones::Zone;

const COURIER: &str = "Bomat Courier";

fn registry() -> CardRegistry {
    let mut r = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut r);
    r
}

/// `you`'s first main phase with priority. Each library is ten cards with
/// Opt on top, then Shock, then Mountains, so an exiled card is known to
/// the test by where it came from.
fn table(r: &CardRegistry, you: PlayerId) -> GameState {
    let mut s = GameState::new(2, 20261114);
    s.turn.active_player = you;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(you);
    for p in [0, 1] {
        add(&mut s, r, p, "Opt", Zone::Library(p));
        add(&mut s, r, p, "Shock", Zone::Library(p));
        for _ in 0..8 { add(&mut s, r, p, "Mountain", Zone::Library(p)); }
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

/// The active player attacks with `attackers`; their attack triggers
/// resolve, and the game stops in the declare attackers step.
fn attack(s: GameState, r: &CardRegistry, attackers: &[ObjectId]) -> GameState {
    let foe = 1 - s.active_player();
    let want: Vec<AttackerDeclaration> = attackers.iter()
        .map(|a| AttackerDeclaration { attacker: *a, defending: DefendingEntity::Player(foe) })
        .collect();
    let s = until(s, r, |s| legal_actions(s, r).iter()
        .any(|a| matches!(a, Action::DeclareAttackers { .. })));
    let s = act(s, r, |a| matches!(a, Action::DeclareAttackers { attackers } if *attackers == want));
    until(s, r, |s| s.stack_is_empty() && s.pending_trigger_queue.is_empty())
}

/// The cards exiled with `source`, by card, sorted.
fn pile(s: &GameState, source: ObjectId) -> Vec<(ObjectId, CardId)> {
    let mut pile: Vec<_> = s.objects.objects_in_zone(Zone::Exile)
        .filter(|o| o.linked_exile_source == Some(source))
        .map(|o| (o.id, o.card_id)).collect();
    pile.sort();
    pile
}

fn card(r: &CardRegistry, name: &str) -> CardId {
    r.card_id_by_name(name).unwrap()
}

fn hand_cards(s: &GameState, p: PlayerId) -> Vec<CardId> {
    let mut cards: Vec<CardId> = s.objects.objects_in_zone(Zone::Hand(p)).map(|o| o.card_id).collect();
    cards.sort();
    cards
}

/// `controller` pays {R}, discards their hand and sacrifices `courier`,
/// and the ability resolves.
fn cash_in(mut s: GameState, r: &CardRegistry, courier: ObjectId) -> GameState {
    let controller = s.objects.get(courier).expect("the Courier").controller;
    s.priority.give_to(controller);
    s.player_mut(controller).mana_pool.add_mana(ManaColor::Red, 1, 0);
    s = act(s, r, |a| matches!(a, Action::ActivateAbility { source, .. } if *source == courier));
    assert!(s.objects.get(courier).is_none_or(|o| o.zone != Zone::Battlefield),
        "the Courier was sacrificed as a cost");
    assert_eq!(s.stack.len(), 1, "its ability is on the stack");
    until(s, r, |s| s.stack_is_empty())
}

/// The attack trigger exiles the top card of the library face down,
/// linked to the Courier; cashing it in returns the card to its owner's
/// hand after the sacrifice.
#[test]
fn the_exiled_card_is_face_down_linked_and_returned() {
    let r = registry();
    for you in [0, 1] {
        let mut s = table(&r, you);
        let courier = enter(&mut s, &r, you, COURIER);
        let library = s.player(you).library_top_to_bottom.len();
        s = attack(s, &r, &[courier]);
        assert_eq!(s.player(you).library_top_to_bottom.len(), library - 1);
        let exiled = pile(&s, courier);
        assert_eq!(exiled.len(), 1, "seat {you}: one card with the Courier");
        assert_eq!(exiled[0].1, card(&r, "Opt"), "the top card");
        let obj = s.objects.get(exiled[0].0).unwrap();
        assert!(obj.status.face_down && obj.zone == Zone::Exile && obj.owner == you);

        s = until(s, &r, |s| s.turn.phase == Phase::PostCombatMain);
        s = cash_in(s, &r, courier);
        assert_eq!(hand_cards(&s, you), vec![card(&r, "Opt")], "the card is in its owner's hand");
        assert!(pile(&s, courier).is_empty());
        let back = s.objects.objects_in_zone(Zone::Hand(you)).next().unwrap();
        assert!(!back.status.face_down && back.linked_exile_source.is_none());
    }
}

/// Two Couriers keep separate piles: cashing in the first returns its two
/// cards and leaves the second's one in exile.
#[test]
fn two_couriers_keep_separate_piles() {
    let r = registry();
    for you in [0, 1] {
        let mut s = table(&r, you);
        let first = enter(&mut s, &r, you, COURIER);
        let second = enter(&mut s, &r, you, COURIER);
        // Triggers resolve last in, first out: the second Courier's exile
        // takes Opt and the first's takes Shock.
        s = attack(s, &r, &[first, second]);
        let by_card = |s: &GameState, c: ObjectId| -> Vec<CardId> {
            pile(s, c).into_iter().map(|(_, card)| card).collect() };
        assert_eq!(by_card(&s, first), vec![card(&r, "Shock")], "seat {you}");
        assert_eq!(by_card(&s, second), vec![card(&r, "Opt")]);

        // An extra combat: the first attacks alone and takes a Mountain.
        Effect::AdditionalCombatPhase.execute(&mut s);
        s = until(s, &r, |s| s.turn.step == Step::BeginCombat);
        Effect::Untap { target: first }.execute(&mut s);
        s = attack(s, &r, &[first]);
        assert_eq!(pile(&s, first).len(), 2);
        assert_eq!(pile(&s, second).len(), 1);

        s = until(s, &r, |s| s.turn.phase == Phase::PostCombatMain);
        s = cash_in(s, &r, first);
        let mut want = vec![card(&r, "Shock"), card(&r, "Mountain")];
        want.sort();
        assert_eq!(hand_cards(&s, you), want, "the first Courier's two cards");
        let left = pile(&s, second);
        assert_eq!(left.len(), 1, "the second's pile is untouched");
        assert!(s.objects.get(left[0].0).unwrap().status.face_down);
        assert!(s.objects.get(second).is_some_and(|o| o.zone == Zone::Battlefield));
    }
}

/// A Courier that leaves and returns is a new object with no pile: the
/// cards it exiled before stay in exile, face down.
#[test]
fn a_courier_that_returns_has_no_pile() {
    let r = registry();
    for you in [0, 1] {
        let mut s = table(&r, you);
        let courier = enter(&mut s, &r, you, COURIER);
        s = attack(s, &r, &[courier]);
        let old_pile = pile(&s, courier);
        assert_eq!(old_pile.len(), 1);
        s = until(s, &r, |s| s.turn.phase == Phase::PostCombatMain);

        let in_hand = s.move_object_to_zone(courier, Zone::Hand(you), MoveCause::SpellResolution)
            .expect("bounced");
        let again = s.move_object_to_zone(in_hand, Zone::Battlefield, MoveCause::SpellResolution)
            .expect("returned");
        s.trigger_event_cursor = s.event_log.len();
        assert_ne!(again, courier, "seat {you}: a new object");
        assert!(pile(&s, again).is_empty());

        s = cash_in(s, &r, again);
        assert!(hand_cards(&s, you).is_empty(), "nothing is exiled with the new Courier");
        assert_eq!(pile(&s, courier), old_pile, "the old pile stays where it is");
        assert!(s.objects.get(old_pile[0].0).unwrap().status.face_down);
    }
}

/// The costs stand alone: with no card exiled, {R}, the hand and the
/// Courier are paid all the same.
#[test]
fn the_costs_are_paid_with_no_linked_cards() {
    let r = registry();
    for you in [0, 1] {
        let mut s = table(&r, you);
        let courier = enter(&mut s, &r, you, COURIER);
        add(&mut s, &r, you, "Shock", Zone::Hand(you));
        add(&mut s, &r, you, "Opt", Zone::Hand(you));
        s = cash_in(s, &r, courier);
        assert!(hand_cards(&s, you).is_empty(), "seat {you}: the hand was discarded");
        assert_eq!(s.objects.count_in_zone(Zone::Graveyard(you)), 3, "two cards and the Courier");
        assert_eq!(s.player(you).mana_pool.total(), 0, "{{R}} was spent");
    }
}

/// "Into their owners' hands": a card a borrowed Courier exiled from
/// another player's library goes back to that player.
#[test]
fn cards_go_to_their_owners_hands() {
    let r = registry();
    for you in [0, 1] {
        let foe = 1 - you;
        let mut s = table(&r, you);
        let courier = enter(&mut s, &r, you, COURIER);
        s = attack(s, &r, &[courier]);
        s = until(s, &r, |s| s.turn.phase == Phase::PostCombatMain);
        // The opponent takes control and attacks with it on their turn: the
        // Courier exiles the top card of its controller's library.
        Effect::ChangeControl { target: courier, new_controller: foe }.execute(&mut s);
        s = until(s, &r, |s| s.active_player() == foe && s.turn.phase == Phase::PreCombatMain);
        let before = hand_cards(&s, foe);
        s = attack(s, &r, &[courier]);
        let owners: Vec<PlayerId> = pile(&s, courier).iter()
            .map(|(id, _)| s.objects.get(*id).unwrap().owner).collect();
        assert_eq!(owners.len(), 2);
        assert!(owners.contains(&you) && owners.contains(&foe), "seat {you}: one card of each");

        s = until(s, &r, |s| s.turn.phase == Phase::PostCombatMain);
        let mine_before = hand_cards(&s, you);
        s = cash_in(s, &r, courier);
        // The controller discarded their hand as a cost, then got their
        // own card back; the other player got theirs.
        let _ = before;
        assert_eq!(hand_cards(&s, foe).len(), 1, "the controller's own exiled card");
        assert_eq!(hand_cards(&s, you).len(), mine_before.len() + 1, "and the owner's");
        assert!(hand_cards(&s, you).contains(&card(&r, "Opt")));
    }
}

/// Face-down cards stay hidden in the view, for both seats, even built
/// from the true state: no name, no cost, no type, only a card back; and
/// the log names no card.
#[test]
fn the_view_and_the_log_hide_the_card_from_both_seats() {
    let r = registry();
    for you in [0, 1] {
        let mut s = table(&r, you);
        let courier = enter(&mut s, &r, you, COURIER);
        let log_from = s.event_log.len();
        s = attack(s, &r, &[courier]);
        let exiled = pile(&s, courier)[0].0;
        for seat in [0, 1] {
            let view = view_state(&s, &r, seat, &[]);
            let owner = &view.players[you as usize];
            assert_eq!(owner.exile_count, 1);
            assert_eq!(owner.exile.len(), 1);
            let shown = &owner.exile[0];
            assert!(shown.face_down, "seat {seat} sees a card back");
            assert_eq!((shown.id, shown.name.as_str(), shown.type_line.as_str()), (exiled, "", ""));
            assert!(shown.mana_cost.is_none() && shown.keywords.is_empty()
                && shown.power.is_none() && !shown.playable);
            let json = serde_json::to_string(&view).unwrap();
            assert!(!json.contains("Opt"), "seat {seat}: the view names it nowhere");

            let lines: Vec<String> = s.event_log[log_from..].iter()
                .filter_map(|e| arcana_core::render::describe_event(e, &s, &r, seat)).collect();
            assert!(lines.iter().all(|l| !l.contains("Opt")), "{lines:?}");
            assert!(s.event_log[log_from..].iter().any(|e| matches!(e,
                GameEvent::Exiled { object_id, .. } if *object_id == exiled)));
            assert!(lines.iter().any(|l| l == "a card is exiled"), "{lines:?}");
        }
    }
}
