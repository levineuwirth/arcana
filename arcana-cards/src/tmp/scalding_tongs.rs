//! Scalding Tongs — `{2}` artifact (Tempest).
//! "At the beginning of your upkeep, if you have three or fewer cards
//! in hand, this artifact deals 1 damage to target opponent or
//! planeswalker." The hand-size gate is a CR 603.4 intervening-if
//! (three or fewer = NOT at-least-four).
//!
//! GAP: the "opponent or planeswalker" target is declared as a plain
//! target player — neither the opponent-only constraint nor the
//! planeswalker half is expressible with this catalog's target
//! filters.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scalding Tongs");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
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
                intervening_if: Some(if_three_or_fewer_in_hand),
                effect: ping_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // GAP: "target opponent or planeswalker" — plain target
                // player (no opponent constraint / planeswalker filter).
                target_requirements: vec![TargetRequirement::target_opponent()],
            },
        ),
    )
}

/// CR 603.4 intervening-if: "if you have three or fewer cards in
/// hand" — i.e. NOT at least four.
fn if_three_or_fewer_in_hand(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    !conditions::hand_at_least(s, you, 4)
}

fn ping_one(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        target: DamageTarget::Player(*p),
        amount: 1,
        source: trig.source,
    }]
}
