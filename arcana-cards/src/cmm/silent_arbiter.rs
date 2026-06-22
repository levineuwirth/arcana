//! Silent Arbiter — `{4}` 1/5 Artifact Creature — Construct.
//! No more than one creature can attack each combat.
//! No more than one creature can block each combat.
//!
//! Both lines are pure static combat-restriction abilities (no trigger word,
//! no activation cost). There is no demonstrated Effect / static primitive
//! that imposes "no more than one creature can attack/block each combat", so
//! both are GAP'd. The card is emitted as a vanilla 1/5 Construct so the bones
//! land in the catalog.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "No more than one creature can attack each combat." — static
// combat-declaration restriction, no expressible primitive.
// GAP: "No more than one creature can block each combat." — same.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Silent Arbiter");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
