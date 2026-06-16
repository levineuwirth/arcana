//! Sivitri, Dragon Master — `{2}{U}{B}` Legendary Planeswalker — Sivitri,
//! starting loyalty (printed) — colors B, U.
//!
//! Oracle text:
//! * `+1`: Until your next turn, creatures can't attack you or planeswalkers
//!   you control unless their controller pays 2 life for each of those
//!   creatures. — combat-payment restriction; not expressible from the
//!   demonstrated `Effect` surface. GAP.
//! * `−3`: Search your library for a Dragon card, reveal it, put it into your
//!   hand, then shuffle. — `Effect::TutorToHand` with a Dragon subtype filter.
//! * `−7`: Destroy all non-Dragon creatures. — sweep every non-Dragon creature
//!   via `Effect::ForEach` + `DestroyPermanent`.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.
//! * CR 704.5i — 0-loyalty state-based sacrifice.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sivitri, Dragon Master");
    let sivitri = reg.interner_mut().intern("Sivitri");
    let _dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sivitri);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, creatures can't attack you or \
                       planeswalkers you control unless their controller pays \
                       2 life for each of those creatures.".into(),
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
                effect: plus_one_tax,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Search your library for a Dragon card, reveal it, \
                       put it into your hand, then shuffle.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_tutor_dragon,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Destroy all non-Dragon creatures.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_destroy_nondragons,
            }),
    )
}

/// `+1`: combat-payment restriction (creatures can't attack unless their
/// controller pays 2 life each).
fn plus_one_tax(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: per-attacker life-payment attack restriction is not expressible
    // from the demonstrated Effect surface.
    Vec::new()
}

/// `−3`: tutor a Dragon card to hand, then shuffle.
fn minus_three_tutor_dragon(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dragon = match reg.interner().lookup("Dragon") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let filter = ObjectFilter::default().with_subtype_sym(dragon);
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter,
        reveal: true,
    }]
}

/// `−7`: destroy all non-Dragon creatures.
fn minus_seven_destroy_nondragons(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dragon = match reg.interner().lookup("Dragon") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::Any)
        .without_subtype_sym(dragon);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
