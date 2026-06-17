//! Daggermaw Megalodon — `{4}{U}{U}` 5/7 blue Shark with Vigilance.
//! "Islandcycling {2}" — a typecycling variant, emitted as generic Cycling {2}.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Daggermaw Megalodon");
    let shark = reg.interner_mut().intern("Shark");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shark);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(7)),
        // Islandcycling {2} is a typecycling variant; per convention emit generic
        // Cycling with the printed cost (the Island-search variant is not
        // separately modeled).
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
