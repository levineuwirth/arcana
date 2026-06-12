//! Illness in the Ranks — `{B}` enchantment. "Creature tokens get
//! -1/-1."
//!
//! Implementation: an ETB trigger installs a
//! [`ContinuousEffect::filtered_pump`] of -1/-1 scoped to creature
//! tokens (unscoped controller — all players' tokens) with duration
//! [`Duration::WhileSourceOnBattlefield`]; the layer-cleanup pipeline
//! auto-expires it when the enchantment leaves.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Illness in the Ranks");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_token_debuff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "creature tokens get -1/-1" (all controllers)
/// anchored to this enchantment, lasting while it remains on the
/// battlefield.
fn etb_install_token_debuff(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            ObjectFilter::creature().tokens_only(),
            -1,
            -1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
