//! The Lady of the Mountain — vanilla 5/5 legendary red-green creature for `{4}{R}{G}`.
//! Legendary Creature — Giant. No abilities; pure stats.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Lady of the Mountain");
    let subtype_giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_giant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
