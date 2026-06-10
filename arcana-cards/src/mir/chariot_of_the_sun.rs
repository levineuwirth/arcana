//! Chariot of the Sun — {3} artifact (Homelands, 1995).
//! "{2}, {T}: Until end of turn, target creature you control gains
//! flying and has base toughness 1." Grants flying and sets base
//! toughness to 1 (power is kept at the creature's current power —
//! a "set toughness only" primitive does not exist).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chariot of the Sun");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{2}, {T}: Until end of turn, target creature you \
                       control gains flying and has base toughness 1."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: fly_with_toughness_one,
            },
        ),
    )
}

fn fly_with_toughness_one(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "has base toughness 1" sets toughness only; SetBasePT sets
    // both, so the power half is filled with the creature's CURRENT
    // power (close, but pre-existing power modifiers are baked in).
    let power = script::power_of(state, *id);
    vec![
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: *id,
            power,
            toughness: 1,
            duration: Duration::EndOfTurn,
        },
    ]
}
