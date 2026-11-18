//! A1.5cg: `is_mana_activation`, the read-only accessor a rollout rule uses
//! to tell a mana ability from every other action. It reads the ability the
//! action's flat index names in each of the index's three tiers: a registry
//! ability (a Mountain's), one the object carries itself (a Treasure's) and
//! one a continuous effect grants (Rishkar, Peema Renegade's, on a creature
//! with a counter).
use arcana_core::actions::Action;
use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::engine::step;
use arcana_core::events::MoveCause;
use arcana_core::legal_actions::{is_mana_activation, legal_actions};
use arcana_core::objects::{GameObject, ObjectId};
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CounterKind, PlayerId};
use arcana_core::zones::Zone;

fn registry() -> CardRegistry {
    let mut r = CardRegistry::new();
    arcana_cards::register_all::register_all(&mut r);
    r
}

/// `p`'s precombat main phase, `p` with priority.
fn table(p: PlayerId) -> GameState {
    let mut s = GameState::new(2, 20261116);
    s.turn.active_player = p;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(p);
    s
}

/// Onto the battlefield through the zone mover, its entry old news: no
/// entry trigger waits and the permanent is not summoning sick.
fn enter(s: &mut GameState, r: &CardRegistry, p: PlayerId, name: &str) -> ObjectId {
    let card = r.card_id_by_name(name).unwrap_or_else(|| panic!("unregistered: {name}"));
    let id = s.allocate_object_id();
    s.objects.insert(GameObject::new(id, p, Zone::Hand(p), card,
        r.get(card).unwrap().initial_characteristics().clone()));
    let id = s.move_object_to_zone(id, Zone::Battlefield, MoveCause::SpellResolution)
        .expect("entered");
    s.trigger_event_cursor = s.event_log.len();
    s.objects.get_mut(id).unwrap().status.summoning_sick = false;
    id
}

fn activations(legal: &[Action], id: ObjectId) -> Vec<&Action> {
    legal.iter()
        .filter(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == id))
        .collect()
}

#[test]
fn a_mana_ability_is_told_apart_in_every_tier_of_the_ability_index() {
    let r = registry();
    for p in [0, 1] {
        let mut s = table(p);
        let mountain = enter(&mut s, &r, p, "Mountain");
        let sorcerer = enter(&mut s, &r, p, "Prodigal Sorcerer");
        enter(&mut s, &r, p, "Rishkar, Peema Renegade");
        let bears = enter(&mut s, &r, p, "Grizzly Bears");
        s.objects.get_mut(bears).unwrap().add_counters(CounterKind::PlusOnePlusOne, 1);
        Effect::CreateCommodityToken { controller: p, kind: CommodityToken::Treasure, count: 1 }
            .execute(&mut s);
        let treasure = s.objects.objects_in_zone(Zone::Battlefield)
            .find(|o| o.commodity == Some(CommodityToken::Treasure)).expect("a Treasure").id;

        // Each permanent's ability sits in the tier the test means it to.
        let tiers = |id: ObjectId| {
            let o = s.objects.get(id).unwrap();
            (r.get(o.card_id).map_or(0, |d| d.activated_abilities.len()),
             o.intrinsic_activated_abilities.len(),
             s.granted_activated_for(id).len())
        };
        assert_eq!(tiers(mountain), (1, 0, 0), "the Mountain's ability is its card's");
        assert_eq!(tiers(treasure).1, 1, "the Treasure carries its own");
        assert_eq!(tiers(bears), (0, 0, 1), "the Bears' is Rishkar's grant");

        let legal = legal_actions(&s, &r);
        for (id, what, mana) in [
            (mountain, "a registry mana ability", true),
            (treasure, "an intrinsic mana ability", true),
            (bears, "a granted mana ability", true),
            (sorcerer, "an ability that is not a mana ability", false),
        ] {
            let offered = activations(&legal, id);
            assert!(!offered.is_empty(), "seat {p}: {what} is offered");
            for a in offered {
                assert_eq!(is_mana_activation(&s, a, &r), mana, "seat {p}: {what}: {a:?}");
            }
        }
        // No other action is one.
        assert!(legal.iter().any(|a| a.is_pass()));
        for a in legal.iter().filter(|a| !matches!(a, Action::ActivateAbility { .. })) {
            assert!(!is_mana_activation(&s, a, &r), "seat {p}: {a:?}");
        }
        // An activation whose source is gone names no ability, and one whose
        // index is past the object's last ability names none either.
        let spend = activations(&legal, treasure)[0].clone();
        let after = step(s.clone(), spend.clone(), &r).0;
        assert!(!is_mana_activation(&after, &spend, &r), "seat {p}: the Treasure was sacrificed");
        let Action::ActivateAbility { targets, mana_payment, additional_costs, .. } =
            activations(&legal, mountain)[0].clone() else { unreachable!() };
        let past = Action::ActivateAbility {
            source: mountain, ability_index: 1, targets, mana_payment, additional_costs };
        assert!(!is_mana_activation(&s, &past, &r), "seat {p}: the Mountain has one ability");
    }
}
