//! Indomitable Ancients — vanilla 2/10 white creature for `{2}{W}{W}`. A
//! Treefolk Warrior with no abilities; a massive toughness wall at four mana.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Indomitable Ancients");
    let subtype_treefolk = reg.interner_mut().intern("Treefolk");
    let subtype_warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_treefolk);
    subtypes.0.insert(subtype_warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(10)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
