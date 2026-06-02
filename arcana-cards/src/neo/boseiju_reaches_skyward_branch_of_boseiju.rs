//! Boseiju Reaches Skyward // Branch of Boseiju — transforming Saga // creature DFC.
//!
//! Front (Boseiju Reaches Skyward): {3}{G} Enchantment — Saga.
//!   (As this Saga enters and after your draw step, add a lore counter.)
//!   I   — Search your library for up to two basic Forest cards, reveal them, put them into
//!         your hand, then shuffle.
//!   II  — Put up to one target land card from your graveyard on top of your library.
//!   III — Exile this Saga, then return it to the battlefield transformed under your control.
//! Back (Branch of Boseiju): Enchantment Creature — Plant with Reach.
//!   "This creature gets +1/+1 for each land you control." (GAP — see below.)
//!
//! GAP: back-face static "gets +1/+1 for each land you control" is a continuous self-buff on the
//!      back face; a dynamic per-land P/T boost is not expressible on a CardFace. The back face is
//!      registered as a 0/0 Plant with Reach.
//! Final-chapter sacrifice (chapter III transform) is handled in the chapter effect.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::script;
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
    let name = reg.interner_mut().intern("Boseiju Reaches Skyward");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Branch of Boseiju");
    let plant_sub = reg.interner_mut().intern("Plant");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(plant_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            keywords: vec![KeywordAbility::Reach],
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
            // I — Search library for up to two basic Forest cards, to hand.
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
            // II — Put up to one target land card from your graveyard on top of your library.
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
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new()
                            .with_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::You),
                    },
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
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

fn chapter_i(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // Up to two basic Forest cards: two tutor-to-hand searches for a basic Forest.
    let forest = script::subtype_filter(reg, "Forest")
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC));
    vec![
        Effect::TutorToHand {
            player: trig.controller,
            filter: forest.clone(),
            reveal: true,
        },
        Effect::TutorToHand {
            player: trig.controller,
            filter: forest,
            reveal: true,
        },
    ]
}

fn chapter_ii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::PutOnTopOfLibrary { target: *id }]
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::ExilePermanent { target: trig.source },
        Effect::ReturnFromExileToBattlefield { target: trig.source },
        Effect::Transform { target: trig.source },
    ]
}
