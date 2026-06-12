//! Gratuitous Violence — `{2}{R}{R}{R}` enchantment. "If a creature you
//! control would deal damage to a permanent or player, it deals double
//! that damage instead."
//!
//! Implementation: an ETB trigger installs a replacement effect
//! (`ReplacementCondition::WouldDealDamage` from creatures you control,
//! `ReplacementKind::DoubleDamage`) with
//! `ReplacementDuration::WhileSourceOnBattlefield`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::replacement::{
    ReplacementCondition, ReplacementDuration, ReplacementEffect, ReplacementKind,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gratuitous Violence");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_double_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "if a creature you control would deal damage to a
/// permanent or player, it deals double that damage instead", anchored to
/// this enchantment and lasting until it leaves the battlefield.
fn etb_install_double_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallReplacementEffect {
        effect: Box::new(ReplacementEffect {
            source: trig.source,
            id: 0,
            condition: ReplacementCondition::WouldDealDamage {
                source_filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::AnyTarget,
                combat: None,
            },
            kind: ReplacementKind::DoubleDamage,
            is_self_replacement: false,
            duration: ReplacementDuration::WhileSourceOnBattlefield,
            state_gate: None,
        }),
    }]
}
