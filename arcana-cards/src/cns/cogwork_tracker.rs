//! Cogwork Tracker — `{4}` 4/4 Artifact Creature — Dog Construct.
//! All three lines are unexpressible with the available surface:
//! - "Reveal this card as you draft it…" — a draft-matters static, not a
//!   game-zone ability.
//! - "This creature attacks each combat if able." — no forced-attack static
//!   primitive.
//! - "…attacks a player you noted for cards named Cogwork Tracker each combat
//!   if able." — draft-noted-player mechanic, not modeled.
//! Bones-only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cogwork Tracker");
    let dog = reg.interner_mut().intern("Dog");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    subtypes.0.insert(construct);

    // GAP: "attacks each combat if able" — no forced-attack static primitive.
    // GAP: draft-reveal / noted-player attack restriction — draft mechanic.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
