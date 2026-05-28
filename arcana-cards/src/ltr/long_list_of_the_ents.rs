//! Long List of the Ents — `{G}` Enchantment — Saga
//!
//! Chapters I-VI — Note a creature type that hasn't been noted for this Saga.
//! When you next cast a creature spell of that type this turn, that creature
//! enters with an additional +1/+1 counter on it.
//! (GAP: "note a creature type" + "when you next cast a creature of that type
//! this turn" triggered ability not expressible; emitting Vec::new() for all
//! chapters.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef, TriggerSelf,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Long List of the Ents");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    let mut def = CardDefinition::new(name, chars)
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
        });

    // Six chapters — all GAP
    for chapter_n in 1u32..=6 {
        def = def.with_triggered_ability(TriggeredAbilityDef {
            id: chapter_n + 1,
            trigger_condition: TriggerCondition::CounterAdded {
                on: TriggerSelf::Source,
                kind: Some(CounterKind::Lore),
                chapter: Some(chapter_n),
            },
            intervening_if: None,
            effect: chapter_gap,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        });
    }

    reg.register(def)
}

fn add_lore_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "note a creature type" + conditional "when you next cast a creature
    // of that type this turn" not expressible with current engine API
    Vec::new()
}
