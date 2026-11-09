//! A1.5bj: entry choices precede entry, for land plays and effect routes.
use arcana_core::actions::{Action, ChoiceContext, ChoiceFollowUp, ChoiceKind, ChoiceResponse};
use arcana_core::effects::{Condition, Effect};
use arcana_core::engine::step;
use arcana_core::events::GameEvent;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::{Characteristics, GameObject, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec, SpellAbilityDef};
use arcana_core::state::GameState;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, CounterKind, ManaColor, PlayerId, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::EngineYield;

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
fn pay_actions(s: &GameState, reg: &CardRegistry) -> Vec<bool> {
    legal_actions(s, reg)
        .iter()
        .filter_map(|a| match a {
            Action::SubmitResolutionChoice {
                response: ChoiceResponse::OptionalCost { pay },
                ..
            } => Some(*pay),
            _ => None,
        })
        .collect()
}
fn answer(s: GameState, reg: &CardRegistry, pay: bool) -> (GameState, EngineYield) {
    let action = legal_actions(&s, reg).into_iter().find(|a| matches!(a,
        Action::SubmitResolutionChoice { response: ChoiceResponse::OptionalCost { pay: p }, .. } if *p == pay)).expect("offered payment answer");
    step(s, action, reg)
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
fn play(s: GameState, reg: &CardRegistry, id: ObjectId) -> GameState {
    let action = legal_actions(&s, reg)
        .into_iter()
        .find(|a| {
            matches!(a,
        Action::PlayLand { object_id, .. } if *object_id == id)
        })
        .expect("legal land play");
    step(s, action, reg).0
}
fn cast(s: GameState, reg: &CardRegistry, id: ObjectId) -> GameState {
    let action = legal_actions(&s, reg)
        .into_iter()
        .find(|a| {
            matches!(a,
        Action::CastSpell { object_id, .. } if *object_id == id)
        })
        .expect("legal cast");
    pass_to_choice(step(s, action, reg).0, reg)
}
fn battlefield_id(s: &GameState, card: CardId) -> ObjectId {
    s.objects
        .objects_in_zone(Zone::Battlefield)
        .find(|o| o.card_id == card)
        .unwrap()
        .id
}
fn assert_waiting(s: &GameState, id: ObjectId, from: Zone, player: PlayerId) {
    assert_eq!(
        s.objects.get(id).unwrap().zone,
        from,
        "entry has not happened"
    );
    let pending = s.pending_choice.as_ref().expect("entry payment");
    assert_eq!(pending.choosing_player, player);
    assert!(
        matches!(pending.context, ChoiceContext::EnteringBattlefield { object_id } if object_id == id)
    );
    assert!(!s
        .event_log
        .iter()
        .any(|e| matches!(e, GameEvent::ZoneChange { object_id, .. } if *object_id == id)));
}

#[test]
fn played_shock_offers_pay_and_decline_to_each_seat_before_entry() {
    for player in [0, 1] {
        for pay in [false, true] {
            let mut reg = CardRegistry::new();
            let pool = arcana_cards::gtc::breeding_pool::register(&mut reg);
            let mut s = table(player);
            let id = add(&mut s, &reg, player, pool, Zone::Hand(player));
            let s = play(s, &reg, id);
            assert_waiting(&s, id, Zone::Hand(player), player);
            assert_eq!(s.player(player).life, 20);
            assert_eq!(pay_actions(&s, &reg), vec![true, false]);
            assert!(!legal_actions(&s, &reg).iter().any(Action::is_pass));
            let s = answer(s, &reg, pay).0;
            let land = s.objects.get(battlefield_id(&s, pool)).unwrap();
            assert_eq!(
                (land.controller, land.is_tapped(), s.player(player).life),
                (player, !pay, if pay { 18 } else { 20 })
            );
            assert_eq!(
                s.event_log
                    .iter()
                    .filter(|e| matches!(e, GameEvent::EntersBattlefield { .. }))
                    .count(),
                1
            );
        }
    }
}

#[test]
fn payment_is_offered_at_exactly_two_life_but_not_one() {
    for life in [1, 2] {
        let mut reg = CardRegistry::new();
        let pool = arcana_cards::gtc::breeding_pool::register(&mut reg);
        let mut s = table(0);
        s.player_mut(0).life = life;
        let id = add(&mut s, &reg, 0, pool, Zone::Hand(0));
        let s = play(s, &reg, id);
        assert_eq!(
            pay_actions(&s, &reg),
            if life == 2 {
                vec![true, false]
            } else {
                vec![false]
            }
        );
        let (s, y) = answer(s, &reg, life == 2);
        assert_eq!(
            s.objects.get(battlefield_id(&s, pool)).unwrap().is_tapped(),
            life == 1
        );
        assert_eq!(
            y.is_game_over(),
            life == 2,
            "zero life loses only after the entry completes"
        );
    }
}

fn pick(s: GameState, reg: &CardRegistry, ids: &[ObjectId]) -> GameState {
    let action = legal_actions(&s, reg).into_iter().find(|a| matches!(a,
        Action::SubmitResolutionChoice { response: ChoiceResponse::PickCards { picked }, .. } if picked == ids)).expect("legal card selection");
    step(s, action, reg).0
}

#[test]
fn windswept_heath_waits_for_the_fetched_lands_choice_before_shuffling() {
    for (player, pay) in [(0, false), (0, true), (1, false), (1, true)] {
        let mut reg = CardRegistry::new();
        let heath = arcana_cards::mh3::windswept_heath::register(&mut reg);
        let pool = arcana_cards::gtc::breeding_pool::register(&mut reg);
        let mut s = table(0);
        s.priority.give_to(player);
        let source = add(&mut s, &reg, player, heath, Zone::Battlefield);
        s.objects.get_mut(source).unwrap().status.summoning_sick = false;
        let target = add(&mut s, &reg, player, pool, Zone::Library(player));
        let other = add(&mut s, &reg, player, pool, Zone::Library(player));
        let action = legal_actions(&s, &reg)
            .into_iter()
            .find(|a| matches!(a, Action::ActivateAbility { source: id, .. } if *id == source))
            .unwrap();
        let s = pass_to_choice(step(s, action, &reg).0, &reg);
        let seed = s.rng_seed;
        let s = pick(s, &reg, &[target]);
        assert_waiting(&s, target, Zone::Library(player), player);
        assert_eq!(s.player(player).library_top_to_bottom, vec![target, other]);
        assert_eq!(s.rng_seed, seed, "shuffle waits for entry");
        assert_eq!(s.player(player).life, 19, "fetch cost paid exactly once");
        let s = answer(s, &reg, pay).0;
        assert_eq!(s.player(player).life, if pay { 17 } else { 19 });
        assert_eq!(
            s.objects.get(battlefield_id(&s, pool)).unwrap().is_tapped(),
            !pay
        );
        assert_eq!(s.player(player).library_top_to_bottom, vec![other]);
        assert_eq!(s.rng_seed, seed + 1);
        assert!(s.pending_resolution.is_none());
    }
}

#[test]
fn paying_farseeks_shock_does_not_override_its_tapped_entry() {
    let mut reg = CardRegistry::new();
    let farseek = arcana_cards::eoc::farseek::register(&mut reg);
    let fountain = arcana_cards::rtr::hallowed_fountain::register(&mut reg);
    let mut s = table(0);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Green, 2, 0);
    let card = add(&mut s, &reg, 0, farseek, Zone::Hand(0));
    let land = add(&mut s, &reg, 0, fountain, Zone::Library(0));
    let s = pick(cast(s, &reg, card), &reg, &[land]);
    assert_waiting(&s, land, Zone::Library(0), 0);
    let s = answer(s, &reg, true).0;
    assert_eq!(s.player(0).life, 18);
    assert!(s
        .objects
        .get(battlefield_id(&s, fountain))
        .unwrap()
        .is_tapped());
}

