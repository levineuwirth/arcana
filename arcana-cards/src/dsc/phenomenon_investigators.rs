//! Phenomenon Investigators — `{2}{U}{B}` 3/4 Human Detective.
//!
//! Oracle:
//! * As this creature enters, choose Believe or Doubt.
//! * Believe — Whenever a nontoken creature you control dies, create a 2/2
//!   black Horror enchantment creature token.
//! * Doubt — At the beginning of your end step, you may return a nonland
//!   permanent you own to your hand. If you do, draw a card.
//!
//! The whole text hangs off an as-enters mode choice ("choose Believe or
//! Doubt") that selects which of two subsequent abilities the creature has.
//! There is no way to gate a creature's printed triggered abilities on an
//! enters-time modal choice with the demonstrated API, so both modal branches
//! are GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phenomenon Investigators");
    let human = reg.interner_mut().intern("Human");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "As this creature enters, choose Believe or Doubt" — an enters-time
    // modal choice that selects which subsequent triggered ability the creature
    // has. Not expressible: the per-mode abilities (Believe's death-trigger
    // token, Doubt's end-step bounce-and-draw) cannot be gated on the choice.
    reg.register(CardDefinition::new(name, chars))
}
