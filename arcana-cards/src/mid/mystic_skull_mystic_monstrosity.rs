//! Mystic Skull // Mystic Monstrosity (transforming DFC, layout "transform")
//!
//! Front face: Mystic Skull — {2} Artifact.
//!   {1}, {T}: Add one mana of any color.
//!   {5}, {T}: Transform this artifact.
//! Back face: Mystic Monstrosity — Artifact Creature — Construct.
//!   Lands you control have "{T}: Add one mana of any color."
//!
//! GAP: "add one mana of any color" — Effect::AddMana requires a fixed color;
//!   modeled as colorless (the color choice is not expressible).
//! GAP: back-face static "Lands you control have '{T}: Add one mana of any color.'"
//!   — granting an activated ability to other permanents (an ability-granting
//!   continuous effect) is not expressible with the demonstrated API.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mystic Skull");
    let construct = reg.interner_mut().intern("Construct");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Mystic Monstrosity");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(construct);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
            subtypes: back_subtypes,
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {1}, {T}: Add one mana of any color.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Add one mana of any color.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..Default::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_any_color,
            })
            // {5}, {T}: Transform this artifact.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}, {T}: Transform this artifact.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                    tap: true,
                    ..Default::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_self,
            }),
    )
}

fn add_any_color(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "any color" choice not available; emitting colorless as placeholder.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn transform_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
