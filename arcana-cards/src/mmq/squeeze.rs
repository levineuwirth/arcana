//! Squeeze — `{3}{U}` enchantment. "Sorcery spells cost {3} more to
//! cast."
//!
//! Implementation: ETB installs
//! `ContinuousEffect::spell_cost_modifier` for sorcery spells, all
//! casters (`ControllerConstraint::Any`), +3 generic delta,
//! `Duration::WhileSourceOnBattlefield`.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Squeeze");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
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

/// ETB: install "sorcery spells cost {3} more to cast" for every
/// player, lasting while this enchantment is on the battlefield.
fn etb_install_tax(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::spell_cost_modifier(
            trig.source,
            ObjectFilter::new().with_types(TypeLine::SORCERY.into()),
            ControllerConstraint::Any,
            3,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
