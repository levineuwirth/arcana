//! Suppression Field — `{1}{W}` enchantment. "Activated abilities cost
//! {2} more to activate unless they're mana abilities."
//!
//! Implementation: ETB installs
//! `ContinuousEffect::ability_cost_modifier` over ALL permanents
//! (unscoped `ObjectFilter::permanent()`) with a +2 generic delta and
//! `Duration::WhileSourceOnBattlefield`.
//!
//! Approximations: the oracle text exempts mana abilities and also
//! reaches abilities activated from non-battlefield zones — the
//! modifier surface taxes activated abilities of permanents; the
//! mana-ability carve-out is not separately expressible and is noted
//! here as a residual gap.

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
    let name = reg.interner_mut().intern("Suppression Field");
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
                effect: etb_install_tax,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB: install "activated abilities cost {2} more to activate" for
/// all permanents, lasting while this enchantment is on the
/// battlefield.
fn etb_install_tax(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP (partial): the "unless they're mana abilities" exemption is
    // not expressible on the cost-modifier surface.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::ability_cost_modifier(
            trig.source,
            ObjectFilter::permanent(),
            2,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
