//! Arcbound Bruiser — `{5}` 0/0 Artifact Creature — Golem with Modular 3.
//! Has Modular 3 (not expressible with the current keyword API;
//! the verify pipeline will flag this gap).
//!
//! # Rules references
//!
//! * CR 702.43 — Modular. This creature enters with three +1/+1
//!   counters on it. When it dies, you may put its +1/+1 counters
//!   on target artifact creature.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arcbound Bruiser");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
