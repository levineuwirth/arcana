//! Goblin Mountaineer — `{R}` 1/1 Goblin Scout with Mountainwalk.
//! Portal Second Age common; a cheap red Goblin with mountainwalk,
//! becoming unblockable against red Mountain-heavy decks.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk (Mountain subtype). This creature can't be
//!   blocked as long as defending player controls a Mountain.
//!   Represented as `KeywordAbility::Landwalk` with the interned
//!   subtype `"Mountain"`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Mountaineer");
    let goblin = reg.interner_mut().intern("Goblin");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(scout);

    let mountain = reg.interner_mut().intern("Mountain");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Landwalk(mountain)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
