//! Balduvian Barbarians — vanilla 3/2 red creature for `{1}{R}{R}`.
//! Human Barbarian from the Ice Age block; no abilities, pure stats.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Balduvian Barbarians");
    let subtype_human = reg.interner_mut().intern("Human");
    let subtype_barbarian = reg.interner_mut().intern("Barbarian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_human);
    subtypes.0.insert(subtype_barbarian);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
