//! Conjured Currency — `{5}{U}` enchantment.
//! "At the beginning of your upkeep, you may exchange control of this
//! enchantment and target permanent you neither own nor control."
//!
//! The exchange is modeled as a pair of `Effect::ChangeControl`s (target
//! to you, this enchantment to the target's controller).
//! // GAP: "you may" — the optional choice on a non-cost action is not
//! // expressible; the exchange is mandatory here.
//! // GAP: "you neither own nor control" — ObjectFilter has no ownership
//! // predicate; approximated as "an opponent controls".

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Conjured Currency");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
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
                effect: exchange_control,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            },
        ),
    )
}

/// "…exchange control of this enchantment and target permanent…"
fn exchange_control(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let their_controller = script::target_controller(state, *id, trig.controller);
    vec![
        Effect::ChangeControl {
            target: *id,
            new_controller: trig.controller,
        },
        Effect::ChangeControl {
            target: trig.source,
            new_controller: their_controller,
        },
    ]
}
