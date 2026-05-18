//! Highborn Ghoul — `{B}{B}` 2/1 Zombie with Intimidate.
//!
//! # Rules references
//!
//! * CR 702.13 — Intimidate. This creature can't be blocked except by
//!   artifact creatures and/or creatures that share a color with it.
//!   Engine wiring lives in the combat blocker filter.
//!
//! Intimidate is a fully-implemented keyword in the engine; listing it
//! in `keywords` is sufficient.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Highborn Ghoul");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Intimidate],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
