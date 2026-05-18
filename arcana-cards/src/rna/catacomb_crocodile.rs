//! Catacomb Crocodile — vanilla 3/7 black creature for `{4}{B}`.
//! No abilities; pure stats.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Catacomb Crocodile");
    let crocodile = reg.interner_mut().intern("Crocodile");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(crocodile);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
