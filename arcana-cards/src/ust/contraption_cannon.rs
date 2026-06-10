//! Contraption Cannon — `{4}` artifact (Unstable).
//! "{2}, Sacrifice this artifact: It deals damage to any target equal
//! to the number of Contraptions you control." Sacrifice activation
//! whose damage amount is computed at resolution from the
//! Contraptions you control.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Contraption Cannon");
    let _contraption = reg.interner_mut().intern("Contraption");
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
                text: "{2}, Sacrifice this artifact: It deals damage to any target equal to the number of Contraptions you control.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_contraption_damage,
            },
        ),
    )
}

fn deal_contraption_damage(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(choice) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let target = match choice {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => {
            DamageTarget::Object(*id)
        }
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => {
            DamageTarget::Player(*p)
        }
        _ => return Vec::new(),
    };
    let amount = script::count_matching(
        state,
        &script::subtype_filter(reg, "Contraption")
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    if amount == 0 {
        return Vec::new();
    }
    vec![Effect::DealDamage { target, amount, source: ctx.source }]
}
