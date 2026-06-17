//! Khenra Scrapper — `{2}{R}` 2/3 Jackal Warrior with Menace.
//! "You may exert this creature as it attacks. When you do, it gets
//! +2/+0 until end of turn."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Khenra Scrapper");
    let jackal = reg.interner_mut().intern("Jackal");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jackal);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };
    // GAP: "You may exert this creature as it attacks. When you do, it gets
    //      +2/+0 until end of turn." — Exert is not in the supported keyword
    //      surface and there is no exert-as-attack hook / "when you exert"
    //      trigger condition in the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
