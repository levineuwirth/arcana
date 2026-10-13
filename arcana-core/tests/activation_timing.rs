//! Activated-ability timing (A1.5s). An ability whose text sets no timing
//! restriction can be activated whenever its controller has priority
//! (CR 602.2): on any player's turn and with spells on the stack. "Activate
//! only as a sorcery", sorcery-speed keywords and loyalty abilities stay at
//! sorcery timing: their controller's main phase, with an empty stack.

use arcana_core::actions::Action;
use arcana_core::combat::{AttackerDeclaration, DefendingEntity};
use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::engine::step;
use arcana_core::legal_actions::legal_actions;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, CounterKind, ManaColor, PlayerId};
use arcana_core::zones::Zone;
use arcana_core::{EngineYield, ObjectId};

// ---------------------------------------------------------------------
// Shared fixtures
// ---------------------------------------------------------------------

fn add(s: &mut GameState, reg: &CardRegistry, owner: PlayerId, card: CardId, zone: Zone) -> ObjectId {
    let id = s.allocate_object_id();
    let chars = reg.get(card).unwrap().base_characteristics.clone();
    s.objects.insert(GameObject::new(id, owner, zone, card, chars));
    id
}

/// `player` holds priority in `phase`/`step` of `active`'s turn.
fn window(s: &mut GameState, active: PlayerId, player: PlayerId, phase: Phase, step: Step) {
    s.turn.active_player = active;
    s.turn.phase = phase;
    s.turn.step = step;
    s.priority.give_to(player);
}

/// Does the priority holder have an activation of `source` among its legal actions?
fn offers(s: &GameState, reg: &CardRegistry, source: ObjectId) -> bool {
    legal_actions(s, reg).iter()
        .any(|a| matches!(a, Action::ActivateAbility { source: src, .. } if *src == source))
}

/// Player 0 casts Lightning Bolt at player 1 in their main phase and keeps
/// priority with the Bolt on the stack.
fn bolt_on_the_stack(mut s: GameState, reg: &CardRegistry, bolt: CardId) -> GameState {
    add(&mut s, reg, 0, bolt, Zone::Hand(0));
    s.player_mut(0).mana_pool.add_mana(ManaColor::Red, 1, 0);
    window(&mut s, 0, 0, Phase::PreCombatMain, Step::Main);
    let cast = legal_actions(&s, reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { .. }))
        .expect("Lightning Bolt is castable");
    let (mut s, mut yld) = step(s, cast, reg);
    while s.pending_choice.is_some() {
        let EngineYield::PendingDecision { legal_actions, .. } = &yld else { panic!("game ended") };
        let next = legal_actions.iter().find(|a| !a.is_pass()).cloned().expect("a target");
        (s, yld) = step(s, next, reg);
    }
    assert!(!s.stack_is_empty(), "the Bolt is on the stack");
    s.priority.give_to(0);
    s
}

/// The windows a sorcery-speed ability must be refused in, each checked
/// against a main-phase window where it must be offered, so the refusals
/// cannot pass vacuously.
fn assert_sorcery_timed(s: &GameState, reg: &CardRegistry, source: ObjectId, bolt: CardId, what: &str) {
    let mut own_main = s.clone();
    window(&mut own_main, 0, 0, Phase::PreCombatMain, Step::Main);
    assert!(offers(&own_main, reg, source), "{what}: offered in its controller's main phase");

    let mut theirs = s.clone();
    window(&mut theirs, 1, 0, Phase::PreCombatMain, Step::Main);
    assert!(!offers(&theirs, reg, source), "{what}: refused on the opponent's turn");

    let mut combat = s.clone();
    window(&mut combat, 0, 0, Phase::Combat, Step::BeginCombat);
    assert!(!offers(&combat, reg, source), "{what}: refused in combat");

    let stacked = bolt_on_the_stack(s.clone(), reg, bolt);
    assert!(!offers(&stacked, reg, source), "{what}: refused with a spell on the stack");
}

// ---------------------------------------------------------------------
// Unrestricted abilities: whenever their controller has priority
// ---------------------------------------------------------------------

/// Prodigal Sorcerer's ping is offered on the opponent's turn and in
/// response to a spell, not only in its controller's main phase.
#[test]
fn unrestricted_ability_is_offered_on_any_turn_and_in_response() {
    let mut reg = CardRegistry::new();
    let sorcerer = arcana_cards::lea::prodigal_sorcerer::register(&mut reg);
    let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
    let mut s = GameState::new(2, 0);
    let id = add(&mut s, &reg, 0, sorcerer, Zone::Battlefield);

    let mut end_step = s.clone();
    window(&mut end_step, 1, 0, Phase::Ending, Step::End);
    assert!(offers(&end_step, &reg, id), "offered in the opponent's end step");

    let mut blockers = s.clone();
    window(&mut blockers, 1, 0, Phase::Combat, Step::DeclareBlockers);
    assert!(offers(&blockers, &reg, id), "offered in the opponent's combat");

    let stacked = bolt_on_the_stack(s, &reg, bolt);
    assert!(offers(&stacked, &reg, id), "offered with a spell on the stack");
}

