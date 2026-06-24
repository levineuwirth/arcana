//! Mayor of Avabruck // Howlpack Alpha — `{1}{G}` Human Advisor Werewolf 1/1 (front) /
//! Werewolf (back). Transform werewolf.
//!
//! Front face:
//!   Other Human creatures you control get +1/+1. (GAP: static continuous effect)
//!   At the beginning of each upkeep, if no spells were cast last turn, transform.
//!
//! Back face (Howlpack Alpha):
//!   Each other creature you control that's a Werewolf or Wolf gets +1/+1. (GAP: static)
//!   At the beginning of your end step, create a 2/2 green Wolf creature token.
//!   At the beginning of each upkeep, if a player cast two or more spells last turn, transform.
//!
//! Werewolf transform conditions ("no spells cast last turn" / "two or more spells
//! last turn") are modeled as intervening-if predicates
//! (conditions::no_spells_cast_last_turn / conditions::a_player_cast_two_or_more_last_turn).
//! The front upkeep trigger is gated to face 0; the back upkeep transform and the
//! back end-step Wolf-token trigger are gated to face 1 (with_trigger_face_gate).
//! GAP: the static +1/+1 anthem effects (front "other Human creatures you control",
//!      back "each other Werewolf or Wolf you control") are continuous P/T-boosting
//!      static abilities and remain unmodeled.

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
    let name = reg.interner_mut().intern("Mayor of Avabruck");
    let human_sub = reg.interner_mut().intern("Human");
    let advisor_sub = reg.interner_mut().intern("Advisor");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(advisor_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Howlpack Alpha — Werewolf
    let back_name = reg.interner_mut().intern("Howlpack Alpha");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    let wolf_sub = reg.interner_mut().intern("Wolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // Pre-intern Wolf for token creation at resolution time
    let _ = wolf_sub; // interned above, available via lookup

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: at beginning of each upkeep, if no spells were cast last turn, transform.
            // Intervening-if modeled via conditions::no_spells_cast_last_turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(iif_no_spells),
                effect: front_upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face: at beginning of each upkeep, if a player cast two or more
            // spells last turn, transform. Intervening-if via
            // conditions::a_player_cast_two_or_more_last_turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(iif_two_or_more),
                effect: front_upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face: at the beginning of your end step, create a 2/2 green
            // Wolf creature token.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_wolf_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 1 fires only on the front face; triggers 2 & 3 only on the back face.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1)
            .with_trigger_face_gate(3, 1)
    )
}

fn iif_no_spells(state: &GameState, _source: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::no_spells_cast_last_turn(state)
}

fn iif_two_or_more(state: &GameState, _source: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::a_player_cast_two_or_more_last_turn(state)
}

fn front_upkeep_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}

/// Back face (Howlpack Alpha): at the beginning of your end step, create a
/// 2/2 green Wolf creature token.
fn end_step_wolf_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wolf = reg.interner().lookup("Wolf").expect("Wolf interned during register()");
    let mut wolf_subtypes = SubtypeSet::default();
    wolf_subtypes.0.insert(wolf);
    let token = TokenDefinition {
        name: wolf,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: wolf_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}
