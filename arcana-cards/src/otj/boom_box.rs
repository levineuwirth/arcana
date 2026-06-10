//! Boom Box — {2} artifact (Jumpstart 2022).
//! "{6}, {T}, Sacrifice this artifact: Destroy up to one target
//! artifact, up to one target creature, and up to one target land."
//! Three up-to-one targets (artifact / creature / land), each
//! destroyed on resolution.

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boom Box");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{6}, {T}, Sacrifice this artifact: Destroy up to one \
                       target artifact, up to one target creature, and up \
                       to one target land."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::new()
                                .with_types(TypeLine::ARTIFACT.into()),
                        ),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature(),
                        ),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::new()
                                .with_types(TypeLine::LAND.into()),
                        ),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: destroy_targets,
            },
        ),
    )
}

fn destroy_targets(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => {
                Some(Effect::DestroyPermanent { target: *id })
            }
            _ => None,
        })
        .collect()
}
