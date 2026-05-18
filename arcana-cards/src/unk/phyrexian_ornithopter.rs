//! Phyrexian Ornithopter — `{1}` 0/2 Artifact Creature — Thopter with
//! Flying and Toxic 1.
//! New Phyrexia common; a colorless flying artifact creature that
//! places one poison counter on players it damages in combat.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying
//!   or Reach.
//! * CR 702.164 — Toxic 1. Causes combat damage to players to also
//!   result in 1 poison counter being placed on that player.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Ornithopter");
    let thopter = reg.interner_mut().intern("Thopter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thopter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Toxic(1)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
