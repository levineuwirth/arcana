//! A1.5ab: a replacement effect bound to its source's time on the
//! battlefield ends when the source leaves, never starts if the source has
//! left before the effect is installed, and keeps its installer as "you"
//! from either seat. Winding Constrictor's enters trigger installs "if one or
//! more counters would be put on an artifact or creature you control, that
//! many plus one are put instead" for as long as it stays.

use arcana_core::engine::step;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::replacement::CounterTarget;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetSelection};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, CounterKind, ManaColor, PlayerId};
use arcana_core::zones::Zone;
use arcana_core::{Action, EngineYield, ObjectId};

struct Cards {
    constrictor: CardId,
    bolt: CardId,
    bears: CardId,
}

fn registry() -> (CardRegistry, Cards) {
    let mut reg = CardRegistry::new();
    let cards = Cards {
        constrictor: arcana_cards::aer::winding_constrictor::register(&mut reg),
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        bears: arcana_cards::lea::grizzly_bears::register(&mut reg),
    };
    (reg, cards)
}

fn add(s: &mut GameState, reg: &CardRegistry, owner: PlayerId, card: CardId, zone: Zone) -> ObjectId {
    let id = s.allocate_object_id();
    let chars = reg.get(card).unwrap().initial_characteristics().clone();
    s.objects.insert(GameObject::new(id, owner, zone, card, chars));
    id
}

/// `seat`'s main phase with priority, Constrictor and Bolt in hand, the mana
/// for both, and a Grizzly Bears on each side.
struct Table {
    s: GameState,
    constrictor: ObjectId,
    bolt: ObjectId,
    own_bears: ObjectId,
    their_bears: ObjectId,
}

fn table(reg: &CardRegistry, c: &Cards, seat: PlayerId) -> Table {
    let mut s = GameState::new(2, 0);
    s.turn.active_player = seat;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(seat);
    for color in [ManaColor::Black, ManaColor::Green, ManaColor::Red] {
        s.player_mut(seat).mana_pool.add_mana(color, 1, 0);
    }
    let constrictor = add(&mut s, reg, seat, c.constrictor, Zone::Hand(seat));
    let bolt = add(&mut s, reg, seat, c.bolt, Zone::Hand(seat));
    let own_bears = add(&mut s, reg, seat, c.bears, Zone::Battlefield);
    let their_bears = add(&mut s, reg, 1 - seat, c.bears, Zone::Battlefield);
    Table { s, constrictor, bolt, own_bears, their_bears }
}

fn hits_object(sel: &TargetSelection, id: ObjectId) -> bool {
    sel.targets.iter().any(|t| matches!(t, TargetChoice::Object(o)
        | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(o)) if *o == id))
}

fn cast(s: &GameState, reg: &CardRegistry, card: ObjectId, aim: impl Fn(&TargetSelection) -> bool) -> Action {
    legal_actions(s, reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, targets, .. }
            if *object_id == card && aim(targets)))
        .unwrap_or_else(|| panic!("{card} is castable as asked"))
}

fn on_battlefield(s: &GameState, card: CardId) -> Option<ObjectId> {
    s.objects.objects_in_zone(Zone::Battlefield).find(|o| o.card_id == card).map(|o| o.id)
}

type Pick<'a> = dyn FnMut(&GameState, PlayerId, &[Action]) -> Option<Action> + 'a;

/// Answer every decision until the stack is empty and no choice is pending:
/// `pick` may answer; otherwise pass.
fn settle(mut s: GameState, mut yld: EngineYield, reg: &CardRegistry, pick: &mut Pick) -> GameState {
    for _ in 0..200 {
        if s.stack_is_empty() && s.pending_choice.is_none() { return s; }
        let EngineYield::PendingDecision { player, legal_actions, .. } = &yld else { return s };
        let a = pick(&s, *player, legal_actions)
            .or_else(|| legal_actions.iter().find(|a| a.is_pass()).cloned())
            .or_else(|| legal_actions.iter().find(|a| !matches!(a, Action::Concede)).cloned())
            .expect("an action");
        (s, yld) = step(s, a, reg);
    }
    panic!("the stack did not settle");
}

/// How many +1/+1 counters one counter becomes on `target`.
fn one_counter_becomes(s: &mut GameState, target: ObjectId) -> u32 {
    s.place_counters(CounterTarget::Object(target), CounterKind::PlusOnePlusOne, 1)
        .map(|(_, n)| n)
        .unwrap_or(0)
}

/// Cast Constrictor and let its trigger resolve, then Bolt it.
fn constrictor_installed_then_bolted(seat: PlayerId) -> (Table, u32) {
    let (reg, c) = registry();
    let mut t = table(&reg, &c, seat);
    let a = cast(&t.s, &reg, t.constrictor, |_| true);
    let (s, y) = step(t.s, a, &reg);
    let mut s = settle(s, y, &reg, &mut |_, _, _| None);
    let on = on_battlefield(&s, c.constrictor).expect("Constrictor resolved");
    let while_on = one_counter_becomes(&mut s, t.own_bears);
    let a = cast(&s, &reg, t.bolt, |sel| hits_object(sel, on));
    let (s, y) = step(s, a, &reg);
    t.s = settle(s, y, &reg, &mut |_, _, _| None);
    assert!(on_battlefield(&t.s, c.constrictor).is_none(), "Bolt killed Constrictor");
    (t, while_on)
}

#[test]
fn a_departed_constrictor_adds_no_counters_from_either_seat() {
    for seat in [0, 1] {
        let (mut t, while_on) = constrictor_installed_then_bolted(seat);
        assert_eq!(while_on, 2, "seat {seat}: while Constrictor is on the battlefield one counter becomes two");
        assert_eq!(one_counter_becomes(&mut t.s, t.own_bears), 1,
            "seat {seat}: after it has left, its controller's counter is one");
        assert_eq!(one_counter_becomes(&mut t.s, t.their_bears), 1,
            "seat {seat}: after it has left, the opponent's counter is one");
        assert!(t.s.replacement_effects.is_empty(), "seat {seat}: its effect ended");
    }
}

#[test]
fn a_constrictor_bolted_in_response_to_its_trigger_installs_nothing() {
    for seat in [0, 1] {
        let (reg, c) = registry();
        let t = table(&reg, &c, seat);
        let a = cast(&t.s, &reg, t.constrictor, |_| true);
        let (s, y) = step(t.s, a, &reg);
        let bolt = t.bolt;
        let mut bolted = false;
        let mut s = settle(s, y, &reg, &mut |s, p, _| {
            // With the enters trigger on the stack, Bolt the Constrictor.
            let on = on_battlefield(s, c.constrictor)?;
            if bolted || p != seat || s.stack_is_empty() { return None; }
            bolted = true;
            Some(cast(s, &reg, bolt, |sel| hits_object(sel, on)))
        });
        assert!(bolted, "seat {seat}: Bolt was cast in response to the trigger");
        assert!(on_battlefield(&s, c.constrictor).is_none(), "seat {seat}: Bolt killed Constrictor");
        assert!(s.replacement_effects.is_empty(),
            "seat {seat}: a trigger resolving after its source left installs nothing");
        assert_eq!(one_counter_becomes(&mut s, t.own_bears), 1, "seat {seat}: own counter");
        assert_eq!(one_counter_becomes(&mut s, t.their_bears), 1, "seat {seat}: opponent's counter");
    }
}
