//! Vraska, Golgari Queen — `{2}{B}{G}` Legendary Planeswalker — Vraska,
//! starting loyalty 4. B/G.
//!
//! Oracle text:
//! * `+2`: You may sacrifice another permanent. If you do, you gain 1
//!   life and draw a card.
//! * `−3`: Destroy target nonland permanent with mana value 3 or less.
//! * `−9`: You get an emblem with "Whenever a creature you control deals
//!   combat damage to a player, that player loses the game."
//!
//! # Scope
//!
//! * `+2`: "you may sacrifice another permanent, if you do …" — the
//!   optional sacrifice cost isn't an `OptionalPaymentKind` (only
//!   Mana / Life exist) — GAP'd.
//! * `−3` destroys a target nonland permanent with mana value ≤ 3
//!   (expressible via filter).
//! * `−9` grants a bespoke combat-damage-loses-the-game emblem — GAP'd.

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
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vraska, Golgari Queen");
    let vraska = reg.interner_mut().intern("Vraska");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vraska);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: You may sacrifice another permanent. If you do, \
                       you gain 1 life and draw a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Destroy target nonland permanent with mana value \
                       3 or less.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()
                        .without_types(TypeLine::LAND.into())
                        .with_max_cmc(3)),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−9: You get an emblem with \"Whenever a creature you \
                       control deals combat damage to a player, that player \
                       loses the game.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine,
            }),
    )
}

/// `+2`: optional sacrifice → gain 1 + draw.
fn plus_two(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: optional-sacrifice cost ("you may sacrifice another permanent")
    // isn't an OptionalPaymentKind (only Mana / Life are supported).
    Vec::new()
}

/// `−3`: destroy target nonland permanent with mana value ≤ 3.
fn minus_three(_s: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}

/// `−9`: combat-damage-loses-the-game emblem.
fn minus_nine(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: bespoke emblem with a "loses the game" combat-damage trigger.
    Vec::new()
}
