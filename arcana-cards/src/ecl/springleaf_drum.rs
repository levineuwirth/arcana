//! Springleaf Drum — `{1}` artifact.
//! "{T}, Tap an untapped creature you control: Add one mana of any color."
//!
//! "Add one mana of any color" is modeled as five mana abilities, one per
//! WUBRG color — choosing which to activate IS the color choice. Each
//! pays `{T}` plus tapping an untapped creature you control
//! (`tap_other`).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

fn drum_cost() -> ActivationCost {
    ActivationCost {
        tap: true,
        tap_other: Some(ObjectFilter {
            types: Some(TypeLine::CREATURE.into()),
            ..ObjectFilter::default()
        }),
        ..ActivationCost::default()
    }
}

fn drum_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: drum_cost(),
        target_requirements: Vec::new(),
        is_mana_ability: true,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect,
    }
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Springleaf Drum");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(drum_ability(
                "{T}, Tap an untapped creature you control: Add {W}.",
                add_white,
            ))
            .with_activated_ability(drum_ability(
                "{T}, Tap an untapped creature you control: Add {U}.",
                add_blue,
            ))
            .with_activated_ability(drum_ability(
                "{T}, Tap an untapped creature you control: Add {B}.",
                add_black,
            ))
            .with_activated_ability(drum_ability(
                "{T}, Tap an untapped creature you control: Add {R}.",
                add_red,
            ))
            .with_activated_ability(drum_ability(
                "{T}, Tap an untapped creature you control: Add {G}.",
                add_green,
            )),
    )
}

fn add_white(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)],
    }]
}

fn add_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}

fn add_black(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
    }]
}

fn add_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}

fn add_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}
