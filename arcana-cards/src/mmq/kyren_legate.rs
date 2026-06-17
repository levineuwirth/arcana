//! Kyren Legate — `{1}{R}` 1/1 Goblin with Haste.
//! "If an opponent controls a Plains and you control a Mountain, you may cast
//! this spell without paying its mana cost."
//!
//! Haste is a base keyword. The conditional alternative-cost casting
//! permission is a static cast-modifier with no expressible primitive — GAP.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kyren Legate");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: conditional "may cast without paying its mana cost" — no alternative-cost cast primitive.
    reg.register(CardDefinition::new(name, chars))
}
