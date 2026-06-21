//! Sulfur Elemental — `{2}{R}` 3/2 Elemental.
//!
//! Oracle:
//! * Flash
//! * Split second (GAP — not a usable KeywordAbility)
//! * White creatures get +1/-1. (static type-wide anthem/debuff — GAP)
//!
//! Only Flash is expressible. Split second and the white-creature
//! continuous debuff are GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sulfur Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        // GAP: Split second — not in the usable KeywordAbility surface.
        ..Default::default()
    };

    // GAP: "White creatures get +1/-1." (static type-wide continuous P/T mod)
    reg.register(CardDefinition::new(name, chars))
}
