//! Spell Queller (D18): it exiles a spell with mana value 4 or less, X
//! included (CR 202.3e), and when it leaves the battlefield the exiled card's
//! owner may cast it without paying its mana cost (CR 118.9), during the
//! leave ability's resolution (CR 608.2g). The free cast chooses its targets
//! and modes, sets X to 0, offers kicker, and ignores card-type timing. A
//! spell exiled after Queller has left stays exiled. Also A1.5p: a spell an
//! effect moves off the stack doesn't resolve from a stranded stack entry.

use arcana_core::actions::{Action, ChoiceKind, ChoiceResponse, CastModifier};
use arcana_core::engine::step;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetSelection};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, ManaColor, PlayerId};
use arcana_core::zones::Zone;
use arcana_core::{EngineYield, ObjectId};

struct Cards {
    queller: CardId,
    bolt: CardId,
    bears: CardId,
}

fn registry() -> (CardRegistry, Cards) {
    let mut reg = CardRegistry::new();
    let cards = Cards {
        queller: arcana_cards::inr::spell_queller::register(&mut reg),
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        bears: arcana_cards::lea::grizzly_bears::register(&mut reg),
    };
    (reg, cards)
}

fn add(s: &mut GameState, reg: &CardRegistry, owner: PlayerId, card: CardId, zone: Zone) -> ObjectId {
    let id = s.allocate_object_id();
    // As a game deals it: a split card is the whole card off the stack.
    let chars = reg.get(card).unwrap().initial_characteristics().clone();
    s.objects.insert(GameObject::new(id, owner, zone, card, chars));
    id
}

/// Player 0's main phase, player 0 holding priority, with mana in each pool:
/// player 0 has `p0_red` red, player 1 has Spell Queller's {1}{W}{U}.
fn table(p0_red: u32) -> GameState {
    let mut s = GameState::new(2, 0);
    s.turn.active_player = 0;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(0);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Red, p0_red, 0);
    s.player_mut(1).mana_pool.add_mana(ManaColor::White, 1, 0);
    s.player_mut(1).mana_pool.add_mana(ManaColor::Blue, 2, 0);
    s
}

fn hits_player(sel: &TargetSelection, p: PlayerId) -> bool {
    sel.targets.iter().any(|t| matches!(t, TargetChoice::Player(q)
        | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(q)) if *q == p))
}

fn hits_object(sel: &TargetSelection, id: ObjectId) -> bool {
    sel.targets.iter().any(|t| matches!(t, TargetChoice::Object(o)
        | TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(o)) if *o == id))
}

/// The cast of `card` whose targets `aim` accepts (and whose X is `x`, when given).
fn cast_action(s: &GameState, reg: &CardRegistry, card: ObjectId, x: Option<u32>,
               aim: impl Fn(&TargetSelection) -> bool) -> Action {
    legal_actions(s, reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { object_id, targets, x_value, .. }
            if *object_id == card && aim(targets) && (x.is_none() || *x_value == x)))
        .unwrap_or_else(|| panic!("{card} is castable as asked"))
}

type Pick<'a> = dyn FnMut(&GameState, PlayerId, &[Action]) -> Option<Action> + 'a;

/// Answer every decision until the stack is empty and no choice is pending:
/// `pick` may answer; otherwise pass, or take the first offer (a free cast's
/// first offer is "don't cast").
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

/// Player 1 answers player 0's spell with Spell Queller, aimed at `spell`.
fn queller_answers<'a>(queller: ObjectId, spell: ObjectId, reg: &'a CardRegistry) -> impl FnMut(&GameState, PlayerId, &[Action]) -> Option<Action> + 'a {
    let mut cast = false;
    move |s: &GameState, p: PlayerId, legal: &[Action]| {
        if p == 1 && !cast && s.objects.get(queller).is_some_and(|o| o.zone == Zone::Hand(1)) {
            cast = true;
            return Some(cast_action(s, reg, queller, None, |_| true));
        }
        legal.iter().find(|a| matches!(a, Action::SubmitResolutionChoice {
            response: ChoiceResponse::ChooseTargets { selection }, .. } if hits_object(selection, spell)))
            .cloned()
    }
}

