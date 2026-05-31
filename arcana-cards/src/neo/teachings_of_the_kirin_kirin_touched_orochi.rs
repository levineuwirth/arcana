//! Teachings of the Kirin // Kirin-Touched Orochi (transforming Saga).
//!
//! Front: Enchantment — Saga, {1}{G}, green.
//!   I — Mill three cards. Create a 1/1 colorless Spirit creature token.
//!   II — Put a +1/+1 counter on target creature you control.
//!   III — Exile this Saga, then return it transformed under your control.
//!         (modeled as Effect::Transform on the III chapter.)
//!
//! Back: Enchantment Creature — Snake Monk, 0/0 base (printed P/T below).
//!   Whenever this creature attacks, choose one — exile a creature card / a
//!   noncreature card from a graveyard, with a reflexive payoff.
//!
//! GAP: the back-face attack trigger is a modal ("choose one") TRIGGERED ability
//! with per-mode targets and reflexive "when you do" sub-triggers. The engine's
//! modal dispatch is a SPELL-ability facility (dispatch_modal_effect +
//! with_mode_effects); there is no modal triggered-ability shape, and no
//! reflexive-trigger primitive. Authored as a face-gated trigger returning
//! Vec::new() rather than inventing API.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
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
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teachings of the Kirin");
    let saga_sub = reg.interner_mut().intern("Saga");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let snake_sub = reg.interner_mut().intern("Snake");
    let monk_sub = reg.interner_mut().intern("Monk");
    let back_name = reg.interner_mut().intern("Kirin-Touched Orochi");
    let _ = spirit_sub;

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(snake_sub);
    back_subtypes.0.insert(monk_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
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
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
            })
            // Back-face attack trigger — modal, see GAP.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: back_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_transform_back(back)
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

fn chapter_i(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let spirit = reg.interner().lookup("Spirit");
    let mut subtypes = SubtypeSet::default();
    if let Some(s) = spirit {
        subtypes.0.insert(s);
    }
    let token = arcana_core::effects::TokenDefinition {
        name: spirit.unwrap_or_default(),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::Mill {
            player: trig.controller,
            count: 3,
        },
        Effect::CreateToken {
            controller: trig.controller,
            token,
        },
    ]
}

fn chapter_ii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "Exile this Saga, then return it transformed under your control."
    vec![Effect::Transform { target: trig.source }]
}

fn back_attacks(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: modal triggered ability ("choose one — exile a creature card / a
    // noncreature card from a graveyard ...") with reflexive "when you do"
    // sub-triggers. No modal-trigger or reflexive-trigger primitive.
    Vec::new()
}
