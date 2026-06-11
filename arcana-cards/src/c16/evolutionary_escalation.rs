//! Evolutionary Escalation — `{1}{G}` enchantment (Battle for Zendikar,
//! 2015). "At the beginning of your upkeep, put three +1/+1 counters on
//! target creature you control and three +1/+1 counters on target creature
//! an opponent controls."
//!
//! Upkeep trigger with TWO creature targets (yours, then an opponent's);
//! each gets three +1/+1 counters.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Evolutionary Escalation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
                effect: escalate_both,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature()
                                .controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature()
                                .controlled_by(ControllerConstraint::Opponent),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
            },
        ),
    )
}

/// "…put three +1/+1 counters on target creature you control and three
/// +1/+1 counters on target creature an opponent controls."
fn escalate_both(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut out = Vec::new();
    for choice in trig.targets.targets.iter().take(2) {
        if let TargetChoice::Object(id) = choice {
            out.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 3,
            });
        }
    }
    out
}
