//! Breakneck Rider // Neck Breaker — `{1}{R}{R}` Human Scout Werewolf 3/3.
//!
//! Front face: "At the beginning of each upkeep, if no spells were cast last
//! turn, transform this creature."
//! Back face (Neck Breaker): "Attacking creatures you control get +1/+0 and
//! have trample." + "At the beginning of each upkeep, if a player cast two or
//! more spells last turn, transform this creature."
//!
//! # GAPs
//! - "If no spells were cast last turn" / "if a player cast two or more spells
//!   last turn" — the precise werewolf upkeep trigger condition is not modeled.
//!   Both transform triggers are wired to `TriggerCondition::PhaseBegins` at
//!   upkeep; the intervening-if condition checking last-turn spell counts is
//!   omitted (GAP: werewolf spell-count condition not modeled).
//! - The back-face static "+1/+0 and trample to attacking creatures" is a
//!   continuous effect; not modeled (GAP: back-face-only static ability not
//!   modeled).
//! - Back-face triggered ability (transform back) is not auto-installed on
//!   transform (GAP: back-face-only triggered ability not modeled).

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
    let name = reg.interner_mut().intern("Breakneck Rider");
    let human_sub = reg.interner_mut().intern("Human");
    let scout_sub = reg.interner_mut().intern("Scout");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(scout_sub);
    subtypes.0.insert(werewolf_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Neck Breaker");
    let mut back_subtypes = SubtypeSet::default();
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    back_subtypes.0.insert(back_werewolf_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face transform trigger: upkeep, if no spells cast last turn
            // GAP: werewolf spell-count condition not modeled — fires every upkeep
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: transform_to_back,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: back-face-only triggered ability (transform back on 2+ spells) not modeled
    )
}

fn transform_to_back(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
