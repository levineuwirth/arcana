//! Essence Reliquary — `{2}{W}` artifact.
//! "{T}: Return another target permanent you control and all Auras you
//! control attached to it to their owner's hand. Activate only during
//! your turn." The bounce of the chosen permanent is wired; the
//! attached-Auras rider, the "another" self-exclusion, and the
//! your-turn-only timing window are GAPs.

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
    let name = reg.interner_mut().intern("Essence Reliquary");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Return another target permanent you control and \
                       all Auras you control attached to it to their owner's \
                       hand. Activate only during your turn."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement {
                    // GAP: "another" — no exclude-source predicate on
                    // ObjectFilter; this card may target itself.
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
                effect: bounce_permanent,
            },
        ),
    )
}

fn bounce_permanent(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "and all Auras you control attached to it" — no accessor for
    // a permanent's attachments; only the chosen permanent is returned.
    // GAP: "Activate only during your turn" — no activation timing window
    // on ActivationCost.
    vec![Effect::ReturnToHand { target: *id }]
}
