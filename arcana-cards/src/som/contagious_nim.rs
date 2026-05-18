//! Contagious Nim — `{2}{B}` 2/2 Phyrexian Zombie with Infect.
//! Scars of Mirrodin common; a black infect creature that spreads
//! poison counters via combat damage to players.
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
    let name = reg.interner_mut().intern("Contagious Nim");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Infect],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
