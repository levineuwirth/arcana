//! Mayael's Aria — `{R}{G}{W}` enchantment.
//! "At the beginning of your upkeep, put a +1/+1 counter on each creature
//! you control if you control a creature with power 5 or greater. Then you
//! gain 10 life if you control a creature with power 10 or greater. Then
//! you win the game if you control a creature with power 20 or greater."
//!
//! Your-upkeep `StepBegins` trigger with three sequential conditional
//! clauses, each gated on "you control a creature with power N or greater"
//! (`Condition::ControlPermanentMatching` over a `with_min_power` filter).
//! These are NOT an intervening-if on the whole trigger — the trigger
//! always fires; each clause checks its own condition at resolution.
//!
//! Clauses 1 (counter on each creature) and 2 (gain 10 life) are wired.
//! Clause 3 ("you win the game") is a GAP: there is no `Effect` variant for
//! winning the game.

use arcana_core::effects::{Condition, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mayael's Aria");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: aria_upkeep,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// The three sequential conditional clauses.
fn aria_upkeep(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let you = trig.controller;

    // Clause 1: "put a +1/+1 counter on each creature you control if you
    // control a creature with power 5 or greater."
    let your_creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        you,
    );
    let counter_each = Effect::Conditional {
        condition: Condition::ControlPermanentMatching(
            ObjectFilter::creature()
                .controlled_by(ControllerConstraint::You)
                .with_min_power(5),
        ),
        then: Box::new(Effect::ForEach {
            targets: your_creatures,
            effect: Box::new(Effect::AddCounters {
                target: NULL_OBJECT_ID,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            }),
        }),
        otherwise: None,
    };

    // Clause 2: "Then you gain 10 life if you control a creature with
    // power 10 or greater."
    let gain_ten = Effect::Conditional {
        condition: Condition::ControlPermanentMatching(
            ObjectFilter::creature()
                .controlled_by(ControllerConstraint::You)
                .with_min_power(10),
        ),
        then: Box::new(Effect::GainLife { player: you, amount: 10 }),
        otherwise: None,
    };

    // Clause 3: "Then you win the game if you control a creature with power
    // 20 or greater."
    // GAP: no Effect variant for winning the game.

    vec![counter_each, gain_ten]
}
