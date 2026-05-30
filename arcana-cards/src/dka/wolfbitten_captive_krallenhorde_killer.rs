//! Wolfbitten Captive // Krallenhorde Killer — `{G}` Creature — Human Werewolf 1/1.
//! Front:
//!   {1}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn.
//!   At the beginning of each upkeep, if no spells were cast last turn, transform.
//! Back: Werewolf 3/3.
//!   {3}{G}: This creature gets +4/+4 until end of turn. Activate only once each turn.
//!   At the beginning of each upkeep, if a player cast two or more spells last turn, transform.
//!
//! # GAPs
//! - "Activate only once each turn" restriction not modeled on ActivationCost.
//! - Front transform trigger "if no spells were cast last turn" — intervening_if not modeled;
//!   trigger fires every upkeep (GAP: day/night / "no spells last turn" condition).
//! - Back transform trigger "if a player cast two or more spells last turn" — same GAP.
//! - GAP: back-face-only triggered ability (back transform trigger) not modeled as separate.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wolfbitten Captive");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human_sub);
    front_subtypes.0.insert(werewolf_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Krallenhorde Killer
    let back_name = reg.interner_mut().intern("Krallenhorde Killer");
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
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front activated: {1}{G}: +2/+2 until end of turn (face 0 only)
            // GAP: "activate only once each turn" not modeled on ActivationCost
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: front_pump,
            })
            // Back activated: {3}{G}: +4/+4 until end of turn (face 1 only)
            // GAP: "activate only once each turn" not modeled on ActivationCost
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G}: This creature gets +4/+4 until end of turn. Activate only once each turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: back_pump,
            })
            // Front transform trigger: at the beginning of each upkeep
            // GAP: "if no spells were cast last turn" — intervening_if not modeled
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
            })
            // GAP: back-face transform trigger "if a player cast two or more spells last turn"
            // — back-face-only triggered ability not modeled.
    )
}

fn front_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn back_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 4,
        toughness: 4,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn transform_to_back(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if no spells were cast last turn" — condition not checked
    vec![Effect::Transform { target: trig.source }]
}
