//! Sanctimony — `{1}{W}` enchantment.
//! "Whenever an opponent taps a Mountain for mana, you may gain 1
//! life."
//!
//! GAP: there is no "a player taps a [filtered] permanent for mana"
//! trigger condition in the engine catalog. The closest variant
//! (`SelfBecomesTapped`) only watches this permanent itself, so the
//! ability is wired as an honest no-op.

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
    let name = reg.interner_mut().intern("Sanctimony");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever an opponent taps a Mountain
                // for mana" has no matching TriggerCondition variant
                // (tapping other permanents for mana is not an exposed
                // event). SelfBecomesTapped is the closest variant and
                // never fires on this enchantment.
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

/// "…you may gain 1 life." (GAP: "you may" resolved as mandatory; the
/// trigger itself is the real gap — see register.)
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
