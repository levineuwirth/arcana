//! Wasp Lancer — `{U/B}{U/B}{U/B}` 3/2 Faerie Soldier with Flying.
//! A hybrid blue-black creature from Shadowmoor; all three mana symbols
//! are hybrid, making the card both blue and black.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying or Reach.
//!   Engine wiring lives in [`arcana_core::combat`]'s blocker filter.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wasp Lancer");
    let faerie = reg.interner_mut().intern("Faerie");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U/B}{U/B}{U/B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
