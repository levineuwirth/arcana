//! Terra Eternal — `{2}{W}` enchantment. "All lands have
//! indestructible."
//!
//! Implementation: an ETB trigger installs a
//! [`ContinuousEffect::filtered_keyword`] granting indestructible to
//! all lands (unscoped — every player's) with duration
//! [`Duration::WhileSourceOnBattlefield`]; the layer-cleanup pipeline
//! auto-expires the effect when the enchantment leaves.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Terra Eternal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
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
                effect: etb_install_indestructible,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "all lands have indestructible" anchored to
/// this enchantment's object id, lasting until it leaves the
/// battlefield.
fn etb_install_indestructible(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_keyword(
            trig.source,
            ObjectFilter {
                types: Some(TypeLine::LAND.into()),
                ..Default::default()
            },
            KeywordAbility::Indestructible,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
