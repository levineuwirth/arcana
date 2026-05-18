//! Wardscale Crocodile — `{4}{G}` 5/3 Crocodile with Hexproof.
//!
//! # Rules references
//!
//! * CR 702.11 — Hexproof. This creature can't be the target of spells
//!   or abilities opponents control. Engine enforces this in the
//!   targeting legality check.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wardscale Crocodile");
    let crocodile = reg.interner_mut().intern("Crocodile");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(crocodile);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