fn return_two(s: &GameState, e: &arcana_core::stack::StackEntry, _: &CardRegistry) -> Vec<Effect> {
    let mut ids = s.objects.ids_in_zone_sorted(Zone::Graveyard(e.controller));
    assert_eq!(ids.len(), 2);
    let second = ids.pop().unwrap();
    vec![
        Effect::Conditional {
            condition: Condition::Custom(|_| true),
            then: Box::new(Effect::Sequence(vec![
                Effect::ReturnFromGraveyardWithCounters {
                    target: ids[0],
                    kind: CounterKind::Charge,
                    count: 3,
                },
                Effect::ReturnFromGraveyardToBattlefield { target: second },
                Effect::GainLife {
                    player: e.controller,
                    amount: 1,
                },
            ])),
            otherwise: None,
        },
        Effect::GainLife {
            player: e.controller,
            amount: 4,
        },
    ]
}
fn spell(reg: &mut CardRegistry, effect: arcana_core::registry::SpellEffectFn) -> CardId {
    let name = reg.interner_mut().intern("Entry continuation probe");
    reg.register(
        CardDefinition::new(
            name,
            Characteristics {
                name,
                types: TypeLine::SORCERY.into(),
                mana_cost: Some(arcana_core::mana::ManaCost::parse("{0}").unwrap()),
                ..Default::default()
            },
        )
        .with_spell_ability(SpellAbilityDef {
            text: "Return two cards, then gain life".into(),
            target_requirements: vec![],
            modal: None,
            effect,
        }),
    )
}

