//! Marsh Threader — `{1}{W}` 2/1 Kor Scout with Swampwalk.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk. Swampwalk: this creature can't be blocked
//!   as long as the defending player controls a Swamp. Engine wiring
//!   maps Swampwalk to `KeywordAbility::Landwalk` keyed on the
//!   interned subtype "Swamp".
//!
//! The generic Scryfall `Landwalk` umbrella keyword is ignored; only
//! the specific `Swampwalk` entry is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marsh Threader");
    let kor = reg.interner_mut().intern("Kor");
    let scout = reg.interner_mut().intern("Scout");
    let swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Landwalk(swamp)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
