//! Altar of the Brood — `{1}` artifact.
//! "Whenever another permanent you control enters, each opponent mills a card."
//! The enters-trigger is a `TriggerCondition::ZoneChange` over permanents you
//! control entering the battlefield; each opponent mills one card on
//! resolution.

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
    let name = reg.interner_mut().intern("Altar of the Brood");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "another permanent" — ObjectFilter cannot exclude the
            // trigger's own source, so this also fires when Altar of the
            // Brood itself enters (over-fires by that one event).
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::permanent()
                    .controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: each_opponent_mills,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn each_opponent_mills(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::Mill { player: p, count: 1 })
        .collect()
}
