//! Jerrard of the Closed Fist — vanilla 6/5 legendary green-red creature for `{3}{R}{G}{G}`.
//! No abilities; pure stats.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jerrard of the Closed Fist");
    let subtype_human = reg.interner_mut().intern("Human");
    let subtype_knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_human);
    subtypes.0.insert(subtype_knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
