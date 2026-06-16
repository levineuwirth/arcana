//! Eldrazi Aggressor — `{2}{R}` 2/3 colorless Eldrazi Drone (Devoid).
//!
//! Oracle:
//! * Devoid (this card has no color).
//! * This creature has haste as long as you control another colorless
//!   creature. (conditional static — GAP)
//!
//! Devoid is not a usable `KeywordAbility`; it is reflected by the colorless
//! color identity. The conditional-haste static isn't expressible.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eldrazi Aggressor");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: conditional static "has haste as long as you control another
    // colorless creature" is not expressible.
    reg.register(CardDefinition::new(name, chars))
}
