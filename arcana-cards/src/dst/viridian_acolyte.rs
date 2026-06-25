//! Viridian Acolyte — `{G}` 1/1 green Elf Shaman.
//! "{1}, {T}: Add one mana of any color."
//! Modeled as five mana abilities, one per WUBRG color; the player picks
//! the color by choosing which ability to activate (shared {1},{T} cost).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Viridian Acolyte");
    let elf = reg.interner_mut().intern("Elf");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(mana_ability("{1}, {T}: Add {W}.", add_white_mana))
            .with_activated_ability(mana_ability("{1}, {T}: Add {U}.", add_blue_mana))
            .with_activated_ability(mana_ability("{1}, {T}: Add {B}.", add_black_mana))
            .with_activated_ability(mana_ability("{1}, {T}: Add {R}.", add_red_mana))
            .with_activated_ability(mana_ability("{1}, {T}: Add {G}.", add_green_mana)),
    )
}

/// `{1}, {T}: Add one mana of any color` — one mana ability per color.
fn mana_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost {
            mana_cost: ManaCost::parse("{1}").unwrap(),
            tap: true,
            ..ActivationCost::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: true,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect,
    }
}

fn add_one(ctx: &ActivationContext, color: ManaColor) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(color, ctx.source)],
    }]
}

fn add_white_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::White)
}
fn add_blue_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Blue)
}
fn add_black_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Black)
}
fn add_red_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Red)
}
fn add_green_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Green)
}
