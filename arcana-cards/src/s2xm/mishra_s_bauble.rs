//! Mishra's Bauble — `{0}` artifact.
//! "{T}, Sacrifice this artifact: Look at the top card of target player's
//! library. Draw a card at the beginning of the next turn's upkeep."
//!
//! The cost (tap + sacrifice self) and the player target are wired.
//! Documented GAPs: there is no peek-at-library-top effect, and no
//! delayed-draw primitive (`DelayedAction` only carries object-directed
//! actions) — the draw is emitted immediately as a timing fidelity gap.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mishra's Bauble");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{0}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}, Sacrifice this artifact: Look at the top card of target player's library. Draw a card at the beginning of the next turn's upkeep.".into(),
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
                effect: peek_and_draw_later,
            },
        ),
    )
}

fn peek_and_draw_later(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Look at the top card of target player's library" — no
    // peek/reveal-top effect exists in the catalog.
    // GAP (fidelity): "Draw a card at the beginning of the next turn's
    // upkeep" — no delayed-draw primitive (DelayedAction actions are
    // object-directed only); the draw happens immediately instead.
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}
