//! Stormtide Leviathan — `{5}{U}{U}{U}` 8/8 Leviathan.
//! Islandwalk.
//! "All lands are Islands in addition to their other types." (static — GAP)
//! "Creatures without flying or islandwalk can't attack." (static — GAP)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stormtide Leviathan");
    let leviathan = reg.interner_mut().intern("Leviathan");
    let island = reg.interner_mut().intern("Island");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(leviathan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Landwalk(island)],
        ..Default::default()
    };

    // GAP: "All lands are Islands in addition to their other types." — static
    //      type-adding continuous ability over all lands.
    // GAP: "Creatures without flying or islandwalk can't attack." — static
    //      attack restriction over all creatures.
    reg.register(CardDefinition::new(name, chars))
}
