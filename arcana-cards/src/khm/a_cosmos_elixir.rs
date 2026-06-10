//! A-Cosmos Elixir — `{4}` artifact (Kaldheim, Arena rebalance).
//! "At the beginning of your end step, draw a card if your life total
//! is greater than your starting life total. Otherwise, you gain 2 life
//! and scry 1."
//! The life comparison is checked at resolution via `script::life`
//! against the standard 20-life start (the starting life total itself
//! is not exposed by any script helper — assumed 20).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Cosmos Elixir");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_payoff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn end_step_payoff(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP (fidelity): "your starting life total" is not exposed by any
    // script helper — assumed to be the standard 20.
    if script::life(state, trig.controller) > 20 {
        vec![Effect::DrawCards { player: trig.controller, count: 1 }]
    } else {
        vec![
            Effect::GainLife { player: trig.controller, amount: 2 },
            Effect::Scry { player: trig.controller, count: 1 },
        ]
    }
}
