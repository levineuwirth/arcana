//! Great Hall of the Citadel — nonbasic land (March of the Machine).
//! "{T}: Add {C}." and "{1}, {T}: Add two mana in any combination of
//! colors. Spend this mana only to cast legendary spells."
//!
//! The two-mana-any-combination ability is approximated as five
//! abilities adding two of one color each (mixed-color pairs are not
//! enumerable in the demonstrated idiom — GAP). The legendary-spells
//! spend restriction is not expressible (GAP).

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
    let name = reg.interner_mut().intern("Great Hall of the Citadel");
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
    // GAP: "Add two mana in any combination of colors" — modeled as five
    // same-color pair abilities; mixed-color pairs are not offered.
    // GAP: "Spend this mana only to cast legendary spells" — mana spend
    // restrictions are not expressible.
    let pairs: [(&str, _); 5] = [
        ("{1}, {T}: Add {W}{W}.", add_two_white as fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>),
        ("{1}, {T}: Add {U}{U}.", add_two_blue),
        ("{1}, {T}: Add {B}{B}.", add_two_black),
        ("{1}, {T}: Add {R}{R}.", add_two_red),
        ("{1}, {T}: Add {G}{G}.", add_two_green),
    ];
    for (text, effect) in pairs {
        def = def.with_activated_ability(ActivatedAbilityDef {
            text: text.into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
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

fn add_two_white(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::White, ctx.source); 2],
    }]
}

fn add_two_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source); 2],
    }]
}

fn add_two_black(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source); 2],
    }]
}

fn add_two_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source); 2],
    }]
}

fn add_two_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source); 2],
    }]
}
