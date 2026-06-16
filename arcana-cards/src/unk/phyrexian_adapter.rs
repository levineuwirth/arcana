//! Phyrexian Adapter — `{1}{U}` 1/3 Phyrexian Wizard with Flying.
//!
//! "Transform" is a DFC marker Scryfall lists but is not in the usable
//! keyword surface (omitted; the front face is what we register). The
//! static "All Incubator tokens you control become Food, Blood, Clue,
//! Treasure, and Powerstone in addition to their other types …" is a
//! board-wide type/ability-granting static with no demonstrated
//! primitive (GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Adapter");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "All Incubator tokens you control become Food, Blood, Clue,
    // Treasure, and Powerstone … and have the respective abilities" —
    // board-wide type/ability transformation static not expressible.
    reg.register(CardDefinition::new(name, chars))
}
