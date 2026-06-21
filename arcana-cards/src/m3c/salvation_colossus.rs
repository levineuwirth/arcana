//! Salvation Colossus — `{6}{W}{W}` 9/9 Artifact Creature — Construct.
//! Flying, vigilance, trample.
//! Whenever you attack, other creatures you control get +2/+2 and gain
//! indestructible until end of turn.
//! Unearth—Pay eight {E}.
//!
//! Unearth with an energy cost is not an expressible keyword/ability shape
//! (no energy-cost activation cost field) — GAP'd below.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Salvation Colossus");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(9)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
        ],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "Whenever you attack" has no dedicated trigger; SelfAttacks
            // is the closest (fires when this creature attacks, not on every
            // attack you make).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_other_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP: Unearth—Pay eight {E} — no energy-cost activation cost field.
}

fn pump_other_creatures(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids: Vec<_> = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    )
    .into_iter()
    .filter(|&id| id != trig.source)
    .collect();
    if ids.is_empty() {
        return Vec::new();
    }
    vec![
        Effect::ForEach {
            targets: ids.clone(),
            effect: Box::new(Effect::Pump {
                target: NULL_OBJECT_ID,
                power: 2,
                toughness: 2,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            }),
        },
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::GrantKeyword {
                target: NULL_OBJECT_ID,
                keyword: KeywordAbility::Indestructible,
                duration: Duration::EndOfTurn,
            }),
        },
    ]
}
