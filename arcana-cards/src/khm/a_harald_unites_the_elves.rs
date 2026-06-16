//! A-Harald Unites the Elves — `{2}{B}{G}` black/green Enchantment — Saga.
//! I — Mill five cards. You may put an Elf or Tyvar card from your graveyard onto the battlefield.
//! II — Put a +1/+1 counter on each Elf you control.
//! III — Whenever an Elf you control attacks this turn, target creature an opponent controls gets -1/-1 until end of turn.
//! GAP: Chapter I "Elf or Tyvar card" — Tyvar filter not in ObjectFilter.
//! GAP: Chapter III "whenever an Elf attacks this turn" — delayed this-turn trigger not in catalog.
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
    let name = reg.interner_mut().intern("A-Harald Unites the Elves");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")), colors: ColorSet::black() | ColorSet::green(), types: TypeLine::ENCHANTMENT.into(), subtypes, supertypes: SupertypeSet::default(), ..Default::default() };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters { kind: CounterKind::Lore, count: 1 })
            .with_triggered_ability(TriggeredAbilityDef { id: 1, trigger_condition: TriggerCondition::PhaseBegins { phase: Phase::PreCombatMain, whose: ControllerConstraint::You }, intervening_if: None, effect: add_lore_counter, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef { id: 2, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(1) }, intervening_if: None, effect: chapter_i, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef { id: 3, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(2) }, intervening_if: None, effect: chapter_ii, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef { id: 4, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(3) }, intervening_if: None, effect: chapter_iii, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() }),
    )
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::Lore, count: 1 }]
}

fn chapter_i(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may put an Elf or Tyvar card from your graveyard onto the battlefield" — Reanimate with Elf/Tyvar filter
    //   not achievable at resolve time (no interner access); emitting Mill only
    vec![Effect::Mill { player: trig.controller, count: 5 }]
}

fn chapter_ii(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let elf_filter = script::subtype_filter(reg, "Elf").controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &elf_filter, trig.controller);
    ids.into_iter().map(|id| Effect::AddCounters { target: id, kind: CounterKind::PlusOnePlusOne, count: 1 }).collect()
}

fn chapter_iii(_state: &GameState, _trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "whenever an Elf you control attacks this turn, target creature -1/-1" — delayed this-turn trigger not in catalog
    Vec::new()
}
