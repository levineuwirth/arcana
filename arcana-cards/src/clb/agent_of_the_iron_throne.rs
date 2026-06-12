//! Agent of the Iron Throne — `{2}{B}` Legendary Enchantment —
//! Background. "Commander creatures you own have 'Whenever an artifact
//! or creature you control is put into a graveyard from the
//! battlefield, each opponent loses 1 life.'"
//!
//! Implementation: ETB installs a commander-filtered triggered-ability
//! grant (`ContinuousEffect::filtered_grant_triggered`) with
//! `Duration::WhileSourceOnBattlefield`. The granted ability fires on
//! a ZoneChange (battlefield → graveyard) of an artifact or creature
//! you control and drains each opponent for 1. 'you own' is
//! approximated by `controlled_by(ControllerConstraint::You)`.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Agent of the Iron Throne");
    let background = reg.interner_mut().intern("Background");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(background);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
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

/// ETB: install the commander grant of "Whenever an artifact or
/// creature you control is put into a graveyard from the battlefield,
/// each opponent loses 1 life."
fn etb_install_grant(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_grant_triggered(
            trig.source,
            ObjectFilter::creature()
                .commander_only()
                .controlled_by(ControllerConstraint::You),
            TriggeredAbilityDef {
                id: arcana_core::triggers::GRANTED_TRIGGER_ID_BASE + 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types_any(TypeLine(
                            TypeLine::ARTIFACT | TypeLine::CREATURE,
                        ))
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: granted_each_opponent_loses_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Granted ability: each opponent loses 1 life.
fn granted_each_opponent_loses_one(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::LoseLife { player: p, amount: 1 })
        .collect()
}
