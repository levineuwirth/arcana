//! Hexing Squelcher — `{1}{R}` 2/2 Goblin Sorcerer.
//! All text is static / non-mana Ward, none expressible:
//!   GAP: "This spell can't be countered."
//!   GAP: "Ward—Pay 2 life." (non-mana ward cost is unsupported → keywords vec empty)
//!   GAP: "Spells you control can't be countered." (global static)
//!   GAP: Other creatures you control have "Ward—Pay 2 life." (granted static)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hexing Squelcher");
    let goblin = reg.interner_mut().intern("Goblin");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(sorcerer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
