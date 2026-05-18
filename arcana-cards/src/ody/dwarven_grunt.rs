//! Dwarven Grunt — `{R}` 1/1 Dwarf with Mountainwalk.
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
    let name = reg.interner_mut().intern("Dwarven Grunt");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let mountain = reg.interner_mut().intern("Mountain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Landwalk(mountain)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
