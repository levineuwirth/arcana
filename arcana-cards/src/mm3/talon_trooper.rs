//! Talon Trooper — `{1}{W}{U}` 2/3 Bird Scout with Flying.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying
//!   or Reach.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Talon Trooper");
    let bird = reg.interner_mut().intern("Bird");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
