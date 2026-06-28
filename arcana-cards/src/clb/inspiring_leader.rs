//! Inspiring Leader — `{2}{W}` Legendary Enchantment — Background, white.
//! "Commander creatures you own have 'Creature tokens you control get +2/+2.'"
//!
//! Background: ETB installs a `filtered_grant_triggered` continuous effect
//! on commander creatures you control. When a commander creature ETBs it
//! fires an ETB trigger that installs a `filtered_pump` (+2/+2) for creature
//! tokens controlled by that player, anchored to the commander while it
//! remains on the battlefield.
//! Note: "you own" is approximated by "you control" per Background convention.

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
use arcana_core::types::{CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inspiring Leader");
    let background = reg.interner_mut().intern("Background");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes: {
            let mut s = SubtypeSet::default();
            s.insert(background);
            s
        },
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_grant,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Effect fn for the triggered ability granted to each commander creature.
/// Fires when the commander ETBs; installs a filtered_pump for creature tokens
/// controlled by the trigger's controller (+2/+2 while the commander is on the
/// battlefield).
fn granted_trigger_effect(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .tokens_only()
        .controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            filter,
            2,
            2,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// ETB trigger of Inspiring Leader itself. Installs a continuous effect that
/// grants commander creatures you control the `granted_trigger_effect` triggered
/// ability (SelfEntersBattlefield → install token pump).
fn etb_grant(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let granted = TriggeredAbilityDef {
        id: GRANTED_TRIGGER_ID_BASE + 1,
        trigger_condition: TriggerCondition::SelfEntersBattlefield,
        intervening_if: None,
        effect: granted_trigger_effect,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    };
    let filter = ObjectFilter::creature()
        .commander_only()
        .controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_grant_triggered(
            trig.source,
            filter,
            granted,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
