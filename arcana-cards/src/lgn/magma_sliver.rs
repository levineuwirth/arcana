//! Magma Sliver — `{3}{R}` 3/3 red Sliver.
//! "All Slivers have '{T}: Target Sliver creature gets +X/+0 until end of
//! turn, where X is the number of Slivers on the battlefield.'"
//!
//! GAP: "all Slivers have" — static ability granting abilities to other
//! permanents not expressible in this shape.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Magma Sliver");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sliver);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    // GAP: "all Slivers have" static ability granting not expressible
    reg.register(CardDefinition::new(name, chars))
}
