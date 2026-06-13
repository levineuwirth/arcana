//! Training Grounds — `{U}` enchantment. "Activated abilities of
//! creatures you control cost {2} less to activate. This effect can't
//! reduce the mana in that cost to less than one mana."
//!
//! Implementation: ETB installs
//! `ContinuousEffect::ability_cost_modifier` over creatures you
//! control with a -2 generic delta and
//! `Duration::WhileSourceOnBattlefield`. The engine floors the generic
//! component at 0 (only the generic component changes; colored pips
//! never do), which renders the "up to {2} less" reduction faithfully;
//! the printed "can't reduce to less than one mana" floor-at-one
//! nuance is a residual approximation (engine floors at zero generic,
//! colored pips always remain).

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
    let name = reg.interner_mut().intern("Training Grounds");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
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

/// ETB: install "activated abilities of creatures you control cost
/// {2} less to activate", lasting while this enchantment is on the
/// battlefield.
fn etb_install_discount(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::ability_cost_modifier(
            trig.source,
            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            -2,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
