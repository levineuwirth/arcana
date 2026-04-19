//! Typhoid Rats — `{B}` 1/1 Rat with Deathtouch. M11 common; the
//! canonical one-mana Deathtouch creature. Seed entry point for the
//! "any damage from this source is lethal" wiring.
//!
//! # Rules references
//!
//! * CR 702.2 — Deathtouch. "Any nonzero amount of damage a source
//!   with deathtouch deals to a creature is enough to destroy it."
//!   Engine wiring: `GameState::deal_damage` stamps
//!   `has_deathtouch_damage` on the target when the source has
//!   Deathtouch (CR 702.2b), and SBAs (CR 704.5g) destroy any
//!   creature so marked.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Typhoid Rats");
    let rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
