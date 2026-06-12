//! Time of Heroes — `{1}{W}` enchantment. "Each creature you control
//! with a level counter on it gets +2/+2."
//!
//! Implementation: an ETB trigger installs a counter-gated
//! [`ContinuousEffect::filtered_pump`] (+2/+2 to creatures you control
//! with a level counter) with [`Duration::WhileSourceOnBattlefield`];
//! the layer-cleanup pipeline auto-expires it when the enchantment
//! leaves the battlefield.

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
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Time of Heroes");
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
                effect: etb_install_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "each creature you control with a level counter
/// on it gets +2/+2" anchored to this enchantment, lasting until it
/// leaves the battlefield.
fn etb_install_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter {
        has_counter: Some(CounterKind::Level),
        ..ObjectFilter::creature().controlled_by(ControllerConstraint::You)
    };
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            filter,
            2,
            2,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
