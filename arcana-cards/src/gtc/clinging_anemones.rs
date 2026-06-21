//! Clinging Anemones — `{3}{U}` 1/4 Jellyfish.
//!
//! Oracle:
//! * Defender
//! * Evolve
//!
//! Both are evergreen-listed keyword abilities (the engine wires Evolve's
//! counter trigger from the keyword). Pure keyword card.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Clinging Anemones");
    let jellyfish = reg.interner_mut().intern("Jellyfish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jellyfish);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender, KeywordAbility::Evolve],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
