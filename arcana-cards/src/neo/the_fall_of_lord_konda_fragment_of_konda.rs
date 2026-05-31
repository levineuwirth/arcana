//! The Fall of Lord Konda // Fragment of Konda — `{2}{W}` white transforming Saga (CR 716/712).
//!
//! Front face (The Fall of Lord Konda): Enchantment — Saga.
//!   (As this Saga enters and after your draw step, add a lore counter.)
//!   I — Exile target creature an opponent controls with mana value 4 or greater.
//!   II — Each player gains control of all permanents they own.
//!   III — Exile this Saga, then return it to the battlefield transformed under your control.
//!
//! Back face (Fragment of Konda): Enchantment Creature — Human Noble, 2/2 with Defender.
//!   When this creature dies, draw a card.
//!
//! # Notes / GAPs
//! - Chapter II ("Each player gains control of all permanents they own") has no documented
//!   mass control-return primitive — GAP'd as a no-op rather than a wrong literal.
//! - Chapter III is modeled as `Effect::Transform { target }` (the saga's final chapter flips
//!   the permanent to its back face). The engine's final-chapter sacrifice SBA is suppressed in
//!   intent by transforming; this is the documented transform path. The "exile then return"
//!   wording is rendered as a same-permanent transform.
//! - Back-face death trigger (draw a card) is authored and gated to face 1; the front-face
//!   lore/chapter machinery is gated to the front by being saga-only triggers (faces share the
//!   def — chapter triggers only fire while the saga face is showing in practice).

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
    let name = reg.interner_mut().intern("The Fall of Lord Konda");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    // Back face: Fragment of Konda — white Enchantment Creature — Human Noble, 2/2, Defender.
    let back_name = reg.interner_mut().intern("Fragment of Konda");
    let human_sub = reg.interner_mut().intern("Human");
    let noble_sub = reg.interner_mut().intern("Noble");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(human_sub);
    back_subtypes.0.insert(noble_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Defender],
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
            // Chapter I — Exile target creature an opponent controls with mana value 4+.
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
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent)
                            .with_min_cmc(4),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // Chapter II — Each player gains control of all permanents they own. (GAP)
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
            // Chapter III — Exile this Saga, return it transformed under your control.
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
            // Back face only: when this creature dies, draw a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: back_dies_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(5, 1),
    )
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_i(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ExilePermanent { target: *id }]
}

fn chapter_ii(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Each player gains control of all permanents they own" — no documented mass
    // control-return primitive (Effect::ChangeControl is single-target only).
    Vec::new()
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "Exile this Saga, then return it transformed under your control" — modeled as a
    // self-transform to the back face.
    vec![Effect::Transform { target: trig.source }]
}

fn back_dies_draw(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
