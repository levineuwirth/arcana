//! Spiritual Focus — `{1}{W}` enchantment (Betrayers of Kamigawa,
//! 2005). "Whenever a spell or ability an opponent controls causes you
//! to discard a card, you gain 2 life and you may draw a card."
//!
//! Wired on `CardDiscarded { player: You }`. GAP: the
//! "caused by a spell or ability an opponent controls" restriction —
//! the discard event carries no cause, so this fires on ANY discard of
//! yours (a known over-fire). The "may" draw is resolved as drawing.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Spiritual Focus");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "a spell or ability an opponent controls
                // causes you to discard" — the CardDiscarded event carries
                // no causing-source, so the opponent-caused restriction is
                // not expressible; this fires on any discard of yours.
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: console,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you gain 2 life and you may draw a card." ("may" resolved as
/// drawing.)
fn console(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::GainLife {
            player: trig.controller,
            amount: 2,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
    ]
}
