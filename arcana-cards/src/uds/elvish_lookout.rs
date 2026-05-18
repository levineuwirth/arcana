//! Elvish Lookout — `{G}` 1/1 Elf with Shroud.
//!
//! # Rules references
//!
//! * CR 702.18 — Shroud. This creature can't be the target of spells
//!   or abilities.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elvish Lookout");
    let elf = reg.interner_mut().intern("Elf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Shroud],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
