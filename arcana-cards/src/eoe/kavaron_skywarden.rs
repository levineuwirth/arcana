//! Kavaron Skywarden — `{4}{R}` 4/5 red Kavu Soldier.
//! Reach.
//! Void — At the beginning of your end step, if a nonland permanent left the
//! battlefield this turn or a spell was warped this turn, put a +1/+1 counter on
//! this creature.
//!
//! Reach is a base keyword (Void is not a base KeywordAbility variant — its
//! rules are the end-step trigger below). The trigger fires at the beginning of
//! your end step and adds a +1/+1 counter to this creature. The Void
//! intervening-if ("a nonland permanent left the battlefield this turn or a
//! spell was warped this turn") has no script/conditions predicate, so the gate
//! is dropped (None) and the trigger fires unconditionally (GAP).

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kavaron Skywarden");
    let kavu = reg.interner_mut().intern("Kavu");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kavu);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            // GAP (intervening-if): "if a nonland permanent left the battlefield
            //      this turn or a spell was warped this turn" has no script /
            //      conditions predicate; the gate is dropped (None).
            intervening_if: None,
            effect: add_counter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn add_counter(
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
