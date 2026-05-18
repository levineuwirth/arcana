//! Zodiac Rat — `{B}` 1/1 Rat with Swampwalk.
//! Portal: Three Kingdoms common; a cheap black rat that can't be
//! blocked as long as the defending player controls a Swamp.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk. This creature can't be blocked as long
//!   as the defending player controls a Swamp.
//!   Engine wiring: `KeywordAbility::Landwalk("Swamp")` is checked
//!   by the combat blocker filter against the defending player's lands.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zodiac Rat");
    let rat = reg.interner_mut().intern("Rat");
    let swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Landwalk(swamp)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
