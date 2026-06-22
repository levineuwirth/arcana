//! Seven Dwarves — `{1}{R}` 2/2 Dwarf.
//!
//! * "This creature gets +1/+1 for each other creature named Seven
//!   Dwarves you control." — a static characteristic-defining P/T
//!   modifier (a continuous self-buff), not a triggered or activated
//!   ability and not expressible with the demonstrated primitives.
//!   GAP'd.
//! * "A deck can have up to seven cards named Seven Dwarves." — a
//!   deckbuilding rule, not a game ability. GAP'd.
//!
//! No expressible abilities remain; only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seven Dwarves");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
