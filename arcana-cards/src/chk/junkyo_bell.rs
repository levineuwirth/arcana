//! Junkyo Bell — `{4}` artifact (Saviors of Kamigawa).
//! "At the beginning of your upkeep, you may have target creature you
//! control get +X/+X until end of turn, where X is the number of creatures
//! you control. If you do, sacrifice that creature at the beginning of the
//! next end step."
//!
//! Wired: the upkeep trigger pumps the targeted creature +X/+X (X computed
//! at resolution) and schedules the next-end-step sacrifice. GAP: the
//! "you may" choice — modeled as mandatory.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Junkyo Bell");
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
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: pump_and_doom,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn pump_and_doom(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "you may have" — the optional choice is not expressible;
    // modeled as mandatory.
    let x = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    ) as i32;
    vec![
        Effect::Pump {
            target: *id,
            power: x,
            toughness: x,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::DelayedAction {
            source: *id,
            controller: trig.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Sacrifice,
        },
    ]
}
