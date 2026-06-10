//! Chimeric Staff — `{4}` artifact (Urza's Saga, 1998).
//! "{X}: This artifact becomes an X/X Construct artifact creature until
//! end of turn." X-cost activation; the animation is AddType + SetBasePT
//! with X read from `ctx.x_value`.
//! GAP: gaining the Construct creature subtype is not expressible.

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
    let name = reg.interner_mut().intern("Chimeric Staff");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{X}: This artifact becomes an X/X Construct artifact \
                       creature until end of turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}").expect("valid cost"),
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
    // GAP: gaining the Construct creature subtype is not expressible.
    let x = ctx.x_value.unwrap_or(0) as i32;
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: x,
            toughness: x,
            duration: Duration::EndOfTurn,
        },
    ]
}
