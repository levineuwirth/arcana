//! Ashnod's Transmogrant — `{1}` artifact.
//! "{T}, Sacrifice this artifact: Put a +1/+1 counter on target nonartifact
//! creature. That creature becomes an artifact in addition to its other
//! types." The counter is wired; the type change is an additive
//! `Effect::AddType` with `Duration::Permanent` (it outlives the
//! sacrificed source).

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
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ashnod's Transmogrant");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice this artifact: Put a +1/+1 counter on target nonartifact creature. That creature becomes an artifact in addition to its other types.".into(),
            cost: ActivationCost {
                tap: true,
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().without_types(TypeLine::ARTIFACT.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: counter_and_artifactize,
        }),
    )
}

fn counter_and_artifactize(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        // "That creature becomes an artifact in addition to its other
        // types" — additive Layer-4 type overlay, permanent (survives
        // the sacrificed source).
        Effect::AddType {
            target: *id,
            types: TypeLine::ARTIFACT.into(),
            duration: Duration::Permanent,
        },
    ]
}