fn in_zone(s: &GameState, card: CardId, zone: Zone) -> Option<ObjectId> {
    s.objects.iter().find(|o| o.card_id == card && o.zone == zone).map(|o| o.id)
}

/// Player 0 casts `spell` (already in hand, aimed by `aim`), player 1 quells
/// it, and the stack settles. Returns the state and the exiled card's id.
fn quell(mut s: GameState, reg: &CardRegistry, c: &Cards, spell: ObjectId, x: Option<u32>,
         aim: impl Fn(&TargetSelection) -> bool) -> (GameState, Option<ObjectId>) {
    let queller = add(&mut s, reg, 1, c.queller, Zone::Hand(1));
    let card = s.objects.get(spell).unwrap().card_id;
    let a = cast_action(&s, reg, spell, x, aim);
    let (s, yld) = step(s, a, reg);
    let on_stack = s.stack.last().map(|e| e.id).expect("the spell is on the stack");
    let s = settle(s, yld, reg, &mut queller_answers(queller, on_stack, reg));
    let exiled = in_zone(&s, card, Zone::Exile);
    (s, exiled)
}

/// Player 0 bolts Spell Queller, and the stack settles with `pick` answering
/// the rest (the leave ability's free cast among it).
fn bolt_the_queller(mut s: GameState, reg: &CardRegistry, c: &Cards, pick: &mut Pick) -> GameState {
    let queller = in_zone(&s, c.queller, Zone::Battlefield).expect("Spell Queller is on the battlefield");
    let bolt = add(&mut s, reg, 0, c.bolt, Zone::Hand(0));
    s.player_mut(0).mana_pool.add_mana(ManaColor::Red, 1, 0);
    s.priority.give_to(0);
    let a = cast_action(&s, reg, bolt, None, |t| hits_object(t, queller));
    let (s, yld) = step(s, a, reg);
    settle(s, yld, reg, pick)
}

/// The free-cast offers in a pending CastWithoutPaying choice.
fn offers(legal: &[Action]) -> Vec<&Action> {
    legal.iter().filter_map(|a| match a {
        Action::SubmitResolutionChoice { response: ChoiceResponse::CastWithoutPaying { cast: Some(c) }, .. } => Some(c.as_ref()),
        _ => None,
    }).collect()
}

fn choice_kind(s: &GameState) -> Option<(PlayerId, ChoiceKind)> {
    s.pending_choice.as_ref().map(|c| (c.choosing_player, c.kind.clone()))
}

// ---------------------------------------------------------------------

/// The exile, and the owner's cast when Spell Queller leaves: the Bolt is
/// exiled without resolving; when Queller dies the Bolt's owner, not
/// Queller's controller, is asked; the Bolt is cast at a chosen target and
/// resolves.
#[test]
fn queller_exiles_a_spell_and_its_owner_casts_it_when_queller_leaves() {
    let (reg, c) = registry();
    let mut s = table(1);
    let bolt = add(&mut s, &reg, 0, c.bolt, Zone::Hand(0));
    let (s, exiled) = quell(s, &reg, &c, bolt, None, |t| hits_player(t, 1));
    assert!(exiled.is_some(), "the Bolt is exiled");
    assert_eq!(s.player(1).life, 20, "the exiled Bolt didn't resolve");

    let mut asked = None;
    let s = bolt_the_queller(s, &reg, &c, &mut |s, p, legal| {
        if let Some((who, ChoiceKind::CastWithoutPaying { .. })) = choice_kind(s) {
            asked = Some(who);
            assert_eq!(p, who);
            let at_p1 = offers(legal).into_iter().find(|a| matches!(a,
                Action::CastSpell { targets, cast_modifier: CastModifier::WithoutPayingManaCost { .. }, .. }
                    if hits_player(targets, 1))).expect("the Bolt can be cast at player 1");
            return legal.iter().find(|a| matches!(a, Action::SubmitResolutionChoice {
                response: ChoiceResponse::CastWithoutPaying { cast: Some(x) }, .. } if x.as_ref() == at_p1)).cloned();
        }
        None
    });
    assert_eq!(asked, Some(0), "the exiled card's owner is asked, not Queller's controller");
    assert_eq!(s.player(1).life, 17, "the free Bolt resolved at player 1");
    assert!(in_zone(&s, c.bolt, Zone::Exile).is_none(), "the Bolt left exile");
}

