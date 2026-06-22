//! Hound of Griselbrand — `{2}{R}{R}` 2/2 Elemental Dog.
//! Double strike.
//! Undying (When this creature dies, if it had no +1/+1 counters on it,
//!   return it to the battlefield under its owner's control with a +1/+1
//!   counter on it.)
//!
//! Both are fully-modeled KeywordAbility variants.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hound of Griselbrand");
    let elemental = reg.interner_mut().intern("Elemental");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::DoubleStrike, KeywordAbility::Undying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
