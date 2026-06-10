//! Crystal Shard — `{3}` artifact (Mirrodin, 2003).
//! "{3}, {T} or {U}, {T}: Return target creature to its owner's hand
//! unless its controller pays {1}." The either-or cost is modeled as TWO
//! activated abilities (one per payable cost); the "unless ... pays" is
//! an OptionalPayment with the punishment (the bounce) in `else_effect`.

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
    let name = reg.interner_mut().intern("Crystal Shard");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}, {T}: Return target creature to its owner's \
                       hand unless its controller pays {1}."
                    .into(),
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
                effect: bounce_unless_paid,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}, {T}: Return target creature to its owner's \
                       hand unless its controller pays {1}."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: bounce_unless_paid,
            }),
    )
}

fn bounce_unless_paid(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
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
