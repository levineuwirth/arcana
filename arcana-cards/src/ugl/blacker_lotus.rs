//! Blacker Lotus — `{0}` artifact (Unglued).
//! "{T}: Tear this artifact into pieces. Add four mana of any one
//! color. Remove the pieces from the game."
//!
//! Modeled as five mana abilities (one per WUBRG color — choosing the
//! ability IS the "any one color" choice), each costing {T} plus
//! exiling this artifact (`exile_self` — "remove the pieces from the
//! game"). The physical tearing is flavor only.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

fn tear_cost() -> ActivationCost {
    ActivationCost {
        tap: true,
        exile_self: true,
        ..ActivationCost::default()
    }
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blacker Lotus");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{0}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Tear this artifact into pieces. Add {W}{W}{W}{W}. Remove the pieces from the game.".into(),
                cost: tear_cost(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_four_white,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Tear this artifact into pieces. Add {U}{U}{U}{U}. Remove the pieces from the game.".into(),
                cost: tear_cost(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_four_blue,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Tear this artifact into pieces. Add {B}{B}{B}{B}. Remove the pieces from the game.".into(),
                cost: tear_cost(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_four_black,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Tear this artifact into pieces. Add {R}{R}{R}{R}. Remove the pieces from the game.".into(),
                cost: tear_cost(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_four_red,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Tear this artifact into pieces. Add {G}{G}{G}{G}. Remove the pieces from the game.".into(),
                cost: tear_cost(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_four_green,
            }),
    )
}

fn add_four_white(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::White, ctx.source); 4],
    }]
}

fn add_four_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source); 4],
    }]
}

fn add_four_black(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source); 4],
    }]
}

fn add_four_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source); 4],
    }]
}

fn add_four_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source); 4],
    }]
}
