//! Young Wolf — `{G}` 1/1 Wolf with Undying.
//! Undying is not expressible with the current demonstrated KeywordAbility API;
//! the verify pipeline should flag this for manual wiring.
//!
//! # Rules references
//!
//! * CR 702.93 — Undying. When this creature dies, if it had no +1/+1 counters
//!   on it, return it to the battlefield under its owner's control with a +1/+1
//!   counter on it.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Young Wolf");
    let wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
