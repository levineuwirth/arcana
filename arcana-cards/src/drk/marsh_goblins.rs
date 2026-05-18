//! Marsh Goblins — `{B}{R}` 1/1 Goblin with Swampwalk.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk. This creature can't be blocked as long as
//!   the defending player controls a land of the named type. Here the
//!   type is Swamp (Swampwalk). Scryfall also lists a generic
//!   "Landwalk" umbrella entry which is ignored per engine conventions;
//!   only the specific Swampwalk entry maps to
//!   `KeywordAbility::Landwalk`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marsh Goblins");
    let goblin = reg.interner_mut().intern("Goblin");
    let swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Landwalk(swamp)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