/// X counts: Disintegrate cast with X = 5 has mana value 6 and can't be
/// targeted (it resolves); cast with X = 3 it has mana value 4 and is exiled.
#[test]
fn chosen_x_counts_toward_quellers_mana_value_limit() {
    let mut reg = CardRegistry::new();
    let c = Cards {
        queller: arcana_cards::inr::spell_queller::register(&mut reg),
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        bears: arcana_cards::lea::grizzly_bears::register(&mut reg),
    };
    let disintegrate = arcana_cards::lea::disintegrate::register(&mut reg);

    let mut s = table(6);
    let spell = add(&mut s, &reg, 0, disintegrate, Zone::Hand(0));
    let (s, exiled) = quell(s, &reg, &c, spell, Some(5), |t| hits_player(t, 1));
    assert!(exiled.is_none(), "X = 5: mana value 6, not exiled");
    assert_eq!(s.player(1).life, 15, "Disintegrate for 5 resolved");

    let mut s = table(4);
    let spell = add(&mut s, &reg, 0, disintegrate, Zone::Hand(0));
    let (s, exiled) = quell(s, &reg, &c, spell, Some(3), |t| hits_player(t, 1));
    assert!(exiled.is_some(), "X = 3: mana value 4, exiled");
    assert_eq!(s.player(1).life, 20);
}

/// Cast without paying its mana cost, an X spell's X is 0 (CR 107.3b).
#[test]
fn free_cast_sets_x_to_zero() {
    let mut reg = CardRegistry::new();
    let c = Cards {
        queller: arcana_cards::inr::spell_queller::register(&mut reg),
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        bears: arcana_cards::lea::grizzly_bears::register(&mut reg),
    };
    let disintegrate = arcana_cards::lea::disintegrate::register(&mut reg);
    let mut s = table(4);
    let spell = add(&mut s, &reg, 0, disintegrate, Zone::Hand(0));
    let (s, _) = quell(s, &reg, &c, spell, Some(3), |t| hits_player(t, 1));
    let mut xs = Vec::new();
    let s = bolt_the_queller(s, &reg, &c, &mut |s, _, legal| {
        if !matches!(choice_kind(s), Some((_, ChoiceKind::CastWithoutPaying { .. }))) { return None; }
        xs = offers(legal).iter().map(|a| match a { Action::CastSpell { x_value, .. } => *x_value, _ => None }).collect();
        legal.iter().find(|a| matches!(a, Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastWithoutPaying { cast: Some(x) }, .. }
            if matches!(x.as_ref(), Action::CastSpell { targets, .. } if hits_player(targets, 1)))).cloned()
    });
    assert!(!xs.is_empty() && xs.iter().all(|x| *x == Some(0)), "every offer casts with X = 0: {xs:?}");
    assert_eq!(s.player(1).life, 20, "Disintegrate for 0 dealt nothing");
}

/// A modal spell is offered in each mode that has a legal target.
#[test]
fn free_cast_offers_each_mode() {
    let mut reg = CardRegistry::new();
    let c = Cards {
        queller: arcana_cards::inr::spell_queller::register(&mut reg),
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        bears: arcana_cards::lea::grizzly_bears::register(&mut reg),
    };
    let abrade = arcana_cards::hou::abrade::register(&mut reg);
    let artifact = arcana_cards::mrd::bonesplitter::register(&mut reg);
    let mut s = table(2);
    let bears = add(&mut s, &reg, 1, c.bears, Zone::Battlefield);
    add(&mut s, &reg, 1, artifact, Zone::Battlefield);
    let spell = add(&mut s, &reg, 0, abrade, Zone::Hand(0));
    let (s, exiled) = quell(s, &reg, &c, spell, None, |t| hits_object(t, bears));
    assert!(exiled.is_some(), "Abrade is exiled");
    let mut modes = Vec::new();
    bolt_the_queller(s, &reg, &c, &mut |s, _, legal| {
        if matches!(choice_kind(s), Some((_, ChoiceKind::CastWithoutPaying { .. }))) && modes.is_empty() {
            modes = offers(legal).iter().filter_map(|a| match a {
                Action::CastSpell { modes, .. } => modes.first().map(|m| m.mode_indices.clone()),
                _ => None,
            }).collect();
        }
        None
    });
    assert!(modes.contains(&vec![0]) && modes.contains(&vec![1]),
        "both modes are offered: {modes:?}");
}

