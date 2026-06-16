//! Dovin, Grand Arbiter — `{1}{W}{U}` Legendary Planeswalker — Dovin,
//! starting loyalty 3. White-blue.
//!
//! Oracle:
//! +1: Until end of turn, whenever a creature you control deals combat damage
//!     to a player, put a loyalty counter on Dovin.
//! −1: Create a 1/1 colorless Thopter artifact creature token with flying. You
//!     gain 1 life.
//! −7: Look at the top ten cards of your library. Put three of them into your
//!     hand and the rest on the bottom of your library in a random order.
//!
//! # Scope
//! * +1 — GAP: "until end of turn, whenever … put a loyalty counter" is a
//!   floating delayed/conditional triggered ability not expressible here.
//! * −1 — modeled: create the flying Thopter token, then gain 1 life.
//! * −7 — GAP: "look at top ten, put three into hand, rest on bottom in random
//!   order" is a bespoke dig not expressible in the demonstrated surface.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dovin, Grand Arbiter");
    let sub = reg.interner_mut().intern("Dovin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);
    let _ = reg.interner_mut().intern("Thopter");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until end of turn, whenever a creature you control \
                       deals combat damage to a player, put a loyalty counter \
                       on Dovin.".into(),
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
                text: "−1: Create a 1/1 colorless Thopter artifact creature \
                       token with flying. You gain 1 life.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_thopter,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Look at the top ten cards of your library. Put three \
                       of them into your hand and the rest on the bottom of \
                       your library in a random order.".into(),
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

/// `+1` — GAP: floating "until end of turn, whenever … loyalty counter".
fn plus_one_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: until-end-of-turn delayed conditional loyalty-gain.
    Vec::new()
}

/// `−1:` create a 1/1 colorless flying Thopter artifact creature, gain 1 life.
fn minus_one_thopter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let thopter_name = reg.interner().lookup("Thopter").unwrap_or_default();
    let mut t_subtypes = SubtypeSet::default();
    t_subtypes.0.insert(thopter_name);
    let token = TokenDefinition {
        name: thopter_name,
        colors: ColorSet::colorless(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes: t_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: ctx.controller,
            token,
        },
        Effect::GainLife {
            player: ctx.controller,
            amount: 1,
        },
    ]
}

/// `−7` — GAP: bespoke look-at-ten / take-three / rest-bottom-random dig.
fn minus_seven_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: look-at-top-ten + put-three-to-hand + rest-on-bottom-random.
    Vec::new()
}
