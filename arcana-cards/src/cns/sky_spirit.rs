//! Sky Spirit — `{1}{W}{U}` 2/2 Spirit with Flying and First Strike.
//! Invasion uncommon; a white-blue flying first striker that excels
//! in aerial combat.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying
//!   or Reach.
//! * CR 702.7 — First Strike. Deals combat damage before creatures
//!   without first strike.
//!
//! Colors: U, W.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sky Spirit");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
