//! High Seas — `{2}{U}` enchantment. "Red creature spells and green
//! creature spells cost {1} more to cast."
//!
//! Implementation: ETB installs TWO
//! `ContinuousEffect::spell_cost_modifier` effects (the filter's
//! `colors` mask is an AND, so the red-OR-green disjunction is
//! rendered as one modifier per color): red creature spells +1 and
//! green creature spells +1, each for all casters
//! (`ControllerConstraint::Any`) with
//! `Duration::WhileSourceOnBattlefield`. A spell that is BOTH red and
//! green (e.g. a {R/G} creature) matches both modifiers and would be
//! taxed {2} instead of {1} — noted as a residual approximation.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
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
    let name = reg.interner_mut().intern("High Seas");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_taxes,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB: install "+{1} to cast" for red creature spells and for green
/// creature spells, for every player, while this enchantment is on
/// the battlefield.
fn etb_install_taxes(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::spell_cost_modifier(
                trig.source,
                ObjectFilter::creature().with_colors(ColorSet::red()),
                ControllerConstraint::Any,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::spell_cost_modifier(
                trig.source,
                ObjectFilter::creature().with_colors(ColorSet::green()),
                ControllerConstraint::Any,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
