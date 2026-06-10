//! Stuffed Bear — `{2}` artifact.
//! "{2}: This artifact becomes a 4/4 green Bear artifact creature
//! until end of turn." Animation activation: AddType + SetBasePT +
//! SetColor (the Bear subtype gain is a GAP).

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Stuffed Bear");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{2}: This artifact becomes a 4/4 green Bear artifact \
                       creature until end of turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
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
    // GAP: the Bear subtype gain is not expressible (AddType covers
    // card types only).
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 4,
            toughness: 4,
            duration: Duration::EndOfTurn,
        },
        Effect::SetColor {
            target: ctx.source,
            colors: ColorSet::green(),
            duration: Duration::EndOfTurn,
        },
    ]
}
