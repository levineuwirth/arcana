//! Kasmina, Enigma Sage — `{1}{G}{U}` Legendary Planeswalker — Kasmina,
//! starting loyalty 5.
//!
//! Oracle text:
//! * Each other planeswalker you control has the loyalty abilities of
//!   Kasmina. (Static ability — GAP, see below.)
//! * `+2`: Scry 1.
//! * `−X`: Create a 0/0 green and blue Fractal creature token. Put X +1/+1
//!   counters on it. (Counter-placement rider GAP, see below.)
//! * `−8`: Search your library for an instant or sorcery card that shares a
//!   color with this planeswalker, exile that card, then shuffle. You may
//!   cast that card without paying its mana cost. (GAP, see below.)
//!
//! # Rules references
//!
//! * CR 606 — loyalty abilities; the engine enforces sorcery-speed,
//!   stack-empty, controller-only, once-per-turn-per-PW activation and the
//!   0-loyalty state-based sacrifice (CR 704.5i).
//! * CR 606 dynamic-X — `−X` uses `remove_loyalty_x: true`; the engine fans
//!   out one activation per X in `1..=loyalty`, threading X via `ctx.x_value`.
//!
//! # Scope
//!
//! * `+2` Scry 1 is modeled fully (the Scryfall "Scry" keyword is this
//!   ability, not a characteristic keyword — `keywords: vec![]`).
//! * `−X` creates the 0/0 green-and-blue Fractal token (subtype "Fractal" IS
//!   interned and inserted — tokens carry full subtypes). The "Put X +1/+1
//!   counters on it" rider is GAP'd: the freshly created token's ObjectId is
//!   assigned engine-side and is not knowable within the same `Vec<Effect>`,
//!   so AddCounters cannot reference it. The dynamic-X loyalty cost is paid.
//! * `−8` (tutor an instant/sorcery sharing a color and cast it free) is
//!   GAP'd: filtered-library-search-and-cast-without-paying is not
//!   expressible from the demonstrated `Effect` surface. The `−8` shell is
//!   declared with the correct loyalty cost; the effect returns an empty vec.
//! * The static "Each other planeswalker you control has the loyalty
//!   abilities of Kasmina" is not a loyalty ability and is not expressible —
//!   GAP'd (not modeled).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kasmina, Enigma Sage");
    let kasmina = reg.interner_mut().intern("Kasmina");
    // "Fractal" interned here so the −X token effect can look it up.
    let _fractal = reg.interner_mut().intern("Fractal");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kasmina);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Scry 1.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_scry,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−X: Create a 0/0 green and blue Fractal creature token. \
                       Put X +1/+1 counters on it."
                    .into(),
                cost: ActivationCost {
                    remove_loyalty_x: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_x_fractal,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: Search your library for an instant or sorcery card \
                       that shares a color with this planeswalker, exile that \
                       card, then shuffle. You may cast that card without \
                       paying its mana cost."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_tutor_cast,
            }),
    )
}

/// `+2`: Scry 1.
fn plus_two_scry(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Scry {
        player: ctx.controller,
        count: 1,
    }]
}

/// `−X`: create a 0/0 green and blue Fractal creature token. ("Put X +1/+1
/// counters on it" is GAP'd — the new token's id isn't knowable in-vec.)
fn minus_x_fractal(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let fractal = reg.interner().lookup("Fractal").expect("Fractal interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(fractal);
    let token = TokenDefinition {
        name: fractal,
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: Vec::new(),
        abilities: Vec::new(),
    };
    // GAP: "Put X +1/+1 counters on it" — the freshly created token's ObjectId
    // is engine-assigned and unknowable from this same Vec<Effect>, so an
    // AddCounters referencing it cannot be emitted here. The token is created
    // faithfully; the X-counter rider is the gap.
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token,
    }]
}

/// `−8`: tutor an instant/sorcery sharing a color with this planeswalker and
/// cast it without paying its mana cost.
fn minus_eight_tutor_cast(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: filtered-library-search-then-cast-without-paying is not expressible
    // from the demonstrated Effect surface. The −8 loyalty cost is still paid.
    Vec::new()
}
