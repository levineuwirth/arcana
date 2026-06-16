//! Bess, Soul Nourisher — `{1}{G}{W}` 1/1 Legendary Human Citizen.
//! Whenever one or more other creatures you control with base power
//! and toughness 1/1 enter, put a +1/+1 counter on Bess.
//! Whenever Bess attacks, each other creature you control with base
//! power and toughness 1/1 gets +X/+X until end of turn, where X is
//! the number of +1/+1 counters on Bess.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

fn one_one_creatures_you_control() -> ObjectFilter {
    // Approximation of "base power and toughness 1/1": current 1 power,
    // 1 toughness, controlled by you. (A true base-P/T predicate is not
    // exposed; this filters on current characteristics.)
    ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_min_power(1)
        .with_max_power(1)
        .with_max_toughness(1)
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bess, Soul Nourisher");
    let human = reg.interner_mut().intern("Human");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: one_one_creatures_you_control(),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: add_counter_to_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_one_ones,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_counter_to_self(
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

fn pump_one_ones(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = state
        .objects
        .get(trig.source)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    if x == 0 {
        return Vec::new();
    }
    let ids = script::ids_matching(
        state,
        &one_one_creatures_you_control(),
        trig.controller,
    );
    // Exclude Bess herself ("each OTHER creature").
    let ids: Vec<_> = ids.into_iter().filter(|&id| id != trig.source).collect();
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: x as i32,
            toughness: x as i32,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