/// Declining leaves the card in exile.
#[test]
fn declined_cast_leaves_the_card_in_exile() {
    let (reg, c) = registry();
    let mut s = table(1);
    let bolt = add(&mut s, &reg, 0, c.bolt, Zone::Hand(0));
    let (s, _) = quell(s, &reg, &c, bolt, None, |t| hits_player(t, 1));
    let mut asked = false;
    let s = bolt_the_queller(s, &reg, &c, &mut |s, _, _| {
        if matches!(choice_kind(s), Some((_, ChoiceKind::CastWithoutPaying { .. }))) { asked = true; }
        None // the driver's default answer to the offer is "don't cast"
    });
    assert!(asked, "the owner was asked");
    assert!(in_zone(&s, c.bolt, Zone::Exile).is_some(), "the Bolt stays exiled");
    assert_eq!(s.player(1).life, 20);
}

/// A spell with no legal target when Queller leaves isn't offered at all.
#[test]
fn spell_with_no_legal_target_is_not_offered() {
    let mut reg = CardRegistry::new();
    let c = Cards {
        queller: arcana_cards::inr::spell_queller::register(&mut reg),
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        bears: arcana_cards::lea::grizzly_bears::register(&mut reg),
    };
    let murder = arcana_cards::isd::murder::register(&mut reg);
    let mut s = table(0);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Black, 3, 0);
    let bears = add(&mut s, &reg, 1, c.bears, Zone::Battlefield);
    let spell = add(&mut s, &reg, 0, murder, Zone::Hand(0));
    let (mut s, exiled) = quell(s, &reg, &c, spell, None, |t| hits_object(t, bears));
    assert!(exiled.is_some(), "Murder is exiled");
    // The Bears leave: once Queller dies too, no creature remains to target.
    s.move_object_to_zone(bears, Zone::Graveyard(1), arcana_core::events::MoveCause::StateBasedAction);
    let mut asked = false;
    let s = bolt_the_queller(s, &reg, &c, &mut |s, _, _| {
        if matches!(choice_kind(s), Some((_, ChoiceKind::CastWithoutPaying { .. }))) { asked = true; }
        None
    });
    assert!(in_zone(&s, c.queller, Zone::Graveyard(1)).is_some(), "Spell Queller died");
    assert!(!asked, "no creature to target: no offer");
    assert!(in_zone(&s, murder, Zone::Exile).is_some(), "Murder stays exiled");
}

/// Exiling isn't countering: a spell that can't be countered is exiled and
/// doesn't resolve.
#[test]
fn queller_exiles_a_spell_that_cant_be_countered() {
    let mut reg = CardRegistry::new();
    let c = Cards {
        queller: arcana_cards::inr::spell_queller::register(&mut reg),
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        bears: arcana_cards::lea::grizzly_bears::register(&mut reg),
    };
    let decay = arcana_cards::rtr::abrupt_decay::register(&mut reg);
    let mut s = table(0);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Black, 1, 0);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Green, 1, 0);
    let bears = add(&mut s, &reg, 1, c.bears, Zone::Battlefield);
    let spell = add(&mut s, &reg, 0, decay, Zone::Hand(0));
    let (s, exiled) = quell(s, &reg, &c, spell, None, |t| hits_object(t, bears));
    assert!(exiled.is_some(), "Abrupt Decay is exiled");
    assert_eq!(s.objects.get(bears).map(|o| o.zone), Some(Zone::Battlefield), "the Bears survive");
}

