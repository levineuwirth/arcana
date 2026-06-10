//! Thumbscrews — `{2}` artifact (Darksteel).
//! "At the beginning of your upkeep, if you have five or more cards in
//! hand, this artifact deals 1 damage to target opponent or
//! planeswalker."
//!
//! An intervening-if upkeep trigger (CR 603.4) gated on hand size,
//! dealing 1 damage to the chosen target.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::targets::ControllerConstraint;
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thumbscrews");
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
                intervening_if: Some(if_hand_five_or_more),
                effect: ping_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // "target opponent or planeswalker" — any_target() is the
                // closest expressible shape (creature/player/planeswalker).
                target_requirements: vec![TargetRequirement::any_target()],
            },
        ),
    )
}

fn if_hand_five_or_more(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::hand_at_least(s, you, 5)
}

fn ping_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let damage_target = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => {
            DamageTarget::Object(*id)
        }
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => {
            DamageTarget::Player(*p)
        }
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        target: damage_target,
        amount: 1,
        source: trig.source,
    }]
}
