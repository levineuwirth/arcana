//! River Bear — `{3}{G}` 3/3 Bear with Islandwalk.
//! Mirage common; a green creature that threatens blue decks relying
//! on Islands.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk (Island subtype). This creature can't be
//!   blocked as long as defending player controls an Island.
//!   Represented as `KeywordAbility::Landwalk` with the interned
//!   subtype `"Island"`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("River Bear");
    let bear = reg.interner_mut().intern("Bear");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bear);

    let island = reg.interner_mut().intern("Island");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Landwalk(island)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
