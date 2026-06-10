//! Sunken Ruins — nonbasic land (Shadowmoor filterland).
//! "{T}: Add {C}." and "{U/B}, {T}: Add {U}{U}, {U}{B}, or {B}{B}."
//!
//! The three output choices are modeled as three separate mana
//! abilities, each costing the hybrid {U/B} + tap — choosing which to
//! activate IS the output choice.

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
    let name = reg.interner_mut().intern("Sunken Ruins");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    let mut def = CardDefinition::new(name, chars).with_activated_ability(
        ActivatedAbilityDef {
            text: "{T}: Add {C}.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: true,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_colorless_mana,
        },
    );
    let filters: [(&str, _); 3] = [
        ("{U/B}, {T}: Add {U}{U}.", add_uu as fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>),
        ("{U/B}, {T}: Add {U}{B}.", add_ub),
        ("{U/B}, {T}: Add {B}{B}.", add_bb),
    ];
    for (text, effect) in filters {
        def = def.with_activated_ability(ActivatedAbilityDef {
            text: text.into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{U/B}").expect("valid cost"),
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
        });
    }
    reg.register(def)
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

fn add_uu(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source); 2],
    }]
}

fn add_ub(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Blue, ctx.source),
            ManaUnit::plain(ManaColor::Black, ctx.source),
        ],
    }]
}

fn add_bb(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source); 2],
    }]
}
