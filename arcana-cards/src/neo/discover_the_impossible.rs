//! Discover the Impossible — `{2}{U}` instant. "Look at the top five
//! cards of your library. Exile one of them face down and put the rest
//! on the bottom of your library in a random order. You may cast the
//! exiled card without paying its mana cost if it's an instant spell
//! with mana value 2 or less. If you don't, put that card into your
//! hand."
//!
//! Modeled with `Effect::DigTopN`: look at the top five, take one into
//! your hand, and put the rest on the bottom in a random order. The
//! "exile face down and optionally cast it for free if it's an instant
//! with mana value 2 or less" branch is not expressible — there is no
//! primitive to exile a dug card face down and offer a free-cast for a
//! filtered card type. We emit the "into your hand" branch, which is
//! the default outcome when the free-cast is declined.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Discover the Impossible");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Look at the top five cards of your library. Exile one of them face down and put the rest on the bottom of your library in a random order. You may cast the exiled card without paying its mana cost if it's an instant spell with mana value 2 or less. If you don't, put that card into your hand.".into(),
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
    // GAP: cannot exile the chosen card face down and offer a free
    // cast when it's an instant with mana value 2 or less. The dug
    // card is taken to hand (the declined-free-cast branch); the rest
    // go to the bottom of the library in a random order.
    vec![Effect::DigTopN {
        player: entry.controller,
        count: 5,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}
