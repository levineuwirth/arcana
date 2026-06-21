//! Goremand — `{4}{B}{B}` 5/5 Demon.
//! As an additional cost to cast this spell, sacrifice a creature.
//! Flying, trample.
//! When this creature enters, each opponent sacrifices a creature.
//!
//! Flying and Trample are base characteristics. The "additional cost to cast:
//! sacrifice a creature" rider is a casting-cost modifier with no primitive on
//! a creature CardDefinition → GAP. The ETB makes each opponent sacrifice a
//! creature, looped over opponents and wrapped in a Sequence.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goremand");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };
    // GAP: "As an additional cost to cast this spell, sacrifice a creature" —
    // a casting-cost modifier with no primitive on a creature CardDefinition.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: each_opponent_sacrifices,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn each_opponent_sacrifices(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::creature(),
            count: 1,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
