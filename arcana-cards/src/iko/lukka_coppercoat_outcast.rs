//! Lukka, Coppercoat Outcast — `{3}{R}{R}` legendary planeswalker,
//! starting loyalty 4. Subtype Lukka; mono-red.
//!
//! Loyalty abilities:
//! * `+1`: Exile the top three cards of your library; creature cards
//!   exiled this way gain "you may cast this card from exile as long as you
//!   control a Lukka planeswalker." GAP — a persistent cast-from-exile
//!   permission (gated on controlling a Lukka) is bespoke (not the
//!   end-of-turn `ImpulseExile` shape).
//! * `−2`: Exile target creature you control, then reveal until a creature
//!   with greater mana value, put it onto the battlefield, rest on bottom.
//!   GAP — the reveal threshold is dynamic (greater than the exiled
//!   creature's mana value); `RevealUntil`'s filter is static.
//! * `−7`: Each creature you control deals damage equal to its power to
//!   each opponent. GAP — multi-source power-based damage to each opponent
//!   is bespoke.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lukka, Coppercoat Outcast");
    let lukka = reg.interner_mut().intern("Lukka");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lukka);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Exile the top three cards of your library. Creature \
                       cards exiled this way gain \"You may cast this card \
                       from exile as long as you control a Lukka \
                       planeswalker.\""
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
                effect: plus_one_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Exile target creature you control, then reveal \
                       cards from the top of your library until you reveal a \
                       creature card with greater mana value. Put that card \
                       onto the battlefield and the rest on the bottom of \
                       your library in a random order."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Each creature you control deals damage equal to \
                       its power to each opponent."
                    .into(),
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
                effect: minus_seven_gap,
            }),
    )
}

fn plus_one_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: persistent cast-from-exile permission gated on controlling a
    // Lukka — not the end-of-turn ImpulseExile shape.
    Vec::new()
}

fn minus_two_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: dynamic reveal threshold (greater than the exiled creature's
    // mana value); RevealUntil's filter is static.
    Vec::new()
}

fn minus_seven_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: multi-source power-based damage to each opponent is bespoke.
    Vec::new()
}
