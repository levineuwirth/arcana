//! Ichorplate Golem — `{3}` 2/3 Artifact Creature — Phyrexian Golem.
//!
//! Oracle:
//! * "Whenever a creature you control enters, if it has one or more oil
//!   counters on it, put an oil counter on it." — a battlefield-enter
//!   ZoneChange trigger for creatures you control. The "if it has one
//!   or more oil counters" gate is on the ENTERING object (not this
//!   source), for which no intervening-if predicate exists, so it is
//!   checked at resolution time on the entering object (CR-faithful
//!   counter read) and the oil counter is added only when present.
//! * "Creatures you control with oil counters on them get +1/+1." —
//!   GAP: a continuous anthem static, not a triggered/activated ability.

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
    let name = reg.interner_mut().intern("Ichorplate Golem");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let golem = reg.interner_mut().intern("Golem");
    let _oil = reg.interner_mut().intern("oil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: add_oil_if_present,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn add_oil_if_present(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.entering_object().unwrap_or(trig.source);
    let Some(oil) = reg.interner().lookup("oil").map(CounterKind::Named) else {
        return Vec::new();
    };
    let current = state
        .objects
        .get(id)
        .map_or(0, |o| o.count_counters(oil));
    if current >= 1 {
        vec![Effect::AddCounters {
            target: id,
            kind: oil,
            count: 1,
        }]
    } else {
        Vec::new()
    }
}
