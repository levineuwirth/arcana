//! Arcbound Worker — `{1}` 0/0 Artifact Creature — Construct with Modular 1.
//! Darksteel common; enters with a +1/+1 counter and transfers its
//! counters to another artifact creature when it dies.
//!
//! # Rules references
//!
//! * CR 702.43 — Modular N. This creature enters with N +1/+1 counters
//!   on it. When it dies, you may put its +1/+1 counters on target
//!   artifact creature. Engine handles both the ETB placement and the
//!   death trigger.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arcbound Worker");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Modular(1)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
