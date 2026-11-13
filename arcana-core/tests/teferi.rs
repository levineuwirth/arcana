//! A1.5aa (D24): Teferi, Time Raveler. "Each opponent can cast spells only
//! any time they could cast a sorcery" holds on every cast path, Spell
//! Queller's free cast and the effects that cast a card mid-resolution
//! (Toshiro Umezawa's) included, for Teferi's controller's opponents only.
//! "+1: Until your next turn, you may cast sorcery spells as though they had
//! flash" lasts until that turn whether or not Teferi stays.

use arcana_core::actions::{Action, ChoiceKind, ChoiceResponse};
use arcana_core::engine::step;
use arcana_core::events::MoveCause;
use arcana_core::legal_actions::{casts_without_paying, legal_actions};
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetSelection};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, CounterKind, ManaColor, PlayerId};
use arcana_core::zones::Zone;
use arcana_core::{EngineYield, ObjectId};

struct Cards {
    teferi: CardId,
    queller: CardId,
    bolt: CardId,
    spike: CardId,
}

fn registry() -> (CardRegistry, Cards) {
    let mut reg = CardRegistry::new();
    let cards = Cards {
        teferi: arcana_cards::rvr::teferi_time_raveler::register(&mut reg),
        queller: arcana_cards::inr::spell_queller::register(&mut reg),
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        spike: arcana_cards::uma::lava_spike::register(&mut reg),
    };
    (reg, cards)
}

fn add(s: &mut GameState, reg: &CardRegistry, owner: PlayerId, card: CardId, zone: Zone) -> ObjectId {
    let id = s.allocate_object_id();
    let chars = reg.get(card).unwrap().initial_characteristics().clone();
    s.objects.insert(GameObject::new(id, owner, zone, card, chars));
    id
}

fn add_teferi(s: &mut GameState, reg: &CardRegistry, c: &Cards, controller: PlayerId) -> ObjectId {
    let id = add(s, reg, controller, c.teferi, Zone::Battlefield);
    s.objects.get_mut(id).unwrap().add_counters(CounterKind::Loyalty, 4);
    id
}

/// `active`'s precombat main phase, `priority` holding priority, two red
/// mana for each player.
fn table(active: PlayerId, priority: PlayerId) -> GameState {
    let mut s = GameState::new(2, 0);
    s.turn.active_player = active;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(priority);
    for p in [0, 1] {
        s.player_mut(p).mana_pool.add_mana(ManaColor::Red, 2, 0);
    }
    s
}

fn offered(s: &GameState, reg: &CardRegistry, card: ObjectId) -> bool {
    legal_actions(s, reg).iter().any(|a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == card))
}

fn hits_player(sel: &TargetSelection, p: PlayerId) -> bool {
    sel.targets.iter().any(|t| matches!(t, TargetChoice::Player(q)
        | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(q)) if *q == p))
}

fn hits_object(sel: &TargetSelection, id: ObjectId) -> bool {
    sel.targets.iter().any(|t| matches!(t, TargetChoice::Object(o)
        | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(o)) if *o == id))
}

fn cast(s: &GameState, reg: &CardRegistry, card: ObjectId, aim: impl Fn(&TargetSelection) -> bool) -> Action {
    legal_actions(s, reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, targets, .. } if *object_id == card && aim(targets)))
        .unwrap_or_else(|| panic!("{card} is castable as asked"))
}

