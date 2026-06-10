//! Thran Forge — `{3}` artifact.
//! "{2}: Until end of turn, target nonartifact creature gets +1/+0 and
//! becomes an artifact in addition to its other types."
//! A mana-only activation (no tap) targeting a nonartifact creature;
//! the effect pairs `Pump` (+1/+0) with an additive `AddType`
//! (artifact), both until end of turn.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thran Forge");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}: Until end of turn, target nonartifact creature gets +1/+0 and becomes an artifact in addition to its other types.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().without_types(TypeLine::ARTIFACT.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: animate_artifact,
        }),
    )
}

fn animate_artifact(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::Pump {
            target: *id,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::AddType {
            target: *id,
            types: TypeLine::ARTIFACT.into(),
            duration: Duration::EndOfTurn,
        },
    ]
}
