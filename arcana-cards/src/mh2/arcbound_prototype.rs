//! Arcbound Prototype — `{1}{W}` 0/0 Artifact Creature — Assembly-Worker
//! with Modular 2.
//!
//! Mirrodin (2003). Printed P/T is 0/0; Modular 2 supplies the entering
//! +1/+1 counters. When it dies those counters may move to another artifact
//! creature.
//!
//! # Rules references
//!
//! * CR 702.43 — Modular. The creature enters with N +1/+1 counters (N=2
//!   here) and the death trigger redistributes them to a target artifact
//!   creature. Engine wiring handles ETB counter placement and the triggered
//!   ability at death.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arcbound Prototype");
    let assembly_worker = reg.interner_mut().intern("Assembly-Worker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(assembly_worker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Modular(2)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
