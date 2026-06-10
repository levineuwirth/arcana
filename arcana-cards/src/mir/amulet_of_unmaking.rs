//! Amulet of Unmaking — `{5}` artifact (Mirage, 1996).
//! "{5}, {T}, Exile this artifact: Exile target artifact, creature, or
//! land. Activate only as a sorcery." The exile-self cost uses
//! `exile_self: true`; the target is a permanent with a type-OR filter.

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
    let name = reg.interner_mut().intern("Amulet of Unmaking");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{5}, {T}, Exile this artifact: Exile target \
                       artifact, creature, or land. Activate only as a \
                       sorcery."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                    tap: true,
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(TypeLine(
                            TypeLine::ARTIFACT
                                | TypeLine::CREATURE
                                | TypeLine::LAND,
                        )),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: unmake,
            },
        ),
    )
}

fn unmake(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ExilePermanent { target: *id }]
}
