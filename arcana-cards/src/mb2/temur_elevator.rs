//! Temur Elevator — nonbasic land (Unknown Event class).
//! "Ascend (If you control ten or more permanents, you get the city's
//! blessing for the rest of the game.)" and "{T}: Add {G}, {U}, or
//! {R}. If you don't have the city's blessing, you lose 1 life."
//! GAP: Ascend / the city's blessing is not a modeled keyword or game
//! state — `keywords: vec![]` is emitted and the "if you don't have
//! the city's blessing" check cannot be made; since the blessing can
//! never be obtained in-engine, the life loss is emitted
//! unconditionally (faithful to the engine's reachable states), which
//! also makes these activations non-mana abilities per the
//! only-AddMana rule.

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
    let name = reg.interner_mut().intern("Temur Elevator");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}. If you don't have the city's blessing, \
                       you lose 1 life."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_green_mana_lose_life,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {U}. If you don't have the city's blessing, \
                       you lose 1 life."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_blue_mana_lose_life,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {R}. If you don't have the city's blessing, \
                       you lose 1 life."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_red_mana_lose_life,
            }),
    )
}

fn add_green_mana_lose_life(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If you don't have the city's blessing" — the city's
    // blessing is unmodeled (and unobtainable in-engine), so the life
    // loss is unconditional.
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
        },
        Effect::LoseLife { player: ctx.controller, amount: 1 },
    ]
}

fn add_blue_mana_lose_life(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
        },
        Effect::LoseLife { player: ctx.controller, amount: 1 },
    ]
}

fn add_red_mana_lose_life(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
        },
        Effect::LoseLife { player: ctx.controller, amount: 1 },
    ]
}