/// If Spell Queller leaves before its exile resolves, its leave ability finds
/// nothing, and the spell exiled afterwards stays exiled.
#[test]
fn spell_exiled_after_queller_left_stays_exiled() {
    let (reg, c) = registry();
    let mut s = table(2);
    let bolt = add(&mut s, &reg, 0, c.bolt, Zone::Hand(0));
    let killer = add(&mut s, &reg, 0, c.bolt, Zone::Hand(0));
    let queller = add(&mut s, &reg, 1, c.queller, Zone::Hand(1));
    let a = cast_action(&s, &reg, bolt, None, |t| hits_player(t, 1));
    let (s, yld) = step(s, a, &reg);
    let first_bolt = s.stack.last().map(|e| e.id).unwrap();
    let mut answer = queller_answers(queller, first_bolt, &reg);
    let mut asked = false;
    let mut killed = false;
    let s = settle(s, yld, &reg, &mut |s, p, legal| {
        if matches!(choice_kind(s), Some((_, ChoiceKind::CastWithoutPaying { .. }))) { asked = true; }
        // Once Queller's exile ability is on the stack, player 0 bolts Queller.
        let exile_waiting = s.stack.iter().any(|e| e.is_ability());
        let on_field = in_zone(s, c.queller, Zone::Battlefield);
        if p == 0 && !killed && exile_waiting && s.pending_choice.is_none() {
            if let Some(q) = on_field {
                killed = true;
                return Some(cast_action(s, &reg, killer, None, |t| hits_object(t, q)));
            }
        }
        answer(s, p, legal)
    });
    assert!(killed, "Queller was bolted with its exile ability waiting");
    assert!(!asked, "the leave ability resolved first and offered nothing");
    assert!(in_zone(&s, c.bolt, Zone::Exile).is_some(), "the first Bolt is exiled for good");
    assert_eq!(s.player(1).life, 20);
}

/// Card-type timing doesn't apply: a sorcery is cast on its owner's
/// opponent's turn, during the leave ability's resolution.
#[test]
fn free_cast_ignores_card_type_timing() {
    let mut reg = CardRegistry::new();
    let c = Cards {
        queller: arcana_cards::inr::spell_queller::register(&mut reg),
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        bears: arcana_cards::lea::grizzly_bears::register(&mut reg),
    };
    let spike = arcana_cards::uma::lava_spike::register(&mut reg);
    let mut s = table(1);
    let spell = add(&mut s, &reg, 0, spike, Zone::Hand(0));
    let (mut s, exiled) = quell(s, &reg, &c, spell, None, |t| hits_player(t, 1));
    assert!(exiled.is_some(), "Lava Spike is exiled");
    // Player 1's turn now.
    s.turn.active_player = 1;
    let mut cast = false;
    let s = bolt_the_queller(s, &reg, &c, &mut |s, _, legal| {
        if !matches!(choice_kind(s), Some((_, ChoiceKind::CastWithoutPaying { .. }))) { return None; }
        cast = true;
        legal.iter().find(|a| matches!(a, Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastWithoutPaying { cast: Some(x) }, .. }
            if matches!(x.as_ref(), Action::CastSpell { targets, .. } if hits_player(targets, 1)))).cloned()
    });
    assert!(cast, "the sorcery was offered on the opponent's turn");
    assert_eq!(s.player(1).life, 17, "Lava Spike resolved");
}

