//! Mana Breach — `{2}{U}` enchantment (Exodus, 1998).
//! "Whenever a player casts a spell, that player returns a land they
//! control to its owner's hand."
//!
//! An any-caster `SpellCast` trigger; "that player" is read via
//! `trig.triggering_caster()`. Fidelity note: the player's CHOICE of
//! which land to return is not modeled — the lowest-id land they
//! control is returned deterministically.

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
    let name = reg.interner_mut().intern("Mana Breach");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Any,
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
/// Fidelity: deterministic lowest-id pick (the controller's choice of
/// land is not modeled).
fn bounce_a_land(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(caster) = trig.triggering_caster() else {
        return Vec::new();
    };
    let lands = script::ids_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You),
        caster,
    );
    let Some(land) = lands.first().copied() else {
        return Vec::new();
    };
    vec![Effect::ReturnToHand { target: land }]
}
