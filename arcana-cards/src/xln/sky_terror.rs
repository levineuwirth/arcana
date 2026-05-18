//! Sky Terror — `{R}{W}` 2/2 red-white Dinosaur with Flying and Menace.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying or Reach.
//! * CR 702.110 — Menace. This creature can't be blocked except by two or
//!   more creatures.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sky Terror");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
