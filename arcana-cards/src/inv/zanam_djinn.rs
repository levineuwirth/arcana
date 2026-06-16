//! Zanam Djinn — `{5}{U}` 5/6 Djinn with Flying.
//! "This creature gets -2/-2 as long as blue is the most common color among all
//! permanents or is tied for most common."
//!
//! Flying is a base keyword. The conditional -2/-2 static is a continuous,
//! board-state-dependent self-modification with no expressible primitive in
//! this class — GAP'd (see doc comment); Flying is still emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zanam Djinn");
    let djinn = reg.interner_mut().intern("Djinn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "gets -2/-2 as long as blue is the most common color among all
    // permanents or tied" — a continuous most-common-color comparison has no
    // expressible primitive in this class.
    reg.register(CardDefinition::new(name, chars))
}
