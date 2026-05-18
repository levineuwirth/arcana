//! Thraben Purebloods — vanilla 3/5 white creature for `{4}{W}`.
//! Dog with no abilities; a high-toughness five-drop used as a basic white
//! creature and Dog-subtype test-case in the seed set.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thraben Purebloods");
    let subtype = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
