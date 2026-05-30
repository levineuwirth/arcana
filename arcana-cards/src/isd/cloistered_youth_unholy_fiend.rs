//! Cloistered Youth // Unholy Fiend — `{1}{W}` Human creature 1/1.
//! At the beginning of your upkeep, you may transform this creature.
//! Back face: Unholy Fiend — Horror creature.
//!   At the beginning of your end step, you lose 1 life.
//!
//! # GAP
//! Back face "At the beginning of your end step, you lose 1 life" is a back-face-only
//! triggered ability. It is not modeled here since the engine does not gate triggered
//! abilities by face. // GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cloistered Youth");
    let human_sub = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Unholy Fiend — Horror creature
    let back_name = reg.interner_mut().intern("Unholy Fiend");
    let horror_sub = reg.interner_mut().intern("Horror");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(horror_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: At the beginning of your upkeep, you may transform this creature.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_may_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
            // GAP: back-face-only triggered ability not modeled.
            // (End step: lose 1 life — would be active on back face only.)
    )
}

fn upkeep_may_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may transform this creature." — modeled as unconditional transform.
    // GAP: the "you may" optional choice is not expressible for triggered abilities
    // (OptionalPayment only handles cost-gate patterns). Engine will auto-execute.
    vec![Effect::Transform { target: trig.source }]
}
