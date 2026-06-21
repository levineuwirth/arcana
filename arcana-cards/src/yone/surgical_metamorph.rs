//! Surgical Metamorph — `{3}{U}` 0/0 Artifact Creature — Phyrexian Shapeshifter.
//! "This spell costs {1} less to cast if you weren't the starting player." → GAP
//! (cost-reduction static).
//! "You may have Surgical Metamorph enter as a copy of any permanent on the
//! battlefield, except it's an artifact in addition to its other types." → GAP
//! (enter-as-a-copy clone replacement; no Effect variant).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Surgical Metamorph");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(shapeshifter);

    // GAP: cost-reduction static and enter-as-a-copy clone replacement.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
