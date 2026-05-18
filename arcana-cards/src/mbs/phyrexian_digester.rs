//! Phyrexian Digester — `{3}` 2/1 Artifact Creature — Phyrexian Construct
//! with Infect.
//! New Phyrexia common; a colorless artifact creature that deals
//! damage as -1/-1 counters to creatures and poison counters to players.
//!
//! # Rules references
//!
//! * CR 702.90 — Infect. This creature deals damage to creatures in
//!   the form of -1/-1 counters and to players in the form of poison
//!   counters. Engine wiring handles both substitution effects.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Digester");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Infect],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
