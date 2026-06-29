//! Feast of the Victorious Dead — `{W}{B}` Enchantment.
//! "At the beginning of your end step, if one or more creatures died this turn, you gain that
//! much life and distribute that many +1/+1 counters among any number of creatures you control."
//! Partial: the life gain is implemented (dynamic via creatures_died_this_turn); the counter
//! distribution ("distribute N among any number") has no engine primitive — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feast of the Victorious Dead");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
        id: 1,
        trigger_condition: TriggerCondition::StepBegins {
            step: Step::End,
            whose: ControllerConstraint::You,
        },
        intervening_if: None,
        effect: end_step,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    }))
}

fn end_step(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let n = script::creatures_died_this_turn(state);
    if n == 0 {
        return Vec::new();
    }
    vec![
        Effect::GainLife { player: trig.controller, amount: n },
        // GAP: "distribute that many +1/+1 counters among any number of creatures you
        // control" — no DistributeCounters effect; DealDamageDivided is for damage only.
    ]
}
