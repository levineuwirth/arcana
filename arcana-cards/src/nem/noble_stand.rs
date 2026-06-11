//! Noble Stand — `{4}{W}` enchantment.
//! "Whenever a creature you control blocks, you gain 2 life."
//!
//! GAP: there is no filtered "a creature [you control] blocks" trigger
//! condition — `SelfBlocks` only watches the source itself (which, as
//! an enchantment, never blocks). The lifegain payoff is authored but
//! unreachable.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Noble Stand");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever a creature you control
                // blocks" has no filtered TriggerCondition variant
                // (SelfBlocks is source-only; CreatureAttacks has no
                // blocking counterpart). SelfBlocks is the closest and
                // never fires on this enchantment.
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: gain_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you gain 2 life."
fn gain_two(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 2,
    }]
}
