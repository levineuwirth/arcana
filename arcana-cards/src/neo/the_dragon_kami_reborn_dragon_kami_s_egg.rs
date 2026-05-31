//! The Dragon-Kami Reborn // Dragon-Kami's Egg — {2}{G} Enchantment — Saga (front),
//! transforming into Enchantment Creature — Egg (back).
//!
//! Front (Saga):
//!   I, II — You gain 2 life. Look at the top three cards of your library. Exile
//!           one of them face down with a hatching counter on it, then put the
//!           rest on the bottom of your library in any order.
//!   III  — Exile this Saga, then return it to the battlefield transformed.
//! Back (Dragon-Kami's Egg): Whenever this creature or a Dragon you control dies,
//!   you may cast a creature spell from among cards you own in exile with hatching
//!   counters on them without paying its mana cost.
//!
//! GAP: chapters I/II "exile one of the top three face down with a hatching counter,
//! rest to bottom in any order" is not expressible — DigTopN puts the chosen card
//! into hand, and there is no face-down-exile-with-named-counter primitive. Only the
//! "gain 2 life" portion of those chapters is authored.
//! GAP: back-face death-trigger "cast a creature spell from exile with hatching
//! counters without paying its mana cost" is not expressible (no free-cast-from-
//! filtered-exile primitive); authored as a no-op.
//! Chapter III "exile, then return transformed" is approximated with Effect::Transform
//! (the resulting state — the transformed permanent on the battlefield — matches).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::targets::ControllerConstraint;
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Dragon-Kami Reborn");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Enchantment Creature — Egg.
    let back_name = reg.interner_mut().intern("Dragon-Kami's Egg");
    let egg_sub = reg.interner_mut().intern("Egg");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(egg_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
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
                effect: chapter_i_ii,
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
                effect: chapter_i_ii,
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

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_i_ii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Look at the top three, exile one face down with a hatching counter,
    // rest to bottom in any order" is not expressible. Only the life gain is authored.
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 2,
    }]
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "Exile this Saga, then return it transformed under your control."
    // Approximated with Transform (resulting battlefield state matches).
    vec![Effect::Transform { target: trig.source }]
}
