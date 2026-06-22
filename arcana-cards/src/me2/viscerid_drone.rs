//! Viscerid Drone — `{1}{U}` 1/2 Homarid Drone.
//!
//! {T}, Sacrifice a creature and a Swamp: Destroy target nonartifact
//!   creature. It can't be regenerated.
//! {T}, Sacrifice a creature and a snow Swamp: Destroy target
//!   creature. It can't be regenerated.
//!
//! Each ability is wired as a tap + sacrifice-a-creature activation
//! that destroys its target. Two cost / effect facets are GAP'd:
//!   * The cost also requires sacrificing a (snow) Swamp in ADDITION
//!     to a creature. `ActivationCost::sacrifice_other` carries a
//!     single filter, so the engine can express only one of the two
//!     distinct sacrifices; the Swamp / snow-Swamp half is GAP'd.
//!   * "It can't be regenerated" — no demonstrated primitive attaches
//!     a no-regeneration rider to `DestroyPermanent`. GAP'd (the
//!     destroy itself is faithful).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Viscerid Drone");
    let homarid = reg.interner_mut().intern("Homarid");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(homarid);
    subtypes.0.insert(drone);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice a creature and a Swamp: Destroy \
                       target nonartifact creature. It can't be \
                       regenerated."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    // GAP: also "Sacrifice a Swamp" — only one
                    // sacrifice_other filter is expressible.
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .without_types(TypeLine::ARTIFACT.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_target,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice a creature and a snow Swamp: \
                       Destroy target creature. It can't be \
                       regenerated."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    // GAP: also "Sacrifice a snow Swamp" — only one
                    // sacrifice_other filter is expressible.
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_target,
            }),
    )
}

fn destroy_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "It can't be regenerated" — no no-regen rider on Destroy.
    vec![Effect::DestroyPermanent { target: *id }]
}
