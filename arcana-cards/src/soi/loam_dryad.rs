//! Loam Dryad — `{G}` 1/2 green Dryad Horror.
//! "{T}, Tap an untapped creature you control: Add one mana of any color."
//! "Add one mana of any color" is modeled as five mana abilities, one per WUBRG
//! color (the player picks the color by choosing which ability to activate; the
//! shared {T} cost means only one fires).
//! GAP: ActivationCost has no "tap another untapped creature you control" cost field.

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
    let name = reg.interner_mut().intern("Loam Dryad");
    let dryad = reg.interner_mut().intern("Dryad");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dryad);
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(mana_ability("{T}, Tap an untapped creature you control: Add {W}.", add_white_mana))
            .with_activated_ability(mana_ability("{T}, Tap an untapped creature you control: Add {U}.", add_blue_mana))
            .with_activated_ability(mana_ability("{T}, Tap an untapped creature you control: Add {B}.", add_black_mana))
            .with_activated_ability(mana_ability("{T}, Tap an untapped creature you control: Add {R}.", add_red_mana))
            .with_activated_ability(mana_ability("{T}, Tap an untapped creature you control: Add {G}.", add_green_mana)),
    )
}

fn mana_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        // GAP: no "tap another untapped creature you control" cost field; using tap only.
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

fn add_white_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)] }]
}
fn add_blue_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)] }]
}
fn add_black_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)] }]
}
fn add_red_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)] }]
}
fn add_green_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)] }]
}
