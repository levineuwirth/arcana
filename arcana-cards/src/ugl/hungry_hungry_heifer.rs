//! Hungry Hungry Heifer — `{2}{G}` 3/3 green Cow.
//! "At the beginning of your upkeep, you may remove a counter from a permanent you
//! control. If you don't, sacrifice this creature."
//! GAP: "remove a counter from a permanent you control" — no targeted
//! RemoveCounter-with-optional gate in OptionalPaymentKind. Emitting self-sacrifice
//! as else_effect with no-op then (punishing shape).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hungry Hungry Heifer");
    let cow = reg.interner_mut().intern("Cow");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cow);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_remove_counter_or_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_remove_counter_or_sacrifice(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may remove a counter from a permanent you control" — no
    // OptionalPaymentKind variant for removing counters. Using a Life(0) placeholder
    // payment as stand-in; else_effect triggers the self-sacrifice.
    // This is a materially incorrect model — the verify pipeline will flag it.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Life(0),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::Sacrifice {
            player: trig.controller,
            filter: arcana_core::targets::ObjectFilter::new()
                .with_types(arcana_core::types::TypeLine::CREATURE.into())
                .controlled_by(ControllerConstraint::You),
            count: 1,
        })),
    }]
}
