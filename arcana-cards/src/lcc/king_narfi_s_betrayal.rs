//! King Narfi's Betrayal — `{1}{U}{B}` blue/black Enchantment — Saga.
//! I — Each player mills four cards. Then you may exile a creature or planeswalker card from each graveyard.
//! II, III — Until end of turn, you may cast spells from cards exiled with this Saga, any mana color.
//! GAP: Chapter I "exile a creature or planeswalker card from each graveyard" — per-graveyard exile choice not in catalog.
//! GAP: Chapter II/III "cast from exile, any color" — cast-from-exile not in catalog.
//! Final-chapter sacrifice is automatic (engine SBA).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("King Narfi's Betrayal");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")), colors: ColorSet::blue() | ColorSet::black(), types: TypeLine::ENCHANTMENT.into(), subtypes, supertypes: SupertypeSet::default(), ..Default::default() };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters { kind: CounterKind::Lore, count: 1 })
            .with_triggered_ability(TriggeredAbilityDef { id: 1, trigger_condition: TriggerCondition::PhaseBegins { phase: Phase::PreCombatMain, whose: ControllerConstraint::You }, intervening_if: None, effect: add_lore_counter, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef { id: 2, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(1) }, intervening_if: None, effect: chapter_i, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef { id: 3, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(2) }, intervening_if: None, effect: chapter_gap, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef { id: 4, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(3) }, intervening_if: None, effect: chapter_gap, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() }),
    )
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::Lore, count: 1 }]
}

fn chapter_i(state: &GameState, _trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    script::all_players(state).into_iter().map(|p| Effect::Mill { player: p, count: 4 }).collect()
    // GAP: "exile a creature or planeswalker card from each graveyard" — not in catalog
}

fn chapter_gap(_state: &GameState, _trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "cast from exile with any color mana" — not in catalog
    Vec::new()
}
