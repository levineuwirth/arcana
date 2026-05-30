//! Tormented Pariah // Rampaging Werewolf — `{3}{R}` Creature — Human Warrior Werewolf 3/2 (red).
//! Front face: At the beginning of each upkeep, if no spells were cast last turn, transform.
//! Back face (Werewolf 5/4): At the beginning of each upkeep, if a player cast two or more
//!   spells last turn, transform back.
//!
//! GAP: "if no spells were cast last turn" / "if a player cast two or more spells last turn"
//!   — exact werewolf day/night trigger conditions (tracking prior-turn spell counts) are not
//!   modeled. Both triggers are wired to StepBegins::Upkeep but fire unconditionally.
//! GAP: Back-face-only triggered ability not modeled (triggers live on CardDefinition).

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tormented Pariah");
    let human_sub = reg.interner_mut().intern("Human");
    let warrior_sub = reg.interner_mut().intern("Warrior");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut front_subs = SubtypeSet::default();
    front_subs.0.insert(human_sub);
    front_subs.0.insert(warrior_sub);
    front_subs.0.insert(werewolf_sub);

    let front_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subs,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Rampaging Werewolf");
    let mut back_subs = SubtypeSet::default();
    let werewolf_back_sub = reg.interner_mut().intern("Werewolf");
    back_subs.0.insert(werewolf_back_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subs,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, front_chars)
            .with_transform_back(back)
            // Front -> Back: upkeep trigger (GAP: should only fire if no spells cast last turn)
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
            })
            // Back -> Front: upkeep trigger (GAP: should only fire if 2+ spells cast last turn)
            // GAP: back-face-only triggered ability not modeled
    )
}

/// GAP: fires unconditionally at each upkeep instead of only when no spells were cast last turn.
fn front_upkeep_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exact condition "no spells cast last turn" not modeled.
    vec![Effect::Transform { target: trig.source }]
}
