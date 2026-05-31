//! The Restoration of Eiganjo // Architect of Restoration (Saga // transforming creature back).
//!
//! Front (Enchantment — Saga, {2}{W}, white):
//!   I  — Search your library for a basic Plains card, reveal it, put it into your hand, then shuffle.
//!   II — You may discard a card. When you do, return target permanent card with mana value 2 or
//!        less from your graveyard to the battlefield tapped.
//!   III — Exile this Saga, then return it to the battlefield transformed under your control.
//! Back (Enchantment Creature — Fox Monk, white, 3/3 vigilance):
//!   Whenever this creature attacks or blocks, create a 1/1 colorless Spirit creature token.
//!
//! GAPs noted inline: the Saga's final chapter (III) is a transform-into-back rather than the usual
//! self-sacrifice — the engine's automatic final-chapter sacrifice SBA does not model the
//! exile-and-return-transformed flow, so chapter III emits Effect::Transform as a best effort and the
//! reflexive "when you do" discard trigger on chapter II is approximated.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::effects::DiscardChoice;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::targets::ControllerConstraint;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Restoration of Eiganjo");
    // Pre-intern strings looked up at resolution time.
    let _ = reg.interner_mut().intern("Spirit");
    let _ = reg.interner_mut().intern("Plains");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Architect of Restoration — Enchantment Creature — Fox Monk, 3/3, Vigilance.
    let back_name = reg.interner_mut().intern("Architect of Restoration");
    let fox = reg.interner_mut().intern("Fox");
    let monk = reg.interner_mut().intern("Monk");
    let mut back_subs = SubtypeSet::default();
    back_subs.0.insert(fox);
    back_subs.0.insert(monk);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subs,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Vigilance],
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
                // II targets a permanent card with mv 2 or less in your graveyard.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::permanent().with_max_cmc(2),
                    },
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
            // Back-face trigger: attacks or blocks -> create a 1/1 Spirit token. No single
            // "attacks or blocks" condition exists, so author one trigger per side, both gated to
            // the back face (face 1).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: back_make_spirit,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(5, 1)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 6,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: back_make_spirit,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(6, 1),
    )
}

fn add_lore_counter(_: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_i(_: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let plains = reg.interner().lookup("Plains");
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter {
            name: plains,
            types: Some(TypeLine::LAND.into()),
            ..ObjectFilter::default()
        },
        reveal: true,
    }]
}

fn chapter_ii(_: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // "You may discard a card. When you do, return target permanent card with mv 2 or less from
    // your graveyard to the battlefield tapped."
    // GAP: the reflexive "when you do" gating on the discard is not modeled; approximated as a
    // discard followed by the reanimation of the chosen graveyard target.
    let Some(target) = trig.targets.targets.first() else {
        return vec![Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        }];
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
    ]
}

fn chapter_iii(_: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // "Exile this Saga, then return it transformed under your control." The engine's final-chapter
    // sacrifice SBA does not model exile-and-return-transformed; emit Transform as a best effort.
    vec![Effect::Transform { target: trig.source }]
}

fn back_make_spirit(_: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned");
    let mut subs = SubtypeSet::default();
    subs.0.insert(spirit);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: arcana_core::effects::TokenDefinition {
            name: spirit,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes: subs,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
