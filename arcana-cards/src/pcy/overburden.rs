//! Overburden — `{1}{U}` enchantment.
//! "Whenever a player puts a nontoken creature onto the battlefield,
//! that player returns a land they control to its owner's hand."
//!
//! A battlefield-bound `ZoneChange` trigger on nontoken creatures (any
//! controller). "That player" is the entering creature's controller;
//! the bounced land is picked deterministically (lowest id) — a
//! documented fidelity gap on the player's choice.

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
    let name = reg.interner_mut().intern("Overburden");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().nontoken(),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: bounce_a_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…that player returns a land they control to its owner's hand."
fn bounce_a_land(
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
    // GAP: "returns a land they control" is that player's choice; the
    // engine picks deterministically (lowest id) — documented fidelity
    // gap.
    let Some(land) = lands.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnToHand { target: *land }]
}
