//! Haliya, Ascendant Cadet — `{2}{G}{W}{W}` 3/3 Legendary Human Soldier.
//! "Whenever Haliya enters or attacks, put a +1/+1 counter on target
//!  creature you control."
//! "Whenever one or more creatures you control with +1/+1 counters on
//!  them deal combat damage to a player, draw a card."
//!
//! The "enters or attacks" clause is decomposed into two triggers (one
//! SelfEntersBattlefield, one SelfAttacks), each putting a +1/+1
//! counter on a target creature you control.
//!
//! GAP: the combat-damage trigger requires a "creature you control with
//!      a +1/+1 counter on it" source filter on DamageDealt; the
//!      counter-presence predicate is not expressible in an ObjectFilter
//!      here, so the second ability is omitted rather than fired on every
//!      combat-damage event.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Haliya, Ascendant Cadet");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
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
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: counter_on_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![target_creature_you_control()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: counter_on_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![target_creature_you_control()],
            }),
    )
}

fn target_creature_you_control() -> TargetRequirement {
    TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
    }
}

fn counter_on_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
