//! Undead Minotaur — vanilla 2/3 black creature for `{2}{B}`. No
//! abilities; pure stats. A Zombie Minotaur from Magic 2013.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Undead Minotaur");
    let subtype_zombie = reg.interner_mut().intern("Zombie");
    let subtype_minotaur = reg.interner_mut().intern("Minotaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_zombie);
    subtypes.0.insert(subtype_minotaur);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
