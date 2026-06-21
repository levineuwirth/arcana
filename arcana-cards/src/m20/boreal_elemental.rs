//! Boreal Elemental — `{4}{U}` 3/4 blue Elemental.
//!
//! Oracle:
//! * Flying
//! * Spells your opponents cast that target this creature cost {2} more to
//!   cast.
//!
//! Flying is a base keyword. The cost-increase line is a static cost-
//! modification ability (no trigger, no cost) and there is no cost-tax
//! primitive in the MultiAbilityCreature surface, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boreal Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    // GAP: static "Spells your opponents cast that target this creature cost
    // {2} more to cast." — no cost-increase / spell-tax primitive in the
    // demonstrated API surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
