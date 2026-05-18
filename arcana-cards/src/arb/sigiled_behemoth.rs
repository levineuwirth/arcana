//! Sigiled Behemoth — `{4}{G}{W}` 5/4 Creature — Beast with Exalted.
//! Shards of Alara uncommon; a large green-white exalted finisher that
//! rewards committing a single attacker.
//!
//! # Rules references
//!
//! * CR 702.90 — Exalted. Whenever a creature you control attacks alone, that
//!   creature gets +1/+1 until end of turn. The runtime exalted pipeline
//!   handles the trigger and pump; listing the keyword in `keywords` is all
//!   that is required here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sigiled Behemoth");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Exalted],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
