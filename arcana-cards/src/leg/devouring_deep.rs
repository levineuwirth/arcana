//! Devouring Deep — `{2}{U}` 1/2 Fish with Islandwalk.
//! The Dark common; a blue Fish that is unblockable against
//! Island-controlling opponents.
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
    let name = reg.interner_mut().intern("Devouring Deep");
    let fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);

    let island = reg.interner_mut().intern("Island");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Landwalk(island)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
