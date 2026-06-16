//! Tezzeret, Betrayer of Flesh — `{2}{U}{U}` Legendary Planeswalker —
//! Tezzeret, starting loyalty 5.
//!
//! Oracle text:
//! * Static: "The first activated ability of an artifact you activate
//!   each turn costs {2} less to activate." (Not a loyalty ability — a
//!   continuous cost-reduction static; GAP'd, no loyalty cost.)
//! * `+1`: Draw two cards. Then discard two cards unless you discard an
//!   artifact card.
//! * `−2`: Target artifact becomes an artifact creature. If it isn't a
//!   Vehicle, it has base power and toughness 4/4.
//! * `−6`: You get an emblem with "Whenever an artifact you control
//!   becomes tapped, draw a card."
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities.
//! * CR 704.5i — 0-loyalty state-based sacrifice.
//!
//! # Scope
//!
//! * `+1` partially modeled: the "draw two cards" half is expressed via
//!   `Effect::DrawCards`. The conditional "discard two unless you
//!   discard an artifact card" rider is not expressible with the
//!   demonstrated discard surface and is omitted.
//! * `−2` is GAP'd: granting a continuous "becomes an artifact creature
//!   with base p/t 4/4 (unless Vehicle)" type/characteristic change is
//!   not expressible with the demonstrated Effect surface; the ability
//!   shell with its target is still emitted.
//! * `−6` is GAP'd: emblem creation is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tezzeret, Betrayer of Flesh");
    let tezzeret = reg.interner_mut().intern("Tezzeret");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tezzeret);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Draw two cards. Then discard two cards unless you \
                       discard an artifact card.".into(),
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
                effect: plus_one_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Target artifact becomes an artifact creature. If it \
                       isn't a Vehicle, it has base power and toughness 4/4.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_animate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: You get an emblem with \"Whenever an artifact you \
                       control becomes tapped, draw a card.\"".into(),
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
                effect: minus_six_emblem,
            }),
    )
}

/// `+1: Draw two cards. Then discard two cards unless you discard an
/// artifact card.`
///
/// Only the "draw two cards" half is expressed; the conditional discard
/// rider is omitted (not expressible with the demonstrated surface).
fn plus_one_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 2,
    }]
}

/// `−2: Target artifact becomes an artifact creature ...`
fn minus_two_animate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: continuous "becomes an artifact creature with base p/t 4/4
    // unless Vehicle" type/characteristic change is not expressible.
    Vec::new()
}

/// `−6: You get an emblem ...`
fn minus_six_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation is not expressible.
    Vec::new()
}
