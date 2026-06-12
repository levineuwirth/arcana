//! Avacynian Missionaries // Lunarch Inquisitors
//!
//! Front face: Creature — Human Cleric, {3}{W}, 3/3.
//! At the beginning of your end step, if this creature is equipped, transform it.
//! GAP: Intervening-if "if this creature is equipped" not expressible as intervening_if fn
//! pointer without state access to check attached Equipment. Trigger fires unconditionally
//! each end step (human will need to act manually).
//!
//! Back face: Creature — Human Cleric.
//! When this creature transforms into Lunarch Inquisitors, you may exile another target
//! creature until this creature leaves the battlefield.
//! GAP: "another" (excluding this creature) is not expressible in the target filter;
//! the exile trigger targets any creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    ControllerConstraint, PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Avacynian Missionaries");
    let human_sub = reg.interner_mut().intern("Human");
    let cleric_sub = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(cleric_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Lunarch Inquisitors");
    let human_sub2 = reg.interner_mut().intern("Human");
    let cleric_sub2 = reg.interner_mut().intern("Cleric");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(human_sub2);
    back_subtypes.0.insert(cleric_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
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
            // At the beginning of your end step, if equipped, transform.
            // GAP: intervening-if "if equipped" not expressible; fires unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // When this creature transforms into Lunarch Inquisitors, you may
            // exile another target creature until this creature leaves the
            // battlefield.
            // GAP: "another" (excluding this creature) not expressible in the
            // target filter; "you may" modeled via the up-to-one target count.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: on_transform_exile,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn end_step_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}

fn on_transform_exile(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ExileUntilSourceLeaves {
        source: trig.source,
        target: *id,
    }]
}
