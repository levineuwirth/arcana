//! Thunder-Thrash Elder — `{2}{R}` 1/1 Creature — Lizard Warrior with Devour 3.
//! Shards of Alara uncommon; enters with three times as many +1/+1
//! counters as creatures sacrificed to it as it entered.
//!
//! # Rules references
//!
//! * CR 702.82 — Devour N. As this creature enters, you may sacrifice
//!   any number of creatures. It enters with N times that many +1/+1
//!   counters on it. Engine handles the ETB sacrifice and counter
//!   placement.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thunder-Thrash Elder");
    let lizard = reg.interner_mut().intern("Lizard");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Devour(3)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
