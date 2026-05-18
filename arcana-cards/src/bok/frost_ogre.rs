//! Frost Ogre — vanilla 5/3 red creature for `{3}{R}{R}`. No
//! abilities; pure stats as an Ogre Warrior.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frost Ogre");
    let subtype_ogre = reg.interner_mut().intern("Ogre");
    let subtype_warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_ogre);
    subtypes.0.insert(subtype_warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
