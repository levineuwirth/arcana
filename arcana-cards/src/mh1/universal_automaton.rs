//! Universal Automaton — `{1}` 1/1 Artifact Creature — Shapeshifter with
//! Changeling. Changeling is not expressible with the current demonstrated
//! KeywordAbility API; the verify pipeline should flag this for manual wiring.
//! Type line is Artifact Creature; only ARTIFACT | CREATURE is set. Multi-type
//! line uses bitwise OR of TypeLine flags per the engine convention.
//!
//! # Rules references
//!
//! * CR 702.73 — Changeling. This card is every creature type.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Universal Automaton");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
