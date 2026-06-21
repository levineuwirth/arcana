//! Titanic Bulvox — `{6}{G}{G}` 7/4 Beast with Trample.
//! Trample.
//! Morph {4}{G}{G}{G}.
//!
//! Trample is a base keyword. Morph is not a `KeywordAbility` variant and
//! has no demonstrated cast-face-down / turn-face-up primitive, so it is
//! GAP'd — only the Trample keyword and the bones are recorded.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Titanic Bulvox");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "Morph {4}{G}{G}{G}" — Morph is not a KeywordAbility variant and
    // has no cast-face-down / turn-face-up primitive in the demonstrated API.

    reg.register(CardDefinition::new(name, chars))
}
