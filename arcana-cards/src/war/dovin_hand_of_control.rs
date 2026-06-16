//! Dovin, Hand of Control — `{2}{W/U}` legendary planeswalker, starting loyalty 3.
//! W/U planeswalker (subtype Dovin).
//!
//! # Rules references
//!
//! * CR 606 — loyalty abilities; CR 606.3 — sorcery-speed, controller-only,
//!   once per turn per planeswalker. CR 704.5i — 0-loyalty sacrifice SBA.
//!
//! # Scope
//!
//! The static cost-increase ("Artifact, instant, and sorcery spells your
//! opponents cast cost {1} more") is a continuous spell-cost modifier, not a
//! loyalty ability, and is GAP'd (no loyalty cost — see below).
//!
//! The sole loyalty ability is `−1`: "Until your next turn, prevent all
//! damage that would be dealt to AND dealt BY target permanent an opponent
//! controls." The demonstrated `Effect::PreventDamage` only shields damage
//! dealt TO a target; the "dealt by" half (preventing the permanent's own
//! damage output) plus the "until your next turn" duration are not faithfully
//! expressible from the demonstrated surface, so the ability is declared with
//! its correct `−1` cost but GAP'd in the body.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
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
    let name = reg.interner_mut().intern("Dovin, Hand of Control");
    let dovin = reg.interner_mut().intern("Dovin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dovin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W/U}").expect("valid cost")),
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
                text: "−1: Until your next turn, prevent all damage that would \
                       be dealt to and dealt by target permanent an opponent \
                       controls.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_prevent,
            }),
    )
}

/// `−1: Until your next turn, prevent all damage dealt to and dealt by
/// target permanent an opponent controls.`
fn minus_one_prevent(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: PreventDamage only shields damage dealt TO a target; the
    // "dealt by" half and the "until your next turn" duration are not
    // expressible from the demonstrated effect surface.
    Vec::new()
}
