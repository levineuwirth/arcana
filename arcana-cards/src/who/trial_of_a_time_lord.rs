//! Trial of a Time Lord — `{1}{W}{W}` Enchantment — Saga
//!
//! I, II, III — Exile target nontoken creature an opponent controls until this
//!              Saga leaves the battlefield. (GAP: "until Saga leaves"
//!              duration not in Duration enum; emitting ExilePermanent without
//!              return clause.)
//! IV — Starting with you, each player votes for innocent or guilty. If guilty
//!      gets more votes, the owner of each card exiled with this Saga puts that
//!      card on the bottom of their library. (GAP: voting mechanic not
//!      expressible; emitting Vec::new().)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef, TriggerSelf,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trial of a Time Lord");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    let exile_target = TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::creature()
                .controlled_by(ControllerConstraint::Opponent)
                .nontoken(),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
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
                intervening_if: None, effect: chapter_exile,
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime,
                target_requirements: vec![exile_target.clone()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(2) },
                intervening_if: None, effect: chapter_exile,
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime,
                target_requirements: vec![exile_target.clone()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(3) },
                intervening_if: None, effect: chapter_exile,
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime,
                target_requirements: vec![exile_target],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(4) },
                intervening_if: None, effect: chapter_iv,
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new(),
            }),
    )
}

fn add_lore_counter(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::Lore, count: 1 }]
}

fn chapter_exile(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        vec![Effect::ExilePermanent { target: *id }]
    } else {
        Vec::new()
    }
}

fn chapter_iv(_s: &GameState, _t: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    // GAP: voting mechanic not expressible
    Vec::new()
}
