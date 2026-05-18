//! Sandbar Merfolk — `{U}` 1/1 Merfolk with Cycling {2}
//! (Urza's Saga, common).
//!
//! # Rules text
//!
//! Cycling {2} ({2}, Discard this card: Draw a card.)
//!
//! # Rules references
//!
//! * CR 702.28 — Cycling. A player may pay the cycling cost and discard the
//!   card to draw a card. Engine records the cost via
//!   `KeywordAbility::Cycling(ManaCost::parse("{2}"))`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sandbar Merfolk");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
