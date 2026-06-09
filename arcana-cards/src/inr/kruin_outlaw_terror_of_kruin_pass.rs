//! Kruin Outlaw // Terror of Kruin Pass — `{1}{R}{R}` transforming DFC werewolf.
//! Front (Kruin Outlaw — Creature — Human Rogue Werewolf, 2/2):
//!   First strike.
//!   At the beginning of each upkeep, if no spells were cast last turn, transform.
//! Back (Terror of Kruin Pass — Creature — Werewolf, 2/2):
//!   Double strike.
//!   Werewolves you control have menace.
//!   At the beginning of each upkeep, if a player cast two or more spells last turn,
//!   transform.
//!
//! GAPs:
//! - Back face "Werewolves you control have menace": a static anthem-style ability
//!   granting menace to a subtype across the board is not expressible; not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::conditions;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kruin Outlaw");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let werewolf = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    subtypes.0.insert(werewolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // Back face: Terror of Kruin Pass — Creature — Werewolf, 2/2, Double strike.
    let back_name = reg.interner_mut().intern("Terror of Kruin Pass");
    let back_werewolf = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::DoubleStrike],
            // GAP: "Werewolves you control have menace" — static board-wide anthem
            //   granting menace to a subtype; not expressible.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front (face 0): if no spells were cast last turn, transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
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
            // Back (face 1): if a player cast two or more spells last turn, transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_two_or_more_last_turn),
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1),
    )
}

fn if_no_spells_last_turn(s: &GameState, _src: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::no_spells_cast_last_turn(s)
}

fn if_two_or_more_last_turn(s: &GameState, _src: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::a_player_cast_two_or_more_last_turn(s)
}

fn transform_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
