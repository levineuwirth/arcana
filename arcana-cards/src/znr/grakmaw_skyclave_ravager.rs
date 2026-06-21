//! Grakmaw, Skyclave Ravager — `{1}{B}{G}` 0/0 Legendary Creature — Hydra Horror.
//!
//! * Grakmaw enters with three +1/+1 counters on it. (Modeled as an
//!   ETB trigger that places three +1/+1 counters on itself.)
//! * Whenever another creature you control dies, if it had a +1/+1 counter
//!   on it, put a +1/+1 counter on Grakmaw.
//! * When Grakmaw dies, create an X/X black and green Hydra creature token,
//!   where X is the number of +1/+1 counters on Grakmaw.
//!
//! GAP: the intervening-if "if it had a +1/+1 counter on it" on the death
//! trigger has no expressible condition helper (the dying object's counter
//! count is gone by resolution); the trigger fires whenever another creature
//! you control dies.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grakmaw, Skyclave Ravager");
    let hydra = reg.interner_mut().intern("Hydra");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_three_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                // GAP: intervening-if "if it had a +1/+1 counter on it".
                intervening_if: None,
                effect: another_creature_died,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_make_hydra,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_three_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 3,
    }]
}

fn another_creature_died(
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

fn dies_make_hydra(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.dying_object().unwrap_or(trig.source);
    let x = state
        .objects
        .get(id)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne)) as i32;
    let hydra = reg.interner().lookup("Hydra").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: hydra,
            colors: ColorSet::black() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(x)),
            toughness: Some(PtValue::Fixed(x)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
