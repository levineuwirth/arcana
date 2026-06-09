//! Summon: Brynhildr — `{1}{R}` red Enchantment Creature — Saga Knight. 2/1. 3 chapters.
//! I — Chain — Exile the top card of your library. During any turn you put a lore counter on this Saga, you may play that card.
//! II, III — Gestalt Mode — When you next cast a creature spell this turn, it gains haste until end of turn.
//! GAP: Chapter I "exile top card, play it on turns you add a lore counter" — conditional play-from-exile not in catalog.
//! Chapter II/III wired via `Effect::NextCastThisTurn { Creature, GainsHaste }`.
//! Final-chapter sacrifice is automatic (engine SBA).

use arcana_core::effects::{Effect, NextCastKind, NextCastRider};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Summon: Brynhildr");
    let saga_sub = reg.interner_mut().intern("Saga");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    subtypes.0.insert(knight_sub);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")), colors: ColorSet::red(), types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(1)), ..Default::default() };
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

fn chapter_i(_state: &GameState, _trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "exile top card, play it on turns you add a lore counter" — conditional play-from-exile not in catalog
    Vec::new()
}

fn chapter_gap(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // II/III — "when you next cast a creature spell this turn, it gains haste until end of turn"
    vec![Effect::NextCastThisTurn { controller: trig.controller, kind: NextCastKind::Creature, rider: NextCastRider::GainsHaste }]
}
