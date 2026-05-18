//! Colossal Dreadmaw and Storm Crow — `{5}{U}{G}` 7/8 Legendary Creature —
//! Dinosaur Bird with Flying and Trample.
//! A meme fusion of two iconic Magic cards representing a formidable
//! evasive trampler.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying
//!   or Reach.
//! * CR 702.19 — Trample. If attacking creature is blocked, excess damage
//!   is dealt to the defending player or planeswalker.
//!
//! Colors: G, U.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Colossal Dreadmaw and Storm Crow");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
