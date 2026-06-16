//! Domri, City Smasher — `{4}{R}{G}` Legendary Planeswalker — Domri,
//! starting loyalty 5 — colors G, R.
//!
//! Oracle text:
//! * `+2`: Creatures you control get +1/+1 and gain haste until end of turn. —
//!   the +1/+1 is a board-wide `Effect::Anthem`. GAP: the board-wide haste
//!   grant is not expressible (`Anthem` grants no keyword and there is no
//!   board-wide grant-keyword effect); only the +1/+1 is emitted.
//! * `−3`: Domri deals 3 damage to any target. — `Effect::DealDamage`.
//! * `−8`: Put three +1/+1 counters on each creature you control. Those
//!   creatures gain trample until end of turn. — `Effect::ForEach` +
//!   `AddCounters`. GAP: the board-wide trample grant is not expressible; only
//!   the counters are emitted.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Domri, City Smasher");
    let domri = reg.interner_mut().intern("Domri");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(domri);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Creatures you control get +1/+1 and gain haste \
                       until end of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_anthem,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Domri, City Smasher deals 3 damage to any target.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: Put three +1/+1 counters on each creature you \
                       control. Those creatures gain trample until end of \
                       turn.".into(),
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
                effect: minus_eight_counters,
            }),
    )
}

/// `+2`: board-wide +1/+1 (haste grant is GAP'd).
fn plus_two_anthem(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: board-wide haste grant not expressible; only the +1/+1 is emitted.
    vec![Effect::Anthem {
        controller: ctx.controller,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
    }]
}

/// `−3`: 3 damage to any target.
fn minus_three_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let target = match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => DamageTarget::Object(*id),
        Some(TargetChoice::Player(p)) => DamageTarget::Player(*p),
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target,
        amount: 3,
    }]
}

/// `−8`: three +1/+1 counters on each creature you control (trample is GAP'd).
fn minus_eight_counters(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: board-wide trample grant not expressible; only counters are emitted.
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 3,
        }),
    }]
}
