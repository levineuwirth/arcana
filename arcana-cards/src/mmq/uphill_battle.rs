//! Uphill Battle — `{2}{R}` enchantment. "Creatures played by your
//! opponents enter tapped."
//!
//! Implementation: an ETB trigger installs a `WouldEnterBattlefield` /
//! `EtbTapped` replacement effect scoped to opponent-controlled
//! creatures, lasting while this enchantment is on the battlefield
//! (the Kismet / Frozen Aether pattern).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::replacement::{
    ReplacementCondition, ReplacementDuration, ReplacementEffect, ReplacementKind,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Uphill Battle");
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
                effect: etb_install_static,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: opponents' creatures enter the battlefield tapped while
/// this enchantment remains on the battlefield.
fn etb_install_static(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallReplacementEffect {
        effect: Box::new(ReplacementEffect {
            source: trig.source,
            id: 0,
            condition: ReplacementCondition::WouldEnterBattlefield {
                object_filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::Opponent),
            },
            kind: ReplacementKind::EtbTapped,
            is_self_replacement: false,
            duration: ReplacementDuration::WhileSourceOnBattlefield,
            state_gate: None,
        }),
    }]
}
