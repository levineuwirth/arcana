//! Dreg Reaver — vanilla 4/3 black creature for `{4}{B}`.
//! Zombie Beast with no abilities; a medium-sized black beater used as a
//! multitype subtype registration test-case.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dreg Reaver");
    let subtype_zombie = reg.interner_mut().intern("Zombie");
    let subtype_beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_zombie);
    subtypes.0.insert(subtype_beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