/// Selfless Spirit answers a Wrath of God cast on its controller's
/// opponent's turn: its sacrifice resolves first, and the creatures it
/// leaves behind survive the Wrath as indestructible.
#[test]
fn selfless_spirit_answers_a_wrath_on_the_opponents_turn() {
    let mut reg = CardRegistry::new();
    let spirit = arcana_cards::soc::selfless_spirit::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let wrath = arcana_cards::cmm::wrath_of_god::register(&mut reg);
    let mut s = GameState::new(2, 0);
    let spirit_id = add(&mut s, &reg, 0, spirit, Zone::Battlefield);
    let bears_id = add(&mut s, &reg, 0, bears, Zone::Battlefield);
    add(&mut s, &reg, 1, wrath, Zone::Hand(1));
    s.player_mut(1).mana_pool.add_mana(ManaColor::White, 4, 0);
    window(&mut s, 1, 1, Phase::PreCombatMain, Step::Main);

    let cast = legal_actions(&s, &reg).into_iter()
        .find(|a| matches!(a, Action::CastSpell { .. }))
        .expect("Wrath of God is castable from four white mana");
    let (mut s, mut yld) = step(s, cast, &reg);
    let mut sacrificed = false;
    while !s.stack_is_empty() || s.pending_choice.is_some() {
        let EngineYield::PendingDecision { player, legal_actions, .. } = &yld else {
            panic!("game ended before the stack emptied");
        };
        let answer = (*player == 0 && !sacrificed).then(|| legal_actions.iter()
            .find(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == spirit_id)))
            .flatten();
        let next = match answer {
            Some(a) => { sacrificed = true; a.clone() }
            None => legal_actions.iter().find(|a| a.is_pass()).cloned()
                .unwrap_or_else(|| panic!("no pass among {legal_actions:?}")),
        };
        (s, yld) = step(s, next, &reg);
    }
    assert!(sacrificed, "Selfless Spirit's sacrifice was offered in response to the Wrath");
    assert_eq!(s.objects.get(bears_id).map(|o| o.zone), Some(Zone::Battlefield),
        "the Bears survive the Wrath as indestructible");
}

/// Mutavault becomes a creature in the opponent's declare-attackers step.
/// (Whether it can then block is A1.5x: combat eligibility reads printed
/// types, so a permanent animated by a continuous effect can't block yet.)
#[test]
fn mutavault_animates_in_the_opponents_declare_attackers_step() {
    let mut reg = CardRegistry::new();
    let mutavault = arcana_cards::clb::mutavault::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = GameState::new(2, 0);
    let vault = add(&mut s, &reg, 0, mutavault, Zone::Battlefield);
    let attacker = add(&mut s, &reg, 1, bears, Zone::Battlefield);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Red, 1, 0);
    s.turn.active_player = 1;
    s.begin_combat();
    s.apply_declared_attackers(vec![AttackerDeclaration {
        attacker, defending: DefendingEntity::Player(0),
    }]);
    window(&mut s, 1, 0, Phase::Combat, Step::DeclareAttackers);

    // Ability 0 is Mutavault's mana ability; 1 is its animation.
    let animate = legal_actions(&s, &reg).into_iter()
        .find(|a| matches!(a, Action::ActivateAbility { source, ability_index: 1, .. } if *source == vault))
        .expect("Mutavault's animation is offered in the opponent's declare-attackers step");
    let (mut s, _) = step(s, animate, &reg);
    assert!(!s.objects.get(vault).unwrap().is_tapped(), "the {{1}} came from the pool, not Mutavault");
    for _ in 0..8 {
        if s.stack_is_empty() { break; }
        (s, _) = step(s, Action::PassPriority, &reg);
    }
    assert!(s.stack_is_empty(), "the animation resolved");
    let chars = s.compute_characteristics(vault).unwrap();
    assert!(chars.types.is_creature(), "Mutavault is a creature until end of turn");
}

// ---------------------------------------------------------------------
// Sorcery-timed abilities stay at sorcery timing
// ---------------------------------------------------------------------

/// "Activate only as a sorcery": Bloodpyre Elemental's file text dropped
/// the clause until the sweep restored it; its gate must hold.
#[test]
fn activate_only_as_a_sorcery_holds() {
    let mut reg = CardRegistry::new();
    let elemental = arcana_cards::ala::bloodpyre_elemental::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
    let mut s = GameState::new(2, 0);
    let id = add(&mut s, &reg, 0, elemental, Zone::Battlefield);
    add(&mut s, &reg, 1, bears, Zone::Battlefield);
    assert_sorcery_timed(&s, &reg, id, bolt, "Bloodpyre Elemental");
}

