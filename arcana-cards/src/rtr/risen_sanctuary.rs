//! Risen Sanctuary — `{5}{G}{W}` 8/8 Elemental with Vigilance.
//! Attacking doesn't cause this creature to tap.
//!
//! # Rules references
//!
//! * CR 702.20 — Vigilance. Attacking doesn't cause this creature
//!   to tap; the engine skips the tap in `apply_declared_attackers`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Risen Sanctuary");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    // Risen Sanctuary is green-white; ColorSet has no multi-color constructor
    // in the reference API, so we use the bitwise combination of the two
    // single-color sets.
    let colors = ColorSet::green() | ColorSet::white();

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{W}").expect("valid cost")),
        colors,
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
