//! Parallel Lives — `{3}{G}` enchantment. "If an effect would create
//! one or more tokens under your control, it creates twice that many
//! of those tokens instead."
//!
//! Implementation: an ETB trigger installs a replacement effect
//! ([`ReplacementCondition::WouldCreateToken`] filtered to tokens
//! created under your control, [`ReplacementKind::MultiplyTokens`]
//! ×2) with [`ReplacementDuration::WhileSourceOnBattlefield`]; the
//! replacement pipeline auto-expires it when the enchantment leaves.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::replacement::{
    ReplacementCondition, ReplacementDuration, ReplacementEffect,
    ReplacementKind,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Parallel Lives");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_token_doubler,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "token creation under your control is
/// doubled" anchored to this enchantment, lasting while it remains on
/// the battlefield.
fn etb_install_token_doubler(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallReplacementEffect {
        effect: Box::new(ReplacementEffect {
            source: trig.source,
            id: 0,
            condition: ReplacementCondition::WouldCreateToken {
                token_filter: ObjectFilter::default()
                    .controlled_by(ControllerConstraint::You),
            },
            kind: ReplacementKind::MultiplyTokens(2),
            is_self_replacement: false,
            duration: ReplacementDuration::WhileSourceOnBattlefield,
            state_gate: None,
        }),
    }]
}
