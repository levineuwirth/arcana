//! Serra's Blessing — `{1}{W}` enchantment. "Creatures you control
//! have vigilance."
//!
//! Implementation: an ETB trigger installs a
//! [`ContinuousEffect::keyword_anthem`] (vigilance,
//! controller-scoped) with duration
//! [`Duration::WhileSourceOnBattlefield`]; the layer-cleanup pipeline
//! auto-expires the effect when the enchantment leaves.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Serra's Blessing");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
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
                effect: etb_install_vigilance,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "creatures you control have vigilance"
/// anchored to this enchantment's object id, lasting until it leaves
/// the battlefield.
fn etb_install_vigilance(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::keyword_anthem(
            trig.source,
            trig.controller,
            KeywordAbility::Vigilance,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
