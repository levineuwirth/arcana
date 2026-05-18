//! Ogre Berserker — `{4}{R}` 4/2 Ogre Berserker with Haste.
//! A straightforward red aggro creature that can attack the turn it
//! enters the battlefield.
//!
//! # Rules references
//!
//! * CR 702.10 — Haste. The creature can attack and activate tap
//!   abilities the turn it enters the battlefield. Engine skips the
//!   "summoning sickness" check for creatures with Haste.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ogre Berserker");
    let ogre = reg.interner_mut().intern("Ogre");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
