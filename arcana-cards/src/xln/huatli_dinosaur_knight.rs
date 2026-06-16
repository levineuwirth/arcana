//! Huatli, Dinosaur Knight — `{4}{R}{W}` Legendary Planeswalker — Huatli,
//! starting loyalty 4.
//!
//! +2: Put two +1/+1 counters on up to one target Dinosaur you control.
//! −3: Target Dinosaur you control deals damage equal to its power to target
//!     creature you don't control.
//! −7: Dinosaurs you control get +4/+4 until end of turn.
//!
//! GAP: the −3 ability targets TWO objects (a Dinosaur you control and a
//!   creature you don't control) and the source of the damage is the chosen
//!   Dinosaur, not Huatli. The demonstrated activated-ability surface gives
//!   `ctx.targets` for the targets but a single DealDamage uses `ctx.source`
//!   as the source. We approximate by having the Dinosaur deal the damage
//!   (source = the chosen Dinosaur) for the correct amount.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::objects::NULL_OBJECT_ID;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Huatli, Dinosaur Knight");
    let huatli = reg.interner_mut().intern("Huatli");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(huatli);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    let dino_you = ObjectFilter::creature()
        .with_subtype_sym(dinosaur)
        .controlled_by(ControllerConstraint::You);
    let dino_you_minus = ObjectFilter::creature()
        .with_subtype_sym(dinosaur)
        .controlled_by(ControllerConstraint::You);
    let creature_not_you =
        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent);

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Put two +1/+1 counters on up to one target Dinosaur \
                       you control."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(dino_you),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Target Dinosaur you control deals damage equal to its \
                       power to target creature you don't control."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(dino_you_minus),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(creature_not_you),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: Dinosaurs you control get +4/+4 until end of turn."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven,
            }),
    )
}

fn plus_two(
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
        count: 2,
    }]
}

fn minus_three(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let dino = match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    let victim = match ctx.targets.targets.get(1) {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    let power = script::power_of(state, dino).max(0) as u32;
    vec![Effect::DealDamage {
        source: dino,
        target: DamageTarget::Object(victim),
        amount: power,
    }]
}

fn minus_seven(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dinosaur = match reg.interner().lookup("Dinosaur") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let filter = ObjectFilter::creature()
        .with_subtype_sym(dinosaur)
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: 4,
            toughness: 4,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
