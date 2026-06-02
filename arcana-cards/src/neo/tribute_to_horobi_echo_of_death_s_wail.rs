//! Tribute to Horobi // Echo of Death's Wail — transforming Saga // creature DFC.
//!
//! Front (Tribute to Horobi): {1}{B} Enchantment — Saga.
//!   (As this Saga enters and after your draw step, add a lore counter.)
//!   I, II — Each opponent creates a 1/1 black Rat Rogue creature token.
//!   III   — Exile this Saga, then return it to the battlefield transformed under your control.
//! Back (Echo of Death's Wail): Enchantment Creature — Spirit 2/2 with Flying, Haste.
//!   "When this creature enters, gain control of all Rat tokens." (GAP — see below.)
//!   "Whenever this creature attacks, you may sacrifice another creature. If you do, draw a card."
//!   (GAP — see below.)
//!
//! GAP: back-face ETB "gain control of all Rat tokens" and attack-trigger "you may sacrifice
//!      another creature, if you do draw a card" are back-face-only abilities; per the
//!      transforming-Saga precedent, back-face triggers are not attached to the front def
//!      (they would also fire pre-transform), so they are noted but unimplemented.

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
    let name = reg.interner_mut().intern("Tribute to Horobi");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Echo of Death's Wail");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(spirit_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
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
            // I — Each opponent creates a 1/1 black Rat Rogue creature token.
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
            // II — Each opponent creates a 1/1 black Rat Rogue creature token.
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

fn chapter_i_ii(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let rat = reg.interner().lookup("Rat").unwrap_or_default();
    let rogue = reg.interner().lookup("Rogue").unwrap_or_default();
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| {
            let mut sub = SubtypeSet::default();
            sub.0.insert(rat);
            sub.0.insert(rogue);
            Effect::CreateToken {
                controller: opp,
                token: arcana_core::effects::TokenDefinition {
                    name: rat,
                    colors: ColorSet::black(),
                    types: TypeLine::CREATURE.into(),
                    subtypes: sub,
                    power: Some(PtValue::Fixed(1)),
                    toughness: Some(PtValue::Fixed(1)),
                    keywords: vec![],
                    abilities: vec![],
                },
            }
        })
        .collect()
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::ExilePermanent { target: trig.source },
        Effect::ReturnFromExileToBattlefield { target: trig.source },
        Effect::Transform { target: trig.source },
    ]
}
