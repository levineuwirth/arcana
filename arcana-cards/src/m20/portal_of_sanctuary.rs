//! Portal of Sanctuary — `{2}{U}` artifact (M20 class).
//! "{1}, {T}: Return target creature you control and each Aura
//! attached to it to their owners' hands. Activate only during your
//! turn." The bounce of the targeted creature is wired; GAP: there is
//! no accessor for the Auras attached to the target (so that half is
//! omitted), and the "Activate only during your turn" timing window is
//! not expressible on ActivatedAbilityDef.

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
    let name = reg.interner_mut().intern("Portal of Sanctuary");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{1}, {T}: Return target creature you control and \
                       each Aura attached to it to their owners' hands. \
                       Activate only during your turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
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
                effect: bounce_with_auras,
            },
        ),
    )
}

fn bounce_with_auras(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "and each Aura attached to it" — no accessor enumerates the
    // Auras attached to the target; only the creature is returned.
    // GAP: "Activate only during your turn" — activation timing
    // windows are not expressible.
    vec![Effect::ReturnToHand { target: *id }]
}
