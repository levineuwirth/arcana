//! Smoldering Butcher — `{3}{B}` 4/2 Elemental Warrior with Wither.
//! Eventide common; a black Wither creature at 4 mana with above-curve
//! power.
//!
//! # Rules references
//!
//! * CR 702.79 — Wither. This creature deals damage to creatures in the
//!   form of -1/-1 counters. Engine wiring handles the counter substitution
//!   in the damage-dealing pipeline.
//!
//! Wither is a fully implemented keyword in the engine; listing it in
//! `keywords` is sufficient.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Smoldering Butcher");
    let elemental = reg.interner_mut().intern("Elemental");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Wither],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
