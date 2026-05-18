//! Talas Merchant — vanilla 1/3 blue creature for `{1}{U}`. A
//! Human Pirate with no abilities, pure stats.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Talas Merchant");
    let subtype_human = reg.interner_mut().intern("Human");
    let subtype_pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_human);
    subtypes.0.insert(subtype_pirate);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
