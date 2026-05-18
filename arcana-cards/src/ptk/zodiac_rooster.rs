//! Zodiac Rooster — `{1}{G}` 2/1 Bird with Plainswalk.
//! Mirage common; part of the Zodiac cycle with landwalk keyed to
//! basic land types. Plainswalk lets it slip past white-mana bases.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk (Plains subtype). This creature can't be
//!   blocked as long as defending player controls a Plains.
//!   Represented as `KeywordAbility::Landwalk` with the interned
//!   subtype `"Plains"`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zodiac Rooster");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    let plains = reg.interner_mut().intern("Plains");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Landwalk(plains)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
