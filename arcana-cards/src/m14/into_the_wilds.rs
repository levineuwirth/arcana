//! Into the Wilds — `{3}{G}` enchantment (Theros, 2013).
//! "At the beginning of your upkeep, look at the top card of your library.
//! If it's a land card, you may put it onto the battlefield."
//!
//! Modeled with `Effect::RevealUntil` capped at one card: if the top card
//! is a land it goes to the battlefield. Fidelity GAPs: a non-land top card
//! should stay on top (RevealUntil sends the rest to the bottom), and the
//! "you may" is unconditional.

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Into the Wilds");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
                effect: peek_for_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…look at the top card of your library. If it's a land card, you may
/// put it onto the battlefield."
fn peek_for_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP (fidelity): a non-land top card should remain on top of the
    // library — RevealUntil sends an unmatched card to the bottom; and
    // "you may" resolves unconditionally.
    vec![Effect::RevealUntil {
        player: trig.controller,
        filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
        found_dest: RevealDest::Battlefield,
        rest: DigRest::BottomRandom,
        max_reveal: Some(1),
    }]
}
