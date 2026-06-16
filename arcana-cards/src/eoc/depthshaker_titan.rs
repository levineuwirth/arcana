//! Depthshaker Titan — `{5}{R}{R}` 5/5 Artifact Creature — Robot.
//! When this creature enters, any number of target noncreature artifacts you
//! control become 3/3 artifact creatures. Sacrifice them at the beginning of
//! the next end step.
//! Each artifact creature you control has melee, trample, and haste. (static)
//!
//! The board-wide static keyword grant has no expressible primitive (Melee is
//! not a usable KeywordAbility, and there is no controlled-set static keyword
//! grant) — GAP'd. The ETB animation + delayed sacrifice IS implemented.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Depthshaker Titan");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: animate_artifacts,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::ARTIFACT.into())
                            .without_types(TypeLine::CREATURE.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Any,
                    controller: None,
                }],
            }),
        // GAP: static "Each artifact creature you control has melee, trample, and haste"
        //      — Melee is not a usable KeywordAbility and there is no controlled-set
        //        static keyword-grant primitive.
    )
}

fn animate_artifacts(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    for t in &trig.targets.targets {
        let TargetChoice::Object(id) = t else { continue; };
        effects.push(Effect::AddType {
            target: *id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::Permanent,
        });
        effects.push(Effect::SetBasePT {
            target: *id,
            power: 3,
            toughness: 3,
            duration: Duration::Permanent,
        });
        effects.push(Effect::DelayedAction {
            source: *id,
            controller: trig.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Sacrifice,
        });
    }
    vec![Effect::Sequence(effects)]
}
