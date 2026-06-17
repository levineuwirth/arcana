//! Dimir Infiltrator — `{U}{B}` 1/3 blue-black Spirit.
//!
//! This creature can't be blocked. — GAP: a pure continuous static with
//! no trigger/activated hook to attach `Effect::CantBeBlocked` to in
//! this card class.
//! Transmute {1}{U}{B}. — GAP: Transmute is not an expressible keyword
//! and there is no discard-from-hand alternative-search cost field.
//!
//! Emitted as faithful vanilla bones.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dimir Infiltrator");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![] as Vec<KeywordAbility>,
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
