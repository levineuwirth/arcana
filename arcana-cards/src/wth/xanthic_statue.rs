//! Xanthic Statue — {8} artifact. "{5}: Until end of turn, this
//! artifact becomes an 8/8 Golem artifact creature with trample."
//!
//! The animation is AddType(CREATURE) + SetBasePT(8/8) + trample until
//! end of turn; the Golem subtype grant is a documented gap.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Xanthic Statue");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{5}: Until end of turn, this artifact becomes an 8/8 \
                       Golem artifact creature with trample."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: animate,
            },
        ),
    )
}

fn animate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Golem subtype grant not expressible (AddType covers card
    // types only).
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 8,
            toughness: 8,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        },
    ]
}
