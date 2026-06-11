//! Lurking Jackals — `{B}` enchantment.
//! "When an opponent has 10 or less life, if this permanent is an
//! enchantment, it becomes a 3/2 Jackal creature."
//!
//! // GAP: trigger — this is a STATE trigger ("when an opponent has 10 or
//! // less life") with no matching condition; approximated as an
//! // each-upkeep check whose intervening-if tests the life condition.
//! // GAP: intervening-if — "if this permanent is an enchantment" (a
//! // source-type check) has no predicate; only the life check is gated.
//! // GAP: the animated creature should gain the Jackal subtype; no
//! // effect sets subtypes.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lurking Jackals");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: state trigger "when an opponent has 10 or less
                // life" — no state-trigger condition exists; checked at
                // each upkeep instead.
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_opponent_at_ten_or_less),
                effect: become_jackal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…an opponent has 10 or less life…"
fn if_opponent_at_ten_or_less(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::opponents(s, you)
        .iter()
        .any(|&p| conditions::life_at_most(s, p, 10))
}

/// "…it becomes a 3/2 Jackal creature."
fn become_jackal(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddType {
            target: trig.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::SetBasePT {
            target: trig.source,
            power: 3,
            toughness: 2,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}
