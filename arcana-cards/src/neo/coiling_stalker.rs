//! Coiling Stalker — `{1}{G}` 2/1 Creature — Snake Ninja.
//!
//! Oracle:
//! * Ninjutsu {1}{G}.
//! * Whenever this creature deals combat damage to a player, put a +1/+1
//!   counter on target creature you control that doesn't have a +1/+1 counter
//!   on it.
//!
//! Decomposition: one triggered ability on combat damage to a player, putting
//! a +1/+1 counter on a target creature you control.
//!
//! GAP: Ninjutsu is not an expressible keyword (not in the supported keyword
//! surface) — omitted.
//! GAP: the target restriction "that doesn't have a +1/+1 counter on it" is
//! not an ObjectFilter predicate in the demonstrated API; the target is a
//! creature you control (best-effort, restriction relaxed).
//!
//! Note: "this creature deals combat damage" uses the established catalog
//! idiom — DamageDealt with an empty source_filter (the engine matches the
//! damage source; a self-only source filter is not separately modeled).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Coiling Stalker");
    let snake = reg.interner_mut().intern("Snake");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: counter_on_target,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
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
