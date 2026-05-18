//! Sandbar Serpent — `{4}{U}` 3/4 Serpent with Cycling {2}.
//! A blue sea creature with Cycling {2}, allowing the player to trade it
//! for a fresh card draw when a 3/4 for five mana is undesirable.
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
    let name = reg.interner_mut().intern("Sandbar Serpent");
    let serpent = reg.interner_mut().intern("Serpent");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
