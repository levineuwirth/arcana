//! Splatter Thug — `{2}{R}` 2/2 Creature — Human Warrior.
//!
//! * First strike.
//! * Unleash (may enter with a +1/+1 counter; can't block while it has one).
//!
//! Both are base keywords. Unleash is a fully-wired parametrized keyword:
//! the engine synthesizes the enter-with-counter choice and the can't-block
//! static.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Splatter Thug");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Unleash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