#[test]
fn an_opponent_of_teferi_casts_only_at_sorcery_timing() {
    let (reg, c) = registry();
    for teferi in [false, true] {
        // Player 0's main phase, empty stack: sorcery timing, Bolt castable.
        let mut s = table(0, 0);
        if teferi { add_teferi(&mut s, &reg, &c, 1); }
        let bolt = add(&mut s, &reg, 0, c.bolt, Zone::Hand(0));
        let second = add(&mut s, &reg, 0, c.bolt, Zone::Hand(0));
        let answer = add(&mut s, &reg, 1, c.bolt, Zone::Hand(1));
        assert!(offered(&s, &reg, bolt), "teferi={teferi}: at sorcery timing an instant is castable");
        // With a Bolt on the stack it is no longer sorcery timing.
        let a = cast(&s, &reg, bolt, |t| hits_player(t, 1));
        let (mut s, _) = step(s, a, &reg);
        assert!(!s.stack_is_empty());
        assert_eq!(offered(&s, &reg, second), !teferi,
            "teferi={teferi}: the second Bolt in response");
        // Teferi's controller is not restricted.
        s.priority.give_to(1);
        assert!(offered(&s, &reg, answer), "teferi={teferi}: Teferi's controller answers at instant speed");
        // On the other player's turn, never sorcery timing for player 0.
        let mut s = table(1, 0);
        if teferi { add_teferi(&mut s, &reg, &c, 1); }
        let bolt = add(&mut s, &reg, 0, c.bolt, Zone::Hand(0));
        assert_eq!(offered(&s, &reg, bolt), !teferi, "teferi={teferi}: on player 1's turn");
    }
}

/// The alternative-zone loops share the hand's timing check: Brazen
/// Borrower's Petty Theft (an instant Adventure) and the flash creature
/// itself both wait for sorcery timing under an opponent's Teferi.
#[test]
fn an_adventure_and_a_flash_creature_wait_too() {
    use arcana_core::actions::CastModifier;
    let mut reg = CardRegistry::new();
    let c = Cards {
        teferi: arcana_cards::rvr::teferi_time_raveler::register(&mut reg),
        queller: arcana_cards::inr::spell_queller::register(&mut reg),
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        spike: arcana_cards::uma::lava_spike::register(&mut reg),
    };
    let borrower = arcana_cards::soc::brazen_borrower_petty_theft::register(&mut reg);
    let grizzly = arcana_cards::lea::grizzly_bears::register(&mut reg);
    for teferi in [false, true] {
        // Player 1's turn: player 0 has priority, so only instant speed works.
        let mut s = table(1, 0);
        s.player_mut(0).mana_pool.add_mana(ManaColor::Blue, 3, 0);
        if teferi { add_teferi(&mut s, &reg, &c, 1); }
        // Something for Petty Theft to return.
        add(&mut s, &reg, 1, grizzly, Zone::Battlefield);
        let card = add(&mut s, &reg, 0, borrower, Zone::Hand(0));
        let casts: Vec<Action> = legal_actions(&s, &reg).into_iter()
            .filter(|a| matches!(a, Action::CastSpell { object_id, .. } if *object_id == card))
            .collect();
        let adventure = casts.iter().any(|a| matches!(a, Action::CastSpell { cast_modifier: CastModifier::Adventure, .. }));
        let creature = casts.iter().any(|a| matches!(a, Action::CastSpell { cast_modifier: CastModifier::None, .. }));
        assert_eq!(adventure, !teferi, "teferi={teferi}: Petty Theft");
        assert_eq!(creature, !teferi, "teferi={teferi}: Brazen Borrower with flash");
    }
}

#[test]
fn a_free_cast_is_refused_to_an_opponent_of_teferi() {
    let (reg, c) = registry();
    for controller in [None, Some(0), Some(1)] {
        let mut s = table(0, 0);
        if let Some(p) = controller { add_teferi(&mut s, &reg, &c, p); }
        let exiled = add(&mut s, &reg, 0, c.spike, Zone::Exile);
        assert_eq!(casts_without_paying(&s, &reg, 0, exiled).is_empty(), controller == Some(1),
            "Teferi controlled by {controller:?}: player 0's free cast");
    }
}

type Pick<'a> = dyn FnMut(&GameState, PlayerId, &[Action]) -> Option<Action> + 'a;

