//! A1.5bc: damage dealt to a planeswalker removes that many loyalty
//! counters (CR 120.3c), from a spell and in combat. Before, it was marked
//! like a creature's damage and the loyalty never moved, so a planeswalker
//! could not be burned or attacked to death.

use arcana_core::actions::Action;
use arcana_core::combat::DefendingEntity;
use arcana_core::engine::step;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetSelection};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, CounterKind, ManaColor, PlayerId};
use arcana_core::zones::Zone;
use arcana_core::{EngineYield, ObjectId};

struct Cards {
    bolt: CardId,
    teferi: CardId,
    bears: CardId,
}

fn registry() -> (CardRegistry, Cards) {
    let mut reg = CardRegistry::new();
    let c = Cards {
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        teferi: arcana_cards::rvr::teferi_time_raveler::register(&mut reg),
        bears: arcana_cards::lea::grizzly_bears::register(&mut reg),
    };
    (reg, c)
}

fn add(s: &mut GameState, reg: &CardRegistry, owner: PlayerId, card: CardId, zone: Zone) -> ObjectId {
    let id = s.allocate_object_id();
    let chars = reg.get(card).unwrap().initial_characteristics().clone();
    s.objects.insert(GameObject::new(id, owner, zone, card, chars));
    if let Zone::Library(p) = zone { s.player_mut(p).library_top_to_bottom.push(id); }
    id
}

/// Player 0's precombat main phase with priority and an empty stack, and
/// player 1's Teferi, Time Raveler at `loyalty`.
fn table(reg: &CardRegistry, c: &Cards, loyalty: u32) -> (GameState, ObjectId) {
    let mut s = GameState::new(2, 0);
    s.turn.active_player = 0;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(0);
    let t = add(&mut s, reg, 1, c.teferi, Zone::Battlefield);
    s.objects.get_mut(t).unwrap().add_counters(CounterKind::Loyalty, loyalty);
    (s, t)
}

fn hits(sel: &TargetSelection, id: ObjectId) -> bool {
    sel.targets.iter().any(|t| matches!(t, TargetChoice::Object(o)
        | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(o)) if *o == id))
}

/// Pass until the stack is empty and nothing is pending.
fn settle(mut s: GameState, mut y: EngineYield, reg: &CardRegistry) -> GameState {
    for _ in 0..100 {
        if s.stack_is_empty() && s.pending_choice.is_none() { return s; }
        let EngineYield::PendingDecision { legal_actions, .. } = &y else { return s };
        let a = legal_actions.iter().find(|a| a.is_pass()).cloned()
            .or_else(|| legal_actions.iter().find(|a| !matches!(a, Action::Concede)).cloned())
            .expect("an action");
        (s, y) = step(s, a, reg);
    }
    panic!("the stack did not settle");
}

fn bolt_teferi(loyalty: u32) -> (GameState, ObjectId, Cards) {
    let (reg, c) = registry();
    let (mut s, t) = table(&reg, &c, loyalty);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Red, 1, 0);
    let bolt = add(&mut s, &reg, 0, c.bolt, Zone::Hand(0));
    let a = legal_actions(&s, &reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, targets, .. } if *object_id == bolt && hits(targets, t)))
        .expect("Lightning Bolt can target Teferi");
    let (s, y) = step(s, a, &reg);
    (settle(s, y, &reg), t, c)
}

#[test]
fn lightning_bolt_removes_three_loyalty_from_teferi() {
    let (s, t, _) = bolt_teferi(4);
    let teferi = s.objects.get(t).expect("Teferi survives with 1 loyalty");
    assert_eq!(teferi.count_counters(CounterKind::Loyalty), 1);
    assert_eq!(teferi.damage_marked, 0, "nothing is marked on a noncreature planeswalker");
}

#[test]
fn lightning_bolt_kills_a_three_loyalty_teferi() {
    let (s, t, c) = bolt_teferi(3);
    assert!(s.objects.get(t).is_none(), "Teferi left the battlefield");
    assert!(s.objects.iter().any(|o| o.card_id == c.teferi && o.zone == Zone::Graveyard(1)),
        "the loyalty SBA put Teferi into its owner's graveyard");
}

/// Grizzly Bears attack Teferi through the engine; player 1 declares no
/// blockers, and the combat damage removes two loyalty.
#[test]
fn an_unblocked_attacker_removes_loyalty_in_combat() {
    let (reg, c) = registry();
    let (mut s, t) = table(&reg, &c, 4);
    for p in [0, 1] { for _ in 0..3 { add(&mut s, &reg, p, c.bolt, Zone::Library(p)); } }
    let bears = add(&mut s, &reg, 0, c.bears, Zone::Battlefield);
    s.objects.get_mut(bears).unwrap().controller = 0;
    let mut attacked = false;
    for _ in 0..60 {
        if attacked && matches!(s.turn.phase, Phase::PostCombatMain | Phase::Ending) { break; }
        let acts = legal_actions(&s, &reg);
        let declare = acts.iter().find(|a| matches!(a, Action::DeclareAttackers { attackers }
            if attackers.len() == 1 && attackers[0].attacker == bears
                && attackers[0].defending == DefendingEntity::Planeswalker(t))).cloned();
        let a = if let Some(d) = declare { attacked = true; d }
            else if let Some(b) = acts.iter().find(|a| matches!(a, Action::DeclareBlockers { blockers } if blockers.is_empty())).cloned() { b }
            else { acts.iter().find(|a| a.is_pass()).cloned().expect("a pass") };
        (s, _) = step(s, a, &reg);
    }
    assert!(attacked, "Grizzly Bears could attack Teferi");
    let teferi = s.objects.get(t).expect("Teferi survives two damage");
    assert_eq!(teferi.count_counters(CounterKind::Loyalty), 2);
    assert_eq!(s.player(1).life, 20, "the damage went to Teferi, not its controller");
}