/// Kicker is an additional cost, still offered and paid when the mana cost
/// isn't (CR 118.9d): both the unkicked and the kicked cast are offered.
#[test]
fn free_cast_offers_kicker() {
    let mut reg = CardRegistry::new();
    let c = Cards {
        queller: arcana_cards::inr::spell_queller::register(&mut reg),
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        bears: arcana_cards::lea::grizzly_bears::register(&mut reg),
    };
    let burst = arcana_cards::zen::burst_lightning::register(&mut reg);
    let mut s = table(1);
    let spell = add(&mut s, &reg, 0, burst, Zone::Hand(0));
    let (mut s, exiled) = quell(s, &reg, &c, spell, None, |t| hits_player(t, 1));
    assert!(exiled.is_some(), "Burst Lightning is exiled");
    s.player_mut(0).mana_pool.add_mana(ManaColor::Red, 4, 0);
    let mut kinds = (false, false);
    let s = bolt_the_queller(s, &reg, &c, &mut |s, _, legal| {
        if !matches!(choice_kind(s), Some((_, ChoiceKind::CastWithoutPaying { .. }))) { return None; }
        let is_kicked = |a: &Action| matches!(a, Action::CastSpell { additional_costs, .. }
            if additional_costs.iter().any(|c| matches!(c, arcana_core::actions::AdditionalCostPayment::Kicker)));
        kinds = (offers(legal).iter().any(|a| !is_kicked(a)), offers(legal).iter().any(|a| is_kicked(a)));
        legal.iter().find(|a| matches!(a, Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastWithoutPaying { cast: Some(x) }, .. }
            if is_kicked(x) && matches!(x.as_ref(), Action::CastSpell { targets, .. } if hits_player(targets, 1)))).cloned()
    });
    assert_eq!(kinds, (true, true), "unkicked and kicked casts are both offered");
    assert_eq!(s.player(1).life, 16, "the kicked Burst Lightning dealt 4");
}

/// A1.5p: Venser bounces a spell to its owner's hand; the spell's stack
/// entry goes with it, and the spell doesn't resolve.
#[test]
fn a_spell_bounced_off_the_stack_does_not_resolve() {
    let mut reg = CardRegistry::new();
    let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
    let venser = arcana_cards::s2x2::venser_shaper_savant::register(&mut reg);
    let mut s = table(1);
    s.player_mut(1).mana_pool.add_mana(ManaColor::Blue, 2, 0);
    let spell = add(&mut s, &reg, 0, bolt, Zone::Hand(0));
    let v = add(&mut s, &reg, 1, venser, Zone::Hand(1));
    let a = cast_action(&s, &reg, spell, None, |t| hits_player(t, 1));
    let (s, yld) = step(s, a, &reg);
    let on_stack = s.stack.last().map(|e| e.id).unwrap();
    let s = settle(s, yld, &reg, &mut queller_answers(v, on_stack, &reg));
    assert!(in_zone(&s, bolt, Zone::Hand(0)).is_some(), "the Bolt is back in hand");
    assert_eq!(s.player(1).life, 20, "the bounced Bolt didn't resolve");
}

/// Spell Queller's second ability triggers when it leaves the battlefield by
/// any route, not only when it dies: bounced, it still offers the cast.
#[test]
fn bouncing_queller_offers_the_cast() {
    let mut reg = CardRegistry::new();
    let c = Cards {
        queller: arcana_cards::inr::spell_queller::register(&mut reg),
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        bears: arcana_cards::lea::grizzly_bears::register(&mut reg),
    };
    let unsummon = arcana_cards::fdn::unsummon::register(&mut reg);
    let mut s = table(1);
    let bolt = add(&mut s, &reg, 0, c.bolt, Zone::Hand(0));
    let (mut s, exiled) = quell(s, &reg, &c, bolt, None, |t| hits_player(t, 1));
    assert!(exiled.is_some(), "the Bolt is exiled");
    let queller = in_zone(&s, c.queller, Zone::Battlefield).expect("Queller is on the battlefield");
    let bounce = add(&mut s, &reg, 0, unsummon, Zone::Hand(0));
    s.player_mut(0).mana_pool.add_mana(ManaColor::Blue, 1, 0);
    s.priority.give_to(0);
    let a = cast_action(&s, &reg, bounce, None, |t| hits_object(t, queller));
    let (s, yld) = step(s, a, &reg);
    let mut asked = false;
    let s = settle(s, yld, &reg, &mut |s, _, legal| {
        if !matches!(choice_kind(s), Some((_, ChoiceKind::CastWithoutPaying { .. }))) { return None; }
        asked = true;
        legal.iter().find(|a| matches!(a, Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastWithoutPaying { cast: Some(x) }, .. }
            if matches!(x.as_ref(), Action::CastSpell { targets, .. } if hits_player(targets, 1)))).cloned()
    });
    assert!(in_zone(&s, c.queller, Zone::Hand(1)).is_some(), "Queller is back in hand");
    assert!(asked, "leaving by bounce offers the cast");
    assert_eq!(s.player(1).life, 17, "the free Bolt resolved");
}

