//! Servant of Nefarox — `{2}{B}` 3/1 Creature — Human Cleric with Exalted.
//! Magic 2013 common; a black exalted creature that rewards attacking alone
//! with a fragile but punchy body.
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
    let name = reg.interner_mut().intern("Servant of Nefarox");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Exalted],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
