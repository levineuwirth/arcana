//! Mortuary — `{3}{B}` enchantment.
//! "Whenever a creature is put into your graveyard from the
//! battlefield, put that card on top of your library."
//!
//! A graveyard-bound `ZoneChange` trigger; the dead card is read via
//! `trig.dying_object()` and put on top of its owner's library.
//! GAP: "your graveyard" is an OWNERSHIP constraint; the filter
//! approximates it with "creature you control" (controller and owner
//! coincide for the common case).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mortuary");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: "into YOUR graveyard" is about card ownership; no
                // owner filter exists, so "creature you control" is the
                // closest approximation.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: recur_to_library_top,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…put that card on top of your library."
fn recur_to_library_top(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(dead) = trig.dying_object() else {
        return Vec::new();
    };
    vec![Effect::PutOnTopOfLibrary { target: dead }]
}
