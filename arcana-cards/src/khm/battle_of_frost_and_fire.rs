//! Battle of Frost and Fire — `{3}{U}{R}` Enchantment — Saga
//!
//! I — This Saga deals 4 damage to each non-Giant creature and each
//!     planeswalker. (GAP: no ObjectFilter for non-Giant; affects all creatures.)
//! II — Scry 3.
//! III — Whenever you cast a spell with mana value 5 or greater this turn,
//!        draw two cards, then discard a card. Wired via
//!        `Effect::EachCastThisTurn { AnyMinCmc(5), DrawTwoDiscardOne }`.

use arcana_core::effects::{EachCastRider, Effect, NextCastKind};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef, TriggerSelf,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Battle of Frost and Fire");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
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
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(2) },
                intervening_if: None, effect: chapter_ii,
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(3) },
                intervening_if: None, effect: chapter_iii,
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new(),
            }),
    )
}

fn add_lore_counter(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::Lore, count: 1 }]
}

fn chapter_i(state: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    let filter = ObjectFilter::creature();
    let ids = script::ids_matching(state, &filter, trig.controller);
    ids.into_iter()
        .map(|id| Effect::DealDamage { target: DamageTarget::Object(id), amount: 4, source: trig.source })
        .collect()
}

fn chapter_ii(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Scry { player: trig.controller, count: 3 }]
}

fn chapter_iii(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    // "Whenever you cast a spell with mana value 5 or greater this
    // turn, draw two cards, then discard a card" — repeating, lapses
    // at end of turn.
    vec![Effect::EachCastThisTurn {
        controller: trig.controller,
        kind: NextCastKind::AnyMinCmc(5),
        rider: EachCastRider::DrawTwoDiscardOne,
    }]
}
