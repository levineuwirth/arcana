//! Searing Meditation — `{1}{R}{W}` enchantment.
//! "Whenever you gain life, you may pay {2}. If you do, this enchantment
//! deals 2 damage to any target."

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Searing Meditation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::LifeGained {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pay_to_sear,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            },
        ),
    )
}

/// "…you may pay {2}. If you do, this enchantment deals 2 damage to any
/// target."
fn pay_to_sear(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let target = match trig.targets.targets.first() {
        Some(TargetChoice::Object(id)) => DamageTarget::Object(*id),
        Some(TargetChoice::Player(p)) => DamageTarget::Player(*p),
        Some(TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id))) => {
            DamageTarget::Object(*id)
        }
        Some(TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p))) => {
            DamageTarget::Player(*p)
        }
        _ => return Vec::new(),
    };
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{2}").expect("valid cost"),
        ),
        then: Box::new(Effect::DealDamage {
            target,
            amount: 2,
            source: trig.source,
        }),
        else_effect: None,
    }]
}
