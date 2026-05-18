//! Debtors' Transport — `{5}{B}` 5/3 Thrull with Afterlife 2.
//! When Debtors' Transport dies, create two 1/1 white and black Spirit
//! creature tokens with flying. (CR 702.151 — Afterlife.)
//!
//! # Rules references
//!
//! * CR 702.151 — Afterlife N. When this permanent dies, create N 1/1
//!   white and black Spirit creature tokens with flying.
//!
//! Afterlife is a fully implemented parametrized keyword; listing
//! `KeywordAbility::Afterlife(2)` in `keywords` is sufficient — the
//! runtime pipeline handles token creation on death.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Debtors' Transport");
    let thrull = reg.interner_mut().intern("Thrull");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thrull);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Afterlife(2)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
