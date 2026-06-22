//! Niblis of Dusk — `{2}{U}` 2/1 Creature — Spirit.
//! Flying.
//! Prowess — GAP: Prowess is not an expressible KeywordAbility variant.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Niblis of Dusk");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Prowess is not in the usable KeywordAbility surface; only
        // Flying is emitted.
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
