//! A-Briar Hydra — `{3}{G}` 5/5 Plant Hydra with Trample.
//! "Domain — Whenever Briar Hydra deals combat damage to a player, put
//!  X +1/+1 counters on target creature you control, where X is the
//!  number of basic land types among lands you control."
//!
//! Trample is wired and the combat-damage-to-player trigger is wired
//! with a "target creature you control" requirement. GAP: X is the
//! domain count (number of basic land types among lands you control)
//! and there is no script helper for basic-land-type / domain counting,
//! so the dynamic amount can't be computed — per the dynamic-amount
//! rule the whole counter effect is GAP'd rather than hardcoded.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Briar Hydra");
    let plant = reg.interner_mut().intern("Plant");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter {
                    name: Some(name),
                    ..ObjectFilter::default()
                },
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: domain_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn domain_counters(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: X = number of basic land types among lands you control
    // (domain) — no script helper for domain / basic-land-type counting.
    Vec::new()
}
