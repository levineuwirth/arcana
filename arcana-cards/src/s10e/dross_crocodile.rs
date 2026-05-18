//! Dross Crocodile — vanilla 5/1 black creature for `{3}{B}`.
//! No abilities; pure stats. Creature types: Zombie Crocodile.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dross Crocodile");
    let subtype_zombie = reg.interner_mut().intern("Zombie");
    let subtype_crocodile = reg.interner_mut().intern("Crocodile");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_zombie);
    subtypes.0.insert(subtype_crocodile);
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