fn settle(mut s: GameState, mut yld: EngineYield, reg: &CardRegistry, pick: &mut Pick) -> GameState {
    for _ in 0..300 {
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

fn in_zone(s: &GameState, card: CardId, zone: Zone) -> Option<ObjectId> {
    s.objects.iter().find(|o| o.card_id == card && o.zone == zone).map(|o| o.id)
}

/// Player 1 controls Teferi and quells player 0's Lava Spike; player 0 kills
/// the Queller at sorcery timing. Player 0 is not asked to cast the Spike,
/// and it stays in exile.
#[test]
fn with_teferi_out_a_quelled_spell_stays_exiled() {
    let (reg, c) = registry();
    let mut s = table(0, 0);
    add_teferi(&mut s, &reg, &c, 1);
    s.player_mut(1).mana_pool.add_mana(ManaColor::White, 1, 0);
    s.player_mut(1).mana_pool.add_mana(ManaColor::Blue, 2, 0);
    let spike = add(&mut s, &reg, 0, c.spike, Zone::Hand(0));
    let queller = add(&mut s, &reg, 1, c.queller, Zone::Hand(1));
    let a = cast(&s, &reg, spike, |t| hits_player(t, 1));
    let (s, y) = step(s, a, &reg);
    let on_stack = s.stack.last().map(|e| e.id).expect("Lava Spike is on the stack");
    let mut quelled = false;
    let s = settle(s, y, &reg, &mut |s, p, legal| {
        if p == 1 && !quelled && s.objects.get(queller).is_some_and(|o| o.zone == Zone::Hand(1)) {
            quelled = true;
            return Some(cast(s, &reg, queller, |_| true));
        }
        legal.iter().find(|a| matches!(a, Action::SubmitResolutionChoice {
            response: ChoiceResponse::ChooseTargets { selection }, .. } if hits_object(selection, on_stack)))
            .cloned()
    });
    assert!(quelled, "Teferi's controller flashed in Spell Queller");
    let exiled = in_zone(&s, c.spike, Zone::Exile).expect("Lava Spike was exiled");
    assert_eq!(s.player(1).life, 20, "the Spike did not resolve");

    // Player 0, at sorcery timing, bolts the Queller.
    let mut s = s;
    let on = in_zone(&s, c.queller, Zone::Battlefield).expect("Spell Queller is on the battlefield");
    let bolt = add(&mut s, &reg, 0, c.bolt, Zone::Hand(0));
    s.priority.give_to(0);
    let a = cast(&s, &reg, bolt, |t| hits_object(t, on));
    let (s, y) = step(s, a, &reg);
    let mut asked = false;
    let s = settle(s, y, &reg, &mut |s, _, _| {
        if matches!(s.pending_choice.as_ref().map(|c| &c.kind), Some(ChoiceKind::CastWithoutPaying { .. })) {
            asked = true;
        }
        None
    });
    assert!(in_zone(&s, c.queller, Zone::Battlefield).is_none(), "Spell Queller died");
    assert!(!asked, "player 0 was not offered the free cast");
    assert_eq!(s.objects.get(exiled).map(|o| o.zone), Some(Zone::Exile), "Lava Spike stays in exile");
    assert_eq!(s.player(1).life, 20);
}

#[test]
fn teferis_plus_one_gives_sorceries_flash_until_its_controllers_next_turn() {
    let (reg, c) = registry();
    let mut s = table(0, 0);
    let teferi = add_teferi(&mut s, &reg, &c, 0);
    let spike = add(&mut s, &reg, 0, c.spike, Zone::Hand(0));
    // On player 1's turn, before the +1, the sorcery is not castable.
    let mut theirs = s.clone();
    theirs.turn.active_player = 1;
    assert!(!offered(&theirs, &reg, spike), "a sorcery on the opponent's turn");

    let plus_one = legal_actions(&s, &reg).into_iter()
        .find(|a| matches!(a, Action::ActivateAbility { source, ability_index: 0, .. } if *source == teferi))
        .expect("the +1 is offered at sorcery timing");
    let (s, y) = step(s, plus_one, &reg);
    let mut s = settle(s, y, &reg, &mut |_, _, _| None);
    assert_eq!(s.objects.get(teferi).unwrap().count_counters(CounterKind::Loyalty), 5);

    s.turn.active_player = 1;
    s.priority.give_to(0);
    assert!(offered(&s, &reg, spike), "after the +1, the sorcery on the opponent's turn");
    // Teferi leaves; the permission stays until player 0's next turn.
    s.move_object_to_zone(teferi, Zone::Graveyard(0), MoveCause::StateBasedAction);
    assert!(offered(&s, &reg, spike), "after Teferi has left");
    s.expire_until_next_turn_effects(0);
    assert!(!offered(&s, &reg, spike), "player 0's next turn ends it");
}

/// The effects that cast a card mid-resolution observe the restriction too:
/// Toshiro Umezawa's trigger targets a Lightning Bolt in its controller's
/// graveyard after an opposing creature dies, and with player 1's Teferi out
/// the Bolt is not cast and stays in the graveyard. Without Teferi it is
/// cast. (The review's probe, kept as the regression.)
#[test]
fn toshiro_cannot_cast_through_an_opposing_teferi() {
    for teferi in [true, false] {
        let (mut reg, c) = registry();
        let toshiro = arcana_cards::bok::toshiro_umezawa::register(&mut reg);
        let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
        let mut s = table(0, 0);
        if teferi { add_teferi(&mut s, &reg, &c, 1); }
        add(&mut s, &reg, 0, toshiro, Zone::Battlefield);
        let bear = add(&mut s, &reg, 1, bears, Zone::Battlefield);
        let gy_bolt = add(&mut s, &reg, 0, c.bolt, Zone::Graveyard(0));
        let hand_bolt = add(&mut s, &reg, 0, c.bolt, Zone::Hand(0));
        let a = cast(&s, &reg, hand_bolt, |sel| hits_object(sel, bear));
        let (s, y) = step(s, a, &reg);
        let mut selected = false;
        let mut cast_from_graveyard = false;
        let s = settle(s, y, &reg, &mut |s, _, legal| {
            cast_from_graveyard |= s.stack.iter().any(|e|
                e.cast_from_zone == Zone::Graveyard(0));
            let a = legal.iter().find(|a| matches!(a, Action::SubmitResolutionChoice {
                response: ChoiceResponse::ChooseTargets { selection }, .. }
                if hits_object(selection, gy_bolt))).cloned();
            selected |= a.is_some();
            a
        });
        assert!(selected, "teferi={teferi}: Toshiro's trigger targeted the graveyard Bolt");
        assert_eq!(cast_from_graveyard, !teferi, "teferi={teferi}: a Bolt cast from the graveyard");
        let still_there = s.objects.get(gy_bolt).map(|o| o.zone) == Some(Zone::Graveyard(0));
        assert_eq!(still_there, teferi, "teferi={teferi}: the graveyard Bolt stays put");
    }
}

/// Each direct-cast effect, run straight and nested in a sequence that
/// executes its steps in place, casts nothing for an opponent of Teferi and
/// casts for anyone else.
#[test]
fn every_direct_cast_effect_observes_teferi() {
    use arcana_core::effects::Effect;
    let (reg, c) = registry();
    let zones = [Zone::Hand(0), Zone::Graveyard(0), Zone::Exile];
    for (i, zone) in zones.into_iter().enumerate() {
        for controller in [None, Some(0), Some(1)] {
            for nested in [false, true] {
                let mut s = table(0, 0);
                if let Some(p) = controller { add_teferi(&mut s, &reg, &c, p); }
                let card = add(&mut s, &reg, 0, c.spike, zone);
                let cast = match i {
                    0 => Effect::CastFromHandFree { player: 0, target: card },
                    1 => Effect::CastFromGraveyard { player: 0, target: card },
                    _ => Effect::CastFromExileFree { player: 0, target: card },
                };
                let effect = if nested { Effect::Sequence(vec![cast]) } else { cast };
                effect.execute(&mut s);
                assert_eq!(s.stack.is_empty(), controller == Some(1),
                    "{zone:?}, nested={nested}, Teferi controlled by {controller:?}");
            }
        }
    }
}
