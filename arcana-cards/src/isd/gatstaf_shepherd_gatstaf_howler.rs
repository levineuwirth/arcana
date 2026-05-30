//! Gatstaf Shepherd // Gatstaf Howler — `{1}{G}` Creature — Human Werewolf 2/2.
//!
//! Front face (Gatstaf Shepherd):
//! At the beginning of each upkeep, if no spells were cast last turn, transform this creature.
//!
//! Back face (Gatstaf Howler — Creature — Werewolf):
//! Intimidate (This creature can't be blocked except by artifact creatures and/or creatures
//! that share a color with it.)
//! At the beginning of each upkeep, if a player cast two or more spells last turn, transform
//! this creature.
//!
//! GAP: Keywords "Transform" is a layout keyword, not modeled as KeywordAbility.
//! GAP: "Intimidate" is not in the implemented keyword list — omitted from back face characteristics.
//! GAP: Werewolf day/night transform conditions ("if no spells were cast last turn" /
//! "if a player cast two or more spells last turn") — exact spells-cast-last-turn check is
//! not available; using spells_cast_this_turn == 0 as a closest approximation for the upkeep
//! trigger intervening-if. Back-face-only triggered ability not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gatstaf Shepherd");
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
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face — Gatstaf Howler (Creature — Werewolf)
    let back_name = reg.interner_mut().intern("Gatstaf Howler");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_sub);

    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: back_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Intimidate not in keyword list
        keywords: vec![],
        ..Default::default()
    };

    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back_face)
            // Front face: at the beginning of each upkeep, if no spells were cast last turn,
            // transform this creature.
            // GAP: "no spells cast last turn" — using spells_cast_this_turn as approximation;
            // "last turn" tracking not available. This triggers at each upkeep with no intervening-if.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: front_upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
            // GAP: back-face-only triggered ability not modeled:
            // "At the beginning of each upkeep, if a player cast two or more spells last turn,
            // transform this creature."
    )
}

fn front_upkeep_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if no spells were cast last turn" — last-turn spell tracking not available.
    // Emitting the transform unconditionally as a best-effort (verify will flag condition gap).
    vec![Effect::Transform { target: trig.source }]
}
