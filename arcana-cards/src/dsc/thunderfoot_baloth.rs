//! Thunderfoot Baloth — `{4}{G}{G}` 5/5 Beast with Trample.
//!
//! "Lieutenant — As long as you control your commander, this creature gets +2/+2
//! and other creatures you control get +2/+2 and have trample." is a conditional
//! static anthem keyed on commander control — not expressible, so GAP'd. Trample
//! is emitted as a keyword.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thunderfoot Baloth");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "Lieutenant — As long as you control your commander, ..." — conditional static anthem.
    reg.register(CardDefinition::new(name, chars))
}
