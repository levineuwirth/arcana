//! Collective Blessing — `{3}{G}{G}{W}` enchantment. "Creatures you
//! control get +3/+3."
//!
//! Implementation: an ETB trigger installs a
//! [`ContinuousEffect::anthem`] (+3/+3 for your creatures) with
//! [`Duration::WhileSourceOnBattlefield`]; the layer-cleanup pipeline
//! auto-expires it when the enchantment leaves the battlefield.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Collective Blessing");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "creatures you control get +3/+3" anchored to
/// this enchantment, lasting until it leaves the battlefield.
fn etb_install_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::anthem(
            trig.source,
            trig.controller,
            3,
            3,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
