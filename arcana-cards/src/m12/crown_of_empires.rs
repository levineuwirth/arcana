//! Crown of Empires — `{2}` artifact (Magic 2012).
//! "{3}, {T}: Tap target creature. Gain control of that creature
//! instead if you control artifacts named Scepter of Empires and
//! Throne of Empires." The instead-branch is computed at resolution
//! with by-name battlefield checks.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crown of Empires");
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
                text: "{3}, {T}: Tap target creature. Gain control of that creature instead if you control artifacts named Scepter of Empires and Throne of Empires.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tap_or_take_control,
            },
        ),
    )
}

fn tap_or_take_control(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let has_both = match (
        reg.interner().lookup("Scepter of Empires"),
        reg.interner().lookup("Throne of Empires"),
    ) {
        (Some(scepter), Some(throne)) => {
            script::count_matching(
                state,
                &ObjectFilter { name: Some(scepter), ..ObjectFilter::default() }
                    .controlled_by(ControllerConstraint::You),
                ctx.controller,
            ) >= 1
                && script::count_matching(
                    state,
                    &ObjectFilter {
                        name: Some(throne),
                        ..ObjectFilter::default()
                    }
                    .controlled_by(ControllerConstraint::You),
                    ctx.controller,
                ) >= 1
        }
        _ => false,
    };
    if has_both {
        vec![Effect::ChangeControl {
            target: *id,
            new_controller: ctx.controller,
        }]
    } else {
        vec![Effect::Tap { target: *id }]
    }
}
