//! Okiba Reckoner Raid // Nezumi Road Captain — transforming Saga // creature DFC.
//!
//! Front (Okiba Reckoner Raid): {B} Enchantment — Saga.
//!   I, II — Each opponent loses 1 life and you gain 1 life.
//!   III   — Exile this Saga, then return it to the battlefield transformed under your control.
//! Back (Nezumi Road Captain): Enchantment Creature — Rat Rogue 2/2 with Menace.
//!   "Vehicles you control have menace." (GAP — see below.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Okiba Reckoner Raid");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Nezumi Road Captain");
    let rat_sub = reg.interner_mut().intern("Rat");
    let rogue_sub = reg.interner_mut().intern("Rogue");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(rat_sub);
    back_subtypes.0.insert(rogue_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Menace],
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
            // First main phase: add a lore counter.
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
            // I — Each opponent loses 1 life and you gain 1 life.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(1),
                },
                intervening_if: None,
                effect: drain_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // II — Each opponent loses 1 life and you gain 1 life.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(2),
                },
                intervening_if: None,
                effect: drain_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // III — Exile this Saga, then return it transformed under your control.
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
            })
            // GAP: back-face static "Vehicles you control have menace" — a continuous
            // ability granting menace to other permanents you control is not expressible
            // with the available effect/static surface.
    )
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn drain_one(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    let mut effects: Vec<Effect> = opponents
        .into_iter()
        .map(|opp| Effect::LoseLife {
            player: opp,
            amount: 1,
        })
        .collect();
    effects.push(Effect::GainLife {
        player: trig.controller,
        amount: 1,
    });
    vec![Effect::Sequence(effects)]
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Exile this Saga, then return it transformed under your control.
    vec![
        Effect::ExilePermanent { target: trig.source },
        Effect::ReturnFromExileToBattlefield { target: trig.source },
        Effect::Transform { target: trig.source },
    ]
}
