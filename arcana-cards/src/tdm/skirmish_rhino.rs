//! Skirmish Rhino — `{W}{B}{G}` 3/4 Rhino.
//!
//! * Trample.
//! * When this creature enters, each opponent loses 2 life and you gain
//!   2 life.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skirmish Rhino");
    let rhino = reg.interner_mut().intern("Rhino");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rhino);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_drain,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_drain(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::LoseLife {
            player: p,
            amount: 2,
        })
        .collect();
    effects.push(Effect::GainLife {
        player: trig.controller,
        amount: 2,
    });
    vec![Effect::Sequence(effects)]
}
