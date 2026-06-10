//! Bazaar of Baghdad — nonbasic land (Arabian Nights).
//! "{T}: Draw two cards, then discard three cards." Produces no mana —
//! its only ability is a non-mana tap activation.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bazaar of Baghdad");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Draw two cards, then discard three cards.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_two_discard_three,
            },
        ),
    )
}

fn draw_two_discard_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 2 },
        Effect::Discard {
            player: ctx.controller,
            count: 3,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
