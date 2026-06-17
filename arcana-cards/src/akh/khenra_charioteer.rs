//! Khenra Charioteer — `{1}{R}{G}` 3/3 Jackal Warrior with Trample.
//! Other creatures you control have trample.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Khenra Charioteer");
    let jackal = reg.interner_mut().intern("Jackal");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jackal);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    // GAP: "Other creatures you control have trample" — a static anthem-style keyword grant;
    // no demonstrated primitive expresses a board-wide static keyword on a CardDefinition.
    reg.register(CardDefinition::new(name, chars))
}
