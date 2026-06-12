//! Nightmares and Daydreams — `{2}{U}` blue Enchantment — Saga. 4 chapters.
//! I, II, III — Until your next turn, whenever you cast an instant or sorcery spell, target player mills
//!   cards equal to that spell's mana value.
//! IV — Draw a card. If a graveyard has twenty or more cards in it, draw three cards instead.
//! GAP-NARROW: Chapter I/II/III "target player mills" — delayed triggers carry no targets;
//!   deterministic pick: the lowest-numbered living opponent mills.
//! GAP: Chapter IV — conditional draw based on any graveyard size not in script API.
//! Final-chapter sacrifice is automatic (engine SBA).

use arcana_core::effects::{Effect, FloatingUntil};
use arcana_core::events::GameEvent;
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
    let name = reg.interner_mut().intern("Nightmares and Daydreams");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
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
                effect: chapter_i_iii,
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
                effect: chapter_i_iii,
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
                effect: chapter_i_iii,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(4),
                },
                intervening_if: None,
                effect: chapter_iv,
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

fn chapter_i_iii(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // "Until your next turn, whenever you cast an instant or sorcery
    // spell, target player mills cards equal to that spell's mana value."
    vec![Effect::ScheduleFloatingTrigger {
        source: trig.source,
        controller: trig.controller,
        condition: TriggerCondition::SpellCast {
            filter: Some(ObjectFilter::new().with_types_any(TypeLine(
                TypeLine::INSTANT | TypeLine::SORCERY,
            ))),
            caster: ControllerConstraint::You,
        },
        effect: mill_for_cast,
        until: FloatingUntil::YourNextTurn,
    }]
}

fn mill_for_cast(
    state: &GameState,
    pt: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let GameEvent::SpellCast { object_id, .. } = &pt.trigger_event else {
        return Vec::new();
    };
    let count = state
        .object_or_lki(*object_id)
        .map_or(0, |o| o.characteristics.mana_value());
    if count == 0 {
        return Vec::new();
    }
    // GAP-NARROW: "target player mills" — delayed triggers carry no
    // targets; deterministic pick: lowest-numbered living opponent.
    let Some(victim) = (0..state.num_players())
        .filter(|p| *p != pt.controller && state.player(*p).is_alive())
        .min()
    else {
        return Vec::new();
    };
    vec![Effect::Mill { player: victim, count }]
}

fn chapter_iv(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if a graveyard has 20+ cards, draw 3 instead" — graveyard-size conditional across all players not in script API
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
