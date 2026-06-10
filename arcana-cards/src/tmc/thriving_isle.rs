//! Thriving Isle — nonbasic land.
//! "This land enters tapped. As it enters, choose a color other than
//! blue." and "{T}: Add {U} or one mana of the chosen color."
//! Enters-tapped is `EntersWithSpec::Tapped`. The as-enters color
//! choice is not modeled — approximated as one mana ability per color
//! (blue plus each possible chosen color) with a GAP note.

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
    let name = reg.interner_mut().intern("Thriving Isle");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    // GAP: "As it enters, choose a color other than blue" — the
    // as-enters color choice (and the lock to that single chosen
    // color) is not modeled; approximated as one mana ability per
    // color so any color is available per activation.
    let mut def = CardDefinition::new(name, chars)
        .with_enters_with(EntersWithSpec::Tapped);
    for (text, effect) in [
        ("{T}: Add {U}.", add_blue as fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>),
        ("{T}: Add {W}.", add_white),
        ("{T}: Add {B}.", add_black),
        ("{T}: Add {R}.", add_red),
        ("{T}: Add {G}.", add_green),
    ] {
        def = def.with_activated_ability(ActivatedAbilityDef {
            text: text.into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: true,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect,
        });
    }
    reg.register(def)
}

fn add_one(color: ManaColor, ctx: &ActivationContext) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(color, ctx.source)],
    }]
}

fn add_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_one(ManaColor::Blue, ctx)
}

fn add_white(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_one(ManaColor::White, ctx)
}

fn add_black(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_one(ManaColor::Black, ctx)
}

fn add_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_one(ManaColor::Red, ctx)
}

fn add_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_one(ManaColor::Green, ctx)
}
