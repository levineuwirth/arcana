//! Bladeback Sliver — `{1}{R}` 2/2 Sliver.
//! "Hellbent — As long as you have no cards in hand, Sliver creatures you control have '{T}: This creature deals 1 damage to target player or planeswalker.'"
//! GAP: "Hellbent — as long as you have no cards in hand" conditional static ability grant — no static layer effect supported for conditional grant-to-all-Slivers; emitting Vec::new().

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bladeback Sliver");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sliver);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // GAP: "Hellbent — grant activated ability to all Slivers you control" — static conditional grant not modeled
    reg.register(CardDefinition::new(name, chars))
}
