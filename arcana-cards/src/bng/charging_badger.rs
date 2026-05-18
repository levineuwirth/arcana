//! Charging Badger — `{G}` 1/1 Badger with Trample.
//!
//! # Rules references
//!
//! * CR 702.19 — Trample. If this creature would assign enough damage to
//!   its blockers, the rest is assigned to the defending player or planeswalker.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Charging Badger");
    let badger = reg.interner_mut().intern("Badger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(badger);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
