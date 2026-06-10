//! Mogg Cannon — `{2}` artifact.
//! "{T}: Target creature you control gets +1/+0 and gains flying until
//! end of turn. Destroy that creature at the beginning of the next end
//! step."
//! One tap activation: a +1/+0 + Flying pump on a targeted creature
//! you control, plus a delayed one-shot at the next end step.
//! GAP fidelity: the delayed action is a SACRIFICE (DelayedAction has
//! no Destroy variant) — same controller, same timing, but
//! regeneration/indestructible interactions differ from a destroy.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mogg Cannon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Target creature you control gets +1/+0 and gains flying until end of turn. Destroy that creature at the beginning of the next end step.".into(),
            cost: ActivationCost::tap_only(),
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
            effect: launch_mogg,
        }),
    )
}

fn launch_mogg(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP fidelity: 'destroy that creature at the beginning of the
    // next end step' modeled as a delayed SACRIFICE (no Destroy
    // DelayedAction variant).
    vec![
        Effect::Pump {
            target: *id,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Flying],
        },
        Effect::DelayedAction {
            source: *id,
            controller: ctx.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Sacrifice,
        },
    ]
}
