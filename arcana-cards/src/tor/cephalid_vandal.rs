//! Cephalid Vandal — `{1}{U}` 1/1 blue Octopus Rogue. "At the beginning
//! of your upkeep, put a shred counter on this creature. Then mill a
//! card for each shred counter on this creature."
//!
//! GAP: "mill a card for each shred counter on this creature" — no
//! script helper to count Named counters on a specific permanent.
//! We emit the AddCounters effect only and GAP the conditional mill.

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
    let name = reg.interner_mut().intern("Cephalid Vandal");
    let octopus = reg.interner_mut().intern("Octopus");
    let rogue = reg.interner_mut().intern("Rogue");
    // Pre-intern the counter name so the effect fn can look it up.
    let _shred = reg.interner_mut().intern("shred");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(octopus);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_shred_and_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_shred_and_mill(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let shred = reg.interner().lookup("shred")
        .expect("shred interned during register()");
    vec![
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::Named(shred),
            count: 1,
        },
        // GAP: mill a card for each shred counter on this creature —
        // no script helper to count Named counters on a specific permanent.
    ]
}
