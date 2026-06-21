//! Thrill-Kill Assassin — `{1}{B}` 1/2 Human Assassin with Deathtouch
//! and Unleash.
//!
//! Oracle:
//! * Deathtouch
//! * Unleash (You may have this creature enter with a +1/+1 counter on it.
//!   It can't block as long as it has a +1/+1 counter on it.)
//!
//! Both are keyword abilities; nothing beyond listing them is required.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thrill-Kill Assassin");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Deathtouch, KeywordAbility::Unleash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
