//! Lion's Eye Diamond — `{0}` artifact.
//! "Discard your hand, Sacrifice this artifact: Add three mana of any
//! one color. Activate only as an instant."
//!
//! Modeled as FIVE mana abilities (one per WUBRG color), each adding
//! three pips of that color — choosing which to activate IS the color
//! choice. Cost is `discard_hand` + `sacrifice` on one ActivationCost.
//! Mana abilities implicitly skip the stack; the "Activate only as an
//! instant" restriction (can't be activated while another spell is
//! being cast) is not separately expressible.
//! // GAP: 'Activate only as an instant' timing restriction on a mana
//! // ability is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lion's Eye Diamond");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{0}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(led_ability(add_three_white))
            .with_activated_ability(led_ability(add_three_blue))
            .with_activated_ability(led_ability(add_three_black))
            .with_activated_ability(led_ability(add_three_red))
            .with_activated_ability(led_ability(add_three_green)),
    )
}

fn led_ability(
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: "Discard your hand, Sacrifice this artifact: Add three mana of any one color. Activate only as an instant.".into(),
        cost: ActivationCost {
            sacrifice: true,
            discard_hand: true,
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

fn add_three_white(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::White, ctx.source); 3],
    }]
}

fn add_three_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source); 3],
    }]
}

fn add_three_black(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source); 3],
    }]
}

fn add_three_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source); 3],
    }]
}

fn add_three_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source); 3],
    }]
}
