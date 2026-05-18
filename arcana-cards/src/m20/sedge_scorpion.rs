//! Sedge Scorpion — `{G}` 1/1 Scorpion with Deathtouch.
//! Any amount of damage this creature deals to another creature is
//! sufficient to destroy it.
//!
//! # Rules references
//!
//! * CR 702.2 — Deathtouch. Any amount of damage this source deals
//!   to a creature is enough to destroy it. Engine wiring marks
//!   lethal damage for deathtouch sources even when the damage
//!   total is below the creature's toughness.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sedge Scorpion");
    let scorpion = reg.interner_mut().intern("Scorpion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scorpion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
