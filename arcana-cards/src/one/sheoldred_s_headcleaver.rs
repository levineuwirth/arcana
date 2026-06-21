//! Sheoldred's Headcleaver — `{3}{B}` 2/4 Phyrexian Warrior with Menace
//! and Toxic 2.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sheoldred's Headcleaver");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace, KeywordAbility::Toxic(2)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
