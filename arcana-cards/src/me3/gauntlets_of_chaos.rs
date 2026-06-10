//! Gauntlets of Chaos — `{5}` artifact.
//! "{5}, Sacrifice this artifact: Exchange control of target artifact,
//! creature, or land you control and target permanent an opponent controls
//! that shares one of those types with it. If those permanents are exchanged
//! this way, destroy all Auras attached to them." The exchange is modeled as
//! two `ChangeControl` effects; the shares-a-type restriction and the
//! attached-Aura destruction are GAPs.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gauntlets of Chaos");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{5}, Sacrifice this artifact: Exchange control of target artifact, creature, or land you control and target permanent an opponent controls that shares one of those types with it. If those permanents are exchanged this way, destroy all Auras attached to them.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(
                                TypeLine::ARTIFACT | TypeLine::CREATURE | TypeLine::LAND,
                            ))
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
            ],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: exchange_control,
        }),
    )
}

fn exchange_control(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut targets = ctx.targets.targets.iter();
    let Some(TargetChoice::Object(yours)) = targets.next() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(theirs)) = targets.next() else {
        return Vec::new();
    };
    let their_controller = script::target_controller(state, *theirs, ctx.controller);
    // GAP: "that shares one of those types with it" — cross-target type
    // sharing is not expressible in target filters.
    // GAP: "destroy all Auras attached to them" — no attached-Aura
    // enumeration is available.
    vec![
        Effect::ChangeControl { target: *yours, new_controller: their_controller },
        Effect::ChangeControl { target: *theirs, new_controller: ctx.controller },
    ]
}
