//! Yargle, Glutton of Urborg — legendary vanilla 9/3 black creature for `{4}{B}`.
//! A Frog Spirit with no abilities — pure stats. Notable for its extreme power-to-toughness ratio.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yargle, Glutton of Urborg");
    let subtype_frog = reg.interner_mut().intern("Frog");
    let subtype_spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_frog);
    subtypes.0.insert(subtype_spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
