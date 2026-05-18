//! Mountain Goat — `{R}` 1/1 Goat with Mountainwalk.
//! Ice Age common; a nimble goat that traverses mountainous terrain
//! with ease.
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
    let name = reg.interner_mut().intern("Mountain Goat");
    let goat = reg.interner_mut().intern("Goat");
    let mountain = reg.interner_mut().intern("Mountain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goat);

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
