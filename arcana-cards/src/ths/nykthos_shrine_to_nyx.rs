//! Nykthos, Shrine to Nyx — Legendary nonbasic land.
//! "{T}: Add {C}." and "{2}, {T}: Choose a color. Add an amount of
//! mana of that color equal to your devotion to that color." The
//! color choice is modeled as five separate devotion-scaled mana
//! abilities (choosing which to activate IS the color choice), each
//! computing the amount via `script::devotion`.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, ManaColor, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nykthos, Shrine to Nyx");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(devotion_ability(
                "{2}, {T}: Add {W} for each of your devotion to white.",
                devotion_white,
            ))
            .with_activated_ability(devotion_ability(
                "{2}, {T}: Add {U} for each of your devotion to blue.",
                devotion_blue,
            ))
            .with_activated_ability(devotion_ability(
                "{2}, {T}: Add {B} for each of your devotion to black.",
                devotion_black,
            ))
            .with_activated_ability(devotion_ability(
                "{2}, {T}: Add {R} for each of your devotion to red.",
                devotion_red,
            ))
            .with_activated_ability(devotion_ability(
                "{2}, {T}: Add {G} for each of your devotion to green.",
                devotion_green,
            )),
    )
}

fn devotion_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost {
            mana_cost: ManaCost::parse("{2}").expect("valid cost"),
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

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn devotion_mana(
    state: &GameState,
    ctx: &ActivationContext,
    colors: ColorSet,
    color: ManaColor,
) -> Vec<Effect> {
    let n = script::devotion(state, ctx.controller, colors);
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(color, ctx.source); n as usize],
    }]
}

fn devotion_white(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    devotion_mana(state, ctx, ColorSet::white(), ManaColor::White)
}

fn devotion_blue(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    devotion_mana(state, ctx, ColorSet::blue(), ManaColor::Blue)
}

fn devotion_black(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    devotion_mana(state, ctx, ColorSet::black(), ManaColor::Black)
}

fn devotion_red(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    devotion_mana(state, ctx, ColorSet::red(), ManaColor::Red)
}

fn devotion_green(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    devotion_mana(state, ctx, ColorSet::green(), ManaColor::Green)
}
