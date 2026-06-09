//! Scorned Villager // Moonscarred Werewolf — `{1}{G}` green Human Werewolf
//! creature 1/1.
//! Front face: {T}: Add {G}.
//! At the beginning of each upkeep, if no spells were cast last turn, transform.
//! Back face (Moonscarred Werewolf): 2/2 Vigilance Werewolf. {T}: Add {G}{G}.
//! At the beginning of each upkeep, if a player cast two or more spells last turn, transform.
//!
//! # GAPs
//! - "If no spells were cast last turn" / "if a player cast two or more spells last turn":
//!   these are precise werewolf day/night trigger conditions not modeled by the engine;
//!   the upkeep trigger is wired but the condition check returns Vec::new() (GAP).
//! - Back face activated ability {T}: Add {G}{G}: face-gated activated ability on back face;
//!   the front-face mana ability uses face_gate: Some(0), back-face uses face_gate: Some(1).
//! - Back face triggered ability (transform back) is modeled as a front-face TriggeredAbilityDef
//!   (engine limitation: back-face-only triggered ability not auto-installed on transform).

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, ManaColor, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scorned Villager");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Moonscarred Werewolf");
    let werewolf_back = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_back);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Vigilance],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: {T}: Add {G}
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: add_green,
            })
            // Back face: {T}: Add {G}{G}
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}{G}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: add_green_green,
            })
            // Front face transform trigger: at beginning of each upkeep, if no spells were cast
            // last turn, transform. Intervening-if modeled via conditions::no_spells_cast_last_turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: arcana_core::targets::ControllerConstraint::Any,
                },
                intervening_if: Some(iif_no_spells),
                effect: front_upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face transform trigger: at beginning of each upkeep, if a player cast 2+ spells
            // last turn, transform back. Intervening-if modeled via
            // conditions::a_player_cast_two_or_more_last_turn.
            // GAP: back-face-only triggered ability not modeled; authored here as always-present.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: arcana_core::targets::ControllerConstraint::Any,
                },
                intervening_if: Some(iif_two_or_more),
                effect: back_upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

fn add_green_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Green, ctx.source),
            ManaUnit::plain(ManaColor::Green, ctx.source),
        ],
    }]
}

fn iif_no_spells(state: &GameState, _source: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::no_spells_cast_last_turn(state)
}

fn iif_two_or_more(state: &GameState, _source: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::a_player_cast_two_or_more_last_turn(state)
}

/// Front face upkeep trigger: if no spells were cast last turn, transform.
/// GAP: "if no spells were cast last turn" condition not checkable; returns Vec::new().
fn front_upkeep_transform(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if no spells were cast last turn" — day/night condition not modeled.
    Vec::new()
}

/// Back face upkeep trigger: if a player cast two or more spells last turn, transform back.
/// GAP: "if a player cast two or more spells last turn" condition not checkable; returns Vec::new().
fn back_upkeep_transform(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if a player cast two or more spells last turn" — day/night condition not modeled.
    Vec::new()
}
