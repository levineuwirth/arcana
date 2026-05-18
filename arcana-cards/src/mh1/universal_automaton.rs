//! Universal Automaton — `{1}` 1/1 Artifact Creature — Shapeshifter with Changeling.
//! Modern Horizons common; a colorless artifact creature that is
//! every creature type simultaneously due to the Changeling ability.
//!
//! # Rules references
//!
//! * CR 702.72 — Changeling. This object is every creature type at
//!   all times. The engine handles the "is every creature type"
//!   property from the keyword; the subtype list in the catalog
//!   reflects only the printed type line (Shapeshifter).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Universal Automaton");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Changeling],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
