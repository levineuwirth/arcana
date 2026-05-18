//! Enraged Revolutionary — `{2}{R}` 2/1 Creature — Human Warrior with Dethrone.
//! A red dethrone creature that grows whenever it attacks the player with the
//! most life (or tied for most).
//!
//! # Rules references
//!
//! * CR 702.105 — Dethrone. Whenever this creature attacks the player with the
//!   most life or tied for most life, put a +1/+1 counter on it. The runtime
//!   dethrone pipeline handles the trigger and counter; listing the keyword in
//!   `keywords` is all that is required here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Enraged Revolutionary");
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
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Dethrone],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
