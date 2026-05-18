//! Keepers of the Faith — vanilla 2/3 white creature for `{1}{W}{W}`. A
//! Human Cleric with no abilities, pure stats.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Keepers of the Faith");
    let subtype_human = reg.interner_mut().intern("Human");
    let subtype_cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_human);
    subtypes.0.insert(subtype_cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
