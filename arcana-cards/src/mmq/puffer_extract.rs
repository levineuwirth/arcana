//! Puffer Extract — `{5}` artifact.
//! "{X}, {T}: Target creature you control gets +X/+X until end of
//! turn. Destroy it at the beginning of the next end step." An X-cost
//! pump on a creature you control plus a delayed one-shot at the next
//! end step (destroy modeled as the delayed Sacrifice action — the
//! closest available `DelayedAction`; GAP noted).

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
    let name = reg.interner_mut().intern("Puffer Extract");
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
                text: "{X}, {T}: Target creature you control gets +X/+X \
                       until end of turn. Destroy it at the beginning of \
                       the next end step."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}").expect("valid cost"),
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
                effect: pump_then_doom,
            },
        ),
    )
}

fn pump_then_doom(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let x = ctx.x_value.unwrap_or(0) as i32;
    // GAP: "Destroy it at the beginning of the next end step" —
    // DelayedAction has no Destroy variant; modeled with the closest
    // action, Sacrifice.
    vec![
        Effect::Pump {
            target: *id,
            power: x,
            toughness: x,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::DelayedAction {
            source: *id,
            controller: ctx.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Sacrifice,
        },
    ]
}
