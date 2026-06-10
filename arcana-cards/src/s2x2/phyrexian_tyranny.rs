//! Phyrexian Tyranny — `{U}{B}{R}` enchantment (Planeshift, 2001).
//! "Whenever a player draws a card, that player loses 2 life unless
//! they pay {2}."
//!
//! Wired on `CardDrawn` (any player). The drawing player is read off
//! the triggering DrawCard event exactly as the Underworld Dreams seed
//! does (no typed accessor exists for CardDrawn); the tax is an
//! `Effect::OptionalPayment` with the punishment in `else_effect`
//! (the "Z unless you pay X" polarity).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::events::GameEvent;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Tyranny");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: tax_the_drawer,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…that player loses 2 life unless they pay {2}."
fn tax_the_drawer(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let GameEvent::DrawCard { player, .. } = trig.trigger_event else {
        return Vec::new();
    };
    vec![Effect::OptionalPayment {
        chooser: player,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{2}").expect("valid cost"),
        ),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::LoseLife { player, amount: 2 })),
    }]
}
