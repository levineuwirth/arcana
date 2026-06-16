//! Ral Zarek, Guest Lecturer — `{1}{B}{B}` Legendary Planeswalker — Ral,
//! starting loyalty 3. Mono-black.
//!
//! Oracle:
//! +1: Surveil 2.
//! −1: Any number of target players each discard a card.
//! −2: Return target creature card with mana value 3 or less from your
//!     graveyard to the battlefield.
//! −7: Flip five coins. Target opponent skips their next X turns, where X is
//!     the number of coins that came up heads.
//!
//! # Scope
//! * +1 — modeled: Surveil 2.
//! * −1 — modeled: any number of target players each discard a card.
//! * −2 — GAP: graveyard-targeting (no any-graveyard sentinel in the surface).
//! * −7 — GAP: dynamic "skip next X turns" where X is a coin-flip count.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ral Zarek, Guest Lecturer");
    let sub = reg.interner_mut().intern("Ral");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    let any_players = TargetRequirement {
        filter: TargetFilter::Player,
        count: TargetCount::Any,
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Surveil 2.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_surveil,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: Any number of target players each discard a card.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![any_players],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_discard,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Return target creature card with mana value 3 or \
                       less from your graveyard to the battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Flip five coins. Target opponent skips their next X \
                       turns, where X is the number of coins that came up \
                       heads.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_gap,
            }),
    )
}

/// `+1: Surveil 2.`
fn plus_one_surveil(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Surveil {
        player: ctx.controller,
        count: 2,
    }]
}

/// `−1:` each target player discards a card.
fn minus_one_discard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Player(p) => Some(Effect::Discard {
                player: *p,
                count: 1,
                choice: DiscardChoice::ControllerChooses,
            }),
            _ => None,
        })
        .collect()
}

/// `−2` — GAP: graveyard-targeting reanimation.
fn minus_two_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: target creature card in your graveyard (no any-graveyard sentinel).
    Vec::new()
}

/// `−7` — GAP: dynamic coin-flip "skip next X turns".
fn minus_seven_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic X (coin-flip count) skip-turns.
    Vec::new()
}
