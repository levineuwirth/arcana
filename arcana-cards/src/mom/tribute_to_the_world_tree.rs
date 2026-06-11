//! Tribute to the World Tree — `{G}{G}{G}` enchantment (March of the
//! Machine, 2023).
//! "Whenever a creature you control enters, draw a card if its power is 3
//! or greater. Otherwise, put two +1/+1 counters on it."
//!
//! A battlefield-bound `ZoneChange` trigger on creatures you control; the
//! branch is decided at resolution by reading the entering creature's power
//! via `script::power_of`.

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
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tribute to the World Tree");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}{G}").expect("valid cost")),
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
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: draw_or_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…draw a card if its power is 3 or greater. Otherwise, put two +1/+1
/// counters on it."
fn draw_or_counters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(entered) = trig.entering_object() else {
        return Vec::new();
    };
    if script::power_of(state, entered) >= 3 {
        vec![Effect::DrawCards {
            player: trig.controller,
            count: 1,
        }]
    } else {
        vec![Effect::AddCounters {
            target: entered,
            kind: CounterKind::PlusOnePlusOne,
            count: 2,
        }]
    }
}
