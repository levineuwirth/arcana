//! Skeleton Shard — `{3}` artifact (Mirrodin Shard cycle).
//! "{3}, {T} or {B}, {T}: Return target artifact creature card from
//! your graveyard to your hand."
//! The either-or cost is modeled as TWO activated abilities ({3}, {T}
//! and {B}, {T}) sharing one resolver.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skeleton Shard");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(shard_ability(
                "{3}, {T}: Return target artifact creature card from your graveyard to your hand.",
                ManaCost::parse("{3}").expect("valid cost"),
            ))
            .with_activated_ability(shard_ability(
                "{B}, {T}: Return target artifact creature card from your graveyard to your hand.",
                ManaCost::parse("{B}").expect("valid cost"),
            )),
    )
}

fn shard_ability(text: &str, mana_cost: ManaCost) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost {
            mana_cost,
            tap: true,
            ..ActivationCost::default()
        },
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Card {
                zone: Zone::Graveyard(0),
                filter: ObjectFilter::new().with_types(TypeLine(
                    TypeLine::ARTIFACT | TypeLine::CREATURE,
                )),
            },
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect: return_artifact_creature,
    }
}

fn return_artifact_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
