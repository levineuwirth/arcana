//! Tyrranax Rex — `{4}{G}{G}{G}` 8/8 green Phyrexian Dinosaur.
//!
//! GAP (static): "This spell can't be countered." — uncounterable static is
//!   not expressible in the usable surface.
//! Trample, Ward {4}, Haste.
//! Toxic 4.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tyrranax Rex");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![
            KeywordAbility::Trample,
            KeywordAbility::Ward(ManaCost::parse("{4}").expect("valid cost")),
            KeywordAbility::Haste,
            KeywordAbility::Toxic(4),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
