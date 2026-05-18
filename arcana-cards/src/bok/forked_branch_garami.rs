//! Forked-Branch Garami — `{3}{G}{G}` 4/4 Spirit with Soulshift 4.
//! A branching spirit who returns fallen companions; printed "soulshift 4" appears
//! twice on the physical card, but Scryfall records a single keyword entry — emitted once.
//!
//! # Rules references
//! * CR 702.45 — Soulshift

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Forked-Branch Garami");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Soulshift(4)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
