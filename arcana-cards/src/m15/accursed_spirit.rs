//! Accursed Spirit — `{3}{B}` 3/2 Spirit with Intimidate.
//! Magic 2014 common; a cursed spirit that can only be blocked by
//! artifact creatures or creatures sharing its color.
//!
//! # Rules references
//!
//! * CR 702.13 — Intimidate. This creature can't be blocked except by
//!   artifact creatures and/or creatures that share a color with it.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Accursed Spirit");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Intimidate],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
