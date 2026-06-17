//! Helbrute — `{3}{B}{R}` 5/4 Artifact Creature — Astartes Dreadnought
//! (black/red) with Haste.
//!
//! * Haste (keyword line).
//! * "Sarcophagus — You may cast this card from your graveyard by exiling
//!   another creature card from your graveyard in addition to paying its
//!   other costs." — an alternative casting permission from the graveyard;
//!   not expressible via the triggered/activated ability surface. GAP.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Helbrute");
    let astartes = reg.interner_mut().intern("Astartes");
    let dreadnought = reg.interner_mut().intern("Dreadnought");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(astartes);
    subtypes.0.insert(dreadnought);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: Sarcophagus — cast from graveyard by exiling another creature
    // card (alternative casting permission not expressible).
    reg.register(CardDefinition::new(name, chars))
}
