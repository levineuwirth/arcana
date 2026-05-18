//! Gorger Wurm — `{3}{R}{G}` 5/5 Creature — Wurm with Devour 1.
//! Conflux common; enters with one +1/+1 counter for each creature
//! sacrificed to it as it entered.
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
    let name = reg.interner_mut().intern("Gorger Wurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Devour(1)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
