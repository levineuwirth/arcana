//! Crystacean — `{3}{U}` 1/6 Crab with Flash.
//! Ikoria: Lair of Behemoths common; a defensive blue crab that can be
//! cast at instant speed to surprise opponents.
//!
//! # Rules references
//!
//! * CR 702.8 — Flash. "You may cast this spell any time you could cast
//!   an instant." Engine checks this during the casting window.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crystacean");
    let crab = reg.interner_mut().intern("Crab");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(crab);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
