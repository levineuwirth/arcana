//! Svega, the Unconventional — `{1}{G}{W}{U}` Legendary Planeswalker — Svega,
//! starting loyalty 5. Colors G, W, U.
//!
//! Landfall — Whenever a land enters under your control, put a loyalty counter
//!   on target planeswalker.
//! −2: For each planeswalker type among planeswalkers you control, create a 1/1
//!   Attendee creature token that's all colors.
//! −X: Choose an Elspeth, Teferi, Liliana, Chandra, or Garruk planeswalker card
//!   name with mana value X. Create a copy of the card with the chosen name. You
//!   may cast the copy without paying its mana cost.
//! Svega can be your commander.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.
//! * CR 113.3c — entering with loyalty counters.
//!
//! # Scope / GAPs
//! * The Landfall ability is a TRIGGERED STATIC (a landfall trigger), NOT a
//!   loyalty ability — it carries no loyalty cost — so it is not wired here. GAP.
//! * −2: "for each planeswalker type among planeswalkers you control" requires
//!   counting DISTINCT planeswalker subtypes among your PWs, which isn't cleanly
//!   expressible. The shell is declared (correct -2 cost); the effect is GAP'd.
//! * −X: "choose a named planeswalker card, create a copy, cast it free" is a
//!   create-a-named-card-from-nowhere effect not in the demonstrated surface.
//!   The shell is declared with the dynamic-X loyalty cost; the effect is GAP'd.
//! * "Svega can be your commander" is commander legality / flavor — ignored.

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
    let name = reg.interner_mut().intern("Svega, the Unconventional");
    let svega = reg.interner_mut().intern("Svega");
    let _attendee = reg.interner_mut().intern("Attendee");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(svega);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: For each planeswalker type among planeswalkers you \
                       control, create a 1/1 Attendee creature token that's all \
                       colors.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-X: Choose an Elspeth, Teferi, Liliana, Chandra, or \
                       Garruk planeswalker card name with mana value X. Create a \
                       copy of the card with the chosen name. You may cast the \
                       copy without paying its mana cost.".into(),
                cost: ActivationCost {
                    remove_loyalty_x: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_x,
            }),
    )
}

/// `-2: For each planeswalker type among planeswalkers you control, create a
/// 1/1 Attendee token that's all colors.`
fn minus_two(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "for each planeswalker type among planeswalkers you control" requires
    //      counting DISTINCT planeswalker subtypes among your controlled PWs,
    //      which isn't expressible with the demonstrated counting surface (which
    //      counts matching objects, not distinct subtypes). The per-type count is
    //      the whole point of the ability, so no degenerate token is created.
    Vec::new()
}

/// `-X: ...create a copy of a named planeswalker card and cast it free.`
fn minus_x(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose a named planeswalker card, create a copy of it, cast the copy
    //      without paying its mana cost" — creating a copy of a card chosen by
    //      name (from nowhere) and casting it free is not expressible with the
    //      demonstrated Effect surface.
    Vec::new()
}
