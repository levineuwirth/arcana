//! Boon Reflection — `{4}{W}` enchantment. "If you would gain life,
//! you gain twice that much life instead."
//!
//! Implementation: an ETB trigger installs a
//! `ReplacementCondition::WouldGainLife` replacement with
//! `ReplacementKind::MultiplyLifeGain(2)`, scoped to the
//! enchantment's controller, lasting while it remains on the
//! battlefield.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::replacement::{
    ReplacementCondition, ReplacementDuration, ReplacementEffect, ReplacementKind,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boon Reflection");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
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
                effect: etb_install_double_lifegain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "if you would gain life, you gain twice that
/// much instead" anchored to this enchantment.
fn etb_install_double_lifegain(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallReplacementEffect {
        effect: Box::new(ReplacementEffect {
            source: trig.source,
            id: 0,
            condition: ReplacementCondition::WouldGainLife {
                player: ControllerConstraint::You,
            },
            kind: ReplacementKind::MultiplyLifeGain(2),
            is_self_replacement: false,
            duration: ReplacementDuration::WhileSourceOnBattlefield,
            state_gate: None,
        }),
    }]
}
