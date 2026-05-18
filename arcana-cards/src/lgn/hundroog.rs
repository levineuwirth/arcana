//! Hundroog — `{6}{G}` 4/7 Beast with Cycling {3}.
//! An overcosted green beast redeemed by Cycling {3}, which lets the player
//! discard it to draw a card rather than cast it for full price.
//!
//! # Rules references
//!
//! * CR 702.28 — Cycling. Pay the cycling cost, discard this card: draw a
//!   card. Implemented as `KeywordAbility::Cycling(ManaCost)`. The
//!   type-search variant is not separately modeled; generic Cycling is used.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hundroog");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{3}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
