//! Raiding Nightstalker — `{2}{B}` 2/2 Nightstalker with Swampwalk.
//! Portal Second Age common; a black creature that becomes unblockable
//! against Swamp-heavy black decks — flavourful role-reversal.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk (Swamp subtype). This creature can't be
//!   blocked as long as defending player controls a Swamp.
//!   Represented as `KeywordAbility::Landwalk` with the interned
//!   subtype `"Swamp"`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Raiding Nightstalker");
    let nightstalker = reg.interner_mut().intern("Nightstalker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightstalker);

    let swamp = reg.interner_mut().intern("Swamp");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Landwalk(swamp)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