#[test]
fn nested_returns_preserve_counters_and_the_tail_without_early_sbas() {
    let mut reg = CardRegistry::new();
    let pool = arcana_cards::gtc::breeding_pool::register(&mut reg);
    let probe = spell(&mut reg, return_two);
    let mut s = table(0);
    s.player_mut(0).life = 2;
    let a = add(&mut s, &reg, 0, pool, Zone::Graveyard(0));
    let b = add(&mut s, &reg, 0, pool, Zone::Graveyard(0));
    let card = add(&mut s, &reg, 0, probe, Zone::Hand(0));
    let s = cast(s, &reg, card);
    assert_waiting(&s, a, Zone::Graveyard(0), 0);
    let (s, y) = answer(s, &reg, true);
    assert!(
        !y.is_game_over(),
        "no SBA between instructions of this resolution"
    );
    assert_waiting(&s, b, Zone::Graveyard(0), 0);
    assert_eq!(
        s.player(0).life,
        0,
        "neither inner nor outer tail ran early"
    );
    assert_eq!(pay_actions(&s, &reg), vec![false]);
    let s = answer(s, &reg, false).0;
    assert_eq!(s.player(0).life, 5, "both tails ran exactly once");
    assert!(s.result.is_none());
    let lands: Vec<_> = s.objects.objects_in_zone(Zone::Battlefield).collect();
    assert_eq!(lands.len(), 2);
    assert_eq!(
        lands
            .iter()
            .map(|o| o.count_counters(CounterKind::Charge))
            .sum::<u32>(),
        3
    );
    assert_eq!(lands.iter().filter(|o| o.is_tapped()).count(), 1);
    assert!(s.pending_resolution.is_none());
}

#[test]
fn a_multi_card_entry_batch_finishes_all_choices_before_any_entry() {
    for generic in [false, true] {
        let mut reg = CardRegistry::new();
        let pool = arcana_cards::gtc::breeding_pool::register(&mut reg);
        let mut s = table(0);
        s.player_mut(0).life = 2;
        let a = add(&mut s, &reg, 0, pool, Zone::Graveyard(0));
        let b = add(&mut s, &reg, 0, pool, Zone::Graveyard(0));
        s.pending_choice_follow_up = Some(if generic {
            ChoiceFollowUp::MoveToZone {
                destination: Zone::Battlefield,
                reveal: true,
                shuffle_library_owner: None,
            }
        } else {
            ChoiceFollowUp::MoveToBattlefield {
                controller: 0,
                tapped: false,
                shuffle_library_owner: None,
            }
        });
        s.push_pending_choice(
            0,
            ChoiceContext::ResolvingStack(999),
            ChoiceKind::PickCards {
                candidates: vec![a, b],
                min: 2,
                max: 2,
            },
        );
        let s = pick(s, &reg, &[a, b]);
        assert_waiting(&s, a, Zone::Graveyard(0), 0);
        let (s, y) = answer(s, &reg, true);
        assert!(!y.is_game_over());
        assert_eq!(s.objects.objects_in_zone(Zone::Battlefield).count(), 0);
        assert_waiting(&s, b, Zone::Graveyard(0), 0);
        assert_eq!(
            pay_actions(&s, &reg),
            vec![false],
            "the first payment cannot be spent twice"
        );
        let s = answer(s, &reg, false).0;
        assert_eq!(s.objects.objects_in_zone(Zone::Battlefield).count(), 2);
        assert_eq!(
            s.objects
                .objects_in_zone(Zone::Battlefield)
                .filter(|o| o.is_tapped())
                .count(),
            1
        );
    }
}

