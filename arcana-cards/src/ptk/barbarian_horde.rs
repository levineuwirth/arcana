//! Barbarian Horde — vanilla 3/3 red creature for `{3}{R}`. No
//! abilities; pure stats. A Human Barbarian Soldier from Odyssey.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Barbarian Horde");
    let subtype_human = reg.interner_mut().intern("Human");
    let subtype_barbarian = reg.interner_mut().intern("Barbarian");
    let subtype_soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_human);
    subtypes.0.insert(subtype_barbarian);
    subtypes.0.insert(subtype_soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
