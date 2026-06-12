//! Light of Day — `{3}{W}` enchantment. "Black creatures can't attack
//! or block."
//!
//! Implementation: two ETB-installed restrictions
//! (`ContinuousEffect::filtered_cant_attack` +
//! `ContinuousEffect::filtered_cant_block`) over black creatures
//! (unscoped — all players'), with
//! `Duration::WhileSourceOnBattlefield`.

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
    let name = reg.interner_mut().intern("Light of Day");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
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
                effect: etb_install_restrictions,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "black creatures can't attack" and "black
/// creatures can't block", each lasting until this enchantment
/// leaves the battlefield.
fn etb_install_restrictions(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_cant_attack(
                trig.source,
                ObjectFilter::creature().with_colors(ColorSet::black()),
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_cant_block(
                trig.source,
                ObjectFilter::creature().with_colors(ColorSet::black()),
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
