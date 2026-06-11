//! Puca's Mischief — `{3}{U}` enchantment (Shadowmoor, 2008).
//! "At the beginning of your upkeep, you may exchange control of target
//! nonland permanent you control and target nonland permanent an opponent
//! controls with equal or lesser mana value."
//!
//! Upkeep trigger with two nonland-permanent targets; the exchange is two
//! `ChangeControl` effects. GAP: the relational "with equal or lesser mana
//! value" constraint between the two targets is not expressible in
//! ObjectFilter. The "you may" is resolved as a yes.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Puca's Mischief");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
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
                effect: mischief_exchange,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // GAP: "with equal or lesser mana value" — a relational
                // constraint between the two chosen targets; ObjectFilter
                // has no cross-target comparison.
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::permanent()
                                .without_types(TypeLine::LAND.into())
                                .controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::permanent()
                                .without_types(TypeLine::LAND.into())
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

/// "…exchange control of target nonland permanent you control and target
/// nonland permanent an opponent controls…"
fn mischief_exchange(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(first) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let Some(second) = trig.targets.targets.get(1) else {
        return Vec::new();
    };
    let TargetChoice::Object(mine) = first else {
        return Vec::new();
    };
    let TargetChoice::Object(theirs) = second else {
        return Vec::new();
    };
    let their_controller = script::target_controller(state, *theirs, trig.controller);
    vec![
        Effect::ChangeControl {
            target: *mine,
            new_controller: their_controller,
        },
        Effect::ChangeControl {
            target: *theirs,
            new_controller: trig.controller,
        },
    ]
}
