//! Metallic Mimic — `{2}` 2/1 Artifact Creature — Shapeshifter.
//! "As this creature enters, choose a creature type."
//! "This creature is the chosen type in addition to its other types."
//! "Each other creature you control of the chosen type enters with an
//!  additional +1/+1 counter on it."
//!
//! All three lines hinge on a player-chosen creature type captured at ETB,
//! for which there is no Effect / choice primitive (no "choose a creature
//! type" effect, no chosen-type-keyed static / replacement). Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Metallic Mimic");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: "choose a creature type" ETB + chosen-type static type-add +
        // chosen-type ETB +1/+1 replacement — no choose-creature-type
        // primitive exists.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
