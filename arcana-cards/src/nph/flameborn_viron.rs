//! Flameborn Viron — vanilla 6/4 red creature for `{4}{R}{R}`. A Phyrexian
//! Insect with no abilities; a large aggressive body at six mana.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flameborn Viron");
    let subtype_phyrexian = reg.interner_mut().intern("Phyrexian");
    let subtype_insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_phyrexian);
    subtypes.0.insert(subtype_insect);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
