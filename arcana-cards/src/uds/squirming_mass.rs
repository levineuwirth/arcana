//! Squirming Mass — `{1}{B}` 1/1 Horror with Fear.
//!
//! # Rules references
//!
//! * CR 702.35 — Fear. This creature can't be blocked except by
//!   artifact creatures and/or black creatures. Engine wiring lives
//!   in the combat blocker filter.
//!
//! Fear is a fully-implemented keyword in the engine; listing it in
//! `keywords` is sufficient.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Squirming Mass");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Fear],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