/// A free cast may cast any face of a split card: Fire // Ice cast as Ice
/// (its right half) taps its target and draws, and cast as Fire it has Fire's
/// mana value, 2, on the stack, not the whole card's 4.
#[test]
fn free_cast_of_a_split_card_casts_either_half_as_that_half() {
    let mut reg = CardRegistry::new();
    let c = Cards {
        queller: arcana_cards::inr::spell_queller::register(&mut reg),
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        bears: arcana_cards::lea::grizzly_bears::register(&mut reg),
    };
    let fire_ice = arcana_cards::apc::fire_ice::register(&mut reg);
    let mut s = table(2);
    let bears = add(&mut s, &reg, 1, c.bears, Zone::Battlefield);
    for _ in 0..2 {
        let id = add(&mut s, &reg, 0, c.bears, Zone::Library(0));
        s.player_mut(0).library_top_to_bottom.push(id);
    }
    let spell = add(&mut s, &reg, 0, fire_ice, Zone::Hand(0));
    let (s, exiled) = quell(s, &reg, &c, spell, None, |t| hits_player(t, 1));
    let exiled = exiled.expect("Fire // Ice is exiled");
    assert_eq!(s.mana_value_of(s.objects.get(exiled).unwrap()), 4, "in exile it is the whole card");
    let faces_in = |legal: &[Action]| -> Vec<arcana_core::actions::CastFace> {
        offers(legal).iter().filter_map(|a| match a {
            Action::CastSpell { cast_modifier: CastModifier::WithoutPayingManaCost { face }, .. } => Some(*face),
            _ => None,
        }).collect()
    };

    // As Fire: the stack entry's object has Fire's mana value.
    let mut fire_mv = None;
    let probe = s.clone();
    let mut probe_state = None;
    bolt_the_queller(probe, &reg, &c, &mut |s, _, legal| {
        if !matches!(choice_kind(s), Some((_, ChoiceKind::CastWithoutPaying { .. }))) {
            if fire_mv.is_none() && probe_state.is_none() { return None; }
            if fire_mv.is_none() {
                if let Some(e) = s.stack.iter().find(|e| s.objects.get(e.id).is_some_and(|o| o.card_id == fire_ice)) {
                    fire_mv = Some(s.mana_value_of(s.objects.get(e.id).unwrap()));
                }
            }
            return None;
        }
        probe_state = Some(());
        legal.iter().find(|a| matches!(a, Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastWithoutPaying { cast: Some(x) }, .. }
            if matches!(x.as_ref(), Action::CastSpell { cast_modifier: CastModifier::WithoutPayingManaCost {
                face: arcana_core::actions::CastFace::Main }, .. }))).cloned()
    });
    assert_eq!(fire_mv, Some(2), "Fire on the stack has mana value 2");

    // As Ice: taps the Bears and draws.
    let mut faces = Vec::new();
    let hand_before = s.objects.objects_in_zone(Zone::Hand(0)).count();
    let s = bolt_the_queller(s, &reg, &c, &mut |s, _, legal| {
        if !matches!(choice_kind(s), Some((_, ChoiceKind::CastWithoutPaying { .. }))) { return None; }
        faces = faces_in(legal);
        legal.iter().find(|a| matches!(a, Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastWithoutPaying { cast: Some(x) }, .. }
            if matches!(x.as_ref(), Action::CastSpell { targets, cast_modifier: CastModifier::WithoutPayingManaCost {
                face: arcana_core::actions::CastFace::SplitRight }, .. } if hits_object(targets, bears)))).cloned()
    });
    assert!(faces.contains(&arcana_core::actions::CastFace::Main)
        && faces.contains(&arcana_core::actions::CastFace::SplitRight), "both halves offered: {faces:?}");
    assert!(s.objects.get(bears).unwrap().is_tapped(), "Ice tapped the Bears");
    assert_eq!(s.objects.objects_in_zone(Zone::Hand(0)).count(), hand_before + 1, "Ice drew a card");
    assert!(s.objects.get(exiled).is_none() && in_zone(&s, fire_ice, Zone::Graveyard(0)).is_some(),
        "Fire // Ice resolved to the graveyard");
}

