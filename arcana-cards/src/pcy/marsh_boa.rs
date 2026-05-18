//! Marsh Boa — `{G}` 1/1 Snake with Swampwalk.
//! Swampwalk is a landwalk variant granting evasion against players who
//! control a Swamp; the keyword is not in the demonstrated API variant
//! list and has been left out of `keywords` — the verify pipeline will
//! route this gap.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk (Swampwalk variant). This creature can't be
//!   blocked as long as the defending player controls a Swamp. Engine
//!   support for landwalk variants is not exposed in the current
//!   `KeywordAbility` enum shown in the reference examples.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marsh Boa");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
