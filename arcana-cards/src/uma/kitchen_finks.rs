//! Kitchen Finks — `{1}{G/W}{G/W}` 3/2 Creature — Ouphe.
//!
//! * "When this creature enters, you gain 2 life." — an ETB trigger
//!   granting its controller 2 life.
//! * Persist (keyword) — "When this creature dies, if it had no -1/-1
//!   counters on it, return it to the battlefield under its owner's
//!   control with a -1/-1 counter on it." Wired via the engine's
//!   `KeywordAbility::Persist`.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Kitchen Finks");
    let ouphe = reg.interner_mut().intern("Ouphe");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ouphe);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G/W}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Persist],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_gain_two_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_gain_two_life(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 2,
    }]
}
