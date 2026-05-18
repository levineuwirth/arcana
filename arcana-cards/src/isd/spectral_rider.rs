//! Spectral Rider — `{W}{W}` 2/2 Spirit Knight with Intimidate.
//! Innistrad common; an aggressive white creature that uses
//! Intimidate to slip past many blockers.
//!
//! # Rules references
//!
//! * CR 702.13 — Intimidate. This creature can't be blocked except
//!   by artifact creatures and/or creatures that share a color with
//!   it. Engine wiring lives in the combat blocker filter.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spectral Rider");
    let spirit = reg.interner_mut().intern("Spirit");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Intimidate],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
