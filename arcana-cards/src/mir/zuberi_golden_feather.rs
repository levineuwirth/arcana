//! Zuberi, Golden Feather — `{4}{W}` 3/3 Legendary Creature — Griffin.
//! Flying.
//! Other Griffin creatures get +1/+1. (Static anthem — GAP'd: a pure
//! continuous P/T-boost static is not expressible via the demonstrated
//! triggered/activated effect surface.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zuberi, Golden Feather");
    let griffin = reg.interner_mut().intern("Griffin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(griffin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "Other Griffin creatures get +1/+1" is a continuous
    // anthem with no triggered/activated decomposition.
    reg.register(CardDefinition::new(name, chars))
}
