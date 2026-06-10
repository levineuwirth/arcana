//! Meteor Crater — nonbasic land (Planeshift).
//! "{T}: Choose a color of a permanent you control. Add one mana of
//! that color."
//!
//! Modeled as five separate mana abilities, one per WUBRG color — the
//! catalog idiom for color-choice mana. The "of a permanent you
//! control" legality restriction on the chosen color is not
//! enforceable (GAP noted).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Meteor Crater");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    // GAP: "Choose a color of a permanent you control" — the chosen color
    // must match a permanent you control; that restriction is not
    // expressible, so all five colors are always offered.
    let mut def = CardDefinition::new(name, chars);
    let any_color: [(&str, _); 5] = [
        ("{T}: Add {W}.", add_white_mana as fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>),
        ("{T}: Add {U}.", add_blue_mana),
        ("{T}: Add {B}.", add_black_mana),
        ("{T}: Add {R}.", add_red_mana),
        ("{T}: Add {G}.", add_green_mana),
    ];
    for (text, effect) in any_color {
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
