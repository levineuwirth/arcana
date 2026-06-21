//! Spinebiter — `{4}{G}{G}` 3/4 Creature — Phyrexian Beast with Infect.
//!
//! Oracle:
//! * Infect.
//! * "You may have this creature assign its combat damage as though it weren't
//!   blocked." — a combat-damage-assignment static; not expressible with the
//!   demonstrated API (no such keyword/effect). GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spinebiter");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Infect],
        ..Default::default()
    };

    // GAP: static "You may have this creature assign its combat damage as though
    // it weren't blocked" — no combat-damage-assignment override.

    reg.register(CardDefinition::new(name, chars))
}
