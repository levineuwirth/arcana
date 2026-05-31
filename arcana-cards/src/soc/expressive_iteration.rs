//! Expressive Iteration — `{U}{R}` sorcery. "Look at the top three
//! cards of your library. Put one of them into your hand, put one of
//! them on the bottom of your library, and exile one of them. You may
//! play the exiled card this turn."
//!
//! Best-effort: `Effect::DigTopN` covers "look at the top three, put
//! one into your hand, put the rest on the bottom of your library."
//! The exile-one-and-may-play-it-this-turn part has no catalog
//! primitive — see GAP below.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Expressive Iteration");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Look at the top three cards of your library. Put one of them into your hand, put one of them on the bottom of your library, and exile one of them. You may play the exiled card this turn.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Best-effort: take one of the top three into hand, rest to bottom.
    // GAP: cannot express "exile one of them and you may play the exiled
    // card this turn" (impulse-draw from a chosen card among the dug
    // pile); DigTopN puts the unchosen cards on the bottom instead.
    vec![Effect::DigTopN {
        player: entry.controller,
        count: 3,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}
