//! Consulate Crackdown — `{3}{W}{W}` enchantment.
//! "When this enchantment enters, exile all artifacts your opponents
//! control until this enchantment leaves the battlefield."
//!
//! The ETB mass exile is wired via one `Effect::ExileUntilSourceLeaves`
//! per opponent artifact — the engine returns the whole exiled batch when
//! this enchantment leaves the battlefield (CR 610.3).

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Consulate Crackdown");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: crackdown,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…exile all artifacts your opponents control until this enchantment
/// leaves the battlefield."
fn crackdown(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // One ExileUntilSourceLeaves per opponent artifact: the engine returns
    // each linked card when this enchantment leaves the battlefield.
    let artifacts = script::ids_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::Opponent),
        trig.controller,
    );
    artifacts
        .into_iter()
        .map(|id| Effect::ExileUntilSourceLeaves {
            source: trig.source,
            target: id,
        })
        .collect()
}
