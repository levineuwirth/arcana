//! Gleam of Battle — `{4}{R}{W}` enchantment.
//! "Whenever a creature you control attacks, put a +1/+1 counter on it."
//!
//! A `CreatureAttacks` trigger filtered to creatures you control; the
//! attacking creature ("it") is read via `trig.attacking_creature()`, and a
//! single +1/+1 counter is placed on it.

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
    let name = reg.interner_mut().intern("Gleam of Battle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: pump_attacker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…put a +1/+1 counter on it."
fn pump_attacker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(attacker) = trig.attacking_creature() else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: attacker,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
