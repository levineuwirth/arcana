//! Raid Bombardment — `{2}{R}` enchantment.
//! "Whenever a creature you control with power 2 or less attacks, this
//! enchantment deals 1 damage to the player or planeswalker that
//! creature is attacking."
//!
//! `CreatureAttacks` filtered to your power-≤2 creatures; the damage
//! goes to the defending player via `trig.defending_player()` (the
//! attacked-planeswalker arm is a documented GAP).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Raid Bombardment");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_max_power(2),
                },
                intervening_if: None,
                effect: bombard_defender,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…deals 1 damage to the player or planeswalker that creature is
/// attacking."
fn bombard_defender(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the attacked-PLANESWALKER arm — defending_player() resolves
    // attacks to the player; planeswalker-directed damage is folded into
    // the player read.
    let Some(defender) = trig.defending_player() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        target: DamageTarget::Player(defender),
        amount: 1,
        source: trig.source,
    }]
}
