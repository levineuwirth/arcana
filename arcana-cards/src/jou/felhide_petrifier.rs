//! Felhide Petrifier — `{2}{B}` 2/3 black Minotaur Warrior with Deathtouch.
//! "Other Minotaur creatures you control have deathtouch."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Felhide Petrifier");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // GAP: static "Other Minotaur creatures you control have deathtouch" — a
    // continuous keyword-granting anthem is neither a triggered nor an
    // activated ability and is not expressible in this card class.

    reg.register(CardDefinition::new(name, chars))
}
