//! Dampening Pulse — `{3}{U}` enchantment. "Creatures your opponents
//! control get -1/-0."
//!
//! Implementation: an ETB trigger installs a filtered pump
//! (`ContinuousEffect::filtered_pump`) of -1/-0 over creatures your
//! opponents control, lasting while the enchantment remains on the
//! battlefield. (The controller-scoped `anthem` builder only covers
//! YOUR creatures; the opponent scope is expressed via the filter.)

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
    let name = reg.interner_mut().intern("Dampening Pulse");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
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
                effect: etb_install_opponent_debuff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "creatures your opponents control get -1/-0"
/// anchored to this enchantment.
fn etb_install_opponent_debuff(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
            -1,
            0,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
