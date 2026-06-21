//! Homarid — `{2}{U}` 2/2 blue Homarid.
//!
//! * "This creature enters with a tide counter on it." Modeled as a
//!   `SelfEntersBattlefield` trigger adding a tide (named) counter.
//!   (Approximation: true enters-with-counters is a replacement effect
//!   not in the demonstrated surface.)
//! * "At the beginning of your upkeep, put a tide counter on this
//!   creature." An upkeep `StepBegins` trigger.
//! * "As long as there is exactly one tide counter on this creature, it
//!   gets -1/-1." GAP: count-gated static continuous P/T not expressible.
//! * "As long as there are exactly three tide counters on this creature,
//!   it gets +1/+1." GAP: same.
//! * "Whenever there are four or more tide counters on this creature,
//!   remove all tide counters from it." A `CounterAdded` trigger keyed to
//!   the 4th tide counter (the creature can never exceed four because it
//!   self-empties), removing all tide counters.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Homarid");
    let homarid = reg.interner_mut().intern("Homarid");
    let tide = reg.interner_mut().intern("tide");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(homarid);

    // GAP: count-gated static continuous P/T modifiers ("exactly one
    // tide counter → -1/-1", "exactly three → +1/+1") are not
    // expressible with the demonstrated API.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: add_tide,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: add_tide,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Named(tide)),
                    chapter: Some(4),
                },
                intervening_if: None,
                effect: remove_all_tide,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_tide(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(tide) = reg.interner().lookup("tide").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: tide,
        count: 1,
    }]
}

fn remove_all_tide(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(tide) = reg.interner().lookup("tide").map(CounterKind::Named) else {
        return Vec::new();
    };
    let n = state
        .objects
        .get(trig.source)
        .map_or(0, |o| o.count_counters(tide));
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::RemoveCounters {
        target: trig.source,
        kind: tide,
        count: n,
    }]
}
