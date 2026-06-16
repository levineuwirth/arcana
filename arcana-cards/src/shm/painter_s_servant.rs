//! Painter's Servant — `{2}` 1/3 Artifact Creature — Scarecrow.
//! As this creature enters, choose a color.
//! All cards that aren't on the battlefield, spells, and permanents are
//! the chosen color in addition to their other colors.
//!
//! Both lines are statics/replacement effects with no triggered- or
//! activated-ability shape, so only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Painter's Servant");
    let scarecrow = reg.interner_mut().intern("Scarecrow");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scarecrow);

    // GAP: "As this creature enters, choose a color" is a replacement /
    // as-enters choice with no triggered-ability shape.
    // GAP: "All cards ... are the chosen color in addition to their other
    // colors" is a global continuous color-granting static, not expressible
    // with the demonstrated triggered/activated API.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
