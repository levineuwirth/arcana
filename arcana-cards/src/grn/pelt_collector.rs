//! Pelt Collector — `{G}` 1/1 Elf Warrior.
//! Whenever another creature you control enters or dies, if that creature's
//! power is greater than this creature's, put a +1/+1 counter on this creature.
//! (As long as it has three or more +1/+1 counters, it has trample — GAP,
//! conditional static keyword grant.)

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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP: "As long as this creature has three or more +1/+1 counters on it, it
// has trample." — a conditional continuous keyword-grant static.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pelt Collector");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_enter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_if_greater_power(state: &GameState, source: arcana_core::objects::ObjectId, other: arcana_core::objects::ObjectId) -> Vec<Effect> {
    if other == source {
        return Vec::new();
    }
    let other_power = script::power_of(state, other);
    let my_power = script::power_of(state, source);
    if other_power > my_power {
        vec![Effect::AddCounters {
            target: source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }]
    } else {
        Vec::new()
    }
}

fn on_enter(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let other = trig.entering_object().unwrap_or(trig.source);
    add_if_greater_power(state, trig.source, other)
}

fn on_dies(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let other = trig.dying_object().unwrap_or(trig.source);
    add_if_greater_power(state, trig.source, other)
}
