//! Tasha, Unholy Archmage — `{2}{U}{B}` Legendary Planeswalker — Tasha,
//! starting loyalty 5 — colors B, U.
//!
//! Oracle text:
//! * `+1`: Until your next turn, whenever a creature attacks you or Tasha,
//!   Unholy Archmage, put a -1/-1 counter on that creature. — a floating
//!   delayed triggered window keyed on combat. GAP: not expressible from the
//!   demonstrated Effect surface.
//! * `−2`: Target opponent puts a creature card of their choice from their
//!   graveyard onto the battlefield under your control. — graveyard
//!   reanimation under your control with opponent choice. GAP.
//! * `−6`: Target opponent reveals cards from the top of their library until
//!   three creature cards are revealed, then puts those cards onto the
//!   battlefield under your control. — reveal-until-N steal. GAP.
//!
//! All three abilities are GAP'd but declared as shells with correct loyalty
//! costs.
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
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tasha, Unholy Archmage");
    let tasha = reg.interner_mut().intern("Tasha");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tasha);

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
                text: "+1: Until your next turn, whenever a creature attacks \
                       you or Tasha, Unholy Archmage, put a -1/-1 counter on \
                       that creature.".into(),
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
                effect: plus_one_window,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Target opponent puts a creature card of their \
                       choice from their graveyard onto the battlefield under \
                       your control.".into(),
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
                effect: minus_two_reanimate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: Target opponent reveals cards from the top of their \
                       library until three creature cards are revealed, then \
                       puts those cards onto the battlefield under your \
                       control and the rest into their graveyard.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_reveal_steal,
            }),
    )
}

/// `+1`: floating combat-triggered -1/-1 window.
fn plus_one_window(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: floating delayed triggered window (whenever a creature attacks you
    // or Tasha) not expressible from the demonstrated Effect surface.
    Vec::new()
}

/// `−2`: opponent-choice graveyard reanimation under your control.
fn minus_two_reanimate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: graveyard reanimation with opponent choice + control change not
    // expressible from the demonstrated Effect surface.
    Vec::new()
}

/// `−6`: reveal-until-three-creatures steal.
fn minus_six_reveal_steal(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reveal-until-N-creatures then put onto battlefield under your
    // control not expressible from the demonstrated Effect surface.
    Vec::new()
}
