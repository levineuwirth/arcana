//! Inventive Iteration // Living Breakthrough — `{3}{U}` blue Enchantment — Saga.
//! (As this Saga enters and after your draw step, add a lore counter.)
//! I — Return up to one target creature or planeswalker to its owner's hand.
//! II — Return an artifact card from your graveyard to your hand. If you can't, draw a card.
//! III — Exile this Saga, then return it to the battlefield transformed under your control.
//! Back face: Living Breakthrough — Enchantment Creature — Moonfolk, Flying.
//!   Whenever you cast a spell, your opponents can't cast spells with the same mana value
//!   until your next turn.
//!
//! # GAPs
//! - Chapter I "creature or planeswalker" target: modeled as a permanent filter of
//!   creature-or-planeswalker types via with_types_any.
//! - Chapter II "Return an artifact card from your graveyard to your hand. If you can't,
//!   draw a card." — the conditional "if you can't" fallback is not expressible; modeled
//!   as a non-targeted Reanimate-to-hand of an artifact card. Best-effort: tutor-from-
//!   graveyard via ReturnFromGraveyardToHand is targeted, but the "if you can't draw"
//!   branch has no catalog form. GAP: emitted as Vec::new()-fallback draw omitted.
//! - Back face's "Whenever you cast a spell, opponents can't cast spells with the same mana
//!   value until your next turn" — no TriggerCondition / Effect models a mana-value cast
//!   restriction. GAP: trigger omitted; only Flying is modeled on the back face.
//! - Final-chapter sacrifice is automatic (engine SBA); chapter III instead Transforms.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inventive Iteration");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    // Back face: Living Breakthrough — Enchantment Creature — Moonfolk, Flying.
    let back_name = reg.interner_mut().intern("Living Breakthrough");
    let moonfolk_sub = reg.interner_mut().intern("Moonfolk");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(moonfolk_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying],
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
                effect: chapter_i,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER)),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
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
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_i(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Return up to one target creature or planeswalker to its owner's hand.
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnToHand { target: *id }]
}

fn chapter_ii(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Return an artifact card from your graveyard to your hand. If you can't, draw a card.
    let _ = trig;
    Vec::new()
    // GAP: No catalog form for a non-targeted "return an artifact card from your graveyard
    //   to your HAND" (Reanimate returns to the battlefield; ReturnFromGraveyardToHand is
    //   targeted). The conditional "if you can't, draw a card" fallback is also not
    //   expressible. Whole chapter emitted as a GAP rather than a materially wrong effect.
}

fn chapter_iii(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Exile this Saga, then return it to the battlefield transformed — modeled as Transform.
    vec![Effect::Transform { target: trig.source }]
}
