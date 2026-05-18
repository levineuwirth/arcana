//! Glissa's Courier — `{1}{G}{G}` 2/3 Phyrexian Horror with Mountainwalk.
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
    let name = reg.interner_mut().intern("Glissa's Courier");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let horror = reg.interner_mut().intern("Horror");
    let mountain = reg.interner_mut().intern("Mountain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Landwalk(mountain)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
