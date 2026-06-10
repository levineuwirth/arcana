//! Erratic Portal — `{4}` artifact.
//! "{1}, {T}: Return target creature to its owner's hand unless its
//! controller pays {1}."
//!
//! The "unless its controller pays" gate is the inverted-polarity
//! `Effect::OptionalPayment` shape: the target's controller is the
//! chooser, paying is a no-op, and declining bounces the creature.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Erratic Portal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{1}, {T}: Return target creature to its owner's hand unless its controller pays {1}.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: bounce_unless_pay,
            },
        ),
    )
}

fn bounce_unless_pay(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let payer = script::target_controller(state, *id, ctx.controller);
    vec![Effect::OptionalPayment {
        chooser: payer,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{1}").expect("valid cost"),
        ),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::ReturnToHand { target: *id })),
    }]
}
