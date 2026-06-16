//! Arcbound Hybrid — `{4}` 0/0 colorless Artifact Creature — Beast.
//!
//! * Haste.
//! * Modular 2 (enters with two +1/+1 counters; when it dies, may move its
//!   +1/+1 counters onto target artifact creature). Both are carried by the
//!   `KeywordAbility::Modular(2)` keyword — no separate ability defs.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arcbound Hybrid");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Haste, KeywordAbility::Modular(2)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
