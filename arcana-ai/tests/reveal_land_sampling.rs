//! A card revealed for a reveal land is known to both players, so each
//! player's sampled worlds keep its identity. A1.5bj's 40-world check is
//! the model (`entry_choice_sampling.rs`).
use arcana_ai::information_set::{determinize, project, sample_world};
use arcana_core::actions::{Action, ChoiceResponse};
use arcana_core::engine::step;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::{GameObject, ObjectId};
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, PlayerId};
use arcana_core::zones::Zone;

fn table(player: PlayerId) -> GameState {
    let mut s = GameState::new(2, 0);
    s.turn.active_player = player;
    s.turn.phase = Phase::PreCombatMain;
    s.turn.step = Step::Main;
    s.priority.give_to(player);
    s
}

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

fn reveal(s: GameState, reg: &CardRegistry, card: Option<ObjectId>) -> GameState {
    let wanted: Vec<ObjectId> = card.into_iter().collect();
    let action = legal_actions(&s, reg).into_iter().find(|a| matches!(a,
        Action::SubmitResolutionChoice { response: ChoiceResponse::PickCards { picked }, .. }
            if *picked == wanted)).expect("offered reveal answer");
    step(s, action, reg).0
}

#[test]
fn a_revealed_card_keeps_its_identity_in_both_players_sampled_worlds() {
    for player in [0, 1] {
        let opponent = 1 - player;
        let mut reg = CardRegistry::new();
        let town = arcana_cards::cmm::port_town::register(&mut reg);
        let plains = arcana_cards::lea::plains::register(&mut reg);
        let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
        let mut s = table(player);
        let shown = add(&mut s, &reg, player, plains, Zone::Hand(player));
        let kept: Vec<_> = (0..3)
            .map(|_| add(&mut s, &reg, player, bears, Zone::Hand(player)))
            .collect();
        let land = add(&mut s, &reg, player, town, Zone::Hand(player));
        for _ in 0..6 {
            add(&mut s, &reg, player, bears, Zone::Library(player));
        }
        let play = legal_actions(&s, &reg).into_iter()
            .find(|a| matches!(a, Action::PlayLand { object_id, .. } if *object_id == land))
            .unwrap();
        let waiting = step(s, play, &reg).0;
        let mut decks = vec![vec![], vec![]];
        decks[player as usize] = vec![
            town, plains, bears, bears, bears, bears, bears, bears, bears, bears, bears,
        ];

        // While the choice waits, the decider's worlds keep the hand the
        // options were listed from, and either answer applies in them.
        let obs = project(&waiting, player);
        assert!(project(&waiting, opponent).anonymous_ids.contains(&shown),
            "nothing is shown before the answer");
        for seed in 0..40 {
            for world in [
                determinize(&obs, &decks, &reg, seed),
                sample_world(&obs, &decks, &reg, seed, seed),
            ] {
                for (answer, tapped) in [(Some(shown), false), (None, true)] {
                    let resolved = reveal(world.clone(), &reg, answer);
                    let entered: Vec<_> =
                        resolved.objects.objects_in_zone(Zone::Battlefield).collect();
                    assert_eq!(entered.len(), 1);
                    assert_eq!((entered[0].card_id, entered[0].is_tapped()), (town, tapped));
                }
            }
        }

        // After the reveal the opponent knows that one card, and only it.
        let s = reveal(waiting, &reg, Some(shown));
        for viewer in [player, opponent] {
            let obs = project(&s, viewer);
            assert!(!obs.anonymous_ids.contains(&shown), "seat {viewer} knows the Plains");
            if viewer == opponent {
                assert!(kept.iter().all(|id| obs.anonymous_ids.contains(id)),
                    "the rest of the hand stays hidden");
            }
            for seed in 0..40 {
                for world in [
                    determinize(&obs, &decks, &reg, seed),
                    sample_world(&obs, &decks, &reg, seed, seed),
                ] {
                    let card = world.objects.get(shown).unwrap();
                    assert_eq!((card.card_id, card.zone), (plains, Zone::Hand(player)),
                        "seat {viewer}, seed {seed}");
                }
            }
        }

        // Declining shows nothing: the opponent's worlds may hold any card
        // from the list in that slot.
        let waiting_again = {
            let mut s = table(player);
            let shown = add(&mut s, &reg, player, plains, Zone::Hand(player));
            let land = add(&mut s, &reg, player, town, Zone::Hand(player));
            let play = legal_actions(&s, &reg).into_iter()
                .find(|a| matches!(a, Action::PlayLand { object_id, .. } if *object_id == land))
                .unwrap();
            (step(s, play, &reg).0, shown)
        };
        let declined = reveal(waiting_again.0, &reg, None);
        assert!(project(&declined, opponent).anonymous_ids.contains(&waiting_again.1));
    }
}
