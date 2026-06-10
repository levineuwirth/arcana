//! Well of Knowledge — `{3}` artifact (Weatherlight).
//! "{2}: Draw a card. Any player may activate this ability but only during
//! their draw step."
//!
//! Wired: the {2}: draw a card activation for the controller. GAP: the
//! "any player may activate" permission and the "only during their draw
//! step" timing window — neither is expressible on `ActivatedAbilityDef`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Well of Knowledge");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}: Draw a card. Any player may activate this ability but only during their draw step.".into(),
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
            effect: draw_one,
        }),
    )
}

fn draw_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Any player may activate this ability but only during their draw
    // step." — any-player activation permission and the draw-step-only
    // timing window are not expressible; modeled as the controller's
    // sorcery-speed activation.
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}
