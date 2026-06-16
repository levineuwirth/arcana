//! Novellamental — `{1}{U}` 2/1 blue Elemental with Flying.
//!
//! Oracle:
//! * Flying.
//! * This creature can block only creatures with flying.
//!
//! The block-restriction static is not expressible with the
//! demonstrated API, so it is GAP'd; Flying is a base characteristic.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Novellamental");
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

    // GAP: static "can block only creatures with flying" — block
    // restriction not expressible with the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
