//! Dispeller's Capsule — `{W}` artifact (white).
//! "{2}{W}, {T}, Sacrifice this artifact: Destroy target artifact or
//! enchantment."
//! One activated ability: mana + tap + sacrifice-self cost, destroying
//! a targeted artifact-or-enchantment permanent.

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
    let name = reg.interner_mut().intern("Dispeller's Capsule");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{W}, {T}, Sacrifice this artifact: Destroy target artifact or enchantment.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{W}").expect("valid cost"),
                tap: true,
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new().with_types_any(TypeLine(
                        TypeLine::ARTIFACT | TypeLine::ENCHANTMENT,
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
            effect: destroy_target,
        }),
    )
}

fn destroy_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}
