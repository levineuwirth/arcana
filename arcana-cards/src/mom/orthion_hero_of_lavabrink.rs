//! Orthion, Hero of Lavabrink — `{3}{R}` 3/3 Legendary Human Soldier.
//! {1}{R}, {T}: Create a token that's a copy of another target creature you
//! control. It gains haste. Sacrifice it at the beginning of the next end step.
//! Activate only as a sorcery.
//! {6}{R}{R}{R}, {T}: Create five tokens that are copies of another target
//! creature you control. They gain haste. Sacrifice them at the next end step.
//! Activate only as a sorcery.
//!
//! Both abilities create token copies of a target creature you control via
//! Effect::CopyPermanent (repeated five times for the second). Sorcery-speed
//! is modeled with is_instant_speed: false. The riders on the MINTED copies —
//! "it/they gain haste" and "sacrifice it/them at the next end step" — are
//! GAP'd: the copy's ObjectId is assigned engine-side at resolution, so the
//! demonstrated primitives cannot grant haste to or schedule a delayed
//! sacrifice on the freshly-minted token.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Orthion, Hero of Lavabrink");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}, {T}: Create a token that's a copy of another target creature you control. It gains haste. Sacrifice it at the beginning of the next end step. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: copy_once,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}{R}{R}{R}, {T}: Create five tokens that are copies of another target creature you control. They gain haste. Sacrifice them at the beginning of the next end step. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}{R}{R}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: copy_five,
            }),
    )
}

fn copy_once(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "It gains haste. Sacrifice it at the beginning of the next end step."
    // — riders on the minted token copy (its ObjectId is engine-assigned at
    // resolution) are not expressible.
    vec![Effect::CopyPermanent { target: *id }]
}

fn copy_five(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "They gain haste. Sacrifice them at the beginning of the next end
    // step." — riders on the minted token copies are not expressible.
    vec![
        Effect::CopyPermanent { target: *id },
        Effect::CopyPermanent { target: *id },
        Effect::CopyPermanent { target: *id },
        Effect::CopyPermanent { target: *id },
        Effect::CopyPermanent { target: *id },
    ]
}
