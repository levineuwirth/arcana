//! Statecraft — `{3}{U}` enchantment. "Prevent all combat damage that
//! would be dealt to and dealt by creatures you control."
//!
//! Implementation: an ETB trigger installs TWO
//! `ReplacementCondition::WouldDealDamage` replacements with
//! `ReplacementKind::PreventAllDamage` and `combat: Some(true)`:
//! one for combat damage dealt TO creatures you control (any source),
//! one for combat damage dealt BY creatures you control (any target).
//! Both last while the enchantment is on the battlefield.

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
    let name = reg.interner_mut().intern("Statecraft");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
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
                effect: etb_install_prevention,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install both halves — "prevent all combat damage dealt
/// TO creatures you control" and "prevent all combat damage dealt BY
/// creatures you control" — anchored to this enchantment.
fn etb_install_prevention(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![
        // Combat damage dealt TO creatures you control.
        Effect::InstallReplacementEffect {
            effect: Box::new(ReplacementEffect {
                source: trig.source,
                id: 0,
                condition: ReplacementCondition::WouldDealDamage {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    combat: Some(true),
                },
                kind: ReplacementKind::PreventAllDamage,
                is_self_replacement: false,
                duration: ReplacementDuration::WhileSourceOnBattlefield,
                state_gate: None,
            }),
        },
        // Combat damage dealt BY creatures you control.
        Effect::InstallReplacementEffect {
            effect: Box::new(ReplacementEffect {
                source: trig.source,
                id: 0,
                condition: ReplacementCondition::WouldDealDamage {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::AnyTarget,
                    combat: Some(true),
                },
                kind: ReplacementKind::PreventAllDamage,
                is_self_replacement: false,
                duration: ReplacementDuration::WhileSourceOnBattlefield,
                state_gate: None,
            }),
        },
    ]
}
