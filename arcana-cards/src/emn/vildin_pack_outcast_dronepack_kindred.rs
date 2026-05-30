//! Vildin-Pack Outcast // Dronepack Kindred — `{4}{R}` red transform creature.
//! Front: Werewolf Horror 4/4, Trample.
//!   "{R}: This creature gets +1/-1 until end of turn."
//!   "{5}{R}{R}: Transform this creature."
//! Back: Eldrazi Werewolf (no mana cost), Trample.
//!   "{1}: This creature gets +1/+0 until end of turn."
//! Back-face activated ability modeled via face_gate: Some(1) on the shared list.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vildin-Pack Outcast");
    let werewolf = reg.interner_mut().intern("Werewolf");
    let horror = reg.interner_mut().intern("Horror");

    let mut front_subs = SubtypeSet::default();
    front_subs.0.insert(werewolf);
    front_subs.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subs,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Dronepack Kindred");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let werewolf_b = reg.interner_mut().intern("Werewolf");

    let mut back_subs = SubtypeSet::default();
    back_subs.0.insert(eldrazi);
    back_subs.0.insert(werewolf_b);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subs,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(7)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: "{R}: This creature gets +1/-1 until end of turn."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}: This creature gets +1/-1 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0),
                effect: front_pump,
            })
            // Front face: "{5}{R}{R}: Transform this creature."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{R}{R}: Transform this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{R}{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_self,
            })
            // Back face: "{1}: This creature gets +1/+0 until end of turn."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: This creature gets +1/+0 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1),
                effect: back_pump,
            }),
    )
}

/// Front face: +1/-1 until end of turn.
fn front_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: -1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

/// Transform this creature.
fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

/// Back face: +1/+0 until end of turn.
fn back_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
