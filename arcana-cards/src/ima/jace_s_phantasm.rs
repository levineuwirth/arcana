//! Jace's Phantasm — `{U}` 1/1 Illusion with Flying.
//! "This creature gets +4/+4 as long as an opponent has ten or more cards
//!  in their graveyard."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jace's Phantasm");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP: "This creature gets +4/+4 as long as an opponent has ten or more
    // cards in their graveyard." — a conditional static continuous P/T boost,
    // not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
