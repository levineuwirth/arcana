//! A-Sigil of Myrkul — `{1}{B}` enchantment (Alchemy rebalance).
//! "At the beginning of combat on your turn, mill a card. When you do,
//! if there are four or more creature cards in your graveyard, put a
//! +1/+1 counter on target creature you control and it gains deathtouch
//! until end of turn."
//!
//! The combat-begins trigger and the mill are faithful; the reflexive
//! "when you do" rider with its filtered-graveyard condition is a
//! documented GAP.

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
    let name = reg.interner_mut().intern("A-Sigil of Myrkul");
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
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: mill_and_empower,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…mill a card. When you do, if four or more creature cards are in
/// your graveyard, put a +1/+1 counter on target creature you control
/// and it gains deathtouch until end of turn."
fn mill_and_empower(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the reflexive "When you do, …" rider — its "four or more
    // CREATURE CARDS in your graveyard" condition has no type-filtered
    // graveyard count helper (graveyard_size is unfiltered), and the
    // reflexive-trigger targeting cannot be declared after the mill.
    // Only the mill is emitted.
    vec![Effect::Mill {
        player: trig.controller,
        count: 1,
    }]
}
