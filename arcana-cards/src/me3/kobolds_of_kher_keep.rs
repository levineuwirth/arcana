//! Kobolds of Kher Keep — vanilla 0/1 red creature for `{0}`. A Kobold
//! with no abilities; a free 0/1 representing the iconic zero-cost
//! kobold token from Kher Keep.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kobolds of Kher Keep");
    let kobold = reg.interner_mut().intern("Kobold");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kobold);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{0}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
