//! Quaketusk Boar — `{3}{R}{R}` 5/5 Elemental Boar with Reach, Trample, and Haste.
//!
//! # Rules references
//!
//! * CR 702.17 — Reach. Can block creatures with flying.
//! * CR 702.19 — Trample. Excess combat damage may be dealt to the player
//!   or planeswalker the creature is attacking.
//! * CR 702.10 — Haste. Can attack and use tap abilities immediately.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quaketusk Boar");
    let elemental = reg.interner_mut().intern("Elemental");
    let boar = reg.interner_mut().intern("Boar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(boar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Reach,
            KeywordAbility::Trample,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
