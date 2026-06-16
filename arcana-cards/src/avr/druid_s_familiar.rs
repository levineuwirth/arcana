//! Druid's Familiar — `{3}{G}` 2/2 Bear.
//!
//! Soulbond (You may pair this creature with another unpaired creature when
//! either enters. They remain paired for as long as you control both of them.)
//! As long as this creature is paired with another creature, each of those
//! creatures gets +2/+2.
//!
//! Soulbond is not in the usable keyword surface and the paired +2/+2 is a
//! static continuous ability gated on the soulbond pairing — neither is
//! expressible with the demonstrated API, so this card carries bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Druid's Familiar");
    let bear = reg.interner_mut().intern("Bear");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bear);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Soulbond keyword not in usable keyword surface.
        ..Default::default()
    };

    // GAP: static "while paired, each of those creatures gets +2/+2" — soulbond
    // pairing + conditional continuous buff not expressible.
    reg.register(CardDefinition::new(name, chars))
}
