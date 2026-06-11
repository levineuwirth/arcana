//! Dance of Flame — enchantment (no mana cost printed).
//! "Whenever a Reveler attacks, Dance of Flame deals 1 damage to each
//! player."
//!
//! A `CreatureAttacks` trigger filtered to the Reveler subtype; the ping
//! fans out over `script::all_players`.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Dance of Flame");
    let reveler = reg.interner_mut().intern("Reveler");
    let chars = Characteristics {
        name,
        // No mana cost printed in the spec; colors follow the (absent)
        // cost's pips.
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .with_subtypes_any(vec![reveler]),
                },
                intervening_if: None,
                effect: burn_everyone,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…Dance of Flame deals 1 damage to each player."
fn burn_everyone(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::DealDamage {
            target: DamageTarget::Player(p),
            amount: 1,
            source: trig.source,
        })
        .collect()
}
