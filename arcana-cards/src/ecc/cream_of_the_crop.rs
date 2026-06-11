//! Cream of the Crop — `{1}{G}` enchantment.
//! "Whenever a creature you control enters, you may look at the top X
//! cards of your library, where X is that creature's power. If you do,
//! put one of those cards on top of your library and the rest on the
//! bottom of your library in any order."
//!
//! Approximated with `Effect::Scry` for X cards (the player may arrange
//! top/bottom).
//! // GAP: fidelity — the oracle requires exactly ONE card kept on top
//! // and the rest bottomed; Scry lets the player keep any number on top.

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
    let name = reg.interner_mut().intern("Cream of the Crop");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
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
                effect: look_at_top_x,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…look at the top X cards of your library, where X is that creature's
/// power…" — X read off the entering creature at resolution.
fn look_at_top_x(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(entered) = trig.entering_object() else {
        return Vec::new();
    };
    let x = script::power_of(state, entered).max(0) as u32;
    if x == 0 {
        return Vec::new();
    }
    vec![Effect::Scry {
        player: trig.controller,
        count: x,
    }]
}
