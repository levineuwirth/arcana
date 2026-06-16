//! Ral, Izzet Viceroy — `{3}{U}{R}` planeswalker, starting loyalty 5.
//! Legendary Planeswalker — Ral.
//!
//! # Rules references
//!
//! * CR 113.3c — printed starting loyalty counters on ETB.
//! * CR 606 — loyalty abilities.
//! * CR 606.3 / CR 704.5i — engine-enforced activation + 0-loyalty SBA.
//!
//! # Scope
//!
//! * `+1` ("Look at the top two cards of your library. Put one into your
//!   hand and the other into your graveyard") — no look-at-top /
//!   distribute-to-hand-and-graveyard primitive in the demonstrated
//!   surface; shell declared, effect GAP'd.
//! * `−3` ("Ral deals damage to target creature equal to the total
//!   number of instant and sorcery cards you own in exile and in your
//!   graveyard") — dynamic amount summed across exile + graveyard zones;
//!   no demonstrated primitive for that count; shell declared with the
//!   correct cost and target, effect GAP'd.
//! * `−8` (emblem) — emblem creation is not demonstrated; shell
//!   declared, effect GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ral, Izzet Viceroy");
    let ral = reg.interner_mut().intern("Ral");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ral);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Look at the top two cards of your library. Put \
                       one of them into your hand and the other into your \
                       graveyard."
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
                effect: plus_one_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Ral, Izzet Viceroy deals damage to target \
                       creature equal to the total number of instant and \
                       sorcery cards you own in exile and in your \
                       graveyard."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: You get an emblem with \"Whenever you cast an \
                       instant or sorcery spell, this emblem deals 4 damage \
                       to any target and you draw two cards.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

/// `+1: Look at the top two cards of your library. Put one into your hand
/// and the other into your graveyard.`
fn plus_one_dig(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no look-at-top-N + distribute-to-hand-and-graveyard primitive
    // in the demonstrated Effect surface.
    Vec::new()
}

/// `−3: Ral deals damage to target creature equal to the total number of
/// instant and sorcery cards you own in exile and in your graveyard.`
fn minus_three_damage(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic damage amount summed across exile + graveyard zones;
    // no demonstrated primitive computes that count.
    Vec::new()
}

/// `−8: You get an emblem with "Whenever you cast an instant or sorcery
/// spell, this emblem deals 4 damage to any target and you draw two
/// cards."`
fn minus_eight_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation is not in the demonstrated Effect surface.
    Vec::new()
}
