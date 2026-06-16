//! Kaya the Inexorable — `{3}{W}{B}` Legendary Planeswalker — Kaya,
//! starting loyalty 5. W/B.
//!
//! Oracle text:
//! * `+1`: Put a ghostform counter on up to one target nontoken
//!   creature. It gains "When this creature dies or is put into exile,
//!   return it to its owner's hand and create a 1/1 white Spirit
//!   creature token with flying."
//! * `−3`: Exile target nonland permanent.
//! * `−7`: You get an emblem with "At the beginning of your upkeep, you
//!   may cast a legendary spell from your hand, from your graveyard, or
//!   from among cards you own in exile without paying its mana cost."
//!
//! # Scope
//!
//! * `+1` puts a ghostform counter AND grants a bespoke
//!   dies-or-exiled triggered ability (return-to-hand + Spirit token);
//!   the counter alone without the granted ability would be
//!   meaningless — GAP'd.
//! * `−3` exiles a target nonland permanent (expressible).
//! * `−7` grants a bespoke cast-from-anywhere emblem — GAP'd.

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
    let name = reg.interner_mut().intern("Kaya the Inexorable");
    let kaya = reg.interner_mut().intern("Kaya");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kaya);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Put a ghostform counter on up to one target \
                       nontoken creature. It gains \"When this creature dies \
                       or is put into exile, return it to its owner's hand \
                       and create a 1/1 white Spirit creature token with \
                       flying.\"".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature().nontoken()),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Exile target nonland permanent.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()
                        .without_types(TypeLine::LAND.into())),
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
                text: "−7: You get an emblem with \"At the beginning of your \
                       upkeep, you may cast a legendary spell from your \
                       hand, from your graveyard, or from among cards you \
                       own in exile without paying its mana cost.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven,
            }),
    )
}

/// `+1`: ghostform counter + bespoke granted ability.
fn plus_one(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the granted "dies or is exiled → return to hand + make a Spirit
    // token" ability is bespoke; the ghostform counter alone is inert.
    Vec::new()
}

/// `−3`: exile target nonland permanent.
fn minus_three(_s: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::ExilePermanent { target: *id }]
}

/// `−7`: cast-from-anywhere legendary emblem.
fn minus_seven(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: bespoke emblem granting a free-cast-legendary upkeep trigger
    // sourcing from hand/graveyard/exile — not expressible.
    Vec::new()
}
