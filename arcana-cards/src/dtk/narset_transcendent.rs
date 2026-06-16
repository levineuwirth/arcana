//! Narset Transcendent — `{2}{W}{U}` Legendary Planeswalker — Narset,
//! starting loyalty 6 — colors U, W.
//!
//! Oracle text:
//! * `+1`: Look at the top card of your library. If it's a noncreature,
//!   nonland card, you may reveal it and put it into your hand. — `DigTopN` of
//!   one with a noncreature-nonland filter (the unchosen tail goes to the
//!   bottom; the printed "leave on top" tail is approximated by
//!   `DigRest::BottomRandom`).
//! * `−2`: When you next cast an instant or sorcery spell this turn, it gains
//!   rebound. — a rebound rider; GAP.
//! * `−9`: You get an emblem with "Your opponents can't cast noncreature
//!   spells." — emblem static; GAP.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::effects::DigRest;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Narset Transcendent");
    let narset = reg.interner_mut().intern("Narset");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(narset);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Look at the top card of your library. If it's a \
                       noncreature, nonland card, you may reveal it and put it \
                       into your hand.".into(),
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
                effect: plus_one_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: When you next cast an instant or sorcery spell this \
                       turn, it gains rebound.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_rebound,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−9: You get an emblem with \"Your opponents can't cast \
                       noncreature spells.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine_emblem,
            }),
    )
}

/// `+1`: look at the top card; take it if noncreature, nonland.
fn plus_one_dig(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::default()
        .without_types(TypeLine::CREATURE.into())
        .without_types(TypeLine::LAND.into());
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 1,
        filter: Some(filter),
        rest: DigRest::BottomRandom,
    }]
}

/// `−2`: next instant/sorcery this turn gains rebound.
fn minus_two_rebound(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: granting rebound to your next instant/sorcery this turn is not
    // expressible from the demonstrated Effect surface.
    Vec::new()
}

/// `−9`: emblem.
fn minus_nine_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblems (static replacement/restriction text) are not expressible.
    Vec::new()
}
