//! Powerstone Minefield — `{2}{R}{W}` enchantment.
//! "Whenever a creature attacks or blocks, this enchantment deals 2 damage
//! to it."
//!
//! The "attacks" half is wired: a `CreatureAttacks` trigger over any
//! creature deals 2 damage to the attacking creature (read via
//! `trig.attacking_creature()`).
//!
//! GAP: the "or blocks" half has no catalog trigger — there is no
//! board-wide "whenever a creature blocks" condition (`SelfBlocks` is
//! self-only), so blocking creatures are not punished. Documented partial.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Powerstone Minefield");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature(),
                },
                intervening_if: None,
                effect: mine,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…this enchantment deals 2 damage to it." (the attacking creature; the
/// "or blocks" half is unmodeled — see the module GAP.)
fn mine(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(attacker) = trig.attacking_creature() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        target: DamageTarget::Object(attacker),
        amount: 2,
        source: trig.source,
    }]
}
