//! Memnarch — `{7}` 4/5 Legendary Artifact Creature — Wizard.
//!
//! {1}{U}{U}: Target permanent becomes an artifact in addition to its
//!   other types. (This effect lasts indefinitely.)
//! {3}{U}: Gain control of target artifact. (This effect lasts indefinitely.)
//!
//! Decomposition: two `ActivatedAbilityDef`s. The first adds the artifact
//! type additively (`Effect::AddType`, `Duration::Permanent`); the second
//! takes permanent control (`Effect::ChangeControl`).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Memnarch");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}{U}: Target permanent becomes an artifact in addition to its other types.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_artifact,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}: Gain control of target artifact.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_control,
            }),
    )
}

fn make_artifact(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::AddType {
        target: *id,
        types: TypeLine::ARTIFACT.into(),
        duration: Duration::Permanent,
    }]
}

fn gain_control(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ChangeControl {
        target: *id,
        new_controller: ctx.controller,
    }]
}
