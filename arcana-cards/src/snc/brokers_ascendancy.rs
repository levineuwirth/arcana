//! Brokers Ascendancy — `{G}{W}{U}` enchantment.
//! "At the beginning of your end step, put a +1/+1 counter on each
//! creature you control and a loyalty counter on each planeswalker you
//! control."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brokers Ascendancy");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: ascendancy_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "+1/+1 counter on each creature you control and a loyalty counter on
/// each planeswalker you control."
fn ascendancy_counters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let planeswalkers = script::ids_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::PLANESWALKER.into())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![
        Effect::ForEach {
            targets: creatures,
            effect: Box::new(Effect::AddCounters {
                target: arcana_core::objects::NULL_OBJECT_ID,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            }),
        },
        Effect::ForEach {
            targets: planeswalkers,
            effect: Box::new(Effect::AddCounters {
                target: arcana_core::objects::NULL_OBJECT_ID,
                kind: CounterKind::Loyalty,
                count: 1,
            }),
        },
    ]
}
