//! Garruk, Caller of Beasts — `{4}{G}{G}` Legendary Planeswalker — Garruk,
//! starting loyalty 4. Mono-green.
//!
//! Oracle:
//! +1: Reveal the top five cards of your library. Put all creature cards
//!     revealed this way into your hand and the rest on the bottom of your
//!     library in any order.
//! −3: You may put a green creature card from your hand onto the battlefield.
//! −7: You get an emblem with "Whenever you cast a creature spell, you may
//!     search your library for a creature card, put it onto the battlefield,
//!     then shuffle."
//!
//! # Scope
//! * +1 — GAP: "reveal top five, take all creatures, rest on bottom in any
//!   order" is a bespoke reveal/sort not expressible in the demonstrated
//!   surface.
//! * −3 — modeled: PutFromHandOntoBattlefield over green creature cards.
//! * −7 — GAP: emblem creation not in Effect catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garruk, Caller of Beasts");
    let sub = reg.interner_mut().intern("Garruk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
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
                text: "+1: Reveal the top five cards of your library. Put all \
                       creature cards revealed this way into your hand and the \
                       rest on the bottom of your library in any order.".into(),
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
                effect: plus_one_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: You may put a green creature card from your hand \
                       onto the battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_put,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You get an emblem with \"Whenever you cast a \
                       creature spell, you may search your library for a \
                       creature card, put it onto the battlefield, then \
                       shuffle.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_gap,
            }),
    )
}

/// `+1` — GAP: bespoke reveal-top-five / take-creatures / rest-bottom.
fn plus_one_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reveal top five, put creatures to hand, rest on bottom in any order.
    Vec::new()
}

/// `−3:` you may put a green creature card from your hand onto the battlefield.
fn minus_three_put(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature().with_colors(ColorSet::green());
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter,
        tapped: false,
    }]
}

/// `−7` — GAP: emblem creation not in Effect catalog.
fn minus_seven_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation not in the demonstrated Effect surface.
    Vec::new()
}
