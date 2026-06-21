//! Renegade Krasis — `{1}{G}{G}` 3/2 Beast Mutant with Evolve.
//!
//! Oracle:
//! * Evolve
//! * Whenever this creature evolves, put a +1/+1 counter on each other creature
//!   you control with a +1/+1 counter on it.
//!
//! Evolve is a base keyword. There is no dedicated "evolves" trigger
//! condition; the closest expressible match is `CounterAdded` for a +1/+1
//! counter on this source (evolve always places one), used here with a GAP
//! note that it over-fires on non-evolve +1/+1 counter additions. The payoff
//! (a +1/+1 counter on each OTHER creature you control that already has a
//! +1/+1 counter) is expressed via a Sequence of per-id AddCounters.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Renegade Krasis");
    let beast = reg.interner_mut().intern("Beast");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Evolve],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: no "evolves" trigger condition — using CounterAdded on a
            // +1/+1 counter to self as the closest match; this over-fires on
            // non-evolve +1/+1 counter placements.
            trigger_condition: TriggerCondition::CounterAdded {
                on: TriggerSelf::Source,
                kind: Some(CounterKind::PlusOnePlusOne),
                chapter: None,
            },
            intervening_if: None,
            effect: on_evolve,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_evolve(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let effects: Vec<Effect> = ids
        .into_iter()
        .filter(|id| *id != trig.source)
        .filter(|id| {
            state
                .objects
                .get(*id)
                .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne))
                > 0
        })
        .map(|id| Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
