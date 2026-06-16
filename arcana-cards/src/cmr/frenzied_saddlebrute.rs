//! Frenzied Saddlebrute — `{4}{R}` 5/4 Orc Warrior with Haste.
//! Static: all creatures can attack your opponents and their planeswalkers
//! as though they had haste (GAP — pure continuous static, not expressible).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frenzied Saddlebrute");
    let orc = reg.interner_mut().intern("Orc");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "All creatures can attack your opponents and planeswalkers your
    // opponents control as though those creatures had haste" — a pure
    // continuous static affecting attack legality; not a triggered/activated
    // ability and not expressible with the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
