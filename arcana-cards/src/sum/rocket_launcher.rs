//! Rocket Launcher — `{4}` artifact.
//! "{2}: This artifact deals 1 damage to any target. Destroy this artifact
//! at the beginning of the next end step. Activate only if you've
//! controlled this artifact continuously since the beginning of your most
//! recent turn."

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rocket Launcher");
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
                text: "{2}: This artifact deals 1 damage to any target. \
                       Destroy this artifact at the beginning of the next \
                       end step. Activate only if you've controlled this \
                       artifact continuously since the beginning of your \
                       most recent turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ping_and_self_destruct,
            },
        ),
    )
}

/// Deal 1 to any target and schedule the self-destruction at the next end
/// step.
fn ping_and_self_destruct(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'Activate only if you've controlled this artifact continuously
    // since the beginning of your most recent turn' — no
    // summoning-sickness-style activation precondition in the
    // ActivationCost catalog; the ability is wired unconditionally.
    let Some(choice) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dmg_target = match choice {
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
    vec![
        Effect::DealDamage { target: dmg_target, amount: 1, source: ctx.source },
        // GAP fidelity: 'Destroy this artifact' modeled as a delayed
        // sacrifice (DelayedAction has no Destroy variant).
        Effect::DelayedAction {
            source: ctx.source,
            controller: ctx.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Sacrifice,
        },
    ]
}
