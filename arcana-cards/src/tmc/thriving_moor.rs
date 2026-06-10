//! Thriving Moor — Land.
//! "This land enters tapped. As it enters, choose a color other than
//! black." and "{T}: Add {B} or one mana of the chosen color."
//!
//! Enters-tapped is wired; the ETB color choice is not expressible, so
//! the "chosen color" half is approximated as one mana ability per
//! non-black color (over-permissive: the real card locks the color at
//! ETB).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thriving Moor");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    // GAP: 'As it enters, choose a color other than black' — ETB color
    // choices are not expressible; the chosen-color mana ability is
    // approximated as one ability per non-black color below.
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Tapped)
            .with_activated_ability(thriving_mana("{T}: Add {B}.", add_black_mana))
            .with_activated_ability(thriving_mana("{T}: Add {W}.", add_white_mana))
            .with_activated_ability(thriving_mana("{T}: Add {U}.", add_blue_mana))
            .with_activated_ability(thriving_mana("{T}: Add {R}.", add_red_mana))
            .with_activated_ability(thriving_mana("{T}: Add {G}.", add_green_mana)),
    )
}

fn thriving_mana(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost::tap_only(),
        target_requirements: Vec::new(),
        is_mana_ability: true,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect,
    }
}

fn add_black_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
    }]
}

fn add_white_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)],
    }]
}

fn add_blue_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}

fn add_red_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}

fn add_green_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}
