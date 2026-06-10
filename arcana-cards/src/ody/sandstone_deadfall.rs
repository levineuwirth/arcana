//! Sandstone Deadfall — `{3}` artifact (Mirage).
//! "{T}, Sacrifice two lands and this artifact: Destroy target
//! attacking creature."
//!
//! The cost is approximated: tap + sacrifice-self + sacrifice ONE land
//! (sacrifice_other supports a single permanent — the second land is a
//! GAP). The "attacking" restriction on the target is likewise not
//! expressible in the demonstrated filter surface (GAP noted).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sandstone Deadfall");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}, Sacrifice two lands and this artifact: Destroy \
                       target attacking creature."
                    .into(),
                // GAP: cost requires sacrificing TWO lands; sacrifice_other
                // supports only one chosen permanent — modeled as one land.
                cost: ActivationCost {
                    tap: true,
                    sacrifice: true,
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::LAND.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                // GAP: target must be an ATTACKING creature — the attacking
                // restriction is not expressible in the demonstrated target
                // filter surface; modeled as target creature.
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_target,
            },
        ),
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
