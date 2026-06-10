//! Cosmos Elixir — `{4}` artifact.
//! "At the beginning of your end step, draw a card if your life total
//! is greater than your starting life total. Otherwise, you gain 2
//! life."
//! A resolution-time branch (not an intervening-if — both branches do
//! something): the trigger fires every end step and the resolver
//! compares the live life total to the starting life total.
//! GAP fidelity: the starting life total is assumed to be 20 (the
//! engine exposes no starting-life accessor in the script surface).

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
    let name = reg.interner_mut().intern("Cosmos Elixir");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: draw_or_gain,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn draw_or_gain(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP fidelity: starting life total assumed to be 20.
    if script::life(state, trig.controller) > 20 {
        vec![Effect::DrawCards { player: trig.controller, count: 1 }]
    } else {
        vec![Effect::GainLife { player: trig.controller, amount: 2 }]
    }
}
