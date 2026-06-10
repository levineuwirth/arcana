//! Misers' Cage — `{3}` artifact (Mirage, 1996).
//! "At the beginning of each opponent's upkeep, if that player has
//! five or more cards in hand, this artifact deals 2 damage to that
//! player." The intervening-if checks the opponent's hand size; in
//! the two-player engine "each opponent" / "that player" is the
//! single opponent.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Misers' Cage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
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
                    whose: ControllerConstraint::Opponent,
                },
                intervening_if: Some(if_opponent_hand_five),
                effect: ping_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn if_opponent_hand_five(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::opponents(s, you)
        .first()
        .map(|&opp| conditions::hand_at_least(s, opp, 5))
        .unwrap_or(false)
}

fn ping_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(&opp) = script::opponents(state, trig.controller).first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        target: DamageTarget::Player(opp),
        amount: 2,
        source: trig.source,
    }]
}
