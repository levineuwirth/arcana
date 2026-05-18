//! Sokenzan Bruiser — `{4}{R}` 3/3 Ogre Warrior with Mountainwalk.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk. Mountainwalk: this creature can't be blocked
//!   as long as the defending player controls a Mountain. Engine wiring
//!   maps Mountainwalk to `KeywordAbility::Landwalk` keyed on the
//!   interned subtype "Mountain".
//!
//! The generic Scryfall `Landwalk` umbrella keyword is ignored; only
//! the specific `Mountainwalk` entry is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sokenzan Bruiser");
    let ogre = reg.interner_mut().intern("Ogre");
    let warrior = reg.interner_mut().intern("Warrior");
    let mountain = reg.interner_mut().intern("Mountain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Landwalk(mountain)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
