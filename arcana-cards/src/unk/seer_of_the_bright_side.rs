//! Seer of the Bright Side — `{1}{B}` 4/4 Human Assassin.
//! "This creature enters with two cage counters on it.
//!  This creature can't attack or block as long as it has a cage counter on it.
//!  At the beginning of your upkeep, remove a cage counter from this creature.
//!  When the last cage counter is removed this way, destroy target creature an
//!  opponent controls."
//!
//! ETB cage counters are modeled as a SelfEntersBattlefield trigger adding two
//! Named("cage") counters. The upkeep trigger removes one cage counter. The
//! static "can't attack or block while it has a cage counter" and the
//! conditional "when the last is removed, destroy target creature" are GAP'd
//! (no static-restriction-gated-on-counter primitive; no "only on last counter
//! removed" intervening condition).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seer of the Bright Side");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let _cage = reg.interner_mut().intern("cage");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: static "can't attack or block as long as it has a cage counter".
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enter_with_cage_counters,
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
                effect: upkeep_remove_cage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn enter_with_cage_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(cage) = reg.interner().lookup("cage") else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(cage),
        count: 2,
    }]
}

fn upkeep_remove_cage(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(cage) = reg.interner().lookup("cage") else { return Vec::new(); };
    // GAP: "When the last cage counter is removed this way, destroy target
    // creature an opponent controls" — needs a remove-then-conditional-on-zero
    // gate that this trigger shape can't express; only the removal is modeled.
    vec![Effect::RemoveCounters {
        target: trig.source,
        kind: CounterKind::Named(cage),
        count: 1,
    }]
}
