//! Sage of Ancient Lore // Werewolf of Ancient Hunger — `{4}{G}` green
//! Legendary Creature — Human Shaman Werewolf (*/*) / Legendary Creature — Werewolf.
//!
//! Front face (Sage of Ancient Lore): */* (power/toughness = cards in your hand)
//!   Vigilance. (GAP: Vigilance listed as keyword but not on front face in oracle)
//!   Sage of Ancient Lore's power and toughness are each equal to the number of
//!   cards in your hand.
//!   When this creature enters, draw a card.
//!   At the beginning of each upkeep, if no spells were cast last turn, transform.
//!
//! Back face (Werewolf of Ancient Hunger):
//!   Vigilance, trample.
//!   Power and toughness equal to total cards in all players' hands.
//!   At the beginning of each upkeep, if a player cast two or more spells last turn,
//!   transform this creature.
//!
//! Both faces' */* P/T are wired as Layer-7a self-CDAs (`self_pt_cda`) installed
//!      on `SelfEntersBattlefield`, each with a face-gated duration so only the
//!      live face's value applies: front = cards in YOUR hand
//!      (`WhileSourceShowsFace(0)`); back = total cards in ALL players' hands
//!      (`WhileSourceShowsFace(1)`). Both faces' bones are `PtValue::Star`.
//! Both werewolf upkeep transforms are wired: the front
//!      ("if no spells were cast last turn") and back ("if a player cast two or
//!      more spells last turn") transforms use the `conditions::` intervening-ifs
//!      and are face-gated (front 0, back 1).

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::ObjectId;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::{Zone, ZoneKind};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sage of Ancient Lore");
    let human_sub = reg.interner_mut().intern("Human");
    let shaman_sub = reg.interner_mut().intern("Shaman");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(shaman_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // */* — defined by the front-face CDA (cards in your hand) installed below.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Werewolf of Ancient Hunger");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            // */* — defined by the back-face CDA (total cards in all hands)
            // installed below, face-gated to the back face.
            power: Some(PtValue::Star),
            toughness: Some(PtValue::Star),
            keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB: install both faces' P/T CDAs (each face-gated so only the
            // live face's value applies) and draw a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_cdas_and_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Front: at beginning of each upkeep, if no spells were cast last turn, transform (face 0).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(iif_no_spells_last_turn),
                effect: upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Back: at beginning of each upkeep, if a player cast two or more spells
            // last turn, transform (face 1).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(iif_two_or_more_spells_last_turn),
                effect: upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Front upkeep transform fires only on front face; back only on back face.
            .with_trigger_face_gate(2, 0)
            .with_trigger_face_gate(3, 1),
    )
}

fn etb_install_cdas_and_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        // Front (Sage of Ancient Lore): P/T = cards in your hand.
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::self_pt_cda(
                trig.source,
                cards_in_your_hand,
                Duration::WhileSourceShowsFace(0),
            ),
        },
        // Back (Werewolf of Ancient Hunger): P/T = total cards in all hands.
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::self_pt_cda(
                trig.source,
                cards_in_all_hands,
                Duration::WhileSourceShowsFace(1),
            ),
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
    ]
}

/// Front CDA: P/T = the number of cards in your hand.
fn cards_in_your_hand(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = s.objects.objects_in_zone(Zone::Hand(who)).count() as i32;
    (n, n)
}

/// Back CDA: P/T = the total number of cards in all players' hands.
fn cards_in_all_hands(s: &GameState, _source: ObjectId) -> (i32, i32) {
    let n = s.objects.objects_in_zone_kind(ZoneKind::Hand).count() as i32;
    (n, n)
}

fn iif_no_spells_last_turn(state: &GameState, _source: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::no_spells_cast_last_turn(state)
}

fn iif_two_or_more_spells_last_turn(state: &GameState, _source: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::a_player_cast_two_or_more_last_turn(state)
}

fn upkeep_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Werewolf upkeep transform; gated by intervening_if + face gate.
    vec![Effect::Transform { target: trig.source }]
}
