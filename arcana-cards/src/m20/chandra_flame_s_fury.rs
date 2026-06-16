//! Chandra, Flame's Fury — `{4}{R}{R}` Legendary Planeswalker — Chandra.
//! Printed starting loyalty 5 (CR 113.3c). Color red.
//!
//! Loyalty abilities (CR 606):
//! * `+1`: Chandra deals 2 damage to any target.
//! * `−2`: Chandra deals 4 damage to target creature and 2 damage to that
//!   creature's controller.
//! * `−8`: Chandra deals 10 damage to target player and each creature
//!   that player controls.
//!
//! All three are expressible: `DealDamage` to the chosen target(s),
//! reading the creature's controller / the targeted player's creatures
//! from state at resolution.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Flame's Fury");
    let chandra = reg.interner_mut().intern("Chandra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chandra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Chandra deals 2 damage to any target.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_bolt,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Chandra deals 4 damage to target creature and 2 damage \
                       to that creature's controller.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_split,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: Chandra deals 10 damage to target player and each \
                       creature that player controls.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_sweep,
            }),
    )
}

/// `+1`: 2 damage to any target.
fn plus_one_bolt(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 2,
    }]
}

/// `−2`: 4 to target creature, 2 to its controller.
fn minus_two_split(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let mut effects = vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 4,
    }];
    if let Some(obj) = state.objects.get(*id) {
        effects.push(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(obj.controller),
            amount: 2,
        });
    }
    effects
}

/// `−8`: 10 to target player and each creature they control.
fn minus_eight_sweep(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let mut effects = vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Player(*p),
        amount: 10,
    }];
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::Player(*p));
    for id in script::ids_matching(state, &filter, ctx.controller) {
        effects.push(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(id),
            amount: 10,
        });
    }
    effects
}
