//! Storm Entity — `{1}{R}` 1/1 Elemental with Haste.
//!
//! "This creature enters with a +1/+1 counter on it for each other spell
//! cast this turn." Modeled as an ETB trigger that counts spells cast this
//! turn (minus this one — the spell that became this creature also counted)
//! and adds that many +1/+1 counters.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Storm Entity");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![arcana_core::effects::KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: enter_with_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn enter_with_counters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "for each OTHER spell cast this turn" — the spell that became this
    // creature is itself counted, so subtract one.
    let total = script::spells_cast_this_turn(state, &ObjectFilter::new(), trig.controller);
    let others = total.saturating_sub(1);
    if others == 0 {
        return Vec::new();
    }
    let id = trig.entering_object().unwrap_or(trig.source);
    vec![Effect::AddCounters {
        target: id,
        kind: CounterKind::PlusOnePlusOne,
        count: others,
    }]
}
