//! Kismet — `{3}{W}` enchantment. "Artifacts, creatures, and lands
//! your opponents control enter tapped."
//!
//! Implementation: an ETB trigger installs a `WouldEnterBattlefield`
//! replacement effect with kind `EtbTapped`, scoped to artifacts,
//! creatures, and lands controlled by opponents, lasting while Kismet
//! remains on the battlefield.

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
    let name = reg.interner_mut().intern("Kismet");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
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
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallReplacementEffect {
        effect: Box::new(ReplacementEffect {
            source: trig.source,
            id: 0,
            condition: ReplacementCondition::WouldEnterBattlefield {
                object_filter: ObjectFilter {
                    types: Some(
                        (TypeLine::ARTIFACT | TypeLine::CREATURE | TypeLine::LAND)
                            .into(),
                    ),
                    ..Default::default()
                }
                .controlled_by(ControllerConstraint::Opponent),
            },
            kind: ReplacementKind::EtbTapped,
            is_self_replacement: false,
            duration: ReplacementDuration::WhileSourceOnBattlefield,
        }),
    }]
}
