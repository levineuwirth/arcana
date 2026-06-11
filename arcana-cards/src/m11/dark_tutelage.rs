//! Dark Tutelage — `{2}{B}` enchantment.
//! "At the beginning of your upkeep, reveal the top card of your library
//! and put that card into your hand. You lose life equal to its mana
//! value."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dark Tutelage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
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
                intervening_if: None,
                effect: tutelage_reveal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…reveal the top card of your library and put that card into your hand.
/// You lose life equal to its mana value."
fn tutelage_reveal(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // The forced top-card-to-hand is modeled as a draw (the reveal itself
    // is informational only).
    // GAP: "you lose life equal to its mana value" — no accessor reads the
    // moved card's mana value, so the life-loss rider is omitted rather
    // than hardcoded.
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
