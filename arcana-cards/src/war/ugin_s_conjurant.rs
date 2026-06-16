//! Ugin's Conjurant — `{X}` 0/0 Spirit Monk.
//! "This creature enters with X +1/+1 counters on it." — X is the cast
//! mana-value, which has no PendingTrigger accessor in this shape, so
//! the ETB counter count is GAP'd.
//! "If damage would be dealt to this creature while it has a +1/+1
//! counter on it, prevent that damage and remove that many +1/+1
//! counters from this creature." — this is a self-replacement effect
//! with no triggered/activated form here → GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ugin's Conjurant");
    let spirit = reg.interner_mut().intern("Spirit");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(monk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: enters_with_x_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn enters_with_x_counters(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: X = the cast mana-value of {X}. A triggered ETB effect has no
    // accessor for the cast's announced X, so the dynamic counter count
    // is unobtainable.
    Vec::new()
}
