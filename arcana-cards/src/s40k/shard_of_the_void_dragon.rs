//! Shard of the Void Dragon — `{4}{B}{B}{B}` 7/7 C'tan with Flying.
//!
//! * Flying.
//! * Spear of the Void Dragon — Whenever this creature attacks, each opponent
//!   sacrifices a nonland permanent of their choice.
//! * Matter Absorption — Whenever an artifact is put into a graveyard from the
//!   battlefield OR is put into exile from the battlefield, put two +1/+1
//!   counters on this creature. (Two destinations → two ZoneChange triggers.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shard of the Void Dragon");
    let ctan = reg.interner_mut().intern("C'tan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ctan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let artifact_filter = ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into());

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: opponents_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: artifact_filter.clone(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: absorb_matter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: artifact_filter,
                    from: Some(Zone::Battlefield),
                    to: Zone::Exile,
                },
                intervening_if: None,
                effect: absorb_matter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn opponents_sacrifice(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let nonland = ObjectFilter::permanent().without_types(TypeLine::LAND.into());
    let effects = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::Sacrifice {
            player: p,
            filter: nonland.clone(),
            count: 1,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}

fn absorb_matter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}
