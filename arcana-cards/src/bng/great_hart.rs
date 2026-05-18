//! Great Hart — vanilla 2/4 white creature for `{3}{W}`. An Elk with no
//! abilities; a solid defensive body at four mana.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Great Hart");
    let subtype_elk = reg.interner_mut().intern("Elk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_elk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
