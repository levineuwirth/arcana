//! Nyxathid — `{1}{B}{B}` 7/7 Elemental.
//!
//! Oracle:
//! * As this creature enters, choose an opponent.
//! * This creature gets -1/-1 for each card in the chosen player's hand.
//!
//! GAP: both clauses form a single characteristic-defining / static P/T
//! modifier driven by an as-enters player choice. There is no expressible
//! primitive for an as-enters "choose a player" plus a continuous self
//! -1/-1-per-card-in-that-hand effect. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nyxathid");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