/// A free cast may cast an Adventure: Bonecrusher Giant's Stomp, cast free,
/// deals its 2 and the card goes on an adventure (CR 715.4).
#[test]
fn free_cast_of_an_adventurer_offers_the_adventure() {
    let mut reg = CardRegistry::new();
    let c = Cards {
        queller: arcana_cards::inr::spell_queller::register(&mut reg),
        bolt: arcana_cards::lea::lightning_bolt::register(&mut reg),
        bears: arcana_cards::lea::grizzly_bears::register(&mut reg),
    };
    let giant = arcana_cards::eld::bonecrusher_giant::register(&mut reg);
    let mut s = table(3);
    let spell = add(&mut s, &reg, 0, giant, Zone::Hand(0));
    let (s, exiled) = quell(s, &reg, &c, spell, None, |_| true);
    assert!(exiled.is_some(), "the Giant is exiled");
    let s = bolt_the_queller(s, &reg, &c, &mut |s, _, legal| {
        if !matches!(choice_kind(s), Some((_, ChoiceKind::CastWithoutPaying { .. }))) { return None; }
        legal.iter().find(|a| matches!(a, Action::SubmitResolutionChoice {
            response: ChoiceResponse::CastWithoutPaying { cast: Some(x) }, .. }
            if matches!(x.as_ref(), Action::CastSpell { targets, cast_modifier: CastModifier::WithoutPayingManaCost {
                face: arcana_core::actions::CastFace::Adventure }, .. } if hits_player(targets, 1)))).cloned()
    });
    assert_eq!(s.player(1).life, 18, "Stomp dealt 2");
    let card = s.objects.iter().find(|o| o.card_id == giant).expect("the Giant card");
    assert!(card.zone.is_exile() && card.adventure_exile_pending, "the Giant is on an adventure");
}

/// CR 601.2f: a cost reduction applies to the total cost, kicker included.
/// With a one-mana reduction, Burst Lightning's kicked cast costs {3} when
/// cast free and {3}{R} from hand.
#[test]
fn cost_reduction_applies_after_kicker() {
    use arcana_core::layers::{ContinuousEffect, Duration};
    use arcana_core::targets::{ControllerConstraint, ObjectFilter};
    let (mut reg, c) = registry();
    let burst = arcana_cards::zen::burst_lightning::register(&mut reg);
    let kicked = |a: &Action| matches!(a, Action::CastSpell { additional_costs, .. }
        if additional_costs.iter().any(|p| matches!(p, arcana_core::actions::AdditionalCostPayment::Kicker)));

    let mut s = table(3);
    let source = add(&mut s, &reg, 0, c.bears, Zone::Battlefield);
    s.add_continuous_effect(ContinuousEffect::spell_cost_modifier(source,
        ObjectFilter::default(), ControllerConstraint::You, -1, Duration::WhileSourceOnBattlefield));
    let exiled = add(&mut s, &reg, 0, burst, Zone::Exile);
    let free = arcana_core::legal_actions::casts_without_paying(&s, &reg, 0, exiled);
    assert!(free.iter().any(kicked), "free: the {{4}} kicker, reduced to {{3}}, fits three mana");

    let mut s = table(4);
    let source = add(&mut s, &reg, 0, c.bears, Zone::Battlefield);
    s.add_continuous_effect(ContinuousEffect::spell_cost_modifier(source,
        ObjectFilter::default(), ControllerConstraint::You, -1, Duration::WhileSourceOnBattlefield));
    let in_hand = add(&mut s, &reg, 0, burst, Zone::Hand(0));
    assert!(legal_actions(&s, &reg).iter().any(|a| kicked(a)
        && matches!(a, Action::CastSpell { object_id, .. } if *object_id == in_hand)),
        "from hand: {{R}} plus the {{4}} kicker, reduced to {{3}}{{R}}, fits four mana");
}
