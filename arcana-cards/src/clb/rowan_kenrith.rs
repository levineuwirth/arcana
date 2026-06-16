//! Rowan Kenrith — `{4}{R}{R}` Legendary Planeswalker — Rowan, starting
//! loyalty 5. Mono-red.
//!
//! Oracle:
//! +2: During target player's next turn, each creature that player controls
//!     attacks if able.
//! −2: Rowan Kenrith deals 3 damage to each tapped creature target player
//!     controls.
//! −8: Target player gets an emblem with "Whenever you activate an ability
//!     that isn't a mana ability, copy it. You may choose new targets for the
//!     copy."
//! Partner with Will Kenrith
//! Rowan Kenrith can be your commander.
//!
//! # Scope
//! * Partner with — GAP: Partner is not in the demonstrated keyword surface.
//! * +2 — GAP: "during target player's next turn, each creature attacks if
//!   able" delayed forced-attack rider is not expressible.
//! * −2 — modeled: 3 damage to each tapped creature the target player controls
//!   (resolved over the current board).
//! * −8 — GAP: emblem creation not in Effect catalog.

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
    let name = reg.interner_mut().intern("Rowan Kenrith");
    let sub = reg.interner_mut().intern("Rowan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

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
                text: "+2: During target player's next turn, each creature \
                       that player controls attacks if able.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Rowan Kenrith deals 3 damage to each tapped creature \
                       target player controls.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_burn,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: Target player gets an emblem with \"Whenever you \
                       activate an ability that isn't a mana ability, copy it. \
                       You may choose new targets for the copy.\"".into(),
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
                effect: minus_eight_gap,
            }),
    )
}

/// `+2` — GAP: delayed "each creature attacks if able next turn".
fn plus_two_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: during target player's next turn, force-attack rider.
    Vec::new()
}

/// `−2:` 3 damage to each tapped creature the target player controls.
fn minus_two_burn(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::Player(*p))
        .tapped_only();
    script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        .map(|id| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(id),
            amount: 3,
        })
        .collect()
}

/// `−8` — GAP: emblem creation not in Effect catalog.
fn minus_eight_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation not in the demonstrated Effect surface.
    Vec::new()
}
