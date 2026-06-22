//! Sarulf, Realm Eater — `{1}{B}{G}` 3/3 Legendary Wolf.
//!
//! * Whenever a permanent an opponent controls is put into a graveyard from
//!   the battlefield, put a +1/+1 counter on Sarulf.
//! * At the beginning of your upkeep, IF Sarulf has one or more +1/+1
//!   counters, you may remove all of them; if you do, exile each other
//!   nonland permanent with mana value <= the number removed.
//!   (Intervening-if gates the trigger; the resolver removes all counters and
//!   exiles by the computed mana-value threshold. The "you may" optionality is
//!   a resolution-time choice not separately gated.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::conditions;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sarulf, Realm Eater");
    let wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .controlled_by(ControllerConstraint::Opponent),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: add_counter,
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
                intervening_if: Some(if_has_counter),
                effect: wrath_by_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn if_has_counter(s: &GameState, source: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::source_counters_at_least(s, source, CounterKind::PlusOnePlusOne, 1)
}

fn wrath_by_counters(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let n = state
        .objects
        .get(trig.source)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    if n == 0 {
        return Vec::new();
    }
    // Exile each OTHER nonland permanent with mana value <= n.
    let filter = ObjectFilter::permanent()
        .without_types(TypeLine::LAND.into())
        .with_max_cmc(n);
    let mut ids = script::ids_matching(state, &filter, trig.controller);
    ids.retain(|&id| id != trig.source);
    vec![
        Effect::RemoveCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: n,
        },
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
        },
    ]
}
