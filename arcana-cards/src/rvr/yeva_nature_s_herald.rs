//! Yeva, Nature's Herald — `{2}{G}{G}` 4/4 Legendary Elf Shaman with Flash.
//!
//! Oracle:
//! * Flash.
//! * You may cast green creature spells as though they had flash. (static
//!   cast-permission — GAP)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yeva, Nature's Herald");
    let elf = reg.interner_mut().intern("Elf");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    // GAP: static "You may cast green creature spells as though they had flash."
    reg.register(CardDefinition::new(name, chars))
}
