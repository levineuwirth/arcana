//! Abian, Luvion Usurper — `{5}{R}{G}` Legendary Planeswalker — Abian,
//! starting loyalty 6 — colors G, R.
//!
//! Oracle text:
//! * As Abian enters, you become Abian (a static identity-swap effect, not a
//!   loyalty ability). GAP — not expressible from the demonstrated surface.
//! * `+3`: Discard your hand, then draw cards equal to the greatest power
//!   among creatures you control. — "Discard your hand" is a dynamic discard
//!   count (whole hand, varies) and the draw count is dynamic. GAP.
//! * `+1`: Create a 3/2 red and green Spirit creature token. — `CreateToken`.
//! * `−X`: Abian deals X damage to any target. — dynamic-X loyalty cost; not
//!   declared. GAP: dynamic-X loyalty cost.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Abian, Luvion Usurper");
    let abian = reg.interner_mut().intern("Abian");
    let _spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(abian);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+3: Discard your hand, then draw cards equal to the \
                       greatest power among creatures you control.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_three_wheel,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 3/2 red and green Spirit creature token."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_spirit,
            }),
    )
    // GAP: dynamic-X loyalty cost "−X: Abian deals X damage to any target."
    // is omitted entirely.
}

/// `+3`: discard whole hand, then draw greatest-power-many cards.
fn plus_three_wheel(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Discard your hand" is a dynamic (whole-hand) discard count and the
    // draw count (greatest power among creatures you control) is dynamic —
    // neither is expressible from the demonstrated Effect surface.
    Vec::new()
}

/// `+1`: create a 3/2 red and green Spirit token.
fn plus_one_spirit(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let token_name = match reg.interner().lookup("Spirit") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(token_name);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::red() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