#[test]
fn permanent_spell_entry_uses_the_same_choice_and_keeps_other_clauses() {
    let mut reg = CardRegistry::new();
    let name = reg.interner_mut().intern("Entry artifact probe");
    let artifact = reg.register(
        CardDefinition::new(
            name,
            Characteristics {
                name,
                types: TypeLine::ARTIFACT.into(),
                mana_cost: Some(arcana_core::mana::ManaCost::parse("{0}").unwrap()),
                ..Default::default()
            },
        )
        .with_enters_with(EntersWithSpec::Tapped)
        .with_enters_with(EntersWithSpec::TappedUnlessPayLife { life: 2 }),
    );
    let mut s = table(0);
    let card = add(&mut s, &reg, 0, artifact, Zone::Hand(0));
    let s = cast(s, &reg, card);
    let stack_id = match s.pending_choice.as_ref().unwrap().context {
        ChoiceContext::EnteringBattlefield { object_id } => object_id,
        _ => panic!("entry context"),
    };
    assert_waiting(&s, stack_id, Zone::Stack, 0);
    assert!(!s
        .event_log
        .iter()
        .any(|e| matches!(e, GameEvent::SpellResolved { .. })));
    let s = answer(s, &reg, true).0;
    assert!(s
        .objects
        .get(battlefield_id(&s, artifact))
        .unwrap()
        .is_tapped());
    assert_eq!(s.player(0).life, 18);
    assert_eq!(
        s.event_log
            .iter()
            .filter(|e| matches!(e, GameEvent::SpellResolved { .. }))
            .count(),
        1
    );
}

fn return_then_optional(
    s: &GameState,
    e: &arcana_core::stack::StackEntry,
    _: &CardRegistry,
) -> Vec<Effect> {
    let target = s.objects.ids_in_zone_sorted(Zone::Exile)[0];
    vec![Effect::Conditional {
        condition: Condition::LifeAtOrAbove(e.controller, 1),
        then: Box::new(Effect::Sequence(vec![
            Effect::ReturnFromExileToBattlefield { target },
            Effect::OptionalPayment {
                chooser: e.controller,
                cost: arcana_core::actions::OptionalPaymentKind::Life(1),
                then: Box::new(Effect::GainLife {
                    player: e.controller,
                    amount: 5,
                }),
                else_effect: None,
            },
            Effect::Conditional {
                condition: Condition::LifeAtOrAbove(e.controller, 22),
                then: Box::new(Effect::GainLife {
                    player: e.controller,
                    amount: 10,
                }),
                otherwise: None,
            },
        ])),
        otherwise: None,
    }]
}

#[test]
fn entry_then_another_nested_choice_keeps_the_branch_before_its_tail() {
    let mut reg = CardRegistry::new();
    let pool = arcana_cards::gtc::breeding_pool::register(&mut reg);
    let probe = spell(&mut reg, return_then_optional);
    let mut s = table(0);
    let land = add(&mut s, &reg, 0, pool, Zone::Exile);
    let card = add(&mut s, &reg, 0, probe, Zone::Hand(0));
    let s = cast(s, &reg, card);
    assert_waiting(&s, land, Zone::Exile, 0);
    let s = answer(s, &reg, true).0;
    let entry_id = s.pending_resolution.as_ref().unwrap().entry.id;
    assert_eq!(
        s.pending_choice.as_ref().unwrap().context,
        ChoiceContext::ResolvingStack(entry_id)
    );
    assert_eq!(s.player(0).life, 18);
    let s = answer(s, &reg, true).0;
    assert_eq!(
        s.player(0).life,
        32,
        "the optional gain precedes the conditional tail"
    );
    assert!(s.pending_resolution.is_none());
}

fn manifest(s: &GameState, e: &arcana_core::stack::StackEntry, _: &CardRegistry) -> Vec<Effect> {
    let _ = s;
    vec![Effect::Manifest {
        player: e.controller,
    }]
}

