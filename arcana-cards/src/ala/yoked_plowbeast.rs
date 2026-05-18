//! Yoked Plowbeast — `{5}{W}{W}` 5/5 Beast with Cycling {2}.
//! A large white creature with Cycling {2}, providing card-draw flexibility
//! when the full casting cost is prohibitive.
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
    let name = reg.interner_mut().intern("Yoked Plowbeast");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
