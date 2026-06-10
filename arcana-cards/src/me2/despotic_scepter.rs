//! Despotic Scepter — `{1}` artifact (Ice Age).
//! "{T}: Destroy target permanent you own. It can't be regenerated."
//!
//! GAP: "you OWN" — the target filter constrains by CONTROLLER
//! (`ControllerConstraint::You`), the closest available predicate;
//! owner-based targeting is not expressible. GAP: "It can't be
//! regenerated" — the no-regeneration rider on destruction is not
//! expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Despotic Scepter");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Destroy target permanent you own. It can't be \
                       regenerated."
                    .into(),
                cost: ActivationCost::tap_only(),
                // GAP: "you own" approximated as "you control".
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_owned,
            },
        ),
    )
}

fn destroy_owned(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "It can't be regenerated" — no-regeneration rider not
    // expressible.
    vec![Effect::DestroyPermanent { target: *id }]
}
