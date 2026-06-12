//! Favorable Winds — `{1}{U}` enchantment. "Creatures you control
//! with flying get +1/+1."
//!
//! Implementation: an ETB trigger installs a filtered pump
//! (`ContinuousEffect::filtered_pump`) over creatures you control
//! with flying, lasting while the enchantment remains on the
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
    let name = reg.interner_mut().intern("Favorable Winds");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
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
                effect: etb_install_flying_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "creatures you control with flying get +1/+1"
/// anchored to this enchantment.
fn etb_install_flying_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            ObjectFilter::creature()
                .controlled_by(ControllerConstraint::You)
                .with_keyword(KeywordAbility::Flying),
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
