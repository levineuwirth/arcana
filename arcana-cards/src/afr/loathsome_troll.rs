//! Loathsome Troll — `{3}{G}{G}` 6/2 Troll.
//! "{3}{G}: Roll a d20. Activate only if this card is in your graveyard.
//!  1—9  | Put this card on top of your library.
//!  10—19| Return this card to your hand.
//!  20   | Return this card to the battlefield tapped."
//!
//! The cost / graveyard activation zone are expressed; the effect body is
//! GAP'd — there is no d20-roll Effect variant in the demonstrated
//! surface, and the three branch outcomes hang off that roll.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Loathsome Troll");
    let troll = reg.interner_mut().intern("Troll");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(troll);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{G}: Roll a d20. Activate only if this card is in your graveyard. 1\u{2014}9: Put this card on top of your library. 10\u{2014}19: Return this card to your hand. 20: Return this card to the battlefield tapped.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: roll_d20,
        }),
    )
}

fn roll_d20(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: d20 roll with three result-range branches — no roll-die
    // Effect variant in the demonstrated surface.
    Vec::new()
}
