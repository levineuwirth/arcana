//! Arcbound Wanderer — `{6}` 0/0 Artifact Creature — Golem with Modular—Sunburst.
//! A colorless golem whose Modular and Sunburst keywords are not yet
//! representable in the engine's demonstrated API; the keyword list is
//! left empty as a best-effort stub for the verify pipeline.
//!
//! # Rules references
//!
//! * CR 702.43 — Modular. (Not yet wired; verify pipeline will flag.)
//! * CR 702.44 — Sunburst. (Not yet wired; verify pipeline will flag.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arcbound Wanderer");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
