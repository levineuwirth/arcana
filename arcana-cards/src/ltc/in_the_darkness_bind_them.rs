//! In the Darkness Bind Them — `{2}{U}{B}{R}` blue/black/red Enchantment — Saga. 4 chapters.
//! I, II, III — Create a 3/3 black Wraith creature token with menace. The Ring tempts you.
//! IV — For each opponent, gain control of up to one target creature until end of turn.
//!   Untap those creatures. They gain haste until end of turn. The Ring tempts you.
//! GAP: "The Ring tempts you" — Ring mechanic not in catalog.
//! GAP: Chapter IV "for each opponent, up to one target creature" — multi-target-per-opponent selection not in catalog.
//! Final-chapter sacrifice is automatic (engine SBA).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("In the Darkness Bind Them");
    let saga_sub = reg.interner_mut().intern("Saga");
    let _wraith = reg.interner_mut().intern("Wraith");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{2}{U}{B}{R}").expect("valid cost")), colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(), types: TypeLine::ENCHANTMENT.into(), subtypes, supertypes: SupertypeSet::default(), ..Default::default() };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters { kind: CounterKind::Lore, count: 1 })
            .with_triggered_ability(TriggeredAbilityDef { id: 1, trigger_condition: TriggerCondition::PhaseBegins { phase: Phase::PreCombatMain, whose: ControllerConstraint::You }, intervening_if: None, effect: add_lore_counter, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef { id: 2, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(1) }, intervening_if: None, effect: chapter_i_iii, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef { id: 3, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(2) }, intervening_if: None, effect: chapter_i_iii, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef { id: 4, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(3) }, intervening_if: None, effect: chapter_i_iii, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
                trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(4) },
                intervening_if: None, effect: chapter_iv, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement { filter: TargetFilter::Permanent(ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent)), count: TargetCount::UpTo(1), controller: None }],
            }),
    )
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::Lore, count: 1 }]
}

fn chapter_i_iii(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let wraith = reg.interner().lookup("Wraith").expect("interned at register");
    let mut ts = SubtypeSet::default();
    ts.0.insert(wraith);
    let token = TokenDefinition { name: wraith, colors: ColorSet::black(), types: TypeLine::CREATURE.into(), subtypes: ts, power: Some(PtValue::Fixed(3)), toughness: Some(PtValue::Fixed(3)), keywords: vec![KeywordAbility::Menace], abilities: vec![] };
    // GAP: "The Ring tempts you" — Ring mechanic not in catalog
    vec![Effect::CreateToken { controller: trig.controller, token }]
}

fn chapter_iv(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::ChangeControlEot { target: *id, new_controller: trig.controller },
        Effect::Untap { target: *id },
        Effect::GrantKeyword { target: *id, keyword: KeywordAbility::Haste, duration: Duration::EndOfTurn },
    ]
}
