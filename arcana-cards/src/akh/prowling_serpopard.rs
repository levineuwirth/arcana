//! Prowling Serpopard — `{1}{G}{G}` 4/3 Creature — Cat Snake.
//!
//! Oracle:
//! * "This spell can't be countered." — a static while-on-the-stack
//!   characteristic with no Effect-catalog representation.
//! * "Creature spells you control can't be countered." — a static continuous
//!   ability affecting other spells; no trigger word, no cost, not
//!   expressible with the documented Effect catalog.
//!
//! Both abilities are pure statics (no keyword, no trigger, no activation
//! cost), so only the bones are emitted. GAP: both can't-be-countered
//! statics.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prowling Serpopard");
    let cat = reg.interner_mut().intern("Cat");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "This spell can't be countered."
    // GAP: static "Creature spells you control can't be countered."
    reg.register(CardDefinition::new(name, chars))
}
