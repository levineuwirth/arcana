//! Ethercaste Knight — `{W}{U}` 1/3 Artifact Creature — Human Knight with Exalted.
//! Common from Alara Reborn (2009); a white-blue artifact creature that rewards
//! attacking alone with the Exalted trigger.
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
    let name = reg.interner_mut().intern("Ethercaste Knight");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Exalted],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
