//! Grizzled Leotau — vanilla 1/5 green/white creature for `{G}{W}`.
//! Cat with no abilities; a high-toughness two-drop used to stress hybrid-color
//! identity and multicolor parsing in the seed set.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grizzled Leotau");
    let subtype = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
