//! Lurking Predators — `{4}{G}{G}` enchantment.
//! "Whenever an opponent casts a spell, reveal the top card of your
//! library. If it's a creature card, put it onto the battlefield.
//! Otherwise, you may put that card on the bottom of your library."
//!
//! Modeled with `Effect::RevealUntil` capped at one card: the top card is
//! revealed; a creature goes to the battlefield.
//! // GAP: fidelity — for a non-creature top card the oracle gives a MAY
//! // choice (leave on top or put on bottom); `DigRest::BottomRandom`
//! // always bottoms it.

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lurking Predators");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: reveal_top,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…reveal the top card of your library. If it's a creature card, put it
/// onto the battlefield. Otherwise, you may put that card on the bottom."
fn reveal_top(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::RevealUntil {
        player: trig.controller,
        filter: ObjectFilter::creature(),
        found_dest: RevealDest::Battlefield,
        rest: DigRest::BottomRandom,
        max_reveal: Some(1),
    }]
}
