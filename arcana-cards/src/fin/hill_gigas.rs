//! Hill Gigas — `{4}{R}{R}` 5/4 Giant with Trample and Haste.
//! "Mountaincycling {2} ({2}, Discard this card: Search your library for a
//! Mountain card, reveal it, put it into your hand, then shuffle.)"
//!
//! Trample and Haste are keywords. Mountaincycling is a typecycling variant;
//! per the keyword rule it maps to the generic `KeywordAbility::Cycling`
//! with its printed `{2}` cost (the type-search refinement is not separately
//! modeled). The engine synthesizes the discard-this-card-to-draw ability.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hill Gigas");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Trample,
            KeywordAbility::Haste,
            KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
