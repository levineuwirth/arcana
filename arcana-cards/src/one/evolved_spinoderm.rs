//! Evolved Spinoderm — `{2}{G}{G}` 5/5 green Phyrexian Beast.
//!
//! * "This creature enters with four oil counters on it." — GAP: a
//!   static enters-with replacement; not expressible as a
//!   triggered/activated ability.
//! * "This creature has trample as long as it has two or fewer oil
//!   counters on it. Otherwise, it has hexproof." — GAP: a counter-
//!   conditioned static keyword grant.
//! * "At the beginning of your upkeep, remove an oil counter from this
//!   creature. Then if it has no oil counters on it, sacrifice it." —
//!   the oil-counter removal is implemented; the trailing conditional
//!   self-sacrifice ("if it has no oil counters") is GAP'd (the
//!   resolution-time Condition surface isn't available here).

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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Evolved Spinoderm");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(beast);
    // Pre-intern the "oil" counter name so the resolver can recover it.
    let _oil = reg.interner_mut().intern("oil");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "At the beginning of your upkeep, remove an oil counter
            // from this creature. Then if it has no oil counters on it,
            // sacrifice it."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_remove_oil,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_remove_oil(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(oil) = reg.interner().lookup("oil").map(CounterKind::Named) else {
        return Vec::new();
    };
    // GAP: the trailing "Then if it has no oil counters on it, sacrifice
    // it" — the resolution-time Condition surface is not available; only
    // the counter removal is emitted.
    vec![Effect::RemoveCounters {
        target: trig.source,
        kind: oil,
        count: 1,
    }]
}
