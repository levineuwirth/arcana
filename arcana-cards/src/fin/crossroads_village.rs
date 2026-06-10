//! Crossroads Village — nonbasic land, subtype Town (Bloomburrow,
//! 2024). "This land enters tapped. As it enters, choose a color.
//! {T}: Add one mana of the chosen color." The as-enters color choice
//! is not expressible; modeled as five mana abilities (one per WUBRG
//! color) — choosing which to activate stands in for the choice.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crossroads Village");
    let town = reg.interner_mut().intern("Town");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(town);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        ..Default::default()
    };
    // GAP: "As it enters, choose a color. {T}: Add one mana of the
    // chosen color." — the as-enters color memory is not expressible;
    // modeled as five per-color mana abilities (strictly more flexible
    // than the printed card).
    let mut def = CardDefinition::new(name, chars)
        .with_enters_with(EntersWithSpec::Tapped);
    let abilities: [(&str, _); 5] = [
        ("{T}: Add {W}.", add_white_mana as fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>),
        ("{T}: Add {U}.", add_blue_mana),
        ("{T}: Add {B}.", add_black_mana),
        ("{T}: Add {R}.", add_red_mana),
        ("{T}: Add {G}.", add_green_mana),
    ];
    for (text, effect) in abilities {
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
