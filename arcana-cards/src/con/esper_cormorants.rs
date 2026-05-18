//! Esper Cormorants — `{2}{W}{U}` 3/3 Artifact Creature — Bird with Flying.
//! A white-blue artifact creature from Shards of Alara.
//!
//! # Rules references
//!
//! * CR 702.9 — Flying. Can only be blocked by creatures with Flying or Reach.
//!   Engine wiring lives in [`arcana_core::combat`]'s blocker filter.
//!
//! This card is both an Artifact and a Creature; both type flags are set.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Esper Cormorants");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
