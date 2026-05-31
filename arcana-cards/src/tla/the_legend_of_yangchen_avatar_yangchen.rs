//! The Legend of Yangchen // Avatar Yangchen — {3}{W}{W} Enchantment — Saga (W).
//!
//! (As this Saga enters and after your draw step, add a lore counter.)
//! I — Starting with you, each player chooses up to one permanent with mana
//!     value 3 or greater from among permanents your opponents control. Exile
//!     those permanents.
//! II — You may have target opponent draw three cards. If you do, draw three cards.
//! III — Exile this Saga, then return it to the battlefield transformed under
//!       your control.
//!
//! This file generates the Saga front face only. Chapter III transforms the
//! Saga into Avatar Yangchen (the back face is its own card definition).

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::registry::EntersWithSpec;
use arcana_core::triggers::TriggerSelf;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Legend of Yangchen");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Lore,
                count: 1,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: add_lore_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(1),
                },
                intervening_if: None,
                effect: chapter_i,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(2),
                },
                intervening_if: None,
                effect: chapter_ii,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(3),
                },
                intervening_if: None,
                effect: chapter_iii,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_i(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Each player chooses up to one permanent with mana value 3+ from among
    // permanents your opponents control; exile those. Mass-exile of opponents'
    // mv>=3 permanents (player-by-player choice not separately modeled).
    let filter = ObjectFilter::permanent()
        .controlled_by(ControllerConstraint::Opponent)
        .with_min_cmc(3);
    let ids = script_ids(state, &filter, trig);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ExilePermanent {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}

fn script_ids(
    state: &GameState,
    filter: &ObjectFilter,
    trig: &PendingTrigger,
) -> Vec<arcana_core::objects::ObjectId> {
    arcana_core::script::ids_matching(state, filter, trig.controller)
}

fn chapter_ii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // You may have target opponent draw three cards. If you do, draw three cards.
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    // "You may" — modeled as a non-optional symmetric draw (the may-clause is a
    // resolution-time choice not separately gated here).
    vec![
        Effect::DrawCards {
            player: *p,
            count: 3,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 3,
        },
    ]
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Exile this Saga, then return it transformed under your control.
    vec![Effect::Transform { target: trig.source }]
}
