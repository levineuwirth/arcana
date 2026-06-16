//! Chandra, the Firebrand — `{3}{R}` Legendary Planeswalker — Chandra,
//! starting loyalty 1. Mono-red.
//!
//! Oracle:
//! +1: Chandra deals 1 damage to any target.
//! −2: When you next cast an instant or sorcery spell this turn, copy that
//!     spell. You may choose new targets for the copy.
//! −6: Chandra deals 6 damage to each of up to six targets.
//!
//! # Scope
//! * +1 — modeled: 1 damage to any target.
//! * −2 — modeled: NextCastThisTurn (InstantOrSorcery, Copy rider).
//! * −6 — modeled: up to six any-targets, 6 damage to each.

use arcana_core::effects::{Effect, NextCastKind, NextCastRider};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, the Firebrand");
    let sub = reg.interner_mut().intern("Chandra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(1),
        ..Default::default()
    };

    let up_to_six = TargetRequirement {
        filter: TargetFilter::AnyTarget,
        count: TargetCount::UpTo(6),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Chandra deals 1 damage to any target.".into(),
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
                effect: plus_one_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: When you next cast an instant or sorcery spell this \
                       turn, copy that spell. You may choose new targets for \
                       the copy.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_copy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: Chandra deals 6 damage to each of up to six \
                       targets.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![up_to_six],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_damage,
            }),
    )
}

fn choice_to_damage_target(c: &TargetChoice) -> Option<DamageTarget> {
    match c {
        TargetChoice::Object(id) => Some(DamageTarget::Object(*id)),
        TargetChoice::Player(p) => Some(DamageTarget::Player(*p)),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => {
            Some(DamageTarget::Object(*id))
        }
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => {
            Some(DamageTarget::Player(*p))
        }
        _ => None,
    }
}

/// `+1:` Chandra deals 1 damage to any target.
fn plus_one_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let Some(dt) = choice_to_damage_target(target) else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 1,
    }]
}

/// `−2:` when you next cast an instant or sorcery this turn, copy it.
fn minus_two_copy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::NextCastThisTurn {
        controller: ctx.controller,
        kind: NextCastKind::InstantOrSorcery,
        rider: NextCastRider::Copy,
    }]
}

/// `−6:` 6 damage to each of up to six targets.
fn minus_six_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    ctx.targets
        .targets
        .iter()
        .filter_map(choice_to_damage_target)
        .map(|dt| Effect::DealDamage {
            source: ctx.source,
            target: dt,
            amount: 6,
        })
        .collect()
}
