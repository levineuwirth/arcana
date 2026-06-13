//! Arcane Melee — `{4}{U}` enchantment. "Instant and sorcery spells
//! cost {2} less to cast."
//!
//! Implementation: ETB installs
//! `ContinuousEffect::spell_cost_modifier` for instant-OR-sorcery
//! spells (`with_types_any`), ALL casters (the oracle text has no
//! "you cast" — every player's spells are discounted, so
//! `ControllerConstraint::Any`), -2 generic delta,
//! `Duration::WhileSourceOnBattlefield`. The engine floors the
//! generic component at 0; colored pips never change.

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
    let name = reg.interner_mut().intern("Arcane Melee");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
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
                effect: etb_install_discount,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB: install "instant and sorcery spells cost {2} less to cast"
/// for every player, lasting while this enchantment is on the
/// battlefield.
fn etb_install_discount(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::spell_cost_modifier(
            trig.source,
            ObjectFilter::new()
                .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
            ControllerConstraint::Any,
            -2,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
