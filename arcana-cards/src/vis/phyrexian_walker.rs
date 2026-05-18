//! Phyrexian Walker — vanilla 0/3 colorless artifact creature for `{0}`.
//! No abilities; pure stats.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Walker");
    let subtype_phyrexian = reg.interner_mut().intern("Phyrexian");
    let subtype_construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_phyrexian);
    subtypes.0.insert(subtype_construct);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{0}").expect("valid cost")),
        colors: ColorSet::default(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
