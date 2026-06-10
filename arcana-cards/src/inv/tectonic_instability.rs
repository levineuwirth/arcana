//! Tectonic Instability — `{2}{R}` enchantment.
//! "Whenever a land enters, tap all lands its controller controls."
//!
//! A battlefield-bound `ZoneChange` trigger on lands (any controller);
//! "its controller" is read off the entering land, and the sweep taps
//! every land that player controls via `Effect::ForEach`.

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
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tectonic Instability");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::LAND.into()),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: tap_their_lands,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…tap all lands its controller controls."
fn tap_their_lands(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(entered) = trig.entering_object() else {
        return Vec::new();
    };
    let player = script::target_controller(state, entered, trig.controller);
    let lands = script::ids_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You),
        player,
    );
    vec![Effect::ForEach {
        targets: lands,
        effect: Box::new(Effect::Tap {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
