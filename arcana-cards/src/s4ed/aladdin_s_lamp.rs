//! Aladdin's Lamp — `{10}` artifact.
//! "{X}, {T}: The next time you would draw a card this turn, instead
//! look at the top X cards of your library, put all but one of them on
//! the bottom of your library in a random order, then draw a card. X
//! can't be 0."
//!
//! Best-effort: the dig payload is `Effect::DigTopN` (look at top X,
//! one to hand, rest to the bottom at random), executed at resolution.
//! GAP: the printed ability is a REPLACEMENT of the next draw this
//! turn; that timing shell is not expressible, so the dig happens
//! immediately instead.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aladdin's Lamp");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{10}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{X}, {T}: The next time you would draw a card this \
                       turn, instead look at the top X cards of your \
                       library, put all but one of them on the bottom of \
                       your library in a random order, then draw a card. \
                       X can't be 0."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: dig_x,
            },
        ),
    )
}

/// "…look at the top X cards of your library, put all but one of them
/// on the bottom of your library in a random order, then draw a card."
fn dig_x(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    if x == 0 {
        // "X can't be 0."
        return Vec::new();
    }
    // GAP: printed as a replacement of the next draw this turn; the
    // replacement shell is not expressible, so the dig resolves
    // immediately (the kept card goes to hand, standing in for the
    // replaced draw).
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: x,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}
