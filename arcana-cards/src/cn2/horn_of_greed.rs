//! Horn of Greed — `{3}` artifact.
//! "Whenever a player plays a land, that player draws a card."
//!
//! Modeled as a `ZoneChange` trigger (any land entering the
//! battlefield, any controller) whose effect draws a card for the
//! entering land's controller. Fidelity note: "plays a land" is
//! approximated by "a land enters the battlefield" — lands PUT onto
//! the battlefield by effects would also fire this (a documented
//! approximation; the trigger catalog has no plays-a-land variant).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Horn of Greed");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: trigger — "whenever a player PLAYS a land" approximated as
            // "a land enters the battlefield" (no plays-a-land variant).
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: land_player_draws,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn land_player_draws(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let land = trig.entering_object().unwrap_or(trig.source);
    let player = script::target_controller(state, land, trig.controller);
    vec![Effect::DrawCards { player, count: 1 }]
}
