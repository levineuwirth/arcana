//! Phyrexian Lens — {3} artifact (Urza's Destiny, 1999).
//! "{T}, Pay 1 life: Add one mana of any color." Modeled as five mana
//! abilities, one per WUBRG color, each costing tap + 1 life —
//! choosing which to activate IS the color choice.

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
    let name = reg.interner_mut().intern("Phyrexian Lens");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    let mut def = CardDefinition::new(name, chars);
    let abilities: [(&str, _); 5] = [
        ("{T}, Pay 1 life: Add {W}.", add_white_mana as fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>),
        ("{T}, Pay 1 life: Add {U}.", add_blue_mana),
        ("{T}, Pay 1 life: Add {B}.", add_black_mana),
        ("{T}, Pay 1 life: Add {R}.", add_red_mana),
        ("{T}, Pay 1 life: Add {G}.", add_green_mana),
    ];
    for (text, effect) in abilities {
        def = def.with_activated_ability(ActivatedAbilityDef {
            text: text.into(),
            cost: ActivationCost { tap: true, life: 1, ..ActivationCost::default() },
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
