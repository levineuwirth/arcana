//! Coiled Tinviper — `{3}` 2/1 Artifact Creature — Snake with First Strike.
//! A colorless artifact creature from the Arcana card set; a mechanical
//! serpent that strikes before most creatures can retaliate, making it
//! an efficient early threat or removal tool in combat.
//!
//! # Rules references
//!
//! * CR 702.7 — First Strike. In combat, creatures with first strike
//!   deal damage in the first combat damage step, before creatures
//!   without first strike or double strike. Engine wiring lives in
//!   the combat damage pipeline, which checks this keyword before
//!   assigning damage steps.
//!
//! First strike is a base characteristic on this card, so nothing
//! beyond listing it in `keywords` is required — the runtime
//! pipelines do the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Coiled Tinviper");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::default(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
