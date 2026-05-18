//! Quakestrider Ceratops — vanilla 12/8 green creature for `{3}{G}{G}{G}`.
//! No abilities; pure stats. Creature type: Dinosaur.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quakestrider Ceratops");
    let subtype = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(12)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
