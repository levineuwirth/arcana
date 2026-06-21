//! Ichor Synthesizer — `{1}{U}` 1/3 Phyrexian Wizard.
//!
//! Whenever you cast a noncreature spell, put an oil counter on this creature.
//! As long as this creature has four or more oil counters on it, it gets +2/+0
//! and can't be blocked.
//!
//! GAP (static): "as long as this creature has four or more oil counters … it
//! gets +2/+0 and can't be blocked" is a pure conditional static continuous
//! ability; not expressible as a TriggeredAbilityDef or ActivatedAbilityDef.
//! The cast-trigger that places oil counters is wired faithfully.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ichor Synthesizer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(wizard);
    // Intern the oil counter name up front.
    let _oil = reg.interner_mut().intern("oil");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: add_oil_counter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Whenever you cast a noncreature spell: put an oil counter on this creature.
fn add_oil_counter(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(oil) = reg.interner().lookup("oil") else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(oil),
        count: 1,
    }]
}
