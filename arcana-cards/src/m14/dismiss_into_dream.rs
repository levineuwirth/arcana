//! Dismiss into Dream — `{6}{U}` enchantment. "Each creature your
//! opponents control is an Illusion in addition to its other types
//! and has 'When this creature becomes the target of a spell or
//! ability, sacrifice it.'"
//!
//! Implementation (partial): an ETB trigger installs a
//! `ContinuousEffect::filtered_grant_triggered` over creatures your
//! opponents control, granting the becomes-target sacrifice trigger
//! (granted id `GRANTED_TRIGGER_ID_BASE + 1`). The granted ability
//! mirrors the catalog's Phantom Beast pattern, including its
//! approximation of "sacrifice it" as "its controller sacrifices a
//! creature" (`Effect::Sacrifice` is filter-based; there is no
//! sacrifice-this-specific-object effect).
//!
//! GAP: "is an Illusion in addition to its other types" — there is no
//! subtype-addition continuous-effect builder.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dismiss into Dream");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}").expect("valid cost")),
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
                effect: etb_install_grant,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: grant "When this creature becomes the target of a
/// spell or ability, sacrifice it." to each creature your opponents
/// control, anchored to this enchantment.
fn etb_install_grant(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_grant_triggered(
            trig.source,
            ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
            TriggeredAbilityDef {
                id: GRANTED_TRIGGER_ID_BASE + 1,
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: granted_on_becomes_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Granted trigger: "sacrifice it" — approximated as the creature's
/// controller sacrifices a creature (mirrors the catalog's Phantom
/// Beast implementation of the identical text).
fn granted_on_becomes_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::creature(),
        count: 1,
    }]
}
