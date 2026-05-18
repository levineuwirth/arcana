//! Soltari Foot Soldier — `{W}` 1/1 Soltari Soldier with Shadow.
//!
//! # Rules references
//!
//! * CR 702.27 — Shadow. This creature can block or be blocked only by
//!   creatures with shadow. Engine wiring lives in the combat blocker
//!   filter.
//!
//! Shadow is a fully-implemented keyword in the engine; listing it in
//! `keywords` is sufficient.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soltari Foot Soldier");
    let soltari = reg.interner_mut().intern("Soltari");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soltari);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Shadow],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
