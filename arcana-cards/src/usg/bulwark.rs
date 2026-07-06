//! Bulwark — `{3}{R}{R}` enchantment.
//! "At the beginning of your upkeep, this enchantment deals X damage
//! to target opponent, where X is the number of cards in your hand
//! minus the number of cards in that player's hand."
//!
//! Upkeep trigger with a player target; X is computed at resolution
//! from the two hand sizes (saturating at zero). GAP: the target
//! cannot be constrained to opponents — `TargetFilter::Player` has no
//! opponent refinement.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bulwark");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
                effect: burn_by_hand_difference,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // GAP: "target OPPONENT" — TargetFilter::Player cannot
                // be constrained to opponents.
                target_requirements: vec![TargetRequirement::target_opponent()],
            },
        ),
    )
}

/// "…deals X damage to target opponent, where X is the number of cards
/// in your hand minus the number of cards in that player's hand."
fn burn_by_hand_difference(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    let x = script::hand_size(state, trig.controller)
        .saturating_sub(script::hand_size(state, *p));
    vec![Effect::DealDamage {
        target: DamageTarget::Player(*p),
        amount: x,
        source: trig.source,
    }]
}
