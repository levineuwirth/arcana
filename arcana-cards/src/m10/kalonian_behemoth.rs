//! Kalonian Behemoth — `{5}{G}{G}` 9/9 Beast with Shroud.
//! A green creature from Magic 2013; a massive finisher protected
//! from targeted spells and abilities.
//!
//! # Rules references
//!
//! * CR 702.18 — Shroud. This permanent can't be the target of spells or
//!   abilities. Engine wiring prevents it from being selected as a legal target.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kalonian Behemoth");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(9)),
        keywords: vec![KeywordAbility::Shroud],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
