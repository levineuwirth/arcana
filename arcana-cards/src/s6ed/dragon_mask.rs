//! Dragon Mask — {3} artifact. "{3}, {T}: Target creature you control
//! gets +2/+2 until end of turn. Return it to its owner's hand at the
//! beginning of the next end step."
//!
//! Pump plus a delayed return-to-hand scheduled on the target.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Dragon Mask");
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
                text: "{3}, {T}: Target creature you control gets +2/+2 \
                       until end of turn. Return it to its owner's hand at \
                       the beginning of the next end step."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
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
                effect: pump_and_schedule_return,
            },
        ),
    )
}

fn pump_and_schedule_return(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::Pump {
            target: *id,
            power: 2,
            toughness: 2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::DelayedAction {
            source: *id,
            controller: ctx.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnToHand,
        },
    ]
}
