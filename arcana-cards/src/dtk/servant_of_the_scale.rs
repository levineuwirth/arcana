//! Servant of the Scale — `{G}` 0/0 Human Soldier.
//!
//! * "This creature enters with a +1/+1 counter on it." GAP: an
//!   enters-with (as-enters) replacement effect, not expressible as a
//!   triggered ability (same posture as Narnam Renegade's revolt
//!   counter).
//! * "When this creature dies, put X +1/+1 counters on target creature
//!   you control, where X is the number of +1/+1 counters on this
//!   creature." — a `SelfDies` trigger targeting a creature you
//!   control, with X computed at resolution from the dying object's
//!   +1/+1 counter count.

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
    let name = reg.interner_mut().intern("Servant of the Scale");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        // GAP: "enters with a +1/+1 counter on it" — as-enters replacement.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_distribute_counters,
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

fn dies_distribute_counters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let dying = trig.dying_object().unwrap_or(trig.source);
    let x = state
        .objects
        .get(dying)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    if x == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: x,
    }]
}
