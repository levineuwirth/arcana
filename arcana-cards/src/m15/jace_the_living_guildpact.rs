//! Jace, the Living Guildpact — `{2}{U}{U}` Legendary Planeswalker — Jace,
//! starting loyalty 5 — colors U.
//!
//! Oracle text:
//! * `+1`: Look at the top two cards of your library. Put one of them into
//!   your graveyard. — approximated by `Effect::Surveil { count: 2 }` (look at
//!   the top two, then put any number into the graveyard).
//! * `−3`: Return another target nonland permanent to its owner's hand. —
//!   `Effect::ReturnToHand`. Approximation: the "another" self-exclusion is
//!   not expressible in the target filter.
//! * `−8`: Each player shuffles their hand and graveyard into their library,
//!   then draws seven cards. — multi-player hand+graveyard library reset. GAP:
//!   not expressible; the contingent seven-card draw is part of the same
//!   reset, so the whole ability is GAP'd.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jace, the Living Guildpact");
    let jace = reg.interner_mut().intern("Jace");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jace);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Look at the top two cards of your library. Put one \
                       of them into your graveyard.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_surveil,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Return another target nonland permanent to its \
                       owner's hand.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .without_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_bounce,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: Each player shuffles their hand and graveyard into \
                       their library, then draws seven cards.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_reset,
            }),
    )
}

/// `+1`: surveil 2 (approximates "look at top two, put one in graveyard").
fn plus_one_surveil(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Surveil { player: ctx.controller, count: 2 }]
}

/// `−3`: bounce a nonland permanent ("another" self-exclusion approximated).
fn minus_three_bounce(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let id = match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    vec![Effect::ReturnToHand { target: id }]
}

/// `−8`: each-player hand+graveyard library reset, then draw seven.
fn minus_eight_reset(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: each-player shuffle hand+graveyard into library is not expressible;
    // the contingent seven-card draw is part of the same reset, so the whole
    // ability is GAP'd rather than emitting a misleading bare draw.
    Vec::new()
}
