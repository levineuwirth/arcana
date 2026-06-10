//! Golem's Heart — `{2}` artifact (Scars of Mirrodin, 2010).
//! "Whenever a player casts an artifact spell, you may gain 1 life."
//! The 'may' is a beneficial no-cost choice modeled as always gaining
//! (deterministic upside).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Golem's Heart");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types(TypeLine::ARTIFACT.into()),
                    ),
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: gain_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn gain_one(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // 'You may gain 1 life' — no-cost beneficial choice; modeled as
    // always gaining.
    vec![Effect::GainLife { player: trig.controller, amount: 1 }]
}
