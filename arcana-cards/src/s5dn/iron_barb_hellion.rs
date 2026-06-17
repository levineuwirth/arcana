//! Iron-Barb Hellion — `{5}{R}` 5/4 Hellion Beast with Haste.
//! "Haste. This creature can't block."
//!
//! Haste is a base keyword; "can't block" is a pure static restriction
//! that the MultiAbilityCreature surface cannot express.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Iron-Barb Hellion");
    let hellion = reg.interner_mut().intern("Hellion");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hellion);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: static "This creature can't block" — no expressible primitive (self can't-block static).
    reg.register(CardDefinition::new(name, chars))
}
