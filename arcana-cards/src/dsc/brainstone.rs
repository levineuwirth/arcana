//! Brainstone — `{1}` artifact.
//! "{2}, {T}, Sacrifice this artifact: Draw three cards, then put two cards
//! from your hand on top of your library in any order." The draw is wired;
//! the hand-to-library put-back is a GAP (see comment).

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
    let name = reg.interner_mut().intern("Brainstone");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{2}, {T}, Sacrifice this artifact: Draw three cards, \
                       then put two cards from your hand on top of your \
                       library in any order."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: brainstorm,
            },
        ),
    )
}

fn brainstorm(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put two cards from your hand on top of your library in any
    // order" — choosing hand cards to put on top of the library is not
    // expressible (PutOnTopOfLibrary needs a known ObjectId; PickAction
    // has no put-on-top variant). Only the draw is wired.
    vec![Effect::DrawCards { player: ctx.controller, count: 3 }]
}
