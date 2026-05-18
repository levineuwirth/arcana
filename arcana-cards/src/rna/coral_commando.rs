//! Coral Commando — vanilla 3/2 blue creature for `{2}{U}`. No
//! abilities; pure stats as a Merfolk Warrior.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Coral Commando");
    let subtype_merfolk = reg.interner_mut().intern("Merfolk");
    let subtype_warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_merfolk);
    subtypes.0.insert(subtype_warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
