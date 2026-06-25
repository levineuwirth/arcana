//! Ilysian Caryatid — `{1}{G}` 1/1 Plant.
//! `{T}: Add one mana of any color. If you control a creature with power 4 or greater,
//! add two mana of any one color instead.`
//! Modeled as five mana abilities, one per WUBRG color; the shared {T} cost
//! means only one fires (command_tower idiom). Each resolver checks the board
//! at resolution: if you control a creature with power 4+, it adds TWO mana of
//! that color, otherwise ONE.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ilysian Caryatid");
    let plant = reg.interner_mut().intern("Plant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(mana_ability(
                "{T}: Add {W}. If you control a creature with power 4 or greater, add {W}{W} instead.",
                add_white,
            ))
            .with_activated_ability(mana_ability(
                "{T}: Add {U}. If you control a creature with power 4 or greater, add {U}{U} instead.",
                add_blue,
            ))
            .with_activated_ability(mana_ability(
                "{T}: Add {B}. If you control a creature with power 4 or greater, add {B}{B} instead.",
                add_black,
            ))
            .with_activated_ability(mana_ability(
                "{T}: Add {R}. If you control a creature with power 4 or greater, add {R}{R} instead.",
                add_red,
            ))
            .with_activated_ability(mana_ability(
                "{T}: Add {G}. If you control a creature with power 4 or greater, add {G}{G} instead.",
                add_green,
            )),
    )
}

fn mana_ability(
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

/// "If you control a creature with power 4 or greater" — true if the controller
/// has any creature on the battlefield with computed power >= 4.
fn controls_power_four(state: &GameState, who: arcana_core::types::PlayerId) -> bool {
    state
        .objects_in_zone(Zone::Battlefield)
        .any(|o| {
            o.controller == who
                && o.characteristics.types.is_creature()
                && arcana_core::script::power_of(state, o.id) >= 4
        })
}

fn add_color(state: &GameState, ctx: &ActivationContext, color: ManaColor) -> Vec<Effect> {
    let count = if controls_power_four(state, ctx.controller) { 2 } else { 1 };
    let mana = (0..count)
        .map(|_| ManaUnit::plain(color, ctx.source))
        .collect();
    vec![Effect::AddMana {
        player: ctx.controller,
        mana,
    }]
}

fn add_white(state: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_color(state, ctx, ManaColor::White)
}

fn add_blue(state: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_color(state, ctx, ManaColor::Blue)
}

fn add_black(state: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_color(state, ctx, ManaColor::Black)
}

fn add_red(state: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_color(state, ctx, ManaColor::Red)
}

fn add_green(state: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_color(state, ctx, ManaColor::Green)
}
