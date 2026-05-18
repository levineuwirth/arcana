//! Warship Scout — vanilla 2/1 red creature for `{R}`. A Human Scout with no
//! abilities; a one-mana aggressive body for red aggro strategies.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Warship Scout");
    let subtype_human = reg.interner_mut().intern("Human");
    let subtype_scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_human);
    subtypes.0.insert(subtype_scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
