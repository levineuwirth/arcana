//! Elspeth Resplendent — `{3}{W}{W}` Legendary Planeswalker — Elspeth,
//! starting loyalty 5.
//!
//! Oracle text:
//! * `+1`: Choose up to one target creature. Put a +1/+1 counter and a
//!   counter from among flying, first strike, lifelink, or vigilance on
//!   it.
//! * `−3`: Look at the top seven cards of your library. You may put a
//!   permanent card with mana value 3 or less from among them onto the
//!   battlefield with a shield counter on it. Put the rest on the bottom
//!   of your library in a random order.
//! * `−7`: Create five 3/3 white Angel creature tokens with flying.
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities.
//! * CR 704.5i — 0-loyalty state-based sacrifice.
//!
//! # Scope
//!
//! * `+1` partially modeled: the "+1/+1 counter" on the (up to one)
//!   target creature is expressed via `Effect::AddCounters`. The
//!   accompanying "counter from among flying / first strike / lifelink /
//!   vigilance" choice has no representation in the demonstrated
//!   `CounterKind` surface and is omitted.
//! * `−3` is GAP'd: look-at-top-N, conditional put-onto-battlefield with
//!   a shield counter, then bottom-the-rest is not expressible with the
//!   demonstrated Effect surface.
//! * `−7` is GAP'd: token creation requires a `TokenDefinition`, whose
//!   construction is not demonstrated in the prompt; the ability shell
//!   with its correct loyalty cost is still emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elspeth Resplendent");
    let elspeth = reg.interner_mut().intern("Elspeth");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elspeth);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Choose up to one target creature. Put a +1/+1 counter \
                       and a counter from among flying, first strike, lifelink, \
                       or vigilance on it.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_counter,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Look at the top seven cards of your library. You may \
                       put a permanent card with mana value 3 or less from among \
                       them onto the battlefield with a shield counter on it. Put \
                       the rest on the bottom of your library in a random order.".into(),
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
                effect: minus_three_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Create five 3/3 white Angel creature tokens with \
                       flying.".into(),
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
                effect: minus_seven_tokens,
            }),
    )
}

/// `+1: Choose up to one target creature. Put a +1/+1 counter ... on it.`
///
/// Only the +1/+1 counter is expressed; the keyword-counter choice has no
/// `CounterKind` representation and is omitted.
fn plus_one_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

/// `−3: Look at the top seven cards of your library ...`
fn minus_three_dig(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: look-at-top-N + conditional put-onto-battlefield with a shield
    // counter + bottom-the-rest is not expressible.
    Vec::new()
}

/// `−7: Create five 3/3 white Angel creature tokens with flying.`
fn minus_seven_tokens(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: token creation requires a TokenDefinition whose construction
    // is not demonstrated in the prompt.
    Vec::new()
}
