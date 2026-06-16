//! Ajani, the Greathearted — `{2}{G}{W}` Legendary Planeswalker — Ajani,
//! starting loyalty 5.
//!
//! Static "Creatures you control have vigilance" is a continuous static
//! ability, not a loyalty ability; not modeled here.
//!
//! * `+1`: You gain 3 life. `Effect::GainLife`.
//! * `−2`: Put a +1/+1 counter on each creature you control and a loyalty
//!   counter on each other planeswalker you control. Expressed via
//!   resolution-time `script::ids_matching` + per-id `AddCounters`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ajani, the Greathearted");
    let ajani = reg.interner_mut().intern("Ajani");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ajani);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You gain 3 life.".into(),
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
                effect: plus_one_gain,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Put a +1/+1 counter on each creature you control and \
                       a loyalty counter on each other planeswalker you \
                       control.".into(),
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
                effect: minus_two_counters,
            }),
    )
}

fn plus_one_gain(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife { player: ctx.controller, amount: 3 }]
}

fn minus_two_counters(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut out = Vec::new();

    let creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    for id in creatures {
        out.push(Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }

    let walkers = script::ids_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::PLANESWALKER.into())
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    for id in walkers {
        if id == ctx.source {
            continue; // "each OTHER planeswalker you control"
        }
        out.push(Effect::AddCounters {
            target: id,
            kind: CounterKind::Loyalty,
            count: 1,
        });
    }

    out
}