#[test]
fn manifested_shock_has_no_entry_payment() {
    let mut reg = CardRegistry::new();
    let pool = arcana_cards::gtc::breeding_pool::register(&mut reg);
    let probe = spell(&mut reg, manifest);
    let mut s = table(0);
    add(&mut s, &reg, 0, pool, Zone::Library(0));
    let card = add(&mut s, &reg, 0, probe, Zone::Hand(0));
    let action = legal_actions(&s, &reg)
        .into_iter()
        .find(|a| {
            matches!(a,
        Action::CastSpell { object_id, .. } if *object_id == card)
        })
        .unwrap();
    let mut s = step(s, action, &reg).0;
    for _ in 0..2 {
        let pass = legal_actions(&s, &reg)
            .into_iter()
            .find(Action::is_pass)
            .unwrap();
        s = step(s, pass, &reg).0;
    }
    assert!(s.pending_choice.is_none());
    let land = s.objects.get(battlefield_id(&s, pool)).unwrap();
    assert!(land.status.face_down && land.is_creature());
    assert!(!land.is_tapped());
    assert_eq!(s.player(0).life, 20);
}

#[test]
fn external_enter_tapped_replacement_survives_payment() {
    use arcana_core::replacement::*;
    let mut reg = CardRegistry::new();
    let pool = arcana_cards::gtc::breeding_pool::register(&mut reg);
    let mut s = table(0);
    s.add_replacement_effect(ReplacementEffect {
        source: 0,
        id: 0,
        condition: ReplacementCondition::WouldEnterBattlefield {
            object_filter: arcana_core::targets::ObjectFilter::default(),
        },
        kind: ReplacementKind::EtbTapped,
        is_self_replacement: false,
        duration: ReplacementDuration::Permanent,
        state_gate: None,
    });
    let land = add(&mut s, &reg, 0, pool, Zone::Hand(0));
    let s = answer(play(s, &reg, land), &reg, true).0;
    assert_eq!(s.player(0).life, 18);
    assert!(s.objects.get(battlefield_id(&s, pool)).unwrap().is_tapped());
}

#[test]
fn library_ordering_waits_for_the_entire_entry_batch() {
    use arcana_core::actions::CardDestination as D;
    let mut reg = CardRegistry::new();
    let pool = arcana_cards::gtc::breeding_pool::register(&mut reg);
    let mut s = table(0);
    let a = add(&mut s, &reg, 0, pool, Zone::Library(0));
    let b = add(&mut s, &reg, 0, pool, Zone::Library(0));
    let rest = add(&mut s, &reg, 0, pool, Zone::Library(0));
    s.push_pending_choice(
        0,
        ChoiceContext::ResolvingStack(999),
        ChoiceKind::OrderCards {
            cards: vec![a, b, rest],
            allowed: vec![D::Battlefield, D::BottomOfLibrary],
        },
    );
    let placements = vec![
        (a, D::Battlefield),
        (b, D::Battlefield),
        (rest, D::BottomOfLibrary),
    ];
    let action = legal_actions(&s, &reg).into_iter().find(|a| matches!(a,
        Action::SubmitResolutionChoice { response: ChoiceResponse::OrderCards { placements: p }, .. } if *p == placements)).unwrap();
    let s = step(s, action, &reg).0;
    assert_waiting(&s, a, Zone::Library(0), 0);
    let s = answer(s, &reg, true).0;
    assert_waiting(&s, b, Zone::Library(0), 0);
    assert_eq!(s.player(0).library_top_to_bottom, vec![a, b, rest]);
    assert_eq!(s.objects.count_in_zone(Zone::Battlefield), 0);
    let s = answer(s, &reg, false).0;
    assert_eq!(s.player(0).library_top_to_bottom, vec![rest]);
    assert_eq!(s.objects.count_in_zone(Zone::Battlefield), 2);
}

