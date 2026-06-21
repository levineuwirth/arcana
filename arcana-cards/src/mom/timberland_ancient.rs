//! Timberland Ancient — `{4}{G}{G}` 6/5 Treefolk with Reach, Trample.
//!
//! Oracle:
//! * Reach, trample — keywords.
//! * Forestcycling {2} — a typecycling variant; per engine convention it is
//!   emitted as the generic `Cycling({2})` keyword (the engine synthesizes
//!   the discard-to-draw activation; the Forest-search variant is not
//!   separately modeled).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Timberland Ancient");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Reach,
            KeywordAbility::Trample,
            KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
