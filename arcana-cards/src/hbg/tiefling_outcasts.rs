//! Tiefling Outcasts — `{R}` 1/1 Tiefling Peasant.
//! Double team (when this attacks, conjure a duplicate into your hand …).
//! "Other Demons, Devils, Imps, and Tieflings you control get +1/+0."
//!
//! Neither ability is expressible for this class:
//! - "Double team" is not a usable KeywordAbility, and its conjure rider
//!   relies on Conjure (an Arena-only mechanic with no Effect variant).
//! - "Other …you control get +1/+0" is a static continuous anthem.
//! Only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tiefling Outcasts");
    let tiefling = reg.interner_mut().intern("Tiefling");
    let peasant = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tiefling);
    subtypes.0.insert(peasant);

    // GAP: "Double team" keyword + its conjure-on-attack rider — not a
    // usable KeywordAbility, and Conjure is not modeled.
    // GAP: "Other Demons, Devils, Imps, and Tieflings you control get
    // +1/+0" — a static continuous anthem.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
