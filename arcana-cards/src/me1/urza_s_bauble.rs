//! Urza's Bauble — `{0}` artifact (Ice Age, 1995).
//! "{T}, Sacrifice this artifact: Look at a card at random in target
//! player's hand. You draw a card at the beginning of the next turn's
//! upkeep."
//!
//! The random hand-peek is not expressible (GAP). The delayed draw is
//! emitted as an immediate draw — `DelayedAction` has no Draw action, so
//! the timing ("beginning of the next turn's upkeep") is a documented GAP.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::mana::ManaCost;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urza's Bauble");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{0}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice this artifact: Look at a card at random in target player's hand. You draw a card at the beginning of the next turn's upkeep.".into(),
            cost: ActivationCost {
                tap: true,
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_player()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: peek_and_delayed_draw,
        }),
    )
}

/// "Look at a card at random in target player's hand. You draw a card at
/// the beginning of the next turn's upkeep."
fn peek_and_delayed_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at a card at random in target player's hand" — no
    // hand-peek Effect exists.
    // GAP: the draw should happen at the beginning of the next turn's
    // upkeep; DelayedAction has no Draw action, so the draw is immediate.
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}
