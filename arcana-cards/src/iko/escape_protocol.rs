//! Escape Protocol — `{1}{U}` enchantment.
//! "Whenever you cycle a card, you may pay {1}. When you do, exile
//! target artifact or creature you control, then return it to the
//! battlefield under its owner's control."
//!
//! Cycling has no dedicated trigger; `CardDiscarded { You }` is the
//! closest condition (cycling discards the card — GAP: it also fires on
//! non-cycling discards). The pay-{1} blink is an OptionalPayment
//! wrapping exile + return.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Escape Protocol");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever you CYCLE a card" has no
                // dedicated condition; CardDiscarded(You) is the closest
                // (cycling pays a discard) but also fires on ordinary
                // discards. The pay-{1} gate keeps the over-fire optional.
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pay_and_blink,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(
                                TypeLine::ARTIFACT | TypeLine::CREATURE,
                            ))
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            },
        ),
    )
}

/// "…you may pay {1}. When you do, exile target artifact or creature
/// you control, then return it to the battlefield…"
fn pay_and_blink(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{1}").expect("valid cost"),
        ),
        then: Box::new(Effect::Sequence(vec![
            Effect::ExilePermanent { target: *id },
            Effect::ReturnFromExileToBattlefield { target: *id },
        ])),
        else_effect: None,
    }]
}
