//! The Flux — `{2}{R}{R}` Enchantment — Saga
//!
//! I — Deals 4 damage to target creature an opponent controls.
//! II, III, IV, V — Exile the top card of your library. You may play that
//!                  card this turn. (GAP: exile-with-play-permission not in
//!                  Effect API; emitting Vec::new().)
//! VI — Add six {R}.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef, TriggerSelf,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Flux");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    let mut def = CardDefinition::new(name, chars)
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
        });

    for ch in 2u32..=5 {
        def = def.with_triggered_ability(TriggeredAbilityDef {
            id: ch + 1, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(ch) },
            intervening_if: None, effect: chapter_mid,
            trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new(),
        });
    }

    def = def.with_triggered_ability(TriggeredAbilityDef {
        id: 7, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(6) },
        intervening_if: None, effect: chapter_vi,
        trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new(),
    });

    reg.register(def)
}

fn add_lore_counter(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::Lore, count: 1 }]
}

fn chapter_i(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else { return Vec::new(); };
    vec![Effect::DealDamage { target: DamageTarget::Object(*id), amount: 4, source: trig.source }]
}

fn chapter_mid(_s: &GameState, _trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    // GAP: exile-with-play-permission this turn
    Vec::new()
}

fn chapter_vi(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    let mana = vec![ManaUnit::plain(ManaColor::Red, trig.source); 6];
    vec![Effect::AddMana { player: trig.controller, mana }]
}
