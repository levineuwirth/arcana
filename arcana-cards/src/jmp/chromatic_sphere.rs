//! Chromatic Sphere — `{1}` artifact.
//! "{1}, {T}, Sacrifice this artifact: Add one mana of any color. Draw a
//! card." Modeled as FIVE separately activatable abilities, one per WUBRG
//! color — choosing which to activate IS the color choice. Each ability's
//! effect adds the color and draws, so none is a pure mana ability
//! (`is_mana_ability: false`).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

fn ability(text: &str, effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost {
            mana_cost: ManaCost::parse("{1}").expect("valid cost"),
            tap: true,
            sacrifice: true,
            ..ActivationCost::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect,
    }
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chromatic Sphere");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ability(
                "{1}, {T}, Sacrifice this artifact: Add {W}. Draw a card.",
                add_white_draw,
            ))
            .with_activated_ability(ability(
                "{1}, {T}, Sacrifice this artifact: Add {U}. Draw a card.",
                add_blue_draw,
            ))
            .with_activated_ability(ability(
                "{1}, {T}, Sacrifice this artifact: Add {B}. Draw a card.",
                add_black_draw,
            ))
            .with_activated_ability(ability(
                "{1}, {T}, Sacrifice this artifact: Add {R}. Draw a card.",
                add_red_draw,
            ))
            .with_activated_ability(ability(
                "{1}, {T}, Sacrifice this artifact: Add {G}. Draw a card.",
                add_green_draw,
            )),
    )
}

fn add_color_draw(ctx: &ActivationContext, color: ManaColor) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(color, ctx.source)],
        },
        Effect::DrawCards { player: ctx.controller, count: 1 },
    ]
}

fn add_white_draw(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    add_color_draw(ctx, ManaColor::White)
}

fn add_blue_draw(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    add_color_draw(ctx, ManaColor::Blue)
}

fn add_black_draw(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    add_color_draw(ctx, ManaColor::Black)
}

fn add_red_draw(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    add_color_draw(ctx, ManaColor::Red)
}

fn add_green_draw(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    add_color_draw(ctx, ManaColor::Green)
}
