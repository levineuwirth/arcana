//! Panacea — `{4}` artifact.
//! "{X}{X}, {T}: Prevent the next X damage that would be dealt to any
//! target this turn."
//!
//! The X-cost activation reads the materialized X from `ctx.x_value`;
//! the shield is an [`Effect::PreventDamage`] with `amount: Some(X)`
//! on the chosen target, expiring at end of turn.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Panacea");
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
                text: "{X}{X}, {T}: Prevent the next X damage that would be dealt to any target this turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{X}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_x,
            },
        ),
    )
}

/// "Prevent the next X damage that would be dealt to any target this
/// turn." X is the activation's materialized X value.
fn prevent_x(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let damage_target = match target {
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
    vec![Effect::PreventDamage {
        target: damage_target,
        amount: Some(x),
        duration: ReplacementDuration::EndOfTurn,
    }]
}
