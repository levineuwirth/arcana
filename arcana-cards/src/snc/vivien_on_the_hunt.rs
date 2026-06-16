//! Vivien on the Hunt — `{4}{G}{G}` Legendary Planeswalker — Vivien,
//! starting loyalty 5 — colors G.
//!
//! Oracle text:
//! * `+2`: You may sacrifice a creature. If you do, search your library for a
//!   creature card with mana value equal to 1 plus the sacrificed creature's
//!   mana value, put it onto the battlefield, then shuffle. — sacrifice-then-
//!   conditional tutor with a dynamic mana-value bound; not expressible. GAP.
//! * `+1`: Mill five cards, then put any number of creature cards milled this
//!   way into your hand. — `Effect::Mill` of 5 is faithful; the "return milled
//!   creatures to hand" rider is not. Best-effort: mill only.
//! * `−1`: Create a 4/4 green Rhino Warrior creature token. — `CreateToken`.
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
    let name = reg.interner_mut().intern("Vivien on the Hunt");
    let vivien = reg.interner_mut().intern("Vivien");
    let _rhino = reg.interner_mut().intern("Rhino");
    let _warrior = reg.interner_mut().intern("Warrior");
    let _token_name = reg.interner_mut().intern("Rhino Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vivien);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: You may sacrifice a creature. If you do, search your \
                       library for a creature card with mana value equal to 1 \
                       plus the sacrificed creature's mana value, put it onto \
                       the battlefield, then shuffle.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_sacrifice_tutor,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Mill five cards, then put any number of creature \
                       cards milled this way into your hand.".into(),
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
                effect: plus_one_mill_five,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: Create a 4/4 green Rhino Warrior creature token."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_rhino_warrior,
            }),
    )
}

/// `+2`: optional-sacrifice + dynamic-mv tutor-to-battlefield.
fn plus_two_sacrifice_tutor(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: optional sacrifice gating a search bounded by (sacrificed mana
    // value + 1), putting the found card onto the battlefield — not
    // expressible from the demonstrated Effect surface.
    Vec::new()
}

/// `+1`: mill five (return-to-hand rider GAP'd).
fn plus_one_mill_five(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "then put any number of creature cards milled this way into your
    // hand" rider is not expressible; the mill is faithful.
    vec![Effect::Mill { player: ctx.controller, count: 5 }]
}

/// `−1`: create a 4/4 green Rhino Warrior token.
fn minus_one_rhino_warrior(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let token_name = match reg.interner().lookup("Rhino Warrior") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Rhino") {
        subtypes.0.insert(s);
    }
    if let Some(s) = reg.interner().lookup("Warrior") {
        subtypes.0.insert(s);
    }
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
