//! Sliver Construct — vanilla 2/2 colorless artifact creature for `{3}`.
//! No abilities; pure stats. Creature types: Sliver Construct.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sliver Construct");
    let subtype_sliver = reg.interner_mut().intern("Sliver");
    let subtype_construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_sliver);
    subtypes.0.insert(subtype_construct);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::default(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
