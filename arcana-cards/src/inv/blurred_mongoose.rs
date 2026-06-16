//! Blurred Mongoose — `{1}{G}` 2/1 Mongoose with Shroud.
//! "This spell can't be countered." (cast-time static — GAP)
//! Shroud is a base keyword.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blurred Mongoose");
    let mongoose = reg.interner_mut().intern("Mongoose");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mongoose);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Shroud],
        ..Default::default()
    };

    // GAP: "This spell can't be countered." — cast-time static, not in the
    // documented Effect/keyword surface.

    reg.register(CardDefinition::new(name, chars))
}
