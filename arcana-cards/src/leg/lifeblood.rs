//! Lifeblood — `{2}{W}{W}` enchantment (Limited Edition Alpha, 1993).
//! "Whenever a Mountain an opponent controls becomes tapped, you gain
//! 1 life."
//!
//! No `TriggerCondition` observes OTHER permanents becoming tapped
//! (only `SelfBecomesTapped` exists) — the trigger is wired on that
//! closest placeholder (inert for an enchantment that never taps) with
//! an honest GAP; the 1-life payoff is wired faithfully.

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
    let name = reg.interner_mut().intern("Lifeblood");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "Whenever a Mountain an opponent controls
                // becomes tapped" — no TriggerCondition observes another
                // permanent becoming tapped; SelfBecomesTapped is the closest
                // placeholder (inert for this enchantment).
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: gain_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you gain 1 life."
fn gain_one(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 1,
    }]
}
