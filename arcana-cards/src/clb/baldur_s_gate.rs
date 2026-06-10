//! Baldur's Gate — Legendary Land — Gate (CLB).
//! "{T}: Add {C}." and "{2}, {T}: Add X mana of any one color, where X is
//! the number of other Gates you control."
//!
//! The X-of-any-one-color activation is modeled as FIVE mana abilities,
//! one per WUBRG color — choosing which to activate IS the color choice;
//! each adds X mana where X counts your other Gates at resolution.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Baldur's Gate");
    let gate = reg.interner_mut().intern("Gate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gate);
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
            .with_activated_ability(gate_mana_ability("{2}, {T}: Add {W} for each other Gate you control.", add_x_white))
            .with_activated_ability(gate_mana_ability("{2}, {T}: Add {U} for each other Gate you control.", add_x_blue))
            .with_activated_ability(gate_mana_ability("{2}, {T}: Add {B} for each other Gate you control.", add_x_black))
            .with_activated_ability(gate_mana_ability("{2}, {T}: Add {R} for each other Gate you control.", add_x_red))
            .with_activated_ability(gate_mana_ability("{2}, {T}: Add {G} for each other Gate you control.", add_x_green)),
    )
}

fn gate_mana_ability(
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

/// X = the number of OTHER Gates you control (total Gates you control
/// minus this one, which is itself a Gate on the battlefield).
fn add_x_of(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
    color: ManaColor,
) -> Vec<Effect> {
    let gates = script::count_matching(
        state,
        &script::subtype_filter(reg, "Gate").controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let x = gates.saturating_sub(1) as usize;
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(color, ctx.source); x],
    }]
}

fn add_x_white(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    add_x_of(state, ctx, reg, ManaColor::White)
}

fn add_x_blue(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    add_x_of(state, ctx, reg, ManaColor::Blue)
}

fn add_x_black(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    add_x_of(state, ctx, reg, ManaColor::Black)
}

fn add_x_red(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    add_x_of(state, ctx, reg, ManaColor::Red)
}

fn add_x_green(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    add_x_of(state, ctx, reg, ManaColor::Green)
}
