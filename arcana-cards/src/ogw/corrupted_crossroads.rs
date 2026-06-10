//! Corrupted Crossroads — nonbasic land.
//! "{T}: Add {C}." and "{T}, Pay 1 life: Add one mana of any color.
//! Spend this mana only to cast a spell with devoid." The any-color
//! choice is modeled as five mana abilities (tap + 1 life each); the
//! devoid spend restriction is a GAP.

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
    let name = reg.interner_mut().intern("Corrupted Crossroads");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    // GAP: "Spend this mana only to cast a spell with devoid" — mana
    // spend restrictions are not expressible; plain mana abilities
    // emitted.
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
        ("{T}, Pay 1 life: Add {W}.", add_white as fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>),
        ("{T}, Pay 1 life: Add {U}.", add_blue),
        ("{T}, Pay 1 life: Add {B}.", add_black),
        ("{T}, Pay 1 life: Add {R}.", add_red),
        ("{T}, Pay 1 life: Add {G}.", add_green),
    ] {
        def = def.with_activated_ability(ActivatedAbilityDef {
            text: text.into(),
            cost: ActivationCost {
                tap: true,
                life: 1,
                ..ActivationCost::default()
            },
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

fn add_white(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_one(ManaColor::White, ctx)
}

fn add_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_one(ManaColor::Blue, ctx)
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
