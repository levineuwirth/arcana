//! Mighty Emergence — `{2}{G}` enchantment (Alara Reborn, 2009).
//! "Whenever a creature you control with power 5 or greater enters, you
//! may put two +1/+1 counters on it."
//!
//! The trigger is a `ZoneChange` filtered to your power>=5 creatures
//! entering the battlefield; the counters land on the entering object.
//! The "you may" is a free resolution-time option with no catalog model
//! (OptionalPayment requires a mana/life cost), so the counters are
//! added unconditionally — a documented fidelity GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mighty Emergence");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_min_power(5),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: counters_on_entering,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "you may put two +1/+1 counters on it."
fn counters_on_entering(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.entering_object() else {
        return Vec::new();
    };
    // GAP: the "you may" choice is not modeled (OptionalPayment requires
    // a mana/life cost) — the counters are added unconditionally.
    vec![Effect::AddCounters {
        target: id,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}
