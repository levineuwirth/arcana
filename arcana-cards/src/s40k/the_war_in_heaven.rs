//! The War in Heaven — `{3}{B}{B}{B}` black Enchantment — Saga.
//! I — You draw three cards and you lose 3 life.
//! II — Mill three cards.
//! III — Choose up to three target creature cards with total mana value 8 or less from your graveyard. Return each to battlefield with a necrodermis counter. They're artifacts in addition to their other types.
//! GAP: Chapter III "total mana value 8 or less across multiple targets" — multi-card CMC constraint not in TargetCount.
//! GAP: Chapter III "necrodermis counter" — CounterKind::Necrodermis not in catalog.
//! GAP: Chapter III "they're artifacts in addition to their other types" — type addition not in catalog.
//! Final-chapter sacrifice is automatic (engine SBA).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The War in Heaven");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{3}{B}{B}{B}").expect("valid cost")), colors: ColorSet::black(), types: TypeLine::ENCHANTMENT.into(), subtypes, supertypes: SupertypeSet::default(), ..Default::default() };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters { kind: CounterKind::Lore, count: 1 })
            .with_triggered_ability(TriggeredAbilityDef { id: 1, trigger_condition: TriggerCondition::PhaseBegins { phase: Phase::PreCombatMain, whose: ControllerConstraint::You }, intervening_if: None, effect: add_lore_counter, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef { id: 2, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(1) }, intervening_if: None, effect: chapter_i, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef { id: 3, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(2) }, intervening_if: None, effect: chapter_ii, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef { id: 4, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(3) }, intervening_if: None, effect: chapter_iii, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement { filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::creature() }, count: TargetCount::UpTo(3), controller: None }] }),
    )
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::Lore, count: 1 }]
}

fn chapter_i(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 3 },
        Effect::LoseLife { player: trig.controller, amount: 3 },
    ]
}

fn chapter_ii(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Mill { player: trig.controller, count: 3 }]
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "necrodermis counter", "they're artifacts in addition" — not in catalog
    trig.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t { Some(Effect::ReturnFromGraveyardToBattlefield { target: *id }) } else { None }
    }).collect()
}
