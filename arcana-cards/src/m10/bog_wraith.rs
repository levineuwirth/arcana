//! Bog Wraith — `{3}{B}` 3/3 Wraith with Swampwalk.
//! Alpha uncommon; a classic swamp-dwelling wraith that passes through
//! swampy terrain unstopped.
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
    let name = reg.interner_mut().intern("Bog Wraith");
    let wraith = reg.interner_mut().intern("Wraith");
    let swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wraith);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Landwalk(swamp)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
