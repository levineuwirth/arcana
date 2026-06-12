//! Always Watching — `{1}{W}{W}` enchantment. "Nontoken creatures you
//! control get +1/+1 and have vigilance."
//!
//! Implementation: an ETB trigger installs TWO continuous effects —
//! a [`ContinuousEffect::filtered_pump`] (+1/+1) and a
//! [`ContinuousEffect::filtered_keyword`] (vigilance), both scoped to
//! NONTOKEN creatures you control (via the `ObjectFilter::nontoken`
//! builder), with duration [`Duration::WhileSourceOnBattlefield`]; the
//! layer-cleanup pipeline auto-expires them when the enchantment
//! leaves.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Always Watching");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_nontoken_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "+1/+1" and "vigilance" for nontoken creatures
/// you control, anchored to this enchantment, lasting while it remains
/// on the battlefield.
fn etb_install_nontoken_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump(
                trig.source,
                ObjectFilter::creature()
                    .nontoken()
                    .controlled_by(ControllerConstraint::You),
                1,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                ObjectFilter::creature()
                    .nontoken()
                    .controlled_by(ControllerConstraint::You),
                KeywordAbility::Vigilance,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
