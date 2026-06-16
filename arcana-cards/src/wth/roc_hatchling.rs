//! Roc Hatchling — `{R}` 0/1 Bird.
//! This creature enters with four shell counters on it.
//! At the beginning of your upkeep, remove a shell counter from this
//! creature.
//! As long as this creature has no shell counters on it, it gets +3/+2
//! and has flying.
//!
//! The ETB four-shell-counter and the upkeep remove-a-shell-counter are
//! wired. The "no shell counters → +3/+2 and flying" line is a static
//! continuous ability gated on a counter count — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Roc Hatchling");
    let bird = reg.interner_mut().intern("Bird");
    let _shell = reg.interner_mut().intern("shell");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enters_with_shells,
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
                effect: remove_a_shell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP (static): "As long as this creature has no shell counters,
        // it gets +3/+2 and has flying" is a counter-gated continuous
        // static ability, not expressible here.
    )
}

fn enters_with_shells(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(shell) = reg.interner().lookup("shell").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters { target: trig.source, kind: shell, count: 4 }]
}

fn remove_a_shell(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(shell) = reg.interner().lookup("shell").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::RemoveCounters { target: trig.source, kind: shell, count: 1 }]
}
