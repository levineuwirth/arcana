//! Old Ghastbark — vanilla 3/6 green/white creature for `{3}{G/W}{G/W}`.
//! Treefolk Warrior with no abilities; a high-toughness hybrid-mana blocker
//! used to stress multicolor cost parsing and color-identity checks.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Old Ghastbark");
    let subtype_treefolk = reg.interner_mut().intern("Treefolk");
    let subtype_warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_treefolk);
    subtypes.0.insert(subtype_warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G/W}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
