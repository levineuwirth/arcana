//! Needlethorn Drake — `{G}{U}` 1/1 Drake with Flying and Deathtouch.
//!
//! # Rules references
//!
//! * CR 702.9  — Flying. Can only be blocked by creatures with Flying
//!   or Reach.
//! * CR 702.2  — Deathtouch. Any amount of damage this creature deals
//!   to a creature is enough to destroy it.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Needlethorn Drake");
    let drake = reg.interner_mut().intern("Drake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
