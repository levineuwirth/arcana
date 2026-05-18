//! Zodiac Dog — `{2}{R}` 2/2 Dog with Mountainwalk.
//! Portal Three Kingdoms common; a red-aligned hound born under the zodiac
//! that runs freely through mountainous terrain.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk (Mountainwalk). This creature can't be blocked
//!   as long as the defending player controls a Mountain.
//!
//! Note: Scryfall also lists the generic `Landwalk` umbrella keyword;
//! only the specific `Mountainwalk` entry is mapped here per engine conventions.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zodiac Dog");
    let dog = reg.interner_mut().intern("Dog");
    let mountain = reg.interner_mut().intern("Mountain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Landwalk(mountain)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
