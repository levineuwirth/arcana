//! Tel-Jilad Fallen — `{2}{G}{G}` 3/1 Phyrexian Elf Warrior.
//!
//! "Protection from artifacts
//!  Infect"
//!
//! Decomposition: Infect is an expressible keyword. Protection (from
//! artifacts) is NOT in the supported keyword surface, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tel-Jilad Fallen");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: "Protection from artifacts" — Protection is not in the
        // supported keyword surface.
        keywords: vec![KeywordAbility::Infect],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
