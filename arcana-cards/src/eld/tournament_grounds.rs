//! Tournament Grounds — nonbasic land.
//! "{T}: Add {C}." and "{T}: Add {R}, {W}, or {B}. Spend this mana
//! only to cast a Knight or Equipment spell." Four mana abilities
//! (the three-color choice as three abilities); the Knight/Equipment
//! spend restriction is a GAP.

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
    let name = reg.interner_mut().intern("Tournament Grounds");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    // GAP: "Spend this mana only to cast a Knight or Equipment spell"
    // — mana spend restrictions are not expressible; plain mana
    // abilities emitted.
    let mut def = CardDefinition::new(name, chars)
        .with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Add {C}.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: true,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_colorless,
        });
    for (text, effect) in [
        ("{T}: Add {R}.", add_red as fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>),
        ("{T}: Add {W}.", add_white),
        ("{T}: Add {B}.", add_black),
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

fn add_colorless(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_one(ManaColor::Colorless, ctx)
}

fn add_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_one(ManaColor::Red, ctx)
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
