//! Darkwatch Elves — `{2}{G}` 2/2 Elf.
//! Protection from black. (GAP — Protection not in usable keyword surface)
//! Cycling {2} ({2}, Discard this card: Draw a card.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Darkwatch Elves");
    let elf = reg.interner_mut().intern("Elf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    // GAP: "Protection from black" — Protection is not part of the usable keyword
    // surface.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
