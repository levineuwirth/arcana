//! Conch Horn — `{2}` artifact.
//! "{1}, {T}, Sacrifice this artifact: Draw two cards, then put a card
//! from your hand on top of your library."
//!
//! GAP: "put a card from your hand on top of your library" — no Effect
//! moves a chosen hand card to the top of the library (PutOnTopOfLibrary
//! takes a known object target; ChooseAnyNumberFromZone has no
//! put-on-top action). Only the draw half is modeled.

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
    let name = reg.interner_mut().intern("Conch Horn");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}, {T}, Sacrifice this artifact: Draw two cards, then put a card from your hand on top of your library.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
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
            effect: draw_two_put_back,
        }),
    )
}

fn draw_two_put_back(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "then put a card from your hand on top of your library" — no
    // hand-to-library-top effect exists; only the draw is modeled.
    vec![Effect::DrawCards { player: ctx.controller, count: 2 }]
}
