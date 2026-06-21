//! Nimble Mongoose — `{G}` 1/1 Creature — Mongoose.
//! Shroud.
//! Threshold — This creature gets +2/+2 as long as there are seven or
//!   more cards in your graveyard.
//!
//! GAP: Threshold is not an expressible KeywordAbility, and the "+2/+2
//! as long as …" line is a STATIC continuous self-pump (no trigger
//! word, no cost) — it cannot be expressed in this card class.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nimble Mongoose");
    let mongoose = reg.interner_mut().intern("Mongoose");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mongoose);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Shroud],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
