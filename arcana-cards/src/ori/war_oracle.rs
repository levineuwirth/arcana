//! War Oracle — `{2}{W}{W}` 3/3 Human Cleric.
//! Lifelink; Renown 1.
//!
//! Renown 1 is parametrized and fully implemented as a keyword; the engine
//! synthesizes the combat-damage trigger that puts a +1/+1 counter on it and
//! makes it renowned the first time it connects.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("War Oracle");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Lifelink, KeywordAbility::Renown(1)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
