//! Tovolar's Huntmaster // Tovolar's Packleader — {4}{G}{G} green transforming DFC
//! (CR 712, layout "transform"), Daybound/Nightbound werewolf.
//!
//! Front face (Tovolar's Huntmaster): Creature — Human Werewolf, 6/6.
//!   When this creature enters, create two 2/2 green Wolf creature tokens.
//!   Daybound (If a player casts no spells during their own turn, it becomes night next turn.)
//! Back face (Tovolar's Packleader): Creature — Werewolf, 6/6.
//!   Whenever this creature enters or attacks, create two 2/2 green Wolf creature tokens.
//!   {2}{G}{G}: Another target Wolf or Werewolf you control fights target creature you don't control.
//!   Nightbound (If a player casts at least two spells during their own turn, it becomes day next turn.)
//!
//! # Notes / GAPs
//! - Daybound/Nightbound: not in the engine keyword surface. Modeled as the werewolf
//!   transform flips via upkeep triggers gated to each face using the day/night spell-count
//!   intervening-if conditions (front->back when no spells were cast last turn; back->front
//!   when a player cast two or more spells last turn). The day/night designator side of
//!   daybound/nightbound is not separately tracked.
//! - Front ETB and back ETB both create two 2/2 green Wolf tokens; the back also fires on
//!   attack. Authored as face-gated triggers.
//! - GAP: the back face's "{2}{G}{G}: Another target Wolf or Werewolf you control fights
//!   target creature you don't control" activated ability — token/face-gated activated
//!   abilities with two targets and a fight payload are not in the demonstrated activated-
//!   ability surface for this card class; not modeled.

use arcana_core::conditions;
use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tovolar's Huntmaster");
    let human = reg.interner_mut().intern("Human");
    let werewolf = reg.interner_mut().intern("Werewolf");
    // Pre-intern token subtype for resolution.
    let _ = reg.interner_mut().intern("Wolf");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human);
    front_subtypes.0.insert(werewolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // Back face: Tovolar's Packleader — Creature — Werewolf, 6/6.
    let back_name = reg.interner_mut().intern("Tovolar's Packleader");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: When this creature enters, create two 2/2 green Wolf tokens.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_two_wolves,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back: Whenever this creature enters, create two 2/2 green Wolf tokens.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_two_wolves,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back: Whenever this creature attacks, create two 2/2 green Wolf tokens.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: make_two_wolves,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Daybound: front->back transform if no spells were cast last turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_no_spells_last_turn),
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Nightbound: back->front transform if a player cast two or more spells last turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_player_cast_two_last_turn),
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 1 + day-flip (4) are front only; 2, 3 + night-flip (5) are back only.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1)
            .with_trigger_face_gate(3, 1)
            .with_trigger_face_gate(4, 0)
            .with_trigger_face_gate(5, 1),
    )
}

fn make_two_wolves(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let wolf_sym = reg.interner().lookup("Wolf").expect("interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(wolf_sym);
    let token = TokenDefinition {
        name: wolf_sym,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token,
        },
    ]
}

fn transform_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform {
        target: trig.source,
    }]
}

fn if_no_spells_last_turn(s: &GameState, _src: ObjectId, _you: PlayerId) -> bool {
    conditions::no_spells_cast_last_turn(s)
}

fn if_player_cast_two_last_turn(s: &GameState, _src: ObjectId, _you: PlayerId) -> bool {
    conditions::a_player_cast_two_or_more_last_turn(s)
}
