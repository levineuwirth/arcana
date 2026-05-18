//! Skeletal Crocodile — vanilla 5/1 black creature for `{3}{B}`. No
//! abilities; pure stats as a Crocodile Skeleton.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skeletal Crocodile");
    let subtype_croc = reg.interner_mut().intern("Crocodile");
    let subtype_skeleton = reg.interner_mut().intern("Skeleton");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_croc);
    subtypes.0.insert(subtype_skeleton);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
