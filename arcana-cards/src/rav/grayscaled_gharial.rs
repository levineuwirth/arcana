//! Grayscaled Gharial — `{U}` 1/1 Crocodile with Islandwalk.
//! Mirage common; a small aquatic reptile that navigates island waterways
//! with ease.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk (Islandwalk). This creature can't be blocked
//!   as long as the defending player controls an Island.
//!
//! Note: Scryfall also lists the generic `Landwalk` umbrella keyword;
//! only the specific `Islandwalk` entry is mapped here per engine conventions.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grayscaled Gharial");
    let crocodile = reg.interner_mut().intern("Crocodile");
    let island = reg.interner_mut().intern("Island");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(crocodile);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Landwalk(island)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
