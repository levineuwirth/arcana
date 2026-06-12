//! Oubliette — `{1}{B}{B}` enchantment.
//! "When this enchantment enters, target creature phases out until this
//! enchantment leaves the battlefield. Tap that creature as it phases in
//! this way."
//!
//! Phasing (CR 702.26) is not modeled. The closest available behavior is
//! `Effect::ExileUntilSourceLeaves` (the pre-errata Oubliette behavior) —
//! the engine returns the card when this enchantment leaves the
//! battlefield. GAP: phasing semantics (attachments/counters preserved)
//! and the tapped re-entry are not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetChoice;
use arcana_core::targets::TargetRequirement;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oubliette");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: oubliette_remove,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn oubliette_remove(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // Phasing approximated as exile-until-source-leaves (pre-errata
    // behavior): the engine returns the creature when this enchantment
    // leaves the battlefield.
    // GAP: true phasing semantics and the tapped re-entry are not modeled.
    vec![Effect::ExileUntilSourceLeaves {
        source: trig.source,
        target: *id,
    }]
}
