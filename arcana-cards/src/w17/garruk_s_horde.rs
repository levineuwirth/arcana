//! Garruk's Horde — `{5}{G}{G}` 7/7 green Beast with Trample.
//!
//! Oracle text:
//! * Trample.
//! * Play with the top card of your library revealed.
//! * You may cast creature spells from the top of your library.
//!
//! Implemented: the Trample keyword.
//!
//! GAP: "Play with the top card of your library revealed" is a static
//! information ability with no `Effect`/static representation — omitted.
//! GAP: "You may cast creature spells from the top of your library" is a
//! static casting-permission ability (top-of-library cast access) the
//! engine doesn't expose — omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garruk's Horde");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
