//! Tamiyo Meets the Story Circle — `{1}{U}` blue Enchantment — Saga.
//! I — Until your next turn, whenever a creature attacks you or a planeswalker you control, it gets -2/-0 until end of turn.
//! II — Discard any number of cards, then investigate twice for each card discarded.
//! III — Shuffle up to three target cards from your graveyard into your library.
//! GAP: Chapter II "investigate twice for each card discarded" — dynamic number of Clue tokens based on discard count not in catalog.
//! GAP: Chapter III "shuffle target cards into library" — shuffle-from-graveyard-to-library not in catalog.
//! Final-chapter sacrifice is automatic (engine SBA).

use arcana_core::combat::DefendingEntity;
use arcana_core::effects::{Effect, FloatingUntil};
use arcana_core::events::GameEvent;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tamiyo Meets the Story Circle");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
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
                target_requirements: Vec::new(),
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

fn add_lore_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::Lore, count: 1 }]
}

fn chapter_i(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // "Until your next turn, whenever a creature attacks you or a
    // planeswalker you control, it gets -2/-0 until end of turn."
    vec![Effect::ScheduleFloatingTrigger {
        source: trig.source,
        controller: trig.controller,
        condition: TriggerCondition::CreatureAttacks {
            filter: ObjectFilter::creature(),
        },
        effect: attacker_penalty,
        until: FloatingUntil::YourNextTurn,
    }]
}

fn attacker_penalty(
    state: &GameState,
    pt: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let GameEvent::CreatureAttacks { attacker, defending } = &pt.trigger_event
    else {
        return Vec::new();
    };
    // Only attacks against the scheduling player or a planeswalker
    // they control qualify.
    let attacks_us = match defending {
        DefendingEntity::Player(p) => *p == pt.controller,
        DefendingEntity::Planeswalker(pw) => state
            .object_or_lki(*pw)
            .is_some_and(|o| o.controller == pt.controller),
        DefendingEntity::Battle(_) => false,
    };
    if !attacks_us {
        return Vec::new();
    }
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::pump(
            pt.source,
            *attacker,
            -2,
            0,
            Duration::EndOfTurn,
        ),
    }]
}

fn chapter_ii(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "discard any number, investigate twice for each discarded" — dynamic clue count not in catalog
    Vec::new()
}

fn chapter_iii(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "shuffle up to three target cards from your graveyard into your library" — shuffle-to-library not in catalog
    Vec::new()
}
