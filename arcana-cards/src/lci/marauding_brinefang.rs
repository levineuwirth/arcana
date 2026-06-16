//! Marauding Brinefang — `{5}{U}{U}` 6/7 Dinosaur.
//! "Ward {3}. Islandcycling {2}."
//!
//! Ward {3} → `KeywordAbility::Ward(ManaCost::parse("{3}"))`.
//! Islandcycling {2} is a landcycling variant; per engine convention the
//! type-search variant is not separately modeled, so it maps to the generic
//! `KeywordAbility::Cycling` with its printed `{2}` cost (the engine
//! synthesizes the discard-to-draw activation).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marauding Brinefang");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![
            KeywordAbility::Ward(ManaCost::parse("{3}").expect("valid cost")),
            KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
