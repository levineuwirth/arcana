//! Familiar Ground — `{2}{G}` enchantment. "Each creature you control
//! can't be blocked by more than one creature." Block cap: an ETB
//! trigger installs a `filtered_max_blockers` continuous effect
//! (creatures you control, cap 1) with
//! `Duration::WhileSourceOnBattlefield`, auto-expired by the
//! layer-cleanup pipeline when the enchantment leaves.

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
    let name = reg.interner_mut().intern("Familiar Ground");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
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
                effect: etb_install_max_blockers,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "each creature you control can't be blocked by
/// more than one creature" anchored to this enchantment's object id,
/// lasting until it leaves the battlefield.
fn etb_install_max_blockers(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_max_blockers(
            trig.source,
            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
