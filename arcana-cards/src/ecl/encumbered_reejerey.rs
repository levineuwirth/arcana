//! Encumbered Reejerey — `{1}{W}` 5/4 Merfolk Soldier.
//! This creature enters with three -1/-1 counters on it.
//! Whenever this creature becomes tapped while it has a -1/-1 counter on it,
//! remove a -1/-1 counter from it.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Encumbered Reejerey");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "Enters with three -1/-1 counters" is a static replacement; modeled
            // as an ETB trigger that adds the three counters (standard approximation).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_three_minus_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: Some(if_has_minus_counter),
                effect: remove_one_minus_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_three_minus_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::MinusOneMinusOne,
        count: 3,
    }]
}

fn if_has_minus_counter(
    s: &GameState,
    src: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::source_has_counter(s, src, CounterKind::MinusOneMinusOne)
}

fn remove_one_minus_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::RemoveCounters {
        target: trig.source,
        kind: CounterKind::MinusOneMinusOne,
        count: 1,
    }]
}
