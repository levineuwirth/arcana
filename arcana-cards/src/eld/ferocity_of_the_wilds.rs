//! Ferocity of the Wilds — `{2}{R}` enchantment. "Attacking non-Human
//! creatures you control get +1/+0 and have trample."
//!
//! Implementation: an ETB trigger installs TWO continuous effects over
//! attacking non-Human creatures you control — a +1/+0 filtered pump
//! and a Trample keyword grant — each lasting while this enchantment
//! remains on the battlefield.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Ferocity of the Wilds");
    // Pre-intern the "Human" subtype so the effect fn can look it up at
    // resolution time (effect fns receive `&CardRegistry`, not the mut
    // interner).
    let _human = reg.interner_mut().intern("Human");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
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
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "attacking non-Human creatures you control get
/// +1/+0 and have trample".
fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let human = reg
        .interner()
        .lookup("Human")
        .expect("Human subtype interned at register time");
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .attacking_only()
        .without_subtype_sym(human);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump(
                trig.source,
                filter.clone(),
                1,
                0,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                filter,
                KeywordAbility::Trample,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
