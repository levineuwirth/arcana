//! Bayou Dragonfly — `{1}{G}` 1/1 Insect with Flying and Swampwalk.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying
//!   or Reach. Engine wiring lives in the combat blocker filter.
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
    let name = reg.interner_mut().intern("Bayou Dragonfly");
    let insect = reg.interner_mut().intern("Insect");
    let swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Landwalk(swamp)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
