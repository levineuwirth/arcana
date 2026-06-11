//! Furnace Celebration — `{1}{R}{R}` enchantment (Scars of Mirrodin).
//! "Whenever you sacrifice another permanent, you may pay {2}. If you
//! do, this enchantment deals 2 damage to any target."

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Furnace Celebration");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: "another" — the sacrificed-permanent filter
                // cannot exclude this source; over-fires if Furnace
                // Celebration itself is sacrificed.
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::permanent(),
                },
                intervening_if: None,
                effect: celebrate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            },
        ),
    )
}

/// "…you may pay {2}. If you do, this enchantment deals 2 damage to
/// any target."
fn celebrate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(choice) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let target = match choice {
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
