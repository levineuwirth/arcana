//! Ivory Guardians — `{4}{W}{W}` 3/3 Creature — Giant Cleric.
//! Protection from red.
//! Creatures named Ivory Guardians get +1/+1 as long as an opponent
//! controls a nontoken red permanent.
//!
//! GAP: Protection is not an expressible `KeywordAbility` variant — the
//! keyword line is dropped (`keywords: vec![]`).
//! GAP: the "+1/+1 as long as …" line is a STATIC continuous anthem
//! (no trigger word, no cost); it is not a triggered/activated ability
//! and cannot be expressed in this card class.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ivory Guardians");
    let giant = reg.interner_mut().intern("Giant");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Protection from red is not an expressible KeywordAbility.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
