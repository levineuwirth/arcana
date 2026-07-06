//! Phyrexian Portal — `{3}` Artifact.
//! "{3}: If your library has ten or more cards in it, target opponent
//! looks at the top ten cards of your library and separates them into
//! two face-down piles. Exile one of those piles. Search the other pile
//! for a card, put it into your hand, then shuffle the rest of that pile
//! into your library."
//!
//! Approximated with `DigTopN` over the top ten cards: you put one of
//! them into your hand and the rest go to the bottom in a random order.
//! GAPs: the two-face-down-pile split + opponent choosing which pile to
//! exile is not modeled (no pile mechanic); the exiled pile is left in
//! the library (bottomed) instead of exiled; and the "library has ten or
//! more cards" activation gate is not enforced.

use arcana_core::effects::{DigRest, Effect};
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
    let name = reg.interner_mut().intern("Phyrexian Portal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}: If your library has ten or more cards in it, target opponent looks at the top ten cards of your library and separates them into two face-down piles. Exile one of those piles. Search the other pile for a card, put it into your hand, then shuffle the rest of that pile into your library.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_opponent()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: portal_dig,
        }),
    )
}

fn portal_dig(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 10,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}
