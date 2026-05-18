//! Slippery Bogle — `{G/U}` 1/1 green-blue Beast with Hexproof.
//! Hybrid mana cost; colors are both G and U per spec.
//!
//! # Rules references
//!
//! * CR 702.11 — Hexproof. This creature can't be the target of spells
//!   or abilities your opponents control.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slippery Bogle");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
