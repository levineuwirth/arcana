//! Dark Suspicions — `{2}{B}{B}` enchantment.
//! "At the beginning of each opponent's upkeep, that player loses X
//! life, where X is the number of cards in that player's hand minus the
//! number of cards in your hand."
//!
//! Wired on `StepBegins { Upkeep, Opponent }`; "that player" is read via
//! the documented 2-player read (`script::opponents(..).first()`), and X
//! is computed from the two hand sizes at resolution.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dark Suspicions");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::Opponent,
            },
            intervening_if: None,
            effect: suspicion_drain,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "…that player loses X life, where X is the number of cards in that
/// player's hand minus the number of cards in your hand."
fn suspicion_drain(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = script::opponents(state, trig.controller).first().copied() else {
        return Vec::new();
    };
    let x = script::hand_size(state, p) as i32
        - script::hand_size(state, trig.controller) as i32;
    let amount = x.max(0) as u32;
    vec![Effect::LoseLife { player: p, amount }]
}
