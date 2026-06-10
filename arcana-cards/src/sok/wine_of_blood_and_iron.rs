//! Wine of Blood and Iron — `{3}` artifact (Kamigawa: Neon Dynasty
//! adjacent flavor; classic sac-rock pump).
//! "{4}: Target creature gets +X/+0 until end of turn, where X is
//! its power. Sacrifice this artifact at the beginning of the next
//! end step."
//! One activated ability: doubles the target's power until end of
//! turn and schedules a delayed self-sacrifice at the next end step.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Wine of Blood and Iron");
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
                text: "{4}: Target creature gets +X/+0 until end of turn, where X is its power. Sacrifice this artifact at the beginning of the next end step.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_by_power,
            },
        ),
    )
}

fn pump_by_power(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let x = script::power_of(state, *id).max(0);
    vec![
        Effect::Pump {
            target: *id,
            power: x,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::DelayedAction {
            source: ctx.source,
            controller: ctx.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Sacrifice,
        },
    ]
}
