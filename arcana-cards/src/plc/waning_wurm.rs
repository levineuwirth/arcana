//! Waning Wurm — `{3}{B}` 7/6 Zombie Wurm with Vanishing 2.
//! Planar Chaos uncommon; a large black wurm that enters with two
//! time counters and is sacrificed when the last is removed at the
//! beginning of upkeep.
//!
//! # Rules references
//!
//! * CR 702.63 — Vanishing N. This permanent enters with N time
//!   counters. At the beginning of its controller's upkeep, remove
//!   a time counter from it. When the last time counter is removed,
//!   sacrifice it. N=2 as given in the oracle text.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Waning Wurm");
    let zombie = reg.interner_mut().intern("Zombie");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(wurm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Vanishing(2)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
