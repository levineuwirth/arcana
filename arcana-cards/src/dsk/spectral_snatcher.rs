//! Spectral Snatcher — `{4}{B}{B}` 6/5 Creature — Spirit.
//!
//! Oracle:
//! * "Ward—Discard a card." — a NON-mana ward cost; not expressible
//!   (only `KeywordAbility::Ward(ManaCost)` exists). GAP.
//! * "Swampcycling {2}" — a typecycling/landcycling variant; per the
//!   cycling convention, emit the generic `KeywordAbility::Cycling`
//!   with the printed `{2}` cost (the type-search rider is the
//!   documented simplification).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP keyword: "Ward—Discard a card" is a non-mana ward cost, which has
//             no expressible KeywordAbility form.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spectral Snatcher");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
