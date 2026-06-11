//! Ripples of Undeath — `{1}{B}` enchantment.
//! "At the beginning of your first main phase, mill three cards. Then
//! you may pay {1} and 3 life. If you do, put a card from among those
//! cards into your hand."
//!
//! The first-main trigger and the mill are faithful; the combined
//! mana+life payment and the pick-from-the-milled-cards retrieval are
//! documented GAPs.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ripples of Undeath");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: mill_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…mill three cards. Then you may pay {1} and 3 life. If you do, put
/// a card from among those cards into your hand."
fn mill_three(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {1} AND 3 life" — OptionalPaymentKind supports
    // mana OR life, not both combined; and "put a card from among those
    // [milled] cards into your hand" cannot reference the just-milled
    // set. Only the mill is emitted.
    vec![Effect::Mill {
        player: trig.controller,
        count: 3,
    }]
}
