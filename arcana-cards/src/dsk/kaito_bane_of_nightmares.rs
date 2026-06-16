//! Kaito, Bane of Nightmares — `{2}{U}{B}` planeswalker, starting
//! loyalty 5. Legendary Planeswalker — Kaito.
//!
//! # Rules references
//!
//! * CR 113.3c — printed starting loyalty counters on ETB.
//! * CR 606 — loyalty abilities.
//! * CR 606.3 / CR 704.5i — engine-enforced activation + 0-loyalty SBA.
//!
//! # Scope
//!
//! * Ninjutsu / Surveil keywords are NOT in the usable keyword surface
//!   for this card class — `keywords: vec![]`, gap noted here.
//! * The "during your turn, as long as Kaito has loyalty counters he's
//!   a 3/4 Ninja with hexproof" clause is a continuous self-static, not
//!   a loyalty ability — not expressible from the demonstrated surface;
//!   GAP'd (omitted).
//! * `+1` (emblem with "Ninjas you control get +1/+1") — emblem
//!   creation is not demonstrated; shell declared, effect GAP'd.
//! * `0` (Surveil 2, then draw a card per opponent who lost life this
//!   turn) — no Surveil primitive and no per-opponent-life-lost count;
//!   shell declared, effect GAP'd.
//! * `−2` (Tap target creature, put two stun counters on it) — no Tap
//!   effect and no stun-counter kind in the demonstrated surface; shell
//!   declared with the correct cost and target, effect GAP'd.

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
    let name = reg.interner_mut().intern("Kaito, Bane of Nightmares");
    let kaito = reg.interner_mut().intern("Kaito");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kaito);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        // Ninjutsu / Surveil are not in the usable keyword surface — GAP.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You get an emblem with \"Ninjas you control get \
                       +1/+1.\""
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
                effect: plus_one_emblem,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Surveil 2. Then draw a card for each opponent who \
                       lost life this turn."
                    .into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_surveil_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Tap target creature. Put two stun counters on it."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_tap_stun,
            }),
    )
}

/// `+1: You get an emblem with "Ninjas you control get +1/+1."`
fn plus_one_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation is not in the demonstrated Effect surface.
    Vec::new()
}

/// `0: Surveil 2. Then draw a card for each opponent who lost life this
/// turn.`
fn zero_surveil_draw(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Surveil primitive, and no "count opponents who lost life
    // this turn" amount in the demonstrated surface.
    Vec::new()
}

/// `−2: Tap target creature. Put two stun counters on it.`
fn minus_two_tap_stun(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Tap effect and no stun-counter kind in the demonstrated
    // surface (CounterKind only Loyalty is shown).
    Vec::new()
}
