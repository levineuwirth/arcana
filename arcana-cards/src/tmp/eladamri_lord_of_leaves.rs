//! Eladamri, Lord of Leaves — `{G}{G}` 2/2 Legendary Elf Warrior (green).
//!
//! Oracle (both lines are static continuous abilities, neither expressible on
//! this card class):
//! * "Other Elf creatures have forestwalk." — a static keyword-grant to a
//!   filtered set of other permanents. GAP'd (no static-grant primitive).
//! * "Other Elves have shroud." — likewise a static keyword grant. GAP'd.
//!
//! Bones (mana cost, colors, types, P/T, Legendary) are emitted faithfully.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eladamri, Lord of Leaves");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    // GAP: static — "Other Elf creatures have forestwalk."
    // GAP: static — "Other Elves have shroud."

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
