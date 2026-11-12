//! Pending entries retain their identities in the decider's sampled worlds.
use arcana_ai::information_set::{determinize, project, sample_world};
use arcana_core::actions::{Action, ChoiceContext, ChoiceFollowUp, ChoiceKind, ChoiceResponse};
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
fn add(
    s: &mut GameState,
    reg: &CardRegistry,
    owner: PlayerId,
    card: CardId,
    zone: Zone,
) -> ObjectId {
    let id = s.allocate_object_id();
    s.objects.insert(GameObject::new(
        id,
        owner,
        zone,
        card,
        reg.get(card).unwrap().initial_characteristics().clone(),
    ));
    if let Zone::Library(p) = zone {
        s.player_mut(p).library_top_to_bottom.push(id);
    }
    id
}
fn pass_to_choice(mut s: GameState, reg: &CardRegistry) -> GameState {
    for _ in 0..12 {
        if s.pending_choice.is_some() {
            return s;
        }
        let pass = legal_actions(&s, reg)
            .into_iter()
            .find(|a| a.is_pass())
            .expect("priority");
        s = step(s, pass, reg).0;
    }
    panic!("no choice reached");
}
fn pick(s: GameState, reg: &CardRegistry, ids: &[ObjectId]) -> GameState {
    let action = legal_actions(&s, reg).into_iter().find(|a| matches!(a,
        Action::SubmitResolutionChoice { response: ChoiceResponse::PickCards { picked }, .. } if picked == ids)).expect("legal card selection");
    step(s, action, reg).0
}
fn answer(s: GameState, reg: &CardRegistry, pay: bool) -> GameState {
    let action = legal_actions(&s, reg).into_iter().find(|a| matches!(a,
        Action::SubmitResolutionChoice { response: ChoiceResponse::OptionalCost { pay: p }, .. } if *p == pay)).expect("offered payment answer");
    step(s, action, reg).0
}

#[test]
fn fetched_shock_keeps_its_identity_in_both_samplers() {
    for player in [0, 1] {
        let mut reg = CardRegistry::new();
        let heath = arcana_cards::mh3::windswept_heath::register(&mut reg);
        let pool = arcana_cards::gtc::breeding_pool::register(&mut reg);
        let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
        let mut s = table(player);
        let source = add(&mut s, &reg, player, heath, Zone::Battlefield);
        s.objects.get_mut(source).unwrap().status.summoning_sick = false;
        let target = add(&mut s, &reg, player, pool, Zone::Library(player));
        let others: Vec<_> = (0..6)
            .map(|_| add(&mut s, &reg, player, bears, Zone::Library(player)))
            .collect();
        for _ in 0..2 {
            add(&mut s, &reg, player, bears, Zone::Hand(player));
        }
        let action = legal_actions(&s, &reg)
            .into_iter()
            .find(|a| {
                matches!(a,
            Action::ActivateAbility { source: id, .. } if *id == source)
            })
            .unwrap();
        let s = pick(
            pass_to_choice(step(s, action, &reg).0, &reg),
            &reg,
            &[target],
        );
        assert!(matches!(s.pending_choice.as_ref().unwrap().context,
            ChoiceContext::EnteringBattlefield { object_id } if object_id == target));
        let obs = project(&s, player);
        assert!(
            !obs.anonymous_ids.contains(&target),
            "the decider knows the fetched land"
        );
        assert!(
            others.iter().all(|id| obs.anonymous_ids.contains(id)),
            "unselected library cards stay hidden"
        );
        assert!(
            project(&s, 1 - player).anonymous_ids.contains(&target),
            "no new disclosure to the opponent"
        );
        let mut decks = vec![vec![], vec![]];
        decks[player as usize] = vec![
            heath, pool, bears, bears, bears, bears, bears, bears, bears, bears,
        ];
        for seed in 0..40 {
            for world in [
                determinize(&obs, &decks, &reg, seed),
                sample_world(&obs, &decks, &reg, seed, seed),
            ] {
                let pending = world.objects.get(target).unwrap();
                assert_eq!(
                    (pending.card_id, pending.zone),
                    (pool, Zone::Library(player))
                );
                for pay in [false, true] {
                    let resolved = answer(world.clone(), &reg, pay);
                    let entered: Vec<_> = resolved
                        .objects
                        .objects_in_zone(Zone::Battlefield)
                        .collect();
                    assert_eq!(entered.len(), 1);
                    assert_eq!((entered[0].card_id, entered[0].is_tapped()), (pool, !pay));
                    assert_eq!(resolved.player(player).life, if pay { 17 } else { 19 });
                }
            }
        }
    }
}

#[test]
fn sampling_keeps_every_selected_card_in_a_waiting_entry_batch() {
    let mut reg = CardRegistry::new();
    let pool = arcana_cards::gtc::breeding_pool::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = table(0);
    let ready = add(&mut s, &reg, 0, bears, Zone::Library(0));
    let first = add(&mut s, &reg, 0, pool, Zone::Library(0));
    let later = add(&mut s, &reg, 0, pool, Zone::Library(0));
    let hidden: Vec<_> = (0..6)
        .map(|_| add(&mut s, &reg, 0, bears, Zone::Library(0)))
        .collect();
    let selected = vec![ready, first, later];
    s.pending_choice_follow_up = Some(ChoiceFollowUp::MoveToBattlefield {
        controller: 0,
        tapped: false,
        shuffle_library_owner: None,
    });
    s.push_pending_choice(
        0,
        ChoiceContext::ResolvingStack(999),
        ChoiceKind::PickCards {
            candidates: selected.clone(),
            min: 3,
            max: 3,
        },
    );
    let s = pick(s, &reg, &selected);
    let decks = vec![
        vec![bears, pool, pool, bears, bears, bears, bears, bears, bears],
        vec![],
    ];
    // Check both the first prompt and the interval after the first payment.
    for first_paid in [false, true] {
        let s = if first_paid {
            answer(s.clone(), &reg, true)
        } else {
            s.clone()
        };
        let obs = project(&s, 0);
        assert!(selected.iter().all(|id| !obs.anonymous_ids.contains(id)));
        assert!(hidden.iter().all(|id| obs.anonymous_ids.contains(id)));
        for seed in 0..40 {
            for mut world in [
                determinize(&obs, &decks, &reg, seed),
                sample_world(&obs, &decks, &reg, seed, seed),
            ] {
                for (id, card) in [(ready, bears), (first, pool), (later, pool)] {
                    let obj = world.objects.get(id).unwrap();
                    assert_eq!((obj.card_id, obj.zone), (card, Zone::Library(0)));
                }
                if !first_paid {
                    world = answer(world, &reg, true);
                }
                world = answer(world, &reg, false);
                let entered: Vec<_> = world.objects.objects_in_zone(Zone::Battlefield).collect();
                assert_eq!(entered.iter().filter(|o| o.card_id == pool).count(), 2);
                assert_eq!(entered.iter().filter(|o| o.card_id == bears).count(), 1);
                assert_eq!(entered.iter().filter(|o| o.is_tapped()).count(), 1);
            }
        }
    }
}
