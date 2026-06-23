//! Street Riot — `{4}{R}` enchantment. "During your turn, creatures you
//! control get +1/+0 and have trample."
//!
//! Implementation: an ETB trigger installs TWO continuous effects over
//! creatures the controller controls, both with
//! `Duration::WhileControllerTurn` (live only while the source's
//! controller is the active player; dims off-turn — the faithful
//! "during your turn" gate):
//!   1. a `filtered_pump` of +1/+0, and
//!   2. a `filtered_keyword` granting Trample.
//! The layer-cleanup pipeline auto-expires both when the enchantment
//! leaves the battlefield.

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
    let name = reg.interner_mut().intern("Street Riot");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
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

/// ETB trigger: install "during your turn, creatures you control get
/// +1/+0 and have trample" as a +1/+0 filtered pump plus a Trample
/// filtered keyword, both `WhileControllerTurn`.
fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let yours = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump(
                trig.source,
                yours.clone(),
                1,
                0,
                Duration::WhileControllerTurn,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                yours,
                KeywordAbility::Trample,
                Duration::WhileControllerTurn,
            ),
        },
    ]
}
