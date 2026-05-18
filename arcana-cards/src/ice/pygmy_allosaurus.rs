//! Pygmy Allosaurus — `{2}{G}` 2/2 Dinosaur with Swampwalk.
//! Ice Age common; a small dinosaur that can slip through swampy terrain.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk (Swampwalk). This creature can't be blocked
//!   as long as the defending player controls a Swamp.
//!
//! Note: Scryfall also lists the generic `Landwalk` umbrella keyword;
//! only the specific `Swampwalk` entry is mapped here per engine conventions.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pygmy Allosaurus");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Landwalk(swamp)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
