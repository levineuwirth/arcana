//! Vivien, Monsters' Advocate — `{3}{G}{G}` Legendary Planeswalker — Vivien, loyalty 4.
//!
//! Static: You may look at the top card of your library any time.
//! Static: You may cast creature spells from the top of your library.
//! +1: Create a 3/3 green Beast creature token. Put your choice of a vigilance
//!   counter, a reach counter, or a trample counter on it.
//! −2: When you next cast a creature spell this turn, search your library for a
//!   creature card with lesser mana value, put it onto the battlefield, then
//!   shuffle.
//!
//! # Scope
//! GAP: both static abilities (look-at-top, cast-creatures-from-top) carry no
//!   loyalty cost and aren't expressible; recorded only in this doc comment.
//! GAP: the +1 "put your choice of a vigilance/reach/trample counter on it"
//!   rider (a keyword-counter choice on the new token, whose id the resolver
//!   can't see) isn't expressible; the 3/3 green Beast is minted plain.
//! GAP: the −2 next-cast rider ("search library for a creature with lesser mana
//!   value, put onto battlefield") isn't among the expressible next-cast riders.
//!   Ability shell declared, effect empty.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::effects::TokenDefinition;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vivien, Monsters' Advocate");
    let vivien = reg.interner_mut().intern("Vivien");
    let _beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vivien);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 3/3 green Beast creature token. Put your choice of a vigilance counter, a reach counter, or a trample counter on it.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_beast,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: When you next cast a creature spell this turn, search your library for a creature card with lesser mana value, put it onto the battlefield, then shuffle.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_gap,
            }),
    )
}

fn plus_one_beast(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the vigilance/reach/trample keyword-counter choice on the new token
    // (whose id the resolver can't see) isn't expressible; minting plain 3/3.
    let beast = reg.interner().lookup("Beast").expect("Beast interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let token = TokenDefinition {
        name: beast,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_two_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: next-cast "search library for a lesser-MV creature, put onto
    // battlefield" rider isn't among the expressible next-cast riders.
    Vec::new()
}
