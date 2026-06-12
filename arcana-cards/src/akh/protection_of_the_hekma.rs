//! Protection of the Hekma — `{4}{W}` enchantment. "If a source an
//! opponent controls would deal damage to you, prevent 1 of that
//! damage."
//!
//! Implementation: an ETB trigger installs a
//! `ReplacementCondition::WouldDealDamage` replacement scoped to
//! opponent-controlled sources with
//! `ReplacementKind::PreventDamageUpTo(1)` (applies per damage event,
//! non-depleting), lasting while the enchantment is on the
//! battlefield.
//!
//! Approximation: "to you" — `TargetFilter::Player` matches damage
//! dealt to ANY player (there is no controller-scoped player target
//! filter on `WouldDealDamage`), so the prevention is over-broad on
//! the player dimension.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::replacement::{
    ReplacementCondition, ReplacementDuration, ReplacementEffect, ReplacementKind,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Protection of the Hekma");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
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
                effect: etb_install_prevention,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "if a source an opponent controls would deal
/// damage to you, prevent 1 of that damage" anchored to this
/// enchantment.
fn etb_install_prevention(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallReplacementEffect {
        effect: Box::new(ReplacementEffect {
            source: trig.source,
            id: 0,
            condition: ReplacementCondition::WouldDealDamage {
                source_filter: ObjectFilter::default()
                    .controlled_by(ControllerConstraint::Opponent),
                target_filter: TargetFilter::Player,
                combat: None,
            },
            kind: ReplacementKind::PreventDamageUpTo(1),
            is_self_replacement: false,
            duration: ReplacementDuration::WhileSourceOnBattlefield,
            state_gate: None,
        }),
    }]
}
