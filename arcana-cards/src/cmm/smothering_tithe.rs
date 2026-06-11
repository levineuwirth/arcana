//! Smothering Tithe — `{3}{W}` enchantment.
//! "Whenever an opponent draws a card, that player may pay {2}. If the
//! player doesn't, you create a Treasure token."
//!
//! Wired on `CardDrawn` (opponent-only). The drawing player is read
//! off the triggering DrawCard event exactly as the Underworld Dreams
//! seed does (no typed accessor exists for CardDrawn); the tax is an
//! `Effect::OptionalPayment` with the Treasure mint in `else_effect`
//! (the "unless they pay" polarity).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{CommodityToken, Effect};
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
    let name = reg.interner_mut().intern("Smothering Tithe");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: tithe_the_drawer,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…that player may pay {2}. If the player doesn't, you create a
/// Treasure token."
fn tithe_the_drawer(
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
        else_effect: Some(Box::new(Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Treasure,
            count: 1,
        })),
    }]
}
