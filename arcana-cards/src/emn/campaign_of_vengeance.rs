//! Campaign of Vengeance — `{3}{W}{B}` enchantment.
//! "Whenever a creature you control attacks, defending player loses 1
//! life and you gain 1 life."
//!
//! A `CreatureAttacks` trigger over your creatures; the defending
//! player is read via the typed `trig.defending_player()` accessor.

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
    let name = reg.interner_mut().intern("Campaign of Vengeance");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: drain_defender,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…defending player loses 1 life and you gain 1 life."
fn drain_defender(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(defender) = trig.defending_player() else {
        return Vec::new();
    };
    vec![
        Effect::LoseLife {
            player: defender,
            amount: 1,
        },
        Effect::GainLife {
            player: trig.controller,
            amount: 1,
        },
    ]
}
