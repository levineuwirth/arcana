//! The Secret Lair — legendary land, subtype Lair (Unfinity, 2022).
//! "{T}: Add {C}." and "{T}, Say the secret word: Add one mana of any
//! color. Scry 1. You gain 1 life." The any-color choice is modeled as
//! five separate activated abilities; the "Say the secret word" cost is
//! an Un-set action with no ActivationCost mapping (GAP — only the tap is
//! paid).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, ManaColor, SubtypeSet, SupertypeSet, TypeLine,
};

fn secret_word_ability(
    color_text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        // GAP: "Say the secret word" — the Un-set verbal cost is not
        // expressible; only the tap is paid.
        text: format!(
            "{{T}}, Say the secret word: Add {}. Scry 1. You gain 1 life.",
            color_text
        ),
        cost: ActivationCost::tap_only(),
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
    let name = reg.interner_mut().intern("The Secret Lair");
    let lair = reg.interner_mut().intern("Lair");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lair);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
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
            .with_activated_ability(secret_word_ability("{W}", word_white))
            .with_activated_ability(secret_word_ability("{U}", word_blue))
            .with_activated_ability(secret_word_ability("{B}", word_black))
            .with_activated_ability(secret_word_ability("{R}", word_red))
            .with_activated_ability(secret_word_ability("{G}", word_green)),
    )
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

fn word_effects(ctx: &ActivationContext, color: ManaColor) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(color, ctx.source)],
        },
        Effect::Scry { player: ctx.controller, count: 1 },
        Effect::GainLife { player: ctx.controller, amount: 1 },
    ]
}

fn word_white(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    word_effects(ctx, ManaColor::White)
}

fn word_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    word_effects(ctx, ManaColor::Blue)
}

fn word_black(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    word_effects(ctx, ManaColor::Black)
}

fn word_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    word_effects(ctx, ManaColor::Red)
}

fn word_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    word_effects(ctx, ManaColor::Green)
}
