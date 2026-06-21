//! Drift of Phantasms — `{2}{U}` 0/5 blue Spirit with Flying and Defender.
//! Transmute {1}{U}{U} lets you discard it to search for a card with the
//! same mana value. Transmute is not in the engine keyword surface, so it
//! is GAP'd; Flying and Defender are base keywords.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drift of Phantasms");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: Transmute {1}{U}{U} — Transmute is not in the usable keyword
    // surface and the tutor-by-same-mana-value variant is not expressible
    // via the activated-ability primitives.
    reg.register(CardDefinition::new(name, chars))
}
