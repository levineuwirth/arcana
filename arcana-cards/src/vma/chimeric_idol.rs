//! Chimeric Idol — `{3}` artifact.
//! "{0}: Tap all lands you control. This artifact becomes a 3/3
//! Turtle artifact creature until end of turn." A free activation:
//! board-wide tap of your lands (ForEach over ids) plus self
//! animation (the Turtle subtype gain is a GAP).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chimeric Idol");
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
                text: "{0}: Tap all lands you control. This artifact \
                       becomes a 3/3 Turtle artifact creature until end of \
                       turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{0}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tap_lands_and_animate,
            },
        ),
    )
}

fn tap_lands_and_animate(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let lands = script::ids_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    // GAP: the Turtle subtype gain is not expressible (AddType covers
    // card types only).
    vec![
        Effect::ForEach {
            targets: lands,
            effect: Box::new(Effect::Tap { target: NULL_OBJECT_ID }),
        },
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
        },
    ]
}
