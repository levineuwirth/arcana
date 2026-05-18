//! Wirewood Guardian — `{5}{G}{G}` 6/6 Elf Mutant.
//! Onslaught common (2002); a large green creature with Forestcycling,
//! allowing it to be discarded to search for a Forest. The cycling
//! keyword family (Forestcycling, Landcycling, Typecycling, Cycling)
//! has no corresponding `KeywordAbility` variant in the demonstrated
//! API; this file registers the base stats only. The verify pipeline
//! should flag the missing cycling ability for manual implementation.
//!
//! # Rules references
//!
//! * CR 702.28 — Cycling / Forestcycling. Not expressible via the
//!   current `KeywordAbility` enum; requires a separate activated
//!   ability implementation.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wirewood Guardian");
    let elf = reg.interner_mut().intern("Elf");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
