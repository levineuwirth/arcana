//! Brink of Madness — `{2}{B}{B}` enchantment.
//! "At the beginning of your upkeep, if you have no cards in hand,
//! sacrifice this enchantment and target opponent discards their
//! hand."
//!
//! Upkeep trigger with a real intervening-if (CR 603.4):
//! `conditions::hand_empty`. The self-sacrifice is modeled as a
//! name-filtered `Effect::Sacrifice` (documented approximation: any
//! copy of this enchantment you control); the discard empties the
//! target player's hand via a live `hand_size` read. Fidelity note:
//! the opponent restriction on the player target is not expressible on
//! a `TargetFilter::Player` requirement.

use arcana_core::conditions;
use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brink of Madness");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
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
                intervening_if: Some(if_hand_empty),
                effect: sacrifice_and_strip_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            },
        ),
    )
}

/// "…if you have no cards in hand…"
fn if_hand_empty(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::hand_empty(s, you)
}

/// "…sacrifice this enchantment and target opponent discards their
/// hand."
fn sacrifice_and_strip_hand(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    let nm = reg.interner().lookup("Brink of Madness");
    vec![
        // Approximation: sacrifices a permanent named Brink of Madness
        // you control (no sacrifice-this-exact-object Effect exists).
        Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter {
                name: nm,
                ..ObjectFilter::default()
            },
            count: 1,
        },
        Effect::Discard {
            player: *p,
            count: script::hand_size(state, *p),
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
