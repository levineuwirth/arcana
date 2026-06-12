//! Sphere of Safety — `{4}{W}` enchantment. "Creatures can't attack
//! you or planeswalkers you control unless their controller pays {X}
//! for each of those creatures, where X is the number of enchantments
//! you control."
//!
//! Implementation: the Ghostly Prison-class
//! [`ContinuousEffect::attack_tax`] builder, installed on ETB with
//! duration [`Duration::WhileSourceOnBattlefield`]. The tax amount is
//! computed when the trigger resolves (the number of enchantments you
//! control at that moment) — see the GAP note in the effect fn.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sphere of Safety");
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
                effect: etb_install_tax,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install the attack tax anchored to this enchantment,
/// lasting until it leaves the battlefield.
fn etb_install_tax(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: X is dynamic ("the number of enchantments you control",
    // re-evaluated whenever the tax applies); attack_tax takes a
    // fixed amount, so this snapshots the enchantment count at the
    // moment the ETB trigger resolves (always >= 1, since this
    // enchantment is on the battlefield).
    let enchantments = ObjectFilter {
        types: Some(TypeLine::ENCHANTMENT.into()),
        ..Default::default()
    }
    .controlled_by(ControllerConstraint::You);
    let x = script::count_matching(state, &enchantments, trig.controller) as u32;
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attack_tax(
            trig.source,
            x,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
