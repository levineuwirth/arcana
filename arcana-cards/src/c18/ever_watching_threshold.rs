//! Ever-Watching Threshold — `{2}{U}` enchantment.
//! "Whenever an opponent attacks, if they attacked you and/or a
//! planeswalker you control, draw a card."
//!
//! Wired on `CreatureAttacks` over opponent creatures. The oracle's
//! trigger is per-ATTACK-EVENT (once per combat regardless of how many
//! creatures attack); `CreatureAttacks` is per-creature, so the
//! frequency is clamped to `OncePerTurn` as the closest approximation.
//! The "attacked you" gate is checked in the effect fn via
//! `trig.defending_player()`.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Ever-Watching Threshold");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever an opponent attacks" is a single
                // per-combat event; CreatureAttacks fires per attacking
                // creature. OncePerTurn approximates one draw per combat.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent),
                },
                intervening_if: None,
                effect: draw_if_attacked_you,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…if they attacked you and/or a planeswalker you control, draw a card."
fn draw_if_attacked_you(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "a planeswalker you control" — attacks on your planeswalkers
    // still resolve to you as the defending player in this engine, so the
    // defending-player check covers both halves of the condition.
    let Some(defender) = trig.defending_player() else {
        return Vec::new();
    };
    if defender != trig.controller {
        return Vec::new();
    }
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
