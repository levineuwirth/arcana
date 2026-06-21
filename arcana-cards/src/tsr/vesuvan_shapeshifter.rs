//! Vesuvan Shapeshifter — `{3}{U}{U}` 0/0 Shapeshifter.
//! As this creature enters or is turned face up, you may choose another
//! creature on the battlefield. If you do, until this creature is turned
//! face down, it becomes a copy of that creature, except it has "At the
//! beginning of your upkeep, you may turn this creature face down." — GAP.
//! Morph {1}{U} — GAP (Morph not in the usable keyword surface).
//!
//! GAP: the choose-and-become-a-copy (with the embedded face-down rider)
//! triggers off both ETB and turned-face-up; the turned-face-up event,
//! the "until turned face down" duration, and the conditional copy are
//! not expressible with the available API.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vesuvan Shapeshifter");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
