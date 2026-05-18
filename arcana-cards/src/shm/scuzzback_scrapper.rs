//! Scuzzback Scrapper — `{R/G}` 1/1 Goblin Warrior with Wither.
//! Shadowmoor common; a red-green hybrid creature that deals damage
//! to creatures as -1/-1 counters.
//!
//! # Rules references
//!
//! * CR 702.77 — Wither. This creature deals damage to creatures in
//!   the form of -1/-1 counters. Engine wiring substitutes the
//!   standard damage application for counter placement.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scuzzback Scrapper");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R/G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Wither],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
