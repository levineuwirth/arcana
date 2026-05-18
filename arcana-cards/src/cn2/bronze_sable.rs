//! Bronze Sable — vanilla 2/1 colorless artifact creature for `{2}`.
//! A Sable with no abilities — generic colorless beater.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bronze Sable");
    let subtype = reg.interner_mut().intern("Sable");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
