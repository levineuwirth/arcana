//! Revenge of Ravens — `{3}{B}` enchantment.
//! "Whenever a creature attacks you or a planeswalker you control, that
//! creature's controller loses 1 life and you gain 1 life."
//!
//! GAP: trigger — "attacks you or a planeswalker you control" has no
//! defending-side filter in the trigger catalog; approximated as
//! "whenever a creature an opponent controls attacks", which is the
//! faithful 2-player read (an opponent's attacker always attacks you or
//! your planeswalkers).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Revenge of Ravens");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::Opponent),
            },
            intervening_if: None,
            effect: drain_attacker_controller,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "…that creature's controller loses 1 life and you gain 1 life."
/// The attacker is opponent-constrained by the trigger filter, so its
/// controller is the documented 2-player opponent read.
fn drain_attacker_controller(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(attacker_controller) =
        script::opponents(state, trig.controller).first().copied()
    else {
        return Vec::new();
    };
    vec![
        Effect::LoseLife {
            player: attacker_controller,
            amount: 1,
        },
        Effect::GainLife {
            player: trig.controller,
            amount: 1,
        },
    ]
}
