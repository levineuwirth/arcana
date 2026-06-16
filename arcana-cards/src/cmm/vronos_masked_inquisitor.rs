//! Vronos, Masked Inquisitor — `{3}{U}{U}` Legendary Planeswalker — Vronos,
//! starting loyalty 5 (printed). Mono-blue.
//!
//! Oracle:
//! +1: Up to two other target planeswalkers you control phase out at the
//!     beginning of the next end step.
//! −2: For each opponent, return up to one target nonland permanent that
//!     player controls to its owner's hand.
//! −7: Target artifact you control becomes a 9/9 Construct artifact creature
//!     and gains vigilance, indestructible, and "This creature can't be
//!     blocked."
//!
//! # Scope
//! * +1 — GAP: phasing (CR 702.26) is not in the demonstrated Effect surface.
//! * −2 — GAP: "for each opponent, return up to one … that player controls"
//!   is a per-opponent dynamic targeting clause not expressible here.
//! * −7 — modeled: SetBasePT 9/9, AddType artifact+creature, grant Vigilance,
//!   Indestructible, and can't-be-blocked. (The "Construct" subtype is not
//!   addable via AddType; the body grants are faithful.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vronos, Masked Inquisitor");
    let sub = reg.interner_mut().intern("Vronos");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    let artifact_you_control = TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::permanent()
                .with_types(TypeLine::ARTIFACT.into())
                .controlled_by(ControllerConstraint::You),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to two other target planeswalkers you control \
                       phase out at the beginning of the next end step.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_phase_out,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: For each opponent, return up to one target nonland \
                       permanent that player controls to its owner's hand.".into(),
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
                effect: minus_two_per_opponent_bounce,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Target artifact you control becomes a 9/9 Construct \
                       artifact creature and gains vigilance, indestructible, \
                       and \"This creature can't be blocked.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![artifact_you_control],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_animate,
            }),
    )
}

/// `+1` — GAP: phasing out is not expressible in the demonstrated Effect surface.
fn plus_one_phase_out(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: phase out (CR 702.26) not in Effect catalog.
    Vec::new()
}

/// `−2` — GAP: per-opponent "up to one target … that player controls" is a
/// dynamic multi-clause bounce not expressible here.
fn minus_two_per_opponent_bounce(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: per-opponent dynamic targeting clause.
    Vec::new()
}

/// `−7` — target artifact you control becomes a 9/9 artifact creature with
/// vigilance, indestructible, and can't be blocked.
fn minus_seven_animate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    vec![
        Effect::SetBasePT {
            target: id,
            power: 9,
            toughness: 9,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::AddType {
            target: id,
            types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::CantBeBlocked {
            target: id,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}
