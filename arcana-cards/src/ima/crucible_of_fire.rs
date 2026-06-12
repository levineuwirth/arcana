//! Crucible of Fire — `{3}{R}` enchantment. "Dragon creatures you
//! control get +3/+3." Tribal layer-7c lord.
//!
//! Implementation: an ETB trigger installs a
//! [`ContinuousEffect::filtered_pump`] scoped to Dragon creatures you
//! control with duration [`Duration::WhileSourceOnBattlefield`]; the
//! layer-cleanup pipeline auto-expires it when the enchantment leaves.

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
    let name = reg.interner_mut().intern("Crucible of Fire");
    // Intern "Dragon" now so the effect fn's lookup is guaranteed to hit.
    let _dragon = reg.interner_mut().intern("Dragon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
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
                effect: etb_install_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "Dragon creatures you control get +3/+3",
/// anchored to this enchantment.
fn etb_install_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dragon = reg
        .interner()
        .lookup("Dragon")
        .expect("Dragon interned during register()");
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            ObjectFilter::creature()
                .controlled_by(ControllerConstraint::You)
                .with_subtype_sym(dragon),
            3,
            3,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
