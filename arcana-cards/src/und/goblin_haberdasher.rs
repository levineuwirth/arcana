//! Goblin Haberdasher — `{2}{R}` 2/2 Goblin Hatificer with Menace.
//! "Other creatures you control wearing hats in their art have menace."
//!
//! Menace is a base keyword. The static keyword-grant to "creatures
//! wearing hats" is a continuous static (no triggered/activated shape)
//! and the "wearing hats in their art" predicate is uncomputable — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Haberdasher");
    let goblin = reg.interner_mut().intern("Goblin");
    let hatificer = reg.interner_mut().intern("Hatificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(hatificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: "Other creatures you control wearing hats in their art have
    // menace." — static keyword grant with an uncomputable predicate.

    reg.register(CardDefinition::new(name, chars))
}
