//! Vaporkin — `{1}{U}` 2/1 Elemental.
//!
//! Flying.
//! This creature can block only creatures with flying.
//!
//! Flying is a base keyword. The "can block only creatures with
//! flying" blocking restriction is a static continuous ability with
//! no expressible primitive — GAP'd below.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vaporkin");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static blocking restriction "can block only creatures with
    // flying" — no Effect/keyword primitive expresses a block-eligibility
    // restriction on the blocker side.
    reg.register(CardDefinition::new(name, chars))
}
