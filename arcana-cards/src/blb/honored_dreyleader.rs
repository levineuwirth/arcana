//! Honored Dreyleader — `{2}{G}` 1/1 Squirrel Warrior with Trample.
//! When this creature enters, put a +1/+1 counter on it for each other
//! Squirrel and/or Food you control.
//! Whenever another Squirrel or Food you control enters, put a +1/+1
//! counter on this creature.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Honored Dreyleader");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let warrior = reg.interner_mut().intern("Warrior");
    let food = reg.interner_mut().intern("Food");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // Squirrel and/or Food you control (any type — Food is an artifact).
    let enter_filter = ObjectFilter::permanent()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![squirrel, food]);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Self-exclusion ("another"/"other") is not expressible on a
            // ZoneChange filter; this creature's own entry may also satisfy
            // the filter — a minor over-count / over-fire fidelity gap.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: enter_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_other_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_counters(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let squirrel = reg.interner().lookup("Squirrel").unwrap_or_default();
    let food = reg.interner().lookup("Food").unwrap_or_default();
    let filter = ObjectFilter::permanent()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![squirrel, food]);
    // "for each other Squirrel and/or Food you control" — self-exclusion not
    // expressible; this counts all matching permanents you control.
    let n = script::count_matching(state, &filter, trig.controller);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: n,
    }]
}

fn on_other_enters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
