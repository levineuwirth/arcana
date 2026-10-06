//! ETB, dies, attack triggers. APNAP ordering of simultaneous triggers.

use arcana_core::actions::ChoiceResponse;
use arcana_core::engine::step;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetChoice;
use arcana_core::types::{CardId, ManaColor};
use arcana_core::zones::Zone;
use arcana_core::{Action, EngineYield};

// ---------------------------------------------------------------------
// Shared fixtures
// ---------------------------------------------------------------------

fn add_object(s: &mut GameState, registry: &CardRegistry, card: CardId, zone: Zone) {
    let id = s.allocate_object_id();
    let chars = registry.get(card).unwrap().base_characteristics.clone();
    s.objects.insert(GameObject::new(id, 0, zone, card, chars));
}

/// Player 0 casts Wrath of God in their main phase, then both players
/// pass until the stack is empty. A target prompt is answered with
/// player 1, the only choice the tests below need.
fn wrath_and_settle(mut s: GameState, registry: &CardRegistry, wrath: CardId) -> GameState {
    add_object(&mut s, registry, wrath, Zone::Hand(0));
    s.player_mut(0).mana_pool.add_mana(ManaColor::White, 4, 0);
    s.priority.give_to(0);
    s.turn.phase = arcana_core::turn::Phase::PreCombatMain;
    s.turn.step = arcana_core::turn::Step::Main;
    let cast = arcana_core::legal_actions::legal_actions(&s, registry)
        .into_iter()
        .find(|a| matches!(a, Action::CastSpell { .. }))
        .expect("Wrath of God is castable from four white mana");
    let (mut s, mut yld) = step(s, cast, registry);
    while !s.stack_is_empty() || s.pending_choice.is_some() {
        let EngineYield::PendingDecision { legal_actions, .. } = &yld else {
            panic!("game ended before the stack emptied");
        };
        let next = legal_actions.iter()
            .find(|a| matches!(a, Action::SubmitResolutionChoice {
                response: ChoiceResponse::ChooseTargets { selection }, .. }
                if selection.targets == [TargetChoice::Player(1)]))
            .or_else(|| legal_actions.iter().find(|a| a.is_pass()))
            .cloned()
            .unwrap_or_else(|| panic!("no target or pass among {legal_actions:?}"));
        (s, yld) = step(s, next, registry);
    }
    s
}

// ---------------------------------------------------------------------
// Dies triggers whose source has left the battlefield
// ---------------------------------------------------------------------

/// The trigger's source is in the graveyard under a new id by the
/// time the ability resolves; resolution must still find its effect
/// through last-known information (CR 603.10, 608.2h).
#[test]
fn dies_trigger_resolves_after_its_source_has_left() {
    let mut registry = CardRegistry::new();
    let traveler = arcana_cards::clu::doomed_traveler::register(&mut registry);
    let wrath = arcana_cards::cmm::wrath_of_god::register(&mut registry);
    let mut s = GameState::new(2, 0);
    add_object(&mut s, &registry, traveler, Zone::Battlefield);

    let s = wrath_and_settle(s, &registry, wrath);
    let spirit = registry.interner().lookup("Spirit").unwrap();
    let spirits = s.objects.objects_in_zone(Zone::Battlefield)
        .filter(|o| o.characteristics.subtypes.contains(spirit))
        .count();
    assert_eq!(spirits, 1, "Doomed Traveler's dies trigger should make a Spirit");
}

/// A targeted dies trigger waits in the trigger queue for its target
/// prompt; the drain must still find its source's printed ability.
#[test]
fn targeted_dies_trigger_survives_the_queue_drain() {
    let mut registry = CardRegistry::new();
    let artist = arcana_cards::soc::blood_artist::register(&mut registry);
    let wrath = arcana_cards::cmm::wrath_of_god::register(&mut registry);
    let mut s = GameState::new(2, 0);
    add_object(&mut s, &registry, artist, Zone::Battlefield);

    let s = wrath_and_settle(s, &registry, wrath);
    assert_eq!(s.player(1).life, 19, "Blood Artist's target should lose 1 life");
    assert_eq!(s.player(0).life, 21, "Blood Artist's controller should gain 1 life");
    assert!(s.lki_held.is_empty(), "held LKI should be released once the stack is empty");
}
