//! Rogue Skycaptain — `{2}{R}` 3/4 Human Rogue Mercenary with Flying.
//!
//! Oracle:
//! * Flying.
//! * At the beginning of your upkeep, put a wage counter on this
//!   creature. You may pay {2} for each wage counter on it. If you
//!   don't, remove all wage counters from this creature and an opponent
//!   gains control of it.
//!
//! The "put a wage counter" half is implemented; the variable-cost
//! "pay {2} per wage counter, else lose control" half is GAP'd
//! (OptionalPayment takes a fixed cost — it cannot scale per counter,
//! and "an opponent gains control" needs no expressible source id).

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Rogue Skycaptain");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    subtypes.0.insert(mercenary);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
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
            effect: upkeep_add_wage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn upkeep_add_wage(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wage = reg
        .interner()
        .lookup("wage")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::Charge);
    // GAP: "You may pay {2} for each wage counter on it. If you don't,
    // remove all wage counters from this creature and an opponent gains
    // control of it." — OptionalPayment uses a fixed cost (can't scale
    // {2} per wage counter), and the punishment lacks an expressible
    // counter-clear-plus-control-handoff form.
    vec![Effect::AddCounters {
        target: trig.source,
        kind: wage,
        count: 1,
    }]
}
