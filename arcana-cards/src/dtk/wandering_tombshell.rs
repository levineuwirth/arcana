//! Wandering Tombshell — vanilla 1/6 black creature for `{3}{B}`. A Zombie
//! Turtle with no abilities; an exceptionally tough defensive body.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wandering Tombshell");
    let subtype_zombie = reg.interner_mut().intern("Zombie");
    let subtype_turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_zombie);
    subtypes.0.insert(subtype_turtle);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
