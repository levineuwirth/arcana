//! Nothic — `{4}{B}` 4/3 black Horror.
//!
//! Oracle:
//! * Weird Insight — When this creature dies, roll a d20.
//!   1—9: you draw a card and lose 1 life.
//!   10—19: you draw two cards and lose 2 life.
//!   20: you draw seven cards and lose 7 life.
//!
//! "Weird Insight" is an ability word (flavor label), not a keyword. The
//! death trigger requires a d20 roll with banded outcomes; there is no
//! die-roll Effect primitive, so the entire trigger is GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nothic");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: death trigger — "roll a d20" with banded draw/lose-life
    // outcomes. No die-roll Effect primitive.

    reg.register(CardDefinition::new(name, chars))
}
