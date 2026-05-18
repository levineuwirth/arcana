//! Gloom Pangolin — vanilla 1/5 black creature for `{2}{B}`.
//! No abilities; a Nightmare Pangolin with high toughness for three mana.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gloom Pangolin");
    let subtype_nightmare = reg.interner_mut().intern("Nightmare");
    let subtype_pangolin = reg.interner_mut().intern("Pangolin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_nightmare);
    subtypes.0.insert(subtype_pangolin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
