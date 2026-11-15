//! A card Bomat Courier exiles face down is hidden from both players, its
//! owner included (CR 406.3): each player's projection anonymizes it, and
//! each player's sampled worlds deal that slot from the cards that player
//! cannot see, keeping it face down and with the Courier that exiled it.
use std::collections::HashSet;

use arcana_ai::information_set::{determinize, project, sample_world};
use arcana_core::effects::Effect;
use arcana_core::objects::{GameObject, ObjectId};
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, PlayerId};
use arcana_core::zones::Zone;

fn add(s: &mut GameState, reg: &CardRegistry, owner: PlayerId, card: CardId, zone: Zone)
    -> ObjectId
{
    let id = s.allocate_object_id();
    s.objects.insert(GameObject::new(id, owner, zone, card,
        reg.get(card).unwrap().initial_characteristics().clone()));
    if let Zone::Library(p) = zone {
        s.player_mut(p).library_top_to_bottom.push(id);
    }
    id
}

#[test]
fn a_face_down_exiled_card_is_hidden_in_both_players_sampled_worlds() {
    for owner in [0, 1] {
        let mut reg = CardRegistry::new();
        let courier = arcana_cards::kld::bomat_courier::register(&mut reg);
        let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
        let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
        let mut s = GameState::new(2, 0);
        s.turn.active_player = owner;
        s.turn.phase = Phase::PreCombatMain;
        s.turn.step = Step::Main;
        s.priority.give_to(owner);
        let on_board = add(&mut s, &reg, owner, courier, Zone::Battlefield);
        // The top card is the one Bolt; four Bears lie under it.
        add(&mut s, &reg, owner, bolt, Zone::Library(owner));
        for _ in 0..4 { add(&mut s, &reg, owner, bears, Zone::Library(owner)); }
        let mut decks = vec![vec![], vec![]];
        decks[owner as usize] = vec![courier, bolt, bears, bears, bears, bears];

        Effect::ExileTopFaceDownLinked { player: owner, link_to: on_board }.execute(&mut s);
        let exiled: Vec<ObjectId> = s.objects.objects_in_zone(Zone::Exile).map(|o| o.id).collect();
        assert_eq!(exiled.len(), 1);
        let slot = exiled[0];
        assert_eq!(s.objects.get(slot).unwrap().card_id, bolt, "the true state holds the Bolt");

        for viewer in [0, 1] {
            let obs = project(&s, viewer);
            assert!(obs.anonymous_ids.contains(&slot),
                "owner {owner}: seat {viewer} cannot see it");
            let hidden = obs.state.objects.get(slot).unwrap();
            assert_eq!(hidden.characteristics.name, Default::default(), "no name");
            assert!(hidden.characteristics.mana_cost.is_none());
            assert!(hidden.status.face_down, "it is publicly face down");
            assert_eq!(hidden.linked_exile_source, Some(on_board), "and publicly with the Courier");

            let mut dealt: HashSet<CardId> = HashSet::new();
            for seed in 0..40 {
                for world in [
                    determinize(&obs, &decks, &reg, seed),
                    sample_world(&obs, &decks, &reg, seed, seed),
                ] {
                    let card = world.objects.get(slot).expect("the slot survives");
                    assert_eq!(card.zone, Zone::Exile);
                    assert!(card.status.face_down, "seat {viewer}, seed {seed}: still face down");
                    assert_eq!(card.linked_exile_source, Some(on_board), "still the Courier's");
                    assert_ne!(card.characteristics.name, Default::default(), "a real card");
                    dealt.insert(card.card_id);
                    // The owner's cards are their decklist, each exactly once.
                    let mut cards: Vec<CardId> = world.objects.iter()
                        .filter(|o| o.owner == owner).map(|o| o.card_id).collect();
                    cards.sort();
                    let mut want = decks[owner as usize].clone();
                    want.sort();
                    assert_eq!(cards, want, "seat {viewer}, seed {seed}");
                }
            }
            assert_eq!(dealt, HashSet::from([bolt, bears]),
                "owner {owner}, seat {viewer}: the slot is any unseen card, not the true one alone");
        }
    }
}
