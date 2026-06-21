//! Caller of the Untamed — `{3}{G}` 2/4 Elf Shaman.
//!
//! "Before you shuffle your deck to start the game, you may reveal this
//! card from your deck and exile a creature card you drafted that isn't
//! in your deck." — GAP: a draft/pre-game deckbuilding action with no
//! engine representation.
//! "{X}, {T}: Create a token that's a copy of a card you exiled with
//! cards named Caller of the Untamed. X is the mana value of that card."
//! — GAP: the copy source is the card exiled by the (unmodeled) pre-game
//! action; there is no object id to copy, and {X} = its mana value is
//! likewise unrepresentable.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Caller of the Untamed");
    let elf = reg.interner_mut().intern("Elf");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
