//! Yargle and Multani — vanilla 18/6 legendary black-green creature for `{3}{B}{B}{G}`.
//! No abilities; pure stats.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yargle and Multani");
    let subtype_frog = reg.interner_mut().intern("Frog");
    let subtype_spirit = reg.interner_mut().intern("Spirit");
    let subtype_elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_frog);
    subtypes.0.insert(subtype_spirit);
    subtypes.0.insert(subtype_elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(18)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
