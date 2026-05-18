//! Pincher Beetles — `{2}{G}` 3/1 Insect with Shroud.
//!
//! # Rules references
//!
//! * CR 702.18 — Shroud. This creature can't be the target of spells
//!   or abilities. Engine wiring checks for Shroud in targeting
//!   legality.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pincher Beetles");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Shroud],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
