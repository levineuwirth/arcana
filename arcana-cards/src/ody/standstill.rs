//! Standstill — `{1}{U}` enchantment.
//! "When a player casts a spell, sacrifice this enchantment. If you do,
//! each of that player's opponents draws three cards."
//!
//! "That player" is read via `trig.triggering_caster()`; their opponents
//! via `script::opponents`. The self-sacrifice uses the name-filtered
//! `Effect::Sacrifice` idiom.
//! // GAP: fidelity — the "if you do" linkage between the sacrifice and
//! // the draws is not enforced (both effects are sequenced).

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
    let name = reg.interner_mut().intern("Standstill");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: break_standstill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…sacrifice this enchantment. If you do, each of that player's
/// opponents draws three cards."
fn break_standstill(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(caster) = trig.triggering_caster() else {
        return Vec::new();
    };
    let mut effects = vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter {
            name: reg.interner().lookup("Standstill"),
            ..ObjectFilter::default()
        },
        count: 1,
    }];
    for p in script::opponents(state, caster) {
        effects.push(Effect::DrawCards {
            player: p,
            count: 3,
        });
    }
    effects
}
