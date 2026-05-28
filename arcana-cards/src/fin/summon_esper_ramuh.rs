//! Summon: Esper Ramuh — `{2}{R}{R}` Enchantment Creature — Saga Wizard (3/3)
//!
//! I — Judgment Bolt — Deals damage equal to the number of noncreature,
//!     nonland cards in your graveyard to target creature an opponent controls.
//!     (GAP: graveyard filter for noncreature nonland not in script API;
//!     using graveyard_size as approximation.)
//! II, III — Wizards you control get +1/+0 until end of turn.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef, TriggerSelf,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Summon: Esper Ramuh");
    let saga_sub = reg.interner_mut().intern("Saga");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    subtypes.0.insert(wizard_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters { kind: CounterKind::Lore, count: 1 })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1, trigger_condition: TriggerCondition::PhaseBegins { phase: Phase::PreCombatMain, whose: ControllerConstraint::You },
                intervening_if: None, effect: add_lore_counter,
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(1) },
                intervening_if: None, effect: chapter_i,
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent)),
                    count: TargetCount::Exactly(1), controller: None,
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(2) },
                intervening_if: None, effect: chapter_ii_iii,
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(3) },
                intervening_if: None, effect: chapter_ii_iii,
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new(),
            }),
    )
}

fn add_lore_counter(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::Lore, count: 1 }]
}

fn chapter_i(state: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else { return Vec::new(); };
    // GAP: graveyard_size as approximation for noncreature nonland count
    let amount = script::graveyard_size(state, trig.controller);
    if amount == 0 { return Vec::new(); }
    vec![Effect::DealDamage { target: DamageTarget::Object(*id), amount, source: trig.source }]
}

fn chapter_ii_iii(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let wizard_filter = script::subtype_filter(reg, "Wizard").controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &wizard_filter, trig.controller);
    ids.into_iter()
        .map(|id| Effect::Pump { target: id, power: 1, toughness: 0, duration: Duration::EndOfTurn, keywords: vec![] })
        .collect()
}
