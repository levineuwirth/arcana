//! Paradise Druid — `{1}{G}` 2/1 Elf Druid.
//!
//! This creature has hexproof as long as it's untapped. (GAP — conditional
//! static continuous ability.)
//! {T}: Add one mana of any color. (GAP — `Effect::AddMana` takes a fixed
//! `ManaColor` per pip; there is no any-color choice primitive.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Paradise Druid");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "has hexproof as long as it's untapped" — conditional static; no
    // triggered/activated hook expresses it.
    // GAP: "{T}: Add one mana of any color" — `Effect::AddMana` takes a fixed
    // `ManaColor` per pip; there is no any-color choice primitive (see
    // Oasis Ritualist precedent).
    reg.register(CardDefinition::new(name, chars))
}