/// Equip is sorcery-speed by rule (CR 702.6a).
#[test]
fn equip_holds_to_sorcery_timing() {
    let mut reg = CardRegistry::new();
    let splitter = arcana_cards::mrd::bonesplitter::register(&mut reg);
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
    let mut s = GameState::new(2, 0);
    let id = add(&mut s, &reg, 0, splitter, Zone::Battlefield);
    add(&mut s, &reg, 0, bears, Zone::Battlefield);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Green, 1, 0);
    assert_sorcery_timed(&s, &reg, id, bolt, "Bonesplitter's equip");
}

/// A Class's level-up is sorcery-speed by rule (CR 716.2a).
#[test]
fn class_level_holds_to_sorcery_timing() {
    let mut reg = CardRegistry::new();
    let class = arcana_cards::afr::barbarian_class::register(&mut reg);
    let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
    let mut s = GameState::new(2, 0);
    let id = add(&mut s, &reg, 0, class, Zone::Battlefield);
    s.objects.get_mut(id).unwrap().add_counters(CounterKind::Level, 1);
    s.player_mut(0).mana_pool.add_mana(ManaColor::Red, 2, 0);
    assert_sorcery_timed(&s, &reg, id, bolt, "Barbarian Class's Level 2");
}

/// A loyalty ability is sorcery-speed (CR 606.3) whatever its
/// `is_instant_speed` says: the loyalty check holds the timing itself.
#[test]
fn loyalty_ability_holds_to_sorcery_timing_even_flagged_instant() {
    use arcana_core::objects::Characteristics;
    use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost,
        ActivationZone, CardDefinition};
    use arcana_core::types::TypeLine;
    fn nothing(_: &GameState, _: &ActivationContext, _: &CardRegistry) -> Vec<Effect> { Vec::new() }

    let mut reg = CardRegistry::new();
    let bolt = arcana_cards::lea::lightning_bolt::register(&mut reg);
    let name = reg.interner_mut().intern("Test Instant-Flagged PW");
    let pw = reg.register(CardDefinition::new(name, Characteristics {
        name,
        types: TypeLine::PLANESWALKER.into(),
        loyalty: Some(3),
        ..Default::default()
    }).with_activated_ability(ActivatedAbilityDef {
        text: "+1: Nothing happens.".into(),
        cost: ActivationCost {
            add_self_counter: Some((CounterKind::Loyalty, 1)),
            ..ActivationCost::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: false,
        is_loyalty_ability: true,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: true,
        face_gate: None,
        effect: nothing,
    }));
    let mut s = GameState::new(2, 0);
    let id = add(&mut s, &reg, 0, pw, Zone::Battlefield);
    assert_sorcery_timed(&s, &reg, id, bolt, "a loyalty ability flagged instant");
}

// ---------------------------------------------------------------------
// The engine's own commodity tokens
// ---------------------------------------------------------------------

/// Food, Clue and Blood are activated at instant speed; Map is "Activate
/// only as a sorcery".
#[test]
fn food_clue_and_blood_are_instant_speed_and_map_is_not() {
    let mut reg = CardRegistry::new();
    let bears = arcana_cards::lea::grizzly_bears::register(&mut reg);
    let mut s = GameState::new(2, 0);
    for kind in [CommodityToken::Food, CommodityToken::Clue, CommodityToken::Blood, CommodityToken::Map] {
        Effect::CreateCommodityToken { controller: 0, kind, count: 1 }.execute(&mut s);
    }
    add(&mut s, &reg, 0, bears, Zone::Battlefield);
    add(&mut s, &reg, 0, bears, Zone::Hand(0));
    s.player_mut(0).mana_pool.add_mana(ManaColor::Green, 6, 0);
    // Each token is found by its one ability's text.
    let token = |s: &GameState, text: &str| s.objects.objects_in_zone(Zone::Battlefield)
        .find(|o| o.is_token && o.intrinsic_activated_abilities.iter().any(|a| a.text.contains(text)))
        .map(|o| o.id)
        .unwrap_or_else(|| panic!("a token whose ability says {text:?}"));
    let (food, clue, blood, map) = (token(&s, "You gain 3 life"), token(&s, "{2}, Sacrifice this artifact: Draw"),
        token(&s, "Discard a card"), token(&s, "explores"));

    let mut own_main = s.clone();
    window(&mut own_main, 0, 0, Phase::PreCombatMain, Step::Main);
    for (id, what) in [(food, "Food"), (clue, "Clue"), (blood, "Blood"), (map, "Map")] {
        assert!(offers(&own_main, &reg, id), "{what}: offered in its controller's main phase");
    }
    let mut theirs = s.clone();
    window(&mut theirs, 1, 0, Phase::Ending, Step::End);
    for (id, what) in [(food, "Food"), (clue, "Clue"), (blood, "Blood")] {
        assert!(offers(&theirs, &reg, id), "{what}: offered on the opponent's turn");
    }
    assert!(!offers(&theirs, &reg, map), "Map: refused on the opponent's turn");
}
