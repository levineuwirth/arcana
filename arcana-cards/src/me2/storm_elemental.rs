//! Storm Elemental — `{5}{U}` 3/4 Elemental.
//!
//! Flying.
//! {U}, Exile the top card of your library: Tap target creature with flying.
//! {U}, Exile the top card of your library: If the exiled card is a snow land,
//! this creature gets +1/+1 until end of turn.
//!
//! "Exile the top card of your library" is not an expressible activation cost
//! (no such ActivationCost field) — both abilities GAP that cost and keep the
//! {U} mana portion. Ability 1's effect (tap target flyer) is implemented.
//! Ability 2's effect depends on the identity of the exiled card, which is not
//! observable without the exile cost, so its effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Storm Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP (cost): "Exile the top card of your library" — no such cost field.
                text: "{U}, Exile the top card of your library: Tap target creature with flying."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tap_flyer,
            })
            .with_activated_ability(ActivatedAbilityDef {
                // GAP (cost): "Exile the top card of your library" — no such cost field.
                text: "{U}, Exile the top card of your library: If the exiled card is a snow \
                       land, this creature gets +1/+1 until end of turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: snow_pump,
            }),
    )
}

fn tap_flyer(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Tap { target: *id }]
}

fn snow_pump(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: effect depends on the identity of the exiled card, which is not
    // observable (the exile cost itself is unexpressible).
    Vec::new()
}
