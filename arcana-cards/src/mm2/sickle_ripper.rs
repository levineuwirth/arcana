//! Sickle Ripper — `{1}{B}` 2/1 Elemental Warrior with Wither.
//! Wither causes damage dealt to creatures to be applied as -1/-1
//! counters instead of regular damage.
//!
//! # Rules references
//!
//! * CR 702.77 — Wither. Damage dealt to creatures by this source is
//!   applied as -1/-1 counters. This keyword is not representable with
//!   the current demonstrated `KeywordAbility` variants; the verify
//!   pipeline will flag this card for manual wiring.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sickle Ripper");
    let elemental = reg.interner_mut().intern("Elemental");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
