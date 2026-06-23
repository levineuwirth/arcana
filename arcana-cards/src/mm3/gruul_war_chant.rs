//! Gruul War Chant — `{2}{R}{G}` enchantment. "Attacking creatures you
//! control get +1/+0 and have menace."
//!
//! Implementation: an ETB trigger installs TWO continuous effects over
//! attacking creatures you control — a +1/+0 filtered pump and a
//! filtered Menace grant — each lasting while this enchantment is on the
//! battlefield.

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
    let name = reg.interner_mut().intern("Gruul War Chant");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "attacking creatures you control get +1/+0 and
/// have menace" as a +1/+0 pump plus a Menace grant, both anchored to
/// this enchantment.
fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump(
                trig.source,
                ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .attacking_only(),
                1,
                0,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .attacking_only(),
                KeywordAbility::Menace,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
