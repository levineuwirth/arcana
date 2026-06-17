//! Thran War Machine — `{4}` 4/5 Artifact Creature — Construct.
//! "Echo {4}" (keyword not in the usable surface — GAP)
//! "This creature attacks each combat if able." (static — GAP)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thran War Machine");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Echo {4} is not in the usable KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static — "This creature attacks each combat if able" is a continuous
    // attack-requirement, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
