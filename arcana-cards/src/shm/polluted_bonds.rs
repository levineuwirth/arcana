//! Polluted Bonds — `{3}{B}{B}` enchantment.
//! "Whenever a land an opponent controls enters, that player loses 2
//! life and you gain 2 life."
//!
//! A battlefield-bound ZoneChange over opponent lands; "that player" is
//! the entering land's controller, read via `trig.entering_object()` +
//! `script::target_controller`.

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
    let name = reg.interner_mut().intern("Polluted Bonds");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::Opponent),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: drain_land_player,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…that player loses 2 life and you gain 2 life."
fn drain_land_player(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(land) = trig.entering_object() else {
        return Vec::new();
    };
    let that_player = script::target_controller(state, land, trig.controller);
    vec![
        Effect::LoseLife {
            player: that_player,
            amount: 2,
        },
        Effect::GainLife {
            player: trig.controller,
            amount: 2,
        },
    ]
}
