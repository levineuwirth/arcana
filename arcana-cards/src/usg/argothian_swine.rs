//! Argothian Swine — `{3}{G}` 3/3 Boar with Trample.
//! From Urza's Saga; a solid green mid-range creature that can power
//! through blockers with trample.
//!
//! # Rules references
//!
//! * CR 702.19 — Trample. If this creature would deal combat damage to a
//!   blocking creature, and the lethal damage threshold is met, excess
//!   damage may be assigned to the defending player or planeswalker.
//!   Engine wiring lives in the combat damage assignment step.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Argothian Swine");
    let boar = reg.interner_mut().intern("Boar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(boar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
