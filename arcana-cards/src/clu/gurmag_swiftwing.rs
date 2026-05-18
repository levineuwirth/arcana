//! Gurmag Swiftwing — `{1}{B}` 1/2 Bat with Flying, First strike, and Haste.
//! A black Bat combining three aggressive keywords for a cost-efficient threat.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying or Reach.
//! * CR 702.7 — First Strike. Deals combat damage before creatures without
//!   first strike.
//! * CR 702.10 — Haste. Can attack the turn it enters the battlefield.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gurmag Swiftwing");
    let bat = reg.interner_mut().intern("Bat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::FirstStrike,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
