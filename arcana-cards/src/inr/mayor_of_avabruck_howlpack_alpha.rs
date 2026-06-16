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
//! GAP: static +1/+1 anthem effects (both faces) not modeled via triggered ability.
//! GAP: werewolf transform conditions ("no spells cast last turn" / "two or more spells last turn")
//!      not expressible as TriggerCondition predicates; transforms unconditionally at upkeep.
//! GAP: back-face-only triggered abilities (end-step token creation, upkeep back-transform)
//!      not auto-installed on transform.

use arcana_core::conditions;
use arcana_core::effects::Effect;
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
            // Back face: at beginning of your end step, create a 2/2 green Wolf token.
            // GAP: back-face-only triggered ability not auto-installed on transform.
            // GAP: back-face upkeep transform (2+ spells last turn) not modeled.
    )
}

fn iif_no_spells(state: &GameState, _source: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::no_spells_cast_last_turn(state)
}

fn front_upkeep_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