fn damage_return_then_counter(
    s: &GameState,
    e: &arcana_core::stack::StackEntry,
    _: &CardRegistry,
) -> Vec<Effect> {
    let creature = s
        .objects
        .objects_in_zone(Zone::Battlefield)
        .find(|o| o.is_creature())
        .unwrap()
        .id;
    let land = s.objects.ids_in_zone_sorted(Zone::Graveyard(0))[0];
    vec![
        Effect::DealDamage {
            source: e.source,
            target: arcana_core::events::DamageTarget::Object(creature),
            amount: 2,
        },
        Effect::ReturnFromGraveyardToBattlefield { target: land },
        Effect::AddCounters {
            target: creature,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}

#[test]
fn a_creature_is_not_killed_mid_resolution_while_entry_waits() {
    let mut reg = CardRegistry::new();
    let pool = arcana_cards::gtc::breeding_pool::register(&mut reg);
    let probe = spell(&mut reg, damage_return_then_counter);
    let mut s = table(0);
    let creature = s.allocate_object_id();
    s.objects.insert(GameObject::new(
        creature,
        0,
        Zone::Battlefield,
        0,
        Characteristics {
            types: TypeLine::CREATURE.into(),
            power: Some(arcana_core::types::PtValue::Fixed(2)),
            toughness: Some(arcana_core::types::PtValue::Fixed(2)),
            ..Default::default()
        },
    ));
    add(&mut s, &reg, 0, pool, Zone::Graveyard(0));
    let card = add(&mut s, &reg, 0, probe, Zone::Hand(0));
    let s = cast(s, &reg, card);
    assert_eq!(s.objects.get(creature).unwrap().damage_marked, 2);
    let s = answer(s, &reg, false).0;
    let survivor = s
        .objects
        .get(creature)
        .expect("later instructions save the creature before SBAs");
    assert_eq!(survivor.zone, Zone::Battlefield);
    assert_eq!(survivor.count_counters(CounterKind::PlusOnePlusOne), 1);
}

fn concede_at_choice(s: GameState, reg: &CardRegistry, player: PlayerId) -> GameState {
    use arcana_core::state::GameResult;
    let action = legal_actions(&s, reg)
        .into_iter()
        .find(Action::is_concede)
        .unwrap();
    let (s, result) = step(s, action, reg);
    assert_eq!(s.result, Some(GameResult::Win(1 - player)));
    assert!(
        matches!(result, EngineYield::GameOver(GameResult::Win(winner)) if winner == 1 - player)
    );
    assert!(s.player(player).has_conceded);
    assert_eq!(
        s.event_log
            .iter()
            .filter(|e| matches!(e, GameEvent::PlayerWins { .. }))
            .count(),
        1
    );
    s
}

#[test]
fn concession_ends_the_game_at_a_shock_entry_choice() {
    for player in [0, 1] {
        let mut reg = CardRegistry::new();
        let pool = arcana_cards::gtc::breeding_pool::register(&mut reg);
        let mut s = table(player);
        let land = add(&mut s, &reg, player, pool, Zone::Hand(player));
        let s = play(s, &reg, land);
        assert_waiting(&s, land, Zone::Hand(player), player);
        let s = concede_at_choice(s, &reg, player);
        assert_eq!(
            s.player(player).life,
            20,
            "no entry payment after concession"
        );
        assert_eq!(s.objects.get(land).unwrap().zone, Zone::Hand(player));
    }
}

#[test]
fn concession_ends_the_game_at_scry_with_or_without_a_deferred_death() {
    let mut reg = CardRegistry::new();
    let jet = arcana_cards::jmp::magma_jet::register(&mut reg);
    let opt = arcana_cards::dom::opt::register(&mut reg);
    let traveler = arcana_cards::clu::doomed_traveler::register(&mut reg);
    let mountain = arcana_cards::lea::mountain::register(&mut reg);
    for kill in [false, true] {
        let mut s = table(0);
        let victim = add(&mut s, &reg, 1, traveler, Zone::Battlefield);
        let card = add(&mut s, &reg, 0, if kill { jet } else { opt }, Zone::Hand(0));
        for _ in 0..4 {
            add(&mut s, &reg, 0, mountain, Zone::Library(0));
        }
        s.player_mut(0).mana_pool.add_mana(ManaColor::Red, 2, 0);
        s.player_mut(0).mana_pool.add_mana(ManaColor::Blue, 1, 0);
        let action = legal_actions(&s, &reg)
            .into_iter()
            .find(|a| {
                matches!(a,
            Action::CastSpell { object_id, targets, .. } if *object_id == card &&
                (!kill || targets.targets.iter().any(|t| t.object_id() == Some(victim))))
            })
            .unwrap();
        let s = pass_to_choice(step(s, action, &reg).0, &reg);
        assert!(matches!(
            s.pending_choice.as_ref().unwrap().kind,
            ChoiceKind::OrderCards { .. }
        ));
        assert_eq!(
            s.objects.get(victim).unwrap().damage_marked,
            if kill { 2 } else { 0 }
        );
        let s = concede_at_choice(s, &reg, 0);
        assert_eq!(
            s.objects.get(victim).unwrap().zone,
            Zone::Battlefield,
            "concession ends the game without resuming the deferred death"
        );
    }
}
