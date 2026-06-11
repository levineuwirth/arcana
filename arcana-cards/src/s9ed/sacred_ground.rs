//! Sacred Ground — `{1}{W}` enchantment.
//! "Whenever a spell or ability an opponent controls causes a land to
//! be put into your graveyard from the battlefield, return that card to
//! the battlefield."
//!
//! GAP: trigger — the "caused by a spell or ability an opponent
//! controls" clause has no cause filter on ZoneChange; the trigger
//! over-fires on any of your lands hitting your graveyard from the
//! battlefield (e.g. your own sacrifices).

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
    let name = reg.interner_mut().intern("Sacred Ground");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::new()
                    .with_types(TypeLine::LAND.into())
                    .controlled_by(ControllerConstraint::You),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: reclaim_the_land,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "…return that card to the battlefield."
fn reclaim_the_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(land) = trig.dying_object() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: land }]
}
