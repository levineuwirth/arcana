//! Galewind Moose — `{4}{G}{G}` 6/6 Elemental Elk with Flash, Vigilance,
//! Reach, and Trample.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Galewind Moose");
    let elemental = reg.interner_mut().intern("Elemental");
    let elk = reg.interner_mut().intern("Elk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(elk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![
            KeywordAbility::Flash,
            KeywordAbility::Vigilance,
            KeywordAbility::Reach,
            KeywordAbility::Trample,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
