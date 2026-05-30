//! Village Messenger // Moonrise Intruder — `{R}` red Creature — Human Werewolf // Creature — Werewolf.
//! Front: 1/1. Haste.
//!   At the beginning of each upkeep, if no spells were cast last turn, transform.
//! ---
//! Back (Moonrise Intruder): Werewolf. Menace.
//!   At the beginning of each upkeep, if a player cast two or more spells last turn,
//!   transform this creature.
//!
//! GAP: "if no spells were cast last turn" — werewolf day-to-night condition not
//!      modeled (no per-turn spell count from LAST turn in engine). Trigger fires
//!      unconditionally at upkeep begin as approximation.
//! GAP: "if a player cast two or more spells last turn" — night-to-day condition
//!      similarly not modeled. Back-face transform trigger also fires unconditionally.
//! GAP: back-face-only triggered ability not auto-installed on transform; both
//!      upkeep triggers are authored on the CardDefinition and fire on both faces.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Village Messenger");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut front_subs = SubtypeSet::default();
    front_subs.0.insert(human_sub);
    front_subs.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subs,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Moonrise Intruder");
    let werewolf_sub2 = reg.interner_mut().intern("Werewolf");
    let mut back_subs = SubtypeSet::default();
    back_subs.0.insert(werewolf_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subs,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Menace],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face trigger: at the beginning of each upkeep, if no spells were
            // cast last turn, transform. GAP: condition not modeled; fires unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: upkeep_transform_day_to_night,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back-face trigger: at the beginning of each upkeep, if a player cast two
            // or more spells last turn, transform back. GAP: condition not modeled; fires
            // unconditionally. GAP: back-face-only trigger fires on both faces.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: upkeep_transform_night_to_day,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_transform_day_to_night(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if no spells were cast last turn" condition not modeled.
    vec![Effect::Transform { target: trig.source }]
}

fn upkeep_transform_night_to_day(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if a player cast two or more spells last turn" condition not modeled.
    vec![Effect::Transform { target: trig.source }]
}
