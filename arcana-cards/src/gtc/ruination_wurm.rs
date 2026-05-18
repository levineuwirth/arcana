//! Ruination Wurm — vanilla 7/6 red-green creature for `{4}{R}{G}`. Creature — Wurm.
//! No abilities; pure stats.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ruination Wurm");
    let subtype_wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_wurm);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
