//! Bladetusk Boar — `{3}{R}` 3/2 Boar with Intimidate.
//! A red aggressive creature that is difficult to block due to intimidate,
//! forcing opponents to use artifact creatures or red creatures to block it.
//!
//! # Rules references
//!
//! * CR 702.13 — Intimidate. This creature can't be blocked except by
//!   artifact creatures and/or creatures that share a color with it.
//!   NOTE: `KeywordAbility::Intimidate` is not present in the demonstrated
//!   API; the verify pipeline should route this card for manual wiring.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bladetusk Boar");
    let boar = reg.interner_mut().intern("Boar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(boar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
