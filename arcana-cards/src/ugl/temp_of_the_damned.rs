//! Temp of the Damned — `{2}{B}` 3/3 Creature — Zombie.
//! As this creature enters, roll a six-sided die. This creature enters with a
//! number of funk counters on it equal to the result.
//! At the beginning of your upkeep, remove a funk counter from this creature.
//! If you can't, sacrifice it.
//!
//! GAP: "As this creature enters, roll a six-sided die … enters with a number
//! of funk counters equal to the result" — there is no die-roll effect nor an
//! enters-with-variable-counters replacement in this card class.
//! GAP: "If you can't, sacrifice it" — the available Condition variants cannot
//! read this source's funk-counter count, so the conditional sacrifice when no
//! funk counter remains is omitted; the upkeep removal is emitted.

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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Temp of the Damned");
    let zombie = reg.interner_mut().intern("Zombie");
    let _funk = reg.interner_mut().intern("funk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: remove_funk,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn remove_funk(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(funk) = reg.interner().lookup("funk").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::RemoveCounters {
        target: trig.source,
        kind: funk,
        count: 1,
    }]
}
