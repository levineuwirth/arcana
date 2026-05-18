//! Hostile Minotaur — `{3}{R}` 3/3 Minotaur with Haste.
//! Core Set 2019 common; a solid red beater that can attack
//! immediately the turn it enters play due to Haste.
//!
//! # Rules references
//!
//! * CR 702.10 — Haste. The creature can attack and use {T} abilities
//!   the turn it comes under the controller's control; the summoning
//!   sickness rule (CR 302.6) is bypassed.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hostile Minotaur");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
