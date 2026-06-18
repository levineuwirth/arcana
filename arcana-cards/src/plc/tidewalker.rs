//! Tidewalker — {2}{U} */* Elemental.
//! Enters with a time counter for each Island you control. Vanishing.
//! Power and toughness each equal the number of time counters on it.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tidewalker");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        // Vanishing has no printed count ("Vanishing"); but it actually uses
        // time counters seeded per-Island, modeled as Vanishing(1) fallback.
        keywords: vec![KeywordAbility::Vanishing(1)],
        ..Default::default()
    };

    // GAP: "enters with a time counter for each Island you control" — no
    //      enters-with-N-counters static rider expressible here.
    // GAP: "power and toughness equal the number of time counters" — */* is a
    //      CDA static; not expressible as a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
