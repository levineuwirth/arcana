//! Ronom Hulk — `{4}{G}` 5/6 Beast.
//! "Protection from snow" (Protection is not an available KeywordAbility
//!  for this card class — GAP'd).
//! "Cumulative upkeep {1}" — modeled as the upkeep age-counter trigger;
//!  the "sacrifice unless you pay {1} for each age counter" payment is
//!  not expressible (cost scales with counters) — GAP'd.

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
    let name = reg.interner_mut().intern("Ronom Hulk");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let _age = reg.interner_mut().intern("age");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: "Protection from snow" — Protection not available here.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: cumulative_upkeep,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn cumulative_upkeep(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let age = reg.interner().lookup("age");
    // GAP: "then sacrifice this permanent unless you pay {1} for each age
    // counter on it" — the optional payment cost scales with counters and
    // is not expressible; only the age-counter add is modeled.
    match age {
        Some(sym) => vec![Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::Named(sym),
            count: 1,
        }],
        None => Vec::new(),
    }
}
