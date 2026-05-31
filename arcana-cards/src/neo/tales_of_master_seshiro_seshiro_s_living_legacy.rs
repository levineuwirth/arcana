//! Tales of Master Seshiro // Seshiro's Living Legacy (transforming Saga, CR 716 + CR 712)
//!
//! Front face: Tales of Master Seshiro — {4}{G} Enchantment — Saga (G).
//!   (As this Saga enters and after your draw step, add a lore counter.)
//!   I, II — Put a +1/+1 counter on target creature or Vehicle you control.
//!           It gains vigilance until end of turn.
//!   III — Exile this Saga, then return it to the battlefield transformed under your control.
//! Back face: Seshiro's Living Legacy — Enchantment Creature — Snake Warrior, 5/5 (G).
//!   Vigilance, haste.
//!
//! The Saga's final-chapter sacrifice is automatic (engine SBA). Chapter III's
//! "exile, then return transformed" is the Saga-transform pattern; modeled here
//! with Effect::Transform on the Saga's source.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::registry::EntersWithSpec;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tales of Master Seshiro");
    let saga_sub = reg.interner_mut().intern("Saga");
    let snake = reg.interner_mut().intern("Snake");
    let warrior = reg.interner_mut().intern("Warrior");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    let vehicle = reg.interner_mut().intern("Vehicle");

    let back_name = reg.interner_mut().intern("Seshiro's Living Legacy");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(snake);
    back_subtypes.0.insert(warrior);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE).into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Haste],
            ..Default::default()
        },
        spell_ability: None,
    };

    // "target creature or Vehicle you control"
    // GAP: "creature OR Vehicle" is an OR across a type and a subtype, which the
    // single-filter ObjectFilter can't express (combining types+subtypes is AND).
    // Modeling the common case — a creature you control — and dropping the Vehicle arm.
    let _ = vehicle;
    let counter_target = || TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::new()
                .controlled_by(ControllerConstraint::You)
                .with_types(TypeLine::CREATURE.into()),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Lore,
                count: 1,
            })
            // add a lore counter at the beginning of your first main phase
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
            // Chapter I
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
                target_requirements: vec![counter_target()],
            })
            // Chapter II
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
                target_requirements: vec![counter_target()],
            })
            // Chapter III — exile, then return transformed (modeled as transform)
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
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::EndOfTurn,
        },
    ]
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "Exile this Saga, then return it transformed under your control."
    // Modeled as an in-place transform to the back face.
    vec![Effect::Transform { target: trig.source }]
}
