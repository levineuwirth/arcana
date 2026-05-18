//! Will-Forged Golem — `{6}` 4/4 Artifact Creature — Golem with Convoke.
//! Convoke is not expressible with the current demonstrated KeywordAbility API;
//! the verify pipeline should flag this for manual wiring.
//!
//! # Rules references
//!
//! * CR 702.51 — Convoke. Your creatures can help cast this spell. Each
//!   creature you tap while casting this spell pays for {1} or one mana of
//!   that creature's color.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Will-Forged Golem");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
