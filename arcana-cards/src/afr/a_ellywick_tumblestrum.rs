//! A-Ellywick Tumblestrum — `{2}{G}{G}` Legendary Planeswalker — Ellywick.
//! Printed starting loyalty 5 (CR 113.3c). Color green.
//!
//! Keyword: "Venture into the dungeon" is a templating phrase used by the
//! `+1`, not a standalone keyword ability — `keywords: vec![]`.
//!
//! Loyalty abilities (CR 606):
//! * `+1`: Venture into the dungeon. — expressible via `Effect::Venture`.
//! * `−2`: Look at the top six cards of your library. You may reveal a
//!   creature card from among them and put it into your hand. If it's
//!   legendary, you gain 3 life. Put the rest on the bottom in a random
//!   order. — GAP (DigTopN takes a single filtered card to hand but the
//!   reflexive "if legendary, gain 3 life" conditional on the taken card
//!   isn't expressible).
//! * `−6`: You get an emblem with "Creatures you control have trample and
//!   haste and get +2/+2 for each differently named dungeon you've
//!   completed." — GAP (emblem with a dynamic-count anthem).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Ellywick Tumblestrum");
    let ellywick = reg.interner_mut().intern("Ellywick");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ellywick);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
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
                text: "+1: Venture into the dungeon.".into(),
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
                effect: plus_one_venture,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Look at the top six cards of your library. You may reveal \
                       a creature card from among them and put it into your hand. \
                       If it's legendary, you gain 3 life. Put the rest on the \
                       bottom of your library in a random order.".into(),
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
                effect: minus_two_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: You get an emblem with \"Creatures you control have \
                       trample and haste and get +2/+2 for each differently named \
                       dungeon you've completed.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

/// `+1`: venture into the dungeon.
fn plus_one_venture(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Venture {
        player: ctx.controller,
    }]
}

/// `−2`: dig with a reflexive legendary-life rider — GAP.
fn minus_two_dig(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: DigTopN takes a filtered card to hand, but the reflexive "if
    // it's legendary, gain 3 life" conditional on the taken card and the
    // bottom-in-random-order rest are not jointly expressible here.
    Vec::new()
}

/// `−6`: emblem — GAP.
fn minus_six_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with a dynamic-count anthem ("+2/+2 for each differently
    // named dungeon completed") is beyond the demonstrated surface.
    Vec::new()
}
