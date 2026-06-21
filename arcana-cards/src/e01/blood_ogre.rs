//! Blood Ogre — `{2}{R}` 2/2 Ogre Warrior.
//! Bloodthirst 1, first strike.
//!
//! Both lines are keywords; nothing else to wire.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blood Ogre");
    let ogre = reg.interner_mut().intern("Ogre");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Bloodthirst(1), KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
