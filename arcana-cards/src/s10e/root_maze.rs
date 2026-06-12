//! Root Maze — `{G}` enchantment. "Artifacts and lands enter tapped."
//!
//! Implementation: the ETB trigger installs a
//! `ReplacementCondition::WouldEnterBattlefield` /
//! `ReplacementKind::EtbTapped` replacement effect over ALL artifacts
//! and lands (no controller constraint — Root Maze is symmetric),
//! lasting while this enchantment remains on the battlefield.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::replacement::{
    ReplacementCondition, ReplacementDuration, ReplacementEffect, ReplacementKind,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Root Maze");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_tapper,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "artifacts and lands enter the battlefield
/// tapped" (all players) anchored to this enchantment.
fn etb_install_tapper(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallReplacementEffect {
        effect: Box::new(ReplacementEffect {
            source: trig.source,
            id: 0,
            condition: ReplacementCondition::WouldEnterBattlefield {
                object_filter: ObjectFilter::default()
                    .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::LAND)),
            },
            kind: ReplacementKind::EtbTapped,
            is_self_replacement: false,
            duration: ReplacementDuration::WhileSourceOnBattlefield,
        }),
    }]
}
