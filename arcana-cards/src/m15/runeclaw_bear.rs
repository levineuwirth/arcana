//! Runeclaw Bear — vanilla 2/2 green creature for `{1}{G}`. No abilities;
//! pure stats. A functional reprint of Grizzly Bears used across the seed set
//! for combat and summoning-sickness tests.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Runeclaw Bear");
    let subtype = reg.interner_mut().intern("Bear");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
